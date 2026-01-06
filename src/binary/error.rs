use inquire::InquireError;
use thiserror::Error;

use crate::binary::proj_scaffold::ProjScaffoldError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum BinaryError {
    Inquire(#[from] InquireError),

    ProjScaffold(ProjScaffoldError),
}
