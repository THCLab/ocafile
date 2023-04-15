use std::fs;

use clap::Parser as ClapParser;
use clap::Subcommand;
use ocafile::ocafile::parse_from_string;
use serde_json;

#[macro_use]
extern crate log;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        file: Option<String>,
    },
    Publish {
        #[arg(short, long)]
        repository: String,
    },
    Sign {
        #[arg(short, long)]
        scid: String,
    },

}



/// TODO extract OCAFILE lib to seperate crate

fn main() {
    env_logger::init();

    let args = Args::parse();


    match &args.command {
        Some(Commands::Build { file }) => {
            info!("Building OCA bundle from oca file");

            let unparsed_file = match file {
                Some(file) => fs::read_to_string(file).expect("Can't read file"),
                None => fs::read_to_string("OCAfile").expect("Can't read file"),
            };

            let mut oca = parse_from_string(unparsed_file);
            println!("{:#?}", oca);
            //let oca_bundle = oca.generate_bundle();
            //let serialized_oca = serde_json::to_string_pretty(&oca_bundle).unwrap();

            //let said = oca_bundle.said.to_string();
            //save to file
            //fs::write(said + ".ocabundle", serialized_oca).expect("Unable to write file");

        }
        Some(Commands::Publish { repository: _ }) => {
            info!("Publish OCA bundle to repository")
        }
        Some(Commands::Sign { scid: _ }) => {
            info!("Sign OCA bundle byc SCID")
        }
        None => {}
    }

    println!("DONE");
}

// ocafile build -i OCAfile
// ocafile build -s scid
// ocafile publish
// ocafile fetch SAI
// ocafile inspect