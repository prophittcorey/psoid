use psoid::{CharacterClass, GameVersion, calculate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <character_name> [version] [class]", args[0]);
        eprintln!();
        eprintln!("Versions: v1, v2, blueburst");
        eprintln!();
        eprintln!("BlueBurst classes:");
        eprintln!("  HUmar, HUnewearl, HUcast, HUcaseal");
        eprintln!("  RAmar, RAmarl, RAcast, RAcaseal");
        eprintln!("  FOmar, FOmarl, FOnewm, FOnewearl");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} foobar", args[0]);
        eprintln!("  {} \"PSO Player\" v1", args[0]);
        eprintln!("  {} \"PSO Player\" blueburst RAmar", args[0]);
        std::process::exit(1);
    }

    let name = &args[1];
    let version_str = args.get(2).map(|s| s.as_str()).unwrap_or("v1v2");
    let class_str = args.get(3).map(|s| s.as_str());

    let version = match version_str.to_lowercase().as_str() {
        "v1" | "1" => GameVersion::V1,
        "v2" | "2" => GameVersion::V2,
        "blueburst" | "bb" => GameVersion::BlueBurst,
        _ => {
            eprintln!("Unknown version: {}", version_str);
            std::process::exit(1);
        }
    };

    let character_class = class_str.map(|class| match class {
        "HUmar" => CharacterClass::HUmar,
        "HUnewearl" => CharacterClass::HUnewearl,
        "HUcast" => CharacterClass::HUcast,
        "HUcaseal" => CharacterClass::HUcaseal,
        "RAmar" => CharacterClass::RAmar,
        "RAmarl" => CharacterClass::RAmarl,
        "RAcast" => CharacterClass::RAcast,
        "RAcaseal" => CharacterClass::RAcaseal,
        "FOmar" => CharacterClass::FOmar,
        "FOmarl" => CharacterClass::FOmarl,
        "FOnewm" => CharacterClass::FOnewm,
        "FOnewearl" => CharacterClass::FOnewearl,
        class => {
            eprintln!("Unknown class: {}", class);
            std::process::exit(1);
        }
    });

    match calculate(name, version, character_class) {
        Ok(guild) => {
            println!("Character Name: {}", name);
            println!(
                "Game Version  : {}",
                match version {
                    GameVersion::V1 => "V1",
                    GameVersion::V2 => "V2",
                    GameVersion::BlueBurst => "BlueBurst",
                }
            );
            if version == GameVersion::BlueBurst {
                println!("Class         : {}", class_str.unwrap_or("N/A"));
            }

            println!();
            println!("{}", guild);
            println!();

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
