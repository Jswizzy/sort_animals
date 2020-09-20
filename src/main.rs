use animal::{convert_animals_to_types, sort_by_color, sort_by_weight, Animal, AnimalDAO};
use structopt::StructOpt;

// Struct Opts here handles CLI argument parsing and messages
/// Given a JSON file of animals parse it into a heterogeneous array
///
/// format of input file: [{"type": "dog", "weight": 17, "color": "white"}, {"type": "cat", "weight: 30, "color": "black"}]
/// example of output: ["Dog": {"weight": 17, "color": "white"}, "Cat": {"weight": 30, "color": "black"}]
#[derive(StructOpt)]
#[structopt(name = "🐶 Sort Animals 🗃 🙈")]
struct Cli {
    /// sort by color
    #[structopt(short, long)]
    color: bool,

    /// sort by weight
    #[structopt(short, long)]
    weight: bool,

    /// reverse the sort order
    #[structopt(short, long)]
    reverse: bool,

    /// The path to the file to parse
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    // read in json file
    // better to use a buffer here but I am keeping this simple
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let deserialized: Vec<AnimalDAO> = serde_json::from_str(&content)?;

    // Convert DAO to Animal types such as Cat or Dog
    let mut animals: Vec<Box<dyn Animal>> = convert_animals_to_types(&deserialized);

    // apply command line flags for sorting
    if args.color {
        animals.sort_by(sort_by_color);
    }

    if args.weight {
        animals.sort_by(sort_by_weight);
    }

    if args.reverse {
        animals.reverse()
    }

    // convert array to JSON string
    let serialized = serde_json::to_string_pretty(&animals)?;

    // write JSON to stdout
    println!("{}", serialized);

    Ok(())
}
