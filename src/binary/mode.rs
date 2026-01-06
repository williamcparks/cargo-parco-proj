use std::fmt::Display;

use inquire::InquireError;

pub enum Mode {
    ProjScaffold,
}

impl Mode {
    pub fn try_new() -> Result<Self, InquireError> {
        inquire::Select::new("What Are You Looking To Do?", vec![Self::ProjScaffold]).prompt()
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ProjScaffold => "Proj Scaffold",
        };
        f.write_str(msg)
    }
}
