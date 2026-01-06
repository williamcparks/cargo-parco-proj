use std::path::PathBuf;

use inquire::InquireError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ProjScaffoldError {
    Metadata(#[from] cargo_metadata::Error),

    Inquire(#[from] InquireError),

    #[error("Failed To Locate Correct `proj` Path. Exiting...")]
    TooManyTries,

    #[error("{source} - {}", path.display())]
    PathIo {
        source: std::io::Error,
        path: PathBuf,
    },
}

macro_rules! io {
    ($result: expr, $path: expr) => {
        match $result {
            Ok(ok) => ok,
            Err(err) => {
                return Err(ProjScaffoldError::PathIo {
                    source: err,
                    path: $path,
                });
            }
        }
    };
}

pub(super) use io;
