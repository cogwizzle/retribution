//! # Map Migration
//!
//! This module contains the migration for the map table in the database.

use super::*;
use serde_json;
use crate::game::map::{Map, Room};
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
        let path = path.replace("~", std::env::var("HOME").unwrap().as_str());
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
        let db = match Connection::open(self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        println!("Creating table.");
        let result = match db.execute(
            "CREATE TABLE IF NOT EXISTS maps (
                name TEXT PRIMARY KEY,
                grid BLOB NOT NULL
            )",
            []
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to create table."),
        };
        match db.close() {
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
        let db = match Connection::open(self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        let result = match db.execute(
            "DROP TABLE IF EXISTS maps",
            []
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to drop table."),
        };
        match db.close() {
            Ok(_) => (),
            Err(_) => return Err("Unable to close database."),
        };
        result
    }
}

/// A function that creates a test area map.
fn test_area() -> Map {
    let room1 = Room::new(String::from("Room 1"), String::from("This is room 1."));
    let room2 = Room::new(String::from("Room 2"), String::from("This is room 2."));
    let room3 = Room::new(String::from("Room 3"), String::from("This is room 3."));
    let room4 = Room::new(String::from("Room 4"), String::from("This is room 4."));
    let room5 = Room::new(String::from("Room 5"), String::from("This is room 5."));
    let mut map = Map::new(String::from("Test Area"), 3, 3);
    map.set_room(1, 1, room1).unwrap();
    map.set_room(1, 0, room2).unwrap();
    map.set_room(1, 2, room3).unwrap();
    map.set_room(0, 1, room4).unwrap();
    map.set_room(2, 1, room5).unwrap();
    map
}

/// Struct for creating a test area map.
pub struct TestArea {
    name: String,
    path: String,
}

impl Migration for TestArea {
    /// Constructor for the TestArea struct.
    fn new(path: String) -> Self {
        let path = path.replace("~", std::env::var("HOME").unwrap().as_str());
        TestArea {
            name: String::from("TestArea"),
            path,
        }
    }

    /// Run the migration.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the migration was successful, or Err if not.
    fn up(&self) -> Result<(), &'static str> {
        let db = match Connection::open(self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        let map_json = match serde_json::to_string(&test_area().grid) {
            Ok(j) => j,
            Err(_) => return Err("Unable to serialize map."),
        };
        let result = match db.execute(
            "INSERT OR IGNORE INTO maps (name, grid) VALUES (?1, ?2)",
            &["test_area", &map_json],
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to insert map."),
        };
        match db.close() {
            Ok(_) => (),
            Err(_) => return Err("Unable to close database."),
        };
        result
    }

    /// Rollback the migration.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the migration was successful, or Err if not.
    fn down(&self) -> Result<(), &'static str> {
        let db = match Connection::open(self.path.as_str()) {
            Ok(c) => c,
            Err(_) => return Err("Unable to open database."),
        };
        let result = match db.execute(
            "DELETE FROM maps WHERE name = ?1",
            &["test_area"],
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to delete map."),
        };
        match db.close() {
            Ok(_) => (),
            Err(_) => return Err("Unable to close database."),
        };
        result
    }
}

/// A function that handles migration errors.
///
/// # Arguments
/// * `name` - A string that is the name of the migration.
/// * `e` - A string that is the error message.
///
/// # Returns
/// * `Result<(), &'static str>` - A result that is Err.
fn handle_migration_error(name: String, e: &str) -> Result<(), &'static str> {
    eprintln!("Migration Error ({}) {}", name, e);
    Err("Migration Error")
}

/// A function that runs the migration to create all map related content.
///
/// # Arguments
/// * `path` - A string that is the path to the database.
///
/// # Returns
/// * `Result<(), &str>` - A result that is Ok, or an error message.
pub fn migrate_up(path: Option<String>) -> Result<(), &'static str> {
    let path = match path {
        Some(p) => p,
        None => String::from(DB_PATH),
    };
    let migration = CreateMapMigration::new(path);
    match migration.up() {
        Ok(_) => (),
        Err(e) => return handle_migration_error(migration.name, e),
    };
    let migration = TestArea::new(migration.path);
    match migration.up() {
        Ok(_) => (),
        Err(e) => return handle_migration_error(migration.name, e),
    };
    Ok(())
}

/// A function that rolls back the migration to create all map related content.
///
/// # Arguments
/// * `path` - A string that is the path to the database.
///
/// # Returns
/// * `Result<(), &str>` - A result that is Ok, or an error message.
pub fn migrate_down(path: Option<String>) -> Result<(), &'static str> {
    let path = match path {
        Some(p) => p,
        None => String::from(DB_PATH),
    };
    let migration = TestArea::new(path);
    match migration.down() {
        Ok(_) => (),
        Err(e) => return handle_migration_error(migration.name, e),
    };
    let migration = CreateMapMigration::new(migration.path);
    match migration.down() {
        Ok(_) => (),
        Err(e) => return handle_migration_error(migration.name, e),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the migrate_up function.
    #[test]
    fn create_map_migration_new() {
        let migration = CreateMapMigration::new(String::from(":memory:"));
        assert_eq!(migration.name, "CreateMapMigration");
        assert_eq!(migration.path, ":memory:");
    }

    /// Test the migrate_up function.
    #[test]
    fn test_area_migration_new() {
        let migration = TestArea::new(String::from(":memory:"));
        assert_eq!(migration.name, "TestArea");
        assert_eq!(migration.path, ":memory:");
    }

    /// Test handle_migration_error function.
    #[test]
    fn handle_migration_error_test() {
        let result = handle_migration_error(String::from("Test"), "Error");
        assert_eq!(result, Err("Migration Error"));
    }
}

