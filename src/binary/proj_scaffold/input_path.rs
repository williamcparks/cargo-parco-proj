use std::{
    convert::Infallible,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::binary::proj_scaffold::ProjScaffoldError;

#[derive(Clone)]
pub struct InputPath {
    value: PathBuf,
}

impl InputPath {
    pub fn try_proj(root: &Path) -> Result<PathBuf, ProjScaffoldError> {
        for _ in 0..3 {
            let path = inquire::CustomType::<InputPath>::new("Proj Path?")
                .with_default(InputPath {
                    value: PathBuf::from("proj"),
                })
                .prompt()?;
            let full_path = root.join(path.value);

            let msg = format!("Is `{}` The Correct Full Path?", full_path.display());
            if inquire::Confirm::new(msg.as_str()).prompt()? {
                return Ok(full_path);
            }
        }

        Err(ProjScaffoldError::TooManyTries)
    }
}

impl Display for InputPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.display())
    }
}

impl FromStr for InputPath {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { value: s.parse()? })
    }
}
