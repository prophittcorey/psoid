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
        Ok(section_id) => {
            println!("Character Name: {}", name);
            println!("Section ID   : {}", section_id.id());
            println!("Guild        : {}", section_id.name());
            println!("Good for     : {}", section_id.best_class());
            println!(
                "Common drops : {} ({}%)",
                section_id.common_drop().0,
                section_id.common_drop().1
            );
            println!(
                "Rare drops   : {} ({}%)",
                section_id.rare_drop().0,
                section_id.rare_drop().1
            );
            println!("MAG type     : {}", section_id.mag_type());

            // Get drop rates directly
            let rates = section_id.drop_rates();
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
