//! Build task runner for generating web artifacts.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use crux_core::type_generation::facet::{Config, TypeRegistry};
use shared::Counter;
use xshell::{Shell, cmd};

fn main() -> Result<()> {
    typegen()?;
    wasm()
}

/// Generate TypeScript types into `web/generated/types`.
fn typegen() -> Result<()> {
    let root = project_root()?;
    let output_dir = root.join("web/generated/types");

    let typegen_app = TypeRegistry::new()
        .register_app::<Counter>()
        .context("failed to register Counter app")?
        .build()
        .context("failed to build type registry")?;

    let config = Config::builder("app", &output_dir)
        .add_extensions()
        .add_runtimes()
        .build();

    typegen_app
        .typescript(&config)
        .context("TypeScript type generation failed")
}

/// Build the shared crate as a WASM package into `web/generated/pkg`.
fn wasm() -> Result<()> {
    let sh = Shell::new()?;
    let root = project_root()?;
    let out_dir = root.join("web/generated/pkg");
    let crate_path = root.join("crates/shared");

    cmd!(
        sh,
        "wasm-pack build --target web --out-dir {out_dir} {crate_path} --features wasm_bindgen"
    )
    .run()
    .context("wasm-pack build failed")
}

/// Resolve the workspace root from the xtask crate's manifest directory.
fn project_root() -> Result<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .context("xtask manifest dir has no parent")
        .map(Path::to_path_buf)
}
