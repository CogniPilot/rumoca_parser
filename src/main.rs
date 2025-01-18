use clap::Parser;
use rumoca_parser::{PrintVisitor, Visitable};

#[derive(Parser, Debug)]
#[command(version, about = "Rumoca Modelica Parser", long_about = None)]
struct Args {
    /// The model file to compile
    #[arg(name = "MODELICA_FILE")]
    model_file: String,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let def = rumoca_parser::parse_file(&args.model_file);

    if args.verbose {
        println!("{:#?}", def);
    }

    let mut visitor = PrintVisitor::default();
    def.accept(&mut visitor);
    Ok(())
}
