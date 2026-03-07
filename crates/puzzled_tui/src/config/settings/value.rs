use concat_idents::concat_idents;
use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize, Serializer, de::Visitor, ser};

use crate::{Action, ActionResolver, HandleAction};

#[derive(Debug)]
pub enum SettingValue {
    Bool(bool),
    Int {
        value: isize,
        min: isize,
        max: isize,
        step: isize,
    },
    Float {
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    },
    List {
        index: usize,
        options: Vec<String>,
    },
}

#[derive(Default)]
struct SettingsSerializer {
    pending_key: Option<String>,
    pending_value: Option<SettingValue>,

    settings: HashMap<String, SettingValue>,
}

macro_rules! int_setting {
    ($($ty:ident)+) => {
        $(
            concat_idents!(fn_name = serialize_, $ty {
                fn fn_name(self, v: $ty) -> Result<Self::Ok, Self::Error> {
                    self.pending_value = Some(SettingValue::Int {
                        value: v as isize,
                        min: $ty::MIN as isize,
                        max: $ty::MAX as isize,
                        step: 1,
                    });

                    Ok(())
                }
            });
        )+
    };
}

pub fn to_settings<T>(value: &T) -> Result<HashMap<String, SettingValue>, fmt::Error>
where
    T: Serialize,
{
    let mut serializer = SettingsSerializer::default();
    value.serialize(&mut serializer)?;

    Ok(serializer.settings)
}

impl Serializer for &mut SettingsSerializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = fmt::Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Numerical settings
    int_setting!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.pending_value = Some(SettingValue::Float {
            value: value as f64,
            min: f32::MIN as f64,
            max: f32::MAX as f64,
            step: 0.1,
        });

        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.pending_value = Some(SettingValue::Float {
            value,
            min: f64::MIN,
            max: f64::MAX,
            step: 0.1,
        });

        Ok(())
    }

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.pending_value = Some(SettingValue::Bool(value));

        Ok(())
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.pending_key = Some(value.to_string());

        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self)
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl ser::SerializeSeq for &mut SettingsSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = fmt::Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Same thing but for tuples.
impl ser::SerializeTuple for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl ser::SerializeTupleStruct for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl ser::SerializeTupleVariant for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl ser::SerializeMap for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;

        if let (Some(key), Some(val)) = (self.pending_key.take(), self.pending_value.take()) {
            self.settings.insert(key, val);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl ser::SerializeStruct for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;

        if let (Some(key), Some(val)) = (self.pending_key.take(), self.pending_value.take()) {
            self.settings.insert(key, val);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl ser::SerializeStructVariant for &mut SettingsSerializer {
    type Ok = ();
    type Error = fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;

        if let (Some(key), Some(val)) = (self.pending_key.take(), self.pending_value.take()) {
            self.settings.insert(key, val);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'de> Deserialize<'de> for SettingValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SettingValueVisitor;

        impl<'de> Visitor<'de> for SettingValueVisitor {
            type Value = SettingValue;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any primitive value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SettingValue::Bool(value))
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SettingValue::Int {
                    value: value as isize,
                    min: 0,
                    max: u8::MAX as isize,
                    step: 1,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut options = Vec::new();

                while let Some(option) = seq.next_element()? {
                    options.push(option);
                }

                Ok(SettingValue::List { index: 0, options })
            }
        }

        deserializer.deserialize_any(SettingValueVisitor)
    }
}

impl<A, T> HandleAction<A, T> for SettingValue {
    type State = ();

    fn on_action(
        &mut self,
        action: Action<A>,
        _resolver: ActionResolver<A, T>,
        _state: &mut Self::State,
    ) {
        match self {
            SettingValue::Bool(b) => match action {
                Action::MoveLeft(_) | Action::MoveRight(_) => *b = !*b,
                _ => {}
            },
            SettingValue::Int {
                value,
                min,
                max,
                step,
            } => {
                *value = match action {
                    Action::MoveLeft(_) => (*value - *step).max(*min),
                    Action::MoveRight(_) => (*value + *step).min(*max),
                    _ => *value,
                }
            }
            SettingValue::Float {
                value,
                min,
                max,
                step,
            } => {
                *value = match action {
                    Action::MoveLeft(_) => (*value - *step).max(*min),
                    Action::MoveRight(_) => (*value + *step).min(*max),
                    _ => *value,
                }
            }
            SettingValue::List { index, options } => {
                let len = options.len();

                *index = match action {
                    Action::MoveLeft(_) => (*index + len - 1) % len,
                    Action::MoveRight(_) => (*index + 1) % len,
                    _ => *index,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::to_settings;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        enum Choice {
            Yes,
            No,
        }

        #[derive(Serialize)]
        struct MySettings {
            choice: Choice,
            a: u16,
            b: u16,
        }

        let settings = MySettings {
            a: 4,
            b: 4,
            choice: Choice::Yes,
        };

        let map = to_settings(&settings).expect("Converted correctly");
        println!("Settings: {map:?}");
        panic!("Manual");
    }
}
