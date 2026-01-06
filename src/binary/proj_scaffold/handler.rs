use crate::binary::proj_scaffold::{
    ProjScaffoldError,
    error::io,
    input_path::InputPath,
    utils::{Template, relative_proj_path},
};

pub fn handler() -> Result<(), ProjScaffoldError> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    let root = metadata.workspace_root.as_std_path();
    let proj_macros_dir = InputPath::try_proj(root)?;
    let proj_file_path = root.join("Proj.toml");
    let relative_proj_path =
        relative_proj_path(proj_macros_dir.as_path(), proj_file_path.as_path());

    io!(
        std::fs::create_dir(proj_macros_dir.as_path()),
        proj_macros_dir
    );

    let src_dir = proj_macros_dir.join("src");
    io!(std::fs::create_dir(src_dir.as_path()), src_dir);

    let cargo_toml = proj_macros_dir.join("Cargo.toml");
    let contents = Template::CargoToml.fill(relative_proj_path.display().to_string());
    io!(std::fs::write(cargo_toml.as_path(), contents), cargo_toml);

    let lib_rs = src_dir.join("lib.rs");
    let contents = Template::LibRs.fill(relative_proj_path.display().to_string());
    io!(std::fs::write(lib_rs.as_path(), contents), lib_rs);

    if !io!(proj_file_path.try_exists(), proj_file_path) {
        let msg = format!(
            "Would You Like To Setup A Proj File @ {}",
            proj_file_path.display()
        );
        if inquire::Confirm::new(msg.as_str()).prompt()? {
            io!(
                std::fs::write(proj_file_path.as_path(), Template::PROJ_TEMPLATE),
                proj_file_path
            );
        }
    }

    Ok(())
}
