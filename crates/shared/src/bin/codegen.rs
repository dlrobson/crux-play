use std::path::PathBuf;

use clap::Parser;
use crux_core::type_generation::facet::{Config, TypeRegistry};
use log::info;
use uniffi::deps::anyhow::Result;

use shared::Counter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let typegen_app = TypeRegistry::new().register_app::<Counter>()?.build()?;

    let name = "app";
    let config = Config::builder(name, &args.output_dir)
        .add_extensions()
        .add_runtimes()
        .build();

    info!("Typegen for TypeScript");
    typegen_app.typescript(&config)?;

    Ok(())
}
