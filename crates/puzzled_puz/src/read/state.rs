use crate::{Warning, read};

#[derive(Debug, Default)]
pub struct PuzState {
    strict: bool,
    pub warnings: Vec<Warning>,
}

impl PuzState {
    pub(crate) fn new(strict: bool) -> Self {
        Self {
            strict,
            warnings: Vec::new(),
        }
    }

    pub(crate) fn ok_or_warn<T>(&mut self, result: read::Result<T>) -> read::Result<Option<T>> {
        match result {
            // Pass through ok/err with strict mode normally
            Ok(val) => Ok(Some(val)),
            Err(err) if self.strict => Err(err),

            // Warn against errors in non-strict mode
            Err(warning) => {
                self.warnings.push(warning);

                Ok(None)
            }
        }
    }
}
