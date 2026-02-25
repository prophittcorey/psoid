//! Phantasy Star Online Section ID Calculator
//!
//! This library calculates the Section ID for a Phantasy Star Online character
//! based on their name. The Section ID determines which guild the character
//! belongs to, which affects drop rates and MAG types.

/// Represents a Section ID in Phantasy Star Online with all associated guild information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionID {
    /// The numeric section ID (0-9)
    pub id: u32,
    /// The guild name
    pub name: &'static str,
    /// Best character class for this guild
    pub best_class: &'static str,
    /// Common drop weapon type and percentage
    pub common_drop: (&'static str, u32),
    /// Rare drop weapon type and percentage
    pub rare_drop: (&'static str, u32),
    /// MAG type for this guild
    pub mag_type: &'static str,
}

impl SectionID {
    /// Get the weapon drop rates for this section ID
    pub fn drop_rates(&self) -> DropRates {
        match self.id {
            0 => DropRates {
                // Viridia
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
            1 => DropRates {
                // Greennill
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
            2 => DropRates {
                // Skyly
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
            3 => DropRates {
                // Bluefull
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
            4 => DropRates {
                // Purplenum
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
            5 => DropRates {
                // Pinkal
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
            6 => DropRates {
                // Redria
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
            7 => DropRates {
                // Oran
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
            8 => DropRates {
                // Yellowboze
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
            9 => DropRates {
                // Whitill
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
            _ => DropRates {
                // Fallback for invalid IDs (shouldn't happen)
                sabers: 0,
                swords: 0,
                daggers: 0,
                partisans: 0,
                slicers: 0,
                handguns: 0,
                rifles: 0,
                machineguns: 0,
                shotguns: 0,
                canes: 0,
                rods: 0,
                wands: 0,
            },
        }
    }

    /// Get a human-readable summary of this section ID
    pub fn summary(&self) -> String {
        format!(
            "Guild: {}, Class: {}, Common: {} ({}%), Rare: {} ({}%), MAG: {}",
            self.name,
            self.best_class,
            self.common_drop.0,
            self.common_drop.1,
            self.rare_drop.0,
            self.rare_drop.1,
            self.mag_type
        )
    }
}

/// Represents a guild in Phantasy Star Online (kept for backward compatibility)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guild {
    /// The name of the guild
    pub name: &'static str,
    /// The section ID (0-9)
    pub section_id: u32,
    /// Best character class for this guild
    pub best_class: &'static str,
    /// Common drop weapon type and percentage
    pub common_drop: (&'static str, u32),
    /// Rare drop weapon type and percentage
    pub rare_drop: (&'static str, u32),
    /// MAG type for this guild
    pub mag_type: &'static str,
}

/// Drop rates for weapons by guild
#[derive(Debug, Clone)]
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

impl DropRates {
    /// Create drop rates for a specific guild by section ID
    pub fn for_section_id(section_id: u32) -> Option<Self> {
        match section_id {
            0 => Some(DropRates {
                // Viridia
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
            }),
            1 => Some(DropRates {
                // Greennill
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
            }),
            2 => Some(DropRates {
                // Skyly
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
            }),
            3 => Some(DropRates {
                // Bluefull
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
            }),
            4 => Some(DropRates {
                // Purplenum
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
            }),
            5 => Some(DropRates {
                // Pinkal
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
            }),
            6 => Some(DropRates {
                // Redria
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
            }),
            7 => Some(DropRates {
                // Oran
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
            }),
            8 => Some(DropRates {
                // Yellowboze
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
            }),
            9 => Some(DropRates {
                // Whitill
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
            }),
            _ => None,
        }
    }
}

/// Get the complete SectionID information for a character name
///
/// The section ID is calculated by summing the ASCII values of all characters
/// in the name and taking modulo 10.
///
/// # Arguments
/// * `name` - The character name (must be ASCII and at most 12 characters)
///
/// # Returns
/// * `Ok(SectionID)` - The section ID with all associated guild information
/// * `Err(String)` - Error message if name is invalid
///
/// # Examples
///
/// ```
/// use psoid::calculate;
///
/// let section_id = calculate("foobar").unwrap();
/// assert_eq!(section_id.id, 3);
/// assert_eq!(section_id.name, "Bluefull");
/// assert_eq!(section_id.best_class, "Hunter");
/// ```
pub fn calculate(name: &str) -> Result<SectionID, String> {
    // Validate name
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if name.len() > 12 {
        return Err("Name must be at most 12 characters long".to_string());
    }

    if !name.is_ascii() {
        return Err("Name must contain only ASCII characters".to_string());
    }

    // Calculate the numeric section ID
    let sum: u32 = name.bytes().map(|b| b as u32).sum();
    let id = sum % 10;

    // Get the guild information for this ID
    get_section_id(id).ok_or_else(|| format!("Invalid section ID: {}", id))
}

/// Calculate the raw numeric section ID from a character name
///
/// This is a lower-level function that only returns the numeric ID (0-9).
/// For most use cases, use `calculate()` instead to get the full SectionID struct.
///
/// # Arguments
/// * `name` - The character name (must be ASCII and at most 12 characters)
///
/// # Returns
/// * `Ok(u32)` - The section ID (0-9)
/// * `Err(String)` - Error message if name is invalid
pub fn calculate_section_id(name: &str) -> Result<u32, String> {
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
    Ok(sum % 10)
}

/// Get the SectionID information for a numeric section ID (0-9)
fn get_section_id(id: u32) -> Option<SectionID> {
    match id {
        0 => Some(SectionID {
            id: 0,
            name: "Viridia",
            best_class: "Ranger",
            common_drop: ("Partisans", 10),
            rare_drop: ("Slicers", 1),
            mag_type: "A",
        }),
        1 => Some(SectionID {
            id: 1,
            name: "Greennill",
            best_class: "Force",
            common_drop: ("Rifles", 13),
            rare_drop: ("Swords", 1),
            mag_type: "A",
        }),
        2 => Some(SectionID {
            id: 2,
            name: "Skyly",
            best_class: "Ranger",
            common_drop: ("Swords", 13),
            rare_drop: ("Machineguns", 1),
            mag_type: "A",
        }),
        3 => Some(SectionID {
            id: 3,
            name: "Bluefull",
            best_class: "Hunter",
            common_drop: ("Partisans", 13),
            rare_drop: ("Wands", 1),
            mag_type: "B",
        }),
        4 => Some(SectionID {
            id: 4,
            name: "Purplenum",
            best_class: "Force",
            common_drop: ("Machineguns", 13),
            rare_drop: ("Daggers", 10),
            mag_type: "B",
        }),
        5 => Some(SectionID {
            id: 5,
            name: "Pinkal",
            best_class: "Force",
            common_drop: ("Wands", 13),
            rare_drop: ("Rifles", 1),
            mag_type: "B",
        }),
        6 => Some(SectionID {
            id: 6,
            name: "Redria",
            best_class: "Hunter",
            common_drop: ("Slicers", 10),
            rare_drop: ("Daggers", 1),
            mag_type: "C",
        }),
        7 => Some(SectionID {
            id: 7,
            name: "Oran",
            best_class: "Force",
            common_drop: ("Daggers", 13),
            rare_drop: ("Rods", 1),
            mag_type: "C",
        }),
        8 => Some(SectionID {
            id: 8,
            name: "Yellowboze",
            best_class: "Ranger",
            common_drop: ("All Equal", 0),
            rare_drop: ("All Equal", 0),
            mag_type: "C",
        }),
        9 => Some(SectionID {
            id: 9,
            name: "Whitill",
            best_class: "Ranger",
            common_drop: ("Machineguns", 10),
            rare_drop: ("Shotguns", 1),
            mag_type: "D",
        }),
        _ => None,
    }
}

/// Get the guild information for a section ID (kept for backward compatibility)
pub fn get_guild(section_id: u32) -> Option<Guild> {
    let guilds = [
        Guild {
            name: "Viridia",
            section_id: 0,
            best_class: "Ranger",
            common_drop: ("Partisans", 10),
            rare_drop: ("Slicers", 1),
            mag_type: "B",
        },
        Guild {
            name: "Greennill",
            section_id: 1,
            best_class: "Force",
            common_drop: ("Rifles", 13),
            rare_drop: ("Swords", 1),
            mag_type: "B",
        },
        Guild {
            name: "Skyly",
            section_id: 2,
            best_class: "Ranger",
            common_drop: ("Swords", 13),
            rare_drop: ("Machineguns", 1),
            mag_type: "B",
        },
        Guild {
            name: "Bluefull",
            section_id: 3,
            best_class: "Hunter",
            common_drop: ("Partisans", 13),
            rare_drop: ("Wands", 1),
            mag_type: "B",
        },
        Guild {
            name: "Purplenum",
            section_id: 4,
            best_class: "Force",
            common_drop: ("Machineguns", 13),
            rare_drop: ("Daggers", 10),
            mag_type: "B",
        },
        Guild {
            name: "Pinkal",
            section_id: 5,
            best_class: "Force",
            common_drop: ("Wands", 13),
            rare_drop: ("Rifles", 1),
            mag_type: "B",
        },
        Guild {
            name: "Redria",
            section_id: 6,
            best_class: "Hunter",
            common_drop: ("Slicers", 10),
            rare_drop: ("Daggers", 1),
            mag_type: "B",
        },
        Guild {
            name: "Oran",
            section_id: 7,
            best_class: "Force",
            common_drop: ("Daggers", 13),
            rare_drop: ("Rods", 1),
            mag_type: "B",
        },
        Guild {
            name: "Yellowboze",
            section_id: 8,
            best_class: "Ranger",
            common_drop: ("None", 0),
            rare_drop: ("None", 0),
            mag_type: "B",
        },
        Guild {
            name: "Whitill",
            section_id: 9,
            best_class: "Ranger",
            common_drop: ("Machineguns", 10),
            rare_drop: ("Shotguns", 1),
            mag_type: "B",
        },
    ];

    guilds.iter().find(|g| g.section_id == section_id).cloned()
}

/// Get guild information for a character name (kept for backward compatibility)
pub fn get_guild_for_name(name: &str) -> Result<Guild, String> {
    let section_id = calculate_section_id(name)?;
    get_guild(section_id).ok_or_else(|| format!("Invalid section ID: {}", section_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_foobar() {
        let result = calculate("foobar").unwrap();
        assert_eq!(result.id, 3);
        assert_eq!(result.name, "Bluefull");
        assert_eq!(result.best_class, "Hunter");
    }

    #[test]
    fn test_calculate_foo_bar() {
        let result = calculate("foo bar").unwrap();
        assert_eq!(result.id, 5);
        assert_eq!(result.name, "Pinkal");
        assert_eq!(result.best_class, "Force");
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
    fn test_section_id_contains_all_info() {
        let section_id = calculate("foobar").unwrap();
        assert_eq!(section_id.id, 3);
        assert_eq!(section_id.name, "Bluefull");
        assert_eq!(section_id.best_class, "Hunter");
        assert_eq!(section_id.common_drop, ("Partisans", 13));
        assert_eq!(section_id.rare_drop, ("Wands", 1));
        assert_eq!(section_id.mag_type, "B");
    }

    #[test]
    fn test_section_id_drop_rates() {
        let section_id = calculate("foobar").unwrap();
        let rates = section_id.drop_rates();
        assert_eq!(rates.partisans, 13);
        assert_eq!(rates.rods, 10);
        assert_eq!(rates.wands, 1);
    }

    #[test]
    fn test_section_id_summary() {
        let section_id = calculate("foobar").unwrap();
        let summary = section_id.summary();
        assert!(summary.contains("Bluefull"));
        assert!(summary.contains("Hunter"));
        assert!(summary.contains("Partisans"));
    }

    #[test]
    fn test_calculate_section_id_foobar() {
        let result = calculate_section_id("foobar").unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_calculate_section_id_foo_bar() {
        let result = calculate_section_id("foo bar").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_get_guild_bluefull() {
        let guild = get_guild(3).unwrap();
        assert_eq!(guild.name, "Bluefull");
        assert_eq!(guild.section_id, 3);
        assert_eq!(guild.best_class, "Hunter");
        assert_eq!(guild.common_drop, ("Partisans", 13));
        assert_eq!(guild.rare_drop, ("Wands", 1));
    }

    #[test]
    fn test_get_guild_pinkal() {
        let guild = get_guild(5).unwrap();
        assert_eq!(guild.name, "Pinkal");
        assert_eq!(guild.section_id, 5);
        assert_eq!(guild.best_class, "Force");
        assert_eq!(guild.common_drop, ("Wands", 13));
        assert_eq!(guild.rare_drop, ("Rifles", 1));
    }

    #[test]
    fn test_get_guild_invalid_section_id() {
        let result = get_guild(10);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_guild_for_name_foobar() {
        let guild = get_guild_for_name("foobar").unwrap();
        assert_eq!(guild.name, "Bluefull");
        assert_eq!(guild.section_id, 3);
    }

    #[test]
    fn test_get_guild_for_name_foo_bar() {
        let guild = get_guild_for_name("foo bar").unwrap();
        assert_eq!(guild.name, "Pinkal");
        assert_eq!(guild.section_id, 5);
    }

    #[test]
    fn test_drop_rates_viridia() {
        let rates = DropRates::for_section_id(0).unwrap();
        assert_eq!(rates.partisans, 10);
        assert_eq!(rates.slicers, 1);
        assert_eq!(rates.shotguns, 11);
    }

    #[test]
    fn test_drop_rates_bluefull() {
        let rates = DropRates::for_section_id(3).unwrap();
        assert_eq!(rates.partisans, 13);
        assert_eq!(rates.rods, 10);
        assert_eq!(rates.wands, 1);
    }

    #[test]
    fn test_drop_rates_invalid_section_id() {
        let result = DropRates::for_section_id(10);
        assert!(result.is_none());
    }

    #[test]
    fn test_all_sections_accessible() {
        for i in 0..10 {
            let section = get_section_id(i);
            assert!(section.is_some(), "Section ID {} should exist", i);
        }
    }

    #[test]
    fn test_all_drop_rates_accessible() {
        for i in 0..10 {
            let section = get_section_id(i).unwrap();
            let rates = section.drop_rates();
            // All drop rates should be positive for valid sections
            assert!(rates.sabers > 0, "Sabers should be > 0 for section {}", i);
        }
    }
}
