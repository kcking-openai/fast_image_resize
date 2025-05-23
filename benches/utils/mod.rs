use std::env;
use std::path::PathBuf;
use std::process::Command;

pub use bencher::*;
pub use resize_functions::*;
pub use results::*;
use serde::Deserialize;

mod bencher;
mod resize_functions;
mod results;
pub mod testing;

const fn get_arch_id_and_name() -> (&'static str, &'static str) {
    #[cfg(target_arch = "x86_64")]
    return ("x86_64", "x86_64");
    #[cfg(target_arch = "aarch64")]
    return ("arm64", "arm64");
    #[cfg(target_arch = "wasm32")]
    return ("wasm32", "Wasm32");
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "wasm32"
    )))]
    return ("unknown", "Unknown");
}

/// Returns the Cargo target directory, possibly calling `cargo metadata` to
/// figure it out.
fn cargo_target_directory() -> Option<PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        target_directory: PathBuf,
    }

    env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let output = Command::new(env::var_os("CARGO")?)
                .args(["metadata", "--format-version", "1"])
                .output()
                .ok()?;
            let metadata: Metadata = serde_json::from_slice(&output.stdout).ok()?;
            Some(metadata.target_directory)
        })
}
