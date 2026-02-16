use crate::io::{Context, Grids, PuzWrite, write};

impl Grids {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer
            .write_all(self.solution.data())
            .context("Solution grid")?;

        writer.write_all(self.state.data()).context("State grid")?;

        Ok(())
    }
}
