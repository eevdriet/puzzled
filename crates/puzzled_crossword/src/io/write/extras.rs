use crate::io::{Context, Extras, PuzWrite, write};

impl Extras {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        if let Some(grbs) = &self.grbs {
            writer.write_all(b"GRBS").context("GRBS header")?;

            for (pos, &byte) in grbs.iter_indexed() {
                let context = format!("Square {pos}");
                writer.write_u8(byte).context(context)?;
            }
        }

        if let Some(rtbl) = &self.rtbl {
            writer.write_all(b"RTBL").context("RTBL header")?;

            for (num, rebus) in rtbl {
                let key = format!("{num:02}:{rebus};");
                let context = format!("Rebus #{num}");

                writer.write_all(key.as_bytes()).context(context)?;
            }

            writer.write_u8(0).context("RTBL EOF bit")?;
        }

        if let Some(ltim) = &self.ltim {
            writer.write_all(b"LTIM").context("LTIM header")?;

            let secs = ltim.elapsed().as_secs();
            let state: u8 = ltim.state().into();

            let format = format!("{secs},{state}");
            writer.write_str0(&format).context("LTIM")?;
        }

        if let Some(gext) = &self.gext {
            writer.write_all(b"GEXT").context("GEXT header")?;

            for (pos, &style) in gext.iter_indexed() {
                let context = format!("Cell {pos} style");
                writer.write_u8(style.bits()).context(context)?;
            }
        }

        Ok(())
    }
}
