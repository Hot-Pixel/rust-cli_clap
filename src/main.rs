use clap::Parser;
//use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow", short = 'm', long = "message")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();

    let message = options.message;
    let eye =  if options.dead { "x" } else { "o" };
    
    if message.to_lowercase().contains("woof") {
        eprintln!("Cats do not woof");
    }

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )");
    println!("    =( I )=");
}
