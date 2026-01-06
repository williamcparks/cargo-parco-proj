use std::process::ExitCode;

#[cfg(feature = "binary")]
mod binary;

#[cfg(feature = "binary")]
fn main() -> ExitCode {
    match binary::handler() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(feature = "binary"))]
fn main() -> ExitCode {
    eprintln!("You Must Enable The Binary Feature To Use Parco-Proj As A Binary");
    ExitCode::FAILURE
}
