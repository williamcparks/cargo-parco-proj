use std::path::{Path, PathBuf};

use pathdiff::diff_paths;

pub fn relative_proj_path(proj_macros_dir: &Path, proj_file_path: &Path) -> PathBuf {
    match diff_paths(proj_file_path, proj_macros_dir) {
        Some(some) => Path::new("../").join(some),
        None => proj_file_path.to_path_buf(),
    }
}

pub enum Template {
    CargoToml,
    LibRs,
}

impl Template {
    pub const PROJ_TEMPLATE: &str = include_str!("../../../assets/Proj.template");
    pub const PARCO_PROJ_VERSION: &str = env!("CARGO_PKG_VERSION");

    pub fn fill(self, proj_path: String) -> String {
        let contents = match self {
            Self::CargoToml => include_str!("../../../assets/Cargo.template"),
            Self::LibRs => include_str!("../../../assets/src/lib.rs"),
        };

        contents
            .replace("{{proj_toml_path}}", proj_path.as_str())
            .replace("{{PARCO_PROJ_VERSION}}", Self::PARCO_PROJ_VERSION)
    }
}
