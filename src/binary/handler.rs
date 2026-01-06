use crate::binary::{error::BinaryError, mode::Mode, proj_scaffold};

pub fn handler() -> Result<(), BinaryError> {
    let mode = Mode::try_new()?;

    match mode {
        Mode::ProjScaffold => proj_scaffold::handler().map_err(BinaryError::ProjScaffold),
    }
}
