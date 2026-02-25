//! Phantasy Star Online Section ID Calculator
//!
//! This library calculates the Section ID for a Phantasy Star Online character
//! based on their name. The Section ID determines which guild the character
//! belongs to, which affects drop rates and MAG types.

use std::fmt;

/// Represents a guild in Phantasy Star Online
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guild {
    Viridia,
    Greennill,
    Skyly,
    Bluefull,
    Purplenum,
    Pinkal,
    Redria,
    Oran,
    Yellowboze,
    Whitill,
}

impl Guild {
    /// Get the numeric ID for this guild (0-9)
    pub fn id(&self) -> u32 {
        match self {
            Guild::Viridia => 0,
            Guild::Greennill => 1,
            Guild::Skyly => 2,
            Guild::Bluefull => 3,
            Guild::Purplenum => 4,
            Guild::Pinkal => 5,
            Guild::Redria => 6,
            Guild::Oran => 7,
            Guild::Yellowboze => 8,
            Guild::Whitill => 9,
        }
    }

    /// Get the name of the guild as a string
    pub fn name(&self) -> &'static str {
        match self {
            Guild::Viridia => "Viridia",
            Guild::Greennill => "Greennill",
            Guild::Skyly => "Skyly",
            Guild::Bluefull => "Bluefull",
            Guild::Purplenum => "Purplenum",
            Guild::Pinkal => "Pinkal",
            Guild::Redria => "Redria",
            Guild::Oran => "Oran",
            Guild::Yellowboze => "Yellowboze",
            Guild::Whitill => "Whitill",
        }
    }

    /// Get the best character class for this guild
    pub fn best_class(&self) -> &'static str {
        match self {
            Guild::Viridia => "Ranger",
            Guild::Greennill => "Force",
            Guild::Skyly => "Ranger",
            Guild::Bluefull => "Hunter",
            Guild::Purplenum => "Force",
            Guild::Pinkal => "Force",
            Guild::Redria => "Hunter",
            Guild::Oran => "Force",
            Guild::Yellowboze => "Ranger",
            Guild::Whitill => "Ranger",
        }
    }

    /// Get the most common drop for this guild
    pub fn common_drop(&self) -> (&'static str, u32) {
        match self {
            Guild::Viridia => ("Partisans", 10),
            Guild::Greennill => ("Rifles", 13),
            Guild::Skyly => ("Swords", 13),
            Guild::Bluefull => ("Partisans", 13),
            Guild::Purplenum => ("Machineguns", 13),
            Guild::Pinkal => ("Wands", 13),
            Guild::Redria => ("Slicers", 10),
            Guild::Oran => ("Daggers", 13),
            Guild::Yellowboze => ("All Equal", 0),
            Guild::Whitill => ("Machineguns", 10),
        }
    }

    /// Get the rarest drop for this guild
    pub fn rare_drop(&self) -> (&'static str, u32) {
        match self {
            Guild::Viridia => ("Slicers", 1),
            Guild::Greennill => ("Swords", 1),
            Guild::Skyly => ("Machineguns", 1),
            Guild::Bluefull => ("Wands", 1),
            Guild::Purplenum => ("Daggers", 10),
            Guild::Pinkal => ("Rifles", 1),
            Guild::Redria => ("Daggers", 1),
            Guild::Oran => ("Rods", 1),
            Guild::Yellowboze => ("All Equal", 0),
            Guild::Whitill => ("Shotguns", 1),
        }
    }

    /// Get the MAG type for this guild
    pub fn mag_type(&self) -> &'static str {
        match self {
            Guild::Viridia => "A",
            Guild::Greennill => "A",
            Guild::Skyly => "A",
            Guild::Bluefull => "B",
            Guild::Purplenum => "B",
            Guild::Pinkal => "B",
            Guild::Redria => "C",
            Guild::Oran => "C",
            Guild::Yellowboze => "C",
            Guild::Whitill => "D",
        }
    }

    /// Get the weapon drop rates for this guild
    pub fn drop_rates(&self) -> DropRates {
        match self {
            Guild::Viridia => DropRates {
                sabers: 13,
                swords: 6,
                daggers: 7,
                partisans: 10,
                slicers: 1,
                handguns: 13,
                rifles: 6,
                machineguns: 6,
                shotguns: 11,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Greennill => DropRates {
                sabers: 13,
                swords: 1,
                daggers: 10,
                partisans: 6,
                slicers: 6,
                handguns: 13,
                rifles: 13,
                machineguns: 7,
                shotguns: 4,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Skyly => DropRates {
                sabers: 13,
                swords: 13,
                daggers: 7,
                partisans: 6,
                slicers: 6,
                handguns: 13,
                rifles: 10,
                machineguns: 1,
                shotguns: 4,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Bluefull => DropRates {
                sabers: 13,
                swords: 7,
                daggers: 6,
                partisans: 13,
                slicers: 6,
                handguns: 13,
                rifles: 7,
                machineguns: 7,
                shotguns: 4,
                canes: 13,
                rods: 10,
                wands: 1,
            },
            Guild::Purplenum => DropRates {
                sabers: 13,
                swords: 3,
                daggers: 10,
                partisans: 3,
                slicers: 6,
                handguns: 13,
                rifles: 7,
                machineguns: 13,
                shotguns: 5,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Pinkal => DropRates {
                sabers: 13,
                swords: 6,
                daggers: 7,
                partisans: 10,
                slicers: 6,
                handguns: 13,
                rifles: 1,
                machineguns: 7,
                shotguns: 4,
                canes: 13,
                rods: 7,
                wands: 13,
            },
            Guild::Redria => DropRates {
                sabers: 13,
                swords: 7,
                daggers: 1,
                partisans: 7,
                slicers: 10,
                handguns: 13,
                rifles: 7,
                machineguns: 7,
                shotguns: 8,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Oran => DropRates {
                sabers: 13,
                swords: 8,
                daggers: 13,
                partisans: 7,
                slicers: 6,
                handguns: 13,
                rifles: 7,
                machineguns: 7,
                shotguns: 4,
                canes: 13,
                rods: 1,
                wands: 8,
            },
            Guild::Yellowboze => DropRates {
                sabers: 13,
                swords: 7,
                daggers: 7,
                partisans: 7,
                slicers: 7,
                handguns: 13,
                rifles: 7,
                machineguns: 7,
                shotguns: 5,
                canes: 13,
                rods: 7,
                wands: 7,
            },
            Guild::Whitill => DropRates {
                sabers: 13,
                swords: 6,
                daggers: 6,
                partisans: 6,
                slicers: 13,
                handguns: 13,
                rifles: 6,
                machineguns: 10,
                shotguns: 1,
                canes: 13,
                rods: 7,
                wands: 6,
            },
        }
    }
}

impl fmt::Display for Guild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (common_name, common_pct) = self.common_drop();
        let (rare_name, rare_pct) = self.rare_drop();
        write!(
            f,
            "Guild: {}, Class: {}, Common: {} ({}%), Rare: {} ({}%), MAG: {}",
            self.name(),
            self.best_class(),
            common_name,
            common_pct,
            rare_name,
            rare_pct,
            self.mag_type()
        )
    }
}

/// Drop rates for weapons by guild
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropRates {
    pub sabers: u32,
    pub swords: u32,
    pub daggers: u32,
    pub partisans: u32,
    pub slicers: u32,
    pub handguns: u32,
    pub rifles: u32,
    pub machineguns: u32,
    pub shotguns: u32,
    pub canes: u32,
    pub rods: u32,
    pub wands: u32,
}

/// Get the complete Guild information for a character name
///
/// The section ID is calculated by summing the ASCII values of all characters
/// in the name and taking modulo 10. The result determines which guild the
/// character belongs to.
///
/// # Arguments
/// * `name` - The character name (must be ASCII and at most 12 characters)
///
/// # Returns
/// * `Ok(Guild)` - The guild enum variant with all associated information
/// * `Err(String)` - Error message if name is invalid
///
/// # Examples
///
/// ```
/// use psoid::calculate;
///
/// let guild = calculate("foobar").unwrap();
/// assert_eq!(guild.id(), 3);
/// assert_eq!(guild.name(), "Bluefull");
/// assert_eq!(guild.best_class(), "Hunter");
/// ```
pub fn calculate(name: &str) -> Result<Guild, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if name.len() > 12 {
        return Err("Name must be at most 12 characters long".to_string());
    }

    if !name.is_ascii() {
        return Err("Name must contain only ASCII characters".to_string());
    }

    let sum: u32 = name.bytes().map(|b| b as u32).sum();

    let guild = match sum % 10 {
        0 => Guild::Viridia,
        1 => Guild::Greennill,
        2 => Guild::Skyly,
        3 => Guild::Bluefull,
        4 => Guild::Purplenum,
        5 => Guild::Pinkal,
        6 => Guild::Redria,
        7 => Guild::Oran,
        8 => Guild::Yellowboze,
        9 => Guild::Whitill,
        _ => unreachable!(),
    };

    Ok(guild)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_foobar() {
        let guild = calculate("foobar").unwrap();
        assert_eq!(guild.id(), 3);
        assert_eq!(guild.name(), "Bluefull");
        assert_eq!(guild.best_class(), "Hunter");
    }

    #[test]
    fn test_calculate_foo_bar() {
        let guild = calculate("foo bar").unwrap();
        assert_eq!(guild.id(), 5);
        assert_eq!(guild.name(), "Pinkal");
        assert_eq!(guild.best_class(), "Force");
    }

    #[test]
    fn test_calculate_empty_name() {
        let result = calculate("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_calculate_too_long() {
        let long_name = "thisnameistoolong";
        let result = calculate(long_name);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Name must be at most 12 characters long"
        );
    }

    #[test]
    fn test_calculate_non_ascii() {
        let result = calculate("café");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Name must contain only ASCII characters"
        );
    }

    #[test]
    fn test_guild_contains_all_info() {
        let guild = calculate("foobar").unwrap();
        assert_eq!(guild.id(), 3);
        assert_eq!(guild.name(), "Bluefull");
        assert_eq!(guild.best_class(), "Hunter");
        assert_eq!(guild.common_drop(), ("Partisans", 13));
        assert_eq!(guild.rare_drop(), ("Wands", 1));
        assert_eq!(guild.mag_type(), "B");
    }

    #[test]
    fn test_guild_drop_rates() {
        let guild = calculate("foobar").unwrap();
        let rates = guild.drop_rates();
        assert_eq!(rates.partisans, 13);
        assert_eq!(rates.rods, 10);
        assert_eq!(rates.wands, 1);
    }

    #[test]
    fn test_guild_display() {
        let guild = calculate("foobar").unwrap();
        let display_str = format!("{}", guild);
        assert!(display_str.contains("Bluefull"));
        assert!(display_str.contains("Hunter"));
        assert!(display_str.contains("Partisans"));
    }

    #[test]
    fn test_all_guilds() {
        let guilds = [
            Guild::Viridia,
            Guild::Greennill,
            Guild::Skyly,
            Guild::Bluefull,
            Guild::Purplenum,
            Guild::Pinkal,
            Guild::Redria,
            Guild::Oran,
            Guild::Yellowboze,
            Guild::Whitill,
        ];

        for (index, guild) in guilds.iter().enumerate() {
            assert_eq!(guild.id(), index as u32);
            assert!(!guild.name().is_empty());
            assert!(!guild.best_class().is_empty());
            let rates = guild.drop_rates();
            assert!(rates.sabers > 0);
        }
    }

    #[test]
    fn test_drop_rates_viridia() {
        let guild = Guild::Viridia;
        let rates = guild.drop_rates();
        assert_eq!(rates.partisans, 10);
        assert_eq!(rates.slicers, 1);
        assert_eq!(rates.shotguns, 11);
    }

    #[test]
    fn test_drop_rates_bluefull() {
        let guild = Guild::Bluefull;
        let rates = guild.drop_rates();
        assert_eq!(rates.partisans, 13);
        assert_eq!(rates.rods, 10);
        assert_eq!(rates.wands, 1);
    }

    #[test]
    fn test_guild_viridia() {
        let guild = Guild::Viridia;
        assert_eq!(guild.id(), 0);
        assert_eq!(guild.name(), "Viridia");
        assert_eq!(guild.best_class(), "Ranger");
    }

    #[test]
    fn test_guild_pinkal() {
        let guild = Guild::Pinkal;
        assert_eq!(guild.id(), 5);
        assert_eq!(guild.name(), "Pinkal");
        assert_eq!(guild.best_class(), "Force");
        assert_eq!(guild.common_drop(), ("Wands", 13));
    }

    #[test]
    fn test_guild_whitill() {
        let guild = Guild::Whitill;
        assert_eq!(guild.id(), 9);
        assert_eq!(guild.name(), "Whitill");
        assert_eq!(guild.rare_drop(), ("Shotguns", 1));
    }
}
