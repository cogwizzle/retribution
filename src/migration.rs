const DB_PATH: &str = crate::DB_PATH;

pub mod map;

/// A struct that represents a map in the game world.
pub trait Migration {
    /// Constructor for the struct.
    fn new(path: String) -> Self;
    /// Run the migration.
    fn up(&self) -> Result<(), &'static str>;
    /// Rollback the migration.
    fn down(&self) -> Result<(), &'static str>;
}
