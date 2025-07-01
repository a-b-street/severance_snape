use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to a .osm.pbf or .xml file to convert
    #[arg(long)]
    input: String,

    /// Map model output file to write
    #[arg(long)]
    output: String,
}

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();

    let map = backend::MapModel::create(&fs_err::read(&args.input)?, backend::Profile::USA)?;
    let writer = std::io::BufWriter::new(fs_err::File::create(&args.output)?);
    bincode::serialize_into(writer, &map)?;
    log::info!("Wrote {}", args.output);

    Ok(())
}
