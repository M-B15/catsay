use std::io::{self, Read};
use anyhow::{Context, Result};
use colored::Colorize;
use clap::Parser;

#[derive(Parser)]
struct Options {

    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String, // [1]

    #[clap(short = 's', long = "sleeping")]
    /// Make the cat sleep
    sleeping: bool, // [2]

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>, // [3]
    
    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool, // [4]

}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    
    let eye = if options.sleeping { "-" } else { "o" }; //[1]

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog."); 
    }
    
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(
                || format!("Could not read file {:?}", path)
            )?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!(
                "{}",
                message.bright_yellow().underline().on_purple()
            );
            println!("{}", cat_picture);
        },
        None => {
            println!(
                "{}",
                message.bright_yellow().underline().on_purple()
            );
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye}.{eye} )", eye=eye.red().bold()); // [2]
            println!("    =(_I_)= ");
        }
    }

    Ok (())
}

