use anyhow::{Result, bail};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to a .osm.pbf or .xml file to convert
    #[arg(long)]
    input: String,

    /// Optional path to a .geotiff file in WGS84 with height in meters
    #[arg(long)]
    elevation: Option<String>,

    /// Map model output file to write
    #[arg(long)]
    output: String,
}

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();

    let mut map = backend::MapModel::create(&fs_err::read(&args.input)?, backend::Profile::USA)?;
    if let Some(path) = &args.elevation {
        map.set_gradients(read_gradients(path, map.get_graph())?);
    }

    let writer = std::io::BufWriter::new(fs_err::File::create(&args.output)?);
    bincode::serialize_into(writer, &map)?;
    log::info!("Wrote {}", args.output);

    Ok(())
}

fn read_gradients(path: &str, graph: &graph::Graph) -> Result<Vec<f64>> {
    log::info!("Reading elevation data from {path}");
    let mut geotiff =
        elevation::GeoTiffElevation::new(std::io::BufReader::new(fs_err::File::open(path)?));
    let mut gradients = Vec::new();
    for road in &graph.roads {
        // TODO This only checks the start and end point
        let pt1 = graph
            .mercator
            .pt_to_wgs84(*road.linestring.coords().next().unwrap());
        let pt2 = graph
            .mercator
            .pt_to_wgs84(*road.linestring.coords().last().unwrap());

        let Some(height1) = geotiff.get_height_for_lon_lat(pt1.x as f32, pt1.y as f32) else {
            bail!("Couldn't get height for {pt1:?}");
        };
        let Some(height2) = geotiff.get_height_for_lon_lat(pt2.x as f32, pt2.y as f32) else {
            bail!("Couldn't get height for {pt2:?}");
        };

        let slope = (height2 - height1) / (road.length_meters as f32) * 100.0;
        // TODO Check it's in a reasonable range
        gradients.push(slope.into());
    }
    Ok(gradients)
}
