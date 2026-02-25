use psoid::{CharacterClass, GameVersion, calculate, calculate_with_version};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <character_name> [version] [class]", args[0]);
        eprintln!();
        eprintln!("Versions: v1v2 (default), blueburst");
        eprintln!();
        eprintln!("BlueBurst classes:");
        eprintln!("  HUmar, HUnewearl, HUcast, HUcaseal");
        eprintln!("  RAmar, RAmarl, RAcast, RAcaseal");
        eprintln!("  FOmar, FOmarl, FOnewm, FOnewearl");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} foobar", args[0]);
        eprintln!("  {} \"PSO Lover\" v1v2", args[0]);
        eprintln!("  {} \"PSO Lover\" blueburst RAmar", args[0]);
        std::process::exit(1);
    }

    let name = &args[1];
    let version_str = args.get(2).map(|s| s.as_str()).unwrap_or("v1v2");
    let class_str = args.get(3).map(|s| s.as_str());

    // Parse version
    let version = match version_str.to_lowercase().as_str() {
        "v1v2" | "v1" | "v2" => GameVersion::V1V2,
        "blueburst" | "bb" => GameVersion::BlueBurst,
        _ => {
            eprintln!("Unknown version: {}", version_str);
            std::process::exit(1);
        }
    };

    // Calculate section ID
    let result = match version {
        GameVersion::V1V2 => calculate(name),
        GameVersion::BlueBurst => {
            let class = match class_str {
                Some("HUmar") => CharacterClass::HUmar,
                Some("HUnewearl") => CharacterClass::HUnewearl,
                Some("HUcast") => CharacterClass::HUcast,
                Some("HUcaseal") => CharacterClass::HUcaseal,
                Some("RAmar") => CharacterClass::RAmar,
                Some("RAmarl") => CharacterClass::RAmarl,
                Some("RAcast") => CharacterClass::RAcast,
                Some("RAcaseal") => CharacterClass::RAcaseal,
                Some("FOmar") => CharacterClass::FOmar,
                Some("FOmarl") => CharacterClass::FOmarl,
                Some("FOnewm") => CharacterClass::FOnewm,
                Some("FOnewearl") => CharacterClass::FOnewearl,
                Some(class) => {
                    eprintln!("Unknown class: {}", class);
                    std::process::exit(1);
                }
                None => {
                    eprintln!("BlueBurst requires a character class");
                    std::process::exit(1);
                }
            };
            calculate_with_version(name, GameVersion::BlueBurst, class)
        }
    };

    match result {
        Ok(guild) => {
            println!("Character Name: {}", name);
            println!(
                "Game Version  : {}",
                match version {
                    GameVersion::V1V2 => "V1/V2",
                    GameVersion::BlueBurst => "BlueBurst",
                }
            );
            if version == GameVersion::BlueBurst {
                println!("Class         : {}", class_str.unwrap_or("N/A"));
            }
            println!();

            // Display the full summary using the Display trait
            println!("{}", guild);
            println!();

            // Get drop rates directly
            let rates = guild.drop_rates();
            println!("Detailed Drop Rates:");
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
