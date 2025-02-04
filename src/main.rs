use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Rumoca Modelica Parser", long_about = None)]
struct Args {
    /// The model file to compile
    #[arg(name = "MODELICA_FILE")]
    model_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let def = rumoca_parser::parse_file(&args.model_file);
    println!("{:#?}", def);
    Ok(())
}
