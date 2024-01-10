use super::*;
use rusqlite::Connection;

/// A struct that represents a map in the game world.
///
/// TODO eventually move this to another file.
pub trait Migration {
    /// Constructor for the struct.
    fn new(path: String) -> Self;
    /// Run the migration.
    fn up(&self) -> Result<(), &'static str>;
    /// Rollback the migration.
    fn down(&self) -> Result<(), &'static str>;
}

/// A struct that represents a migration to create the map table in the database.
struct CreateMapMigration {
    name: String,
    path: String,
}

impl Migration for CreateMapMigration {
    /// Constructor for the CreateMapMigration struct.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the migration.
    ///
    /// # Returns
    /// * `CreateMapMigration` - A new CreateMapMigration.
    fn new(path: String) -> Self {
        CreateMapMigration {
            name: String::from("CreateMapMigration"),
            path,
        }
    }

    /// Create the map table in the database.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the table was created, or Err if not.
    fn up(&self) -> Result<(), &'static str> {
        let conn = match Connection::open(&self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        let result = match conn.execute(
            "CREATE TABLE IF NOT EXISTS maps (
                name TEXT PRIMARY KEY,
                grid BLOB NOT NULL
            )",
            []
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to create table."),
        };
        match conn.close() {
            Ok(_) => (),
            Err(_) => return Err("Unable to close database."),
        };
        result
    }

    /// Drop the map table in the database.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the table was dropped, or Err if not.
    fn down(&self) -> Result<(), &'static str> {
        let conn = match Connection::open(&self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        let result = match conn.execute(
            "DROP TABLE IF EXISTS maps",
            []
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to drop table."),
        };
        match conn.close() {
            Ok(_) => (),
            Err(_) => return Err("Unable to close database."),
        };
        result
    }
}

/// A function that runs the migration to create all map related content.
///
/// # Arguments
/// * `path` - A string that is the path to the database.
///
/// # Returns
/// * `Result<(), &str>` - A result that is Ok, or an error message.
///
/// # Examples
/// ```
/// use retribution::migration::map;
///
/// let result = map::migrate_up(Some(String::from(":memory:")));
/// assert_eq!(result, Ok(()));
/// ```
pub fn migrate_up(path: Option<String>) -> Result<(), &'static str> {
    let path = match path {
        Some(p) => p,
        None => String::from(DB_PATH),
    };
    let migration = CreateMapMigration::new(path);
    let handle_migration = |e| {
        eprintln!("Migration Error ({}) {}", migration.name, e);
    };
    migration.up().unwrap_or_else(handle_migration);
    Ok(())
}

/// A function that rolls back the migration to create all map related content.
///
/// # Arguments
/// * `path` - A string that is the path to the database.
///
/// # Returns
/// * `Result<(), &str>` - A result that is Ok, or an error message.
///
/// # Examples
/// ```
/// use retribution::migration::map;
///
/// let result = map::migrate_down(Some(String::from(":memory:")));
/// assert_eq!(result, Ok(()));
/// ```
pub fn migrate_down(path: Option<String>) -> Result<(), &'static str> {
    let path = match path {
        Some(p) => p,
        None => String::from(DB_PATH),
    };
    let migration = CreateMapMigration::new(path);
    let handle_migration = |e| {
        eprintln!("Migration Error ({}) {}", migration.name, e);
    };
    migration.down().unwrap_or_else(handle_migration);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_map_migration_new() {
        let migration = CreateMapMigration::new(String::from(":memory:"));
        assert_eq!(migration.name, "CreateMapMigration");
        assert_eq!(migration.path, ":memory:");
    }
}

