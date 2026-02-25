use psoid::calculate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <character_name>", args[0]);
        eprintln!("Example: {} foobar", args[0]);
        std::process::exit(1);
    }

    let name = &args[1];

    match calculate(name) {
        Ok(guild) => {
            println!("Character Name: {}", name);
            println!();

            // Display the full summary using the Display trait
            println!("{}", guild);
            println!();

            // Get drop rates directly
            let rates = guild.drop_rates();
            println!("\nDetailed Drop Rates:");
            println!("  Sabers      : {}%", rates.sabers);
            println!("  Swords      : {}%", rates.swords);
            println!("  Daggers     : {}%", rates.daggers);
            println!("  Partisans   : {}%", rates.partisans);
            println!("  Slicers     : {}%", rates.slicers);
            println!("  Handguns    : {}%", rates.handguns);
            println!("  Rifles      : {}%", rates.rifles);
            println!("  Machineguns : {}%", rates.machineguns);
            println!("  Shotguns    : {}%", rates.shotguns);
            println!("  Canes       : {}%", rates.canes);
            println!("  Rods        : {}%", rates.rods);
            println!("  Wands       : {}%", rates.wands);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
