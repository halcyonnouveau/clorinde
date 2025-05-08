// This file was generated with `clorinde`. Do not modify.

// Re-export postgres-types
pub use postgres_types;

// Functions for our mod API
pub mod queries;
pub mod types;
pub mod client;
pub mod array_iterator;
pub mod utils;
pub mod type_traits;
pub mod domain;

// Add FromStr implementations for all PostgreSQL enum types
impl std::str::FromStr for types::Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("Invalid Color variant: {}", s))
        }
    }
}

impl std::str::FromStr for types::Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err(format!("Invalid Status variant: {}", s))
        }
    }
}

impl std::str::FromStr for types::Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "north" => Ok(Self::North),
            "south" => Ok(Self::South),
            "east" => Ok(Self::East),
            "west" => Ok(Self::West),
            _ => Err(format!("Invalid Direction variant: {}", s))
        }
    }
}