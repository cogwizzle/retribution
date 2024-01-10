use super::*;
use rusqlite::Connection;

pub trait Migration {
    fn new(path: String) -> Self;
    fn up(&self) -> Result<(), &'static str>;
    fn down(&self) -> Result<(), &'static str>;
}

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

    #[test]
    fn test_up() {
        let migration = CreateMapMigration::new(String::from(":memory:"));
        let result = migration.up();
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_down() {
        let migration = CreateMapMigration::new(String::from(":memory:"));
        let result = migration.down();
        assert_eq!(result, Ok(()));
    }
}

