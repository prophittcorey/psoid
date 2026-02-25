# PSO Section ID Calculator

A rust library for calculating section IDs in Phantasy Star Online. 

Supports:
- **V1 and V2** - Original algorithm based on ASCII character values
- **Blue Burst** - Enhanced algorithm with character class offsets

### V1 and V2 Calculation

```rust
use psoid::{calculate, GameVersion};

fn main() {
    let guild = calculate("foobar", GameVersion::V1, None).unwrap();
    
    println!("Guild: {}", guild.name());            // "Bluefull"
    println!("ID: {}", guild.id());                 // 3
    println!("Best Class: {}", guild.best_class()); // "Hunter"
}
```

### Blue Burst Calculation

```rust
use psoid::{calculate, GameVersion, CharacterClass};

fn main() {
    let guild = calculate(
        "PSO Lover",
        GameVersion::BlueBurst,
        Some(CharacterClass::RAmar),
    ).unwrap();
    
    println!("Guild: {}", guild.name());    // "Skyly"
    println!("ID: {}", guild.id());         // 2
}
```

### CLI Example

A command-line tool is included in `examples/pso-calc.rs`:

```bash
# V1 calculation
cargo run --example pso-calc -- "foobar" v1

# V2 calculation
cargo run --example pso-calc -- "foobar" v2

# Blue Burst with class
cargo run --example pso-calc -- "PSO Lover" blueburst RAmar
```

### Drop Rate Tables

Each guild has different weapon drop rates. All guilds have 13% drops for Sabers, Handguns, and Canes.

| Section ID | Common Drop | Rare Drop | MAG |
|-------|-------------|-----------|-----|
| Viridia | Partisans (10%) | Slicers (1%) | A |
| Greennill | Rifles (13%) | Swords (1%) | A |
| Skyly | Swords (13%) | Machineguns (1%) | A |
| Bluefull | Partisans (13%) | Wands (1%) | B |
| Purplenum | Machineguns (13%) | Daggers (10%) | B |
| Pinkal | Wands (13%) | Rifles (1%) | B |
| Redria | Slicers (10%) | Daggers (1%) | C |
| Oran | Daggers (13%) | Rods (1%) | C |
| Yellowboze | All Equal (7%) | All Equal (7%) | C |
| Whitill | Machineguns (10%) | Shotguns (1%) | D |

For complete drop rate details, use `guild.drop_rates()`:

```rust
let guild = calculate("foobar", GameVersion::V1, None)?;
let rates = guild.drop_rates();

println!("Partisans: {}%", rates.partisans);
println!("Wands: {}%", rates.wands);
// ... and 10 other weapon types
```
