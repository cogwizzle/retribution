//! # Map Migration
//!
//! This module contains the migration for the map table in the database.

use super::*;
use crate::game::map::{GridSquare, Map, Portal, Room};
use rusqlite::Connection;
use serde_json;

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
        let db = Connection::open(self.path.as_str()).map_err(|_| "Unable to open database.")?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS maps (
                name TEXT PRIMARY KEY,
                grid BLOB NOT NULL
            )",
            [],
        )
        .map_err(|_| "Unable to create table.")?;
        db.close().map_err(|_| "Unable to close database.")?;
        Ok(())
    }

    /// Drop the map table in the database.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the table was dropped, or Err if not.
    fn down(&self) -> Result<(), &'static str> {
        let db = Connection::open(self.path.as_str()).map_err(|_| "Unable to open database.")?;
        db.execute("DROP TABLE IF EXISTS maps", [])
            .map_err(|_| "Unable to drop table.")?;
        db.close().map_err(|_| "Unable to close database.")?;
        Ok(())
    }
}

/// A function that creates a test area map.
///
/// Room formation:
/// ```text
/// [  x  ] [ r 4 ] [  x  ]
/// [ r 2 ] [ r 1 ] [ r 3 ]
/// [  x  ] [  p  ] [  x  ]
/// ```
pub fn test_area() -> Map {
    let room1 = GridSquare::Room(Room::new(
        String::from("Room 1"),
        String::from("This is room 1."),
    ));
    let room2 = GridSquare::Room(Room::new(
        String::from("Room 2"),
        String::from("This is room 2."),
    ));
    let room3 = GridSquare::Room(Room::new(
        String::from("Room 3"),
        String::from("This is room 3."),
    ));
    let room4 = GridSquare::Room(Room::new(
        String::from("Room 4"),
        String::from("This is room 4."),
    ));
    let portal = GridSquare::Portal(Portal::new(
        String::from("test_area_2_portal"),
        String::from("Test Area 2"),
        (1, 0),
    ));
    let mut map = Map::new(String::from("Test Area"), 3, 3);
    map.set_grid_square(1, 1, room1).unwrap();
    map.set_grid_square(1, 0, room2).unwrap();
    map.set_grid_square(1, 2, room3).unwrap();
    map.set_grid_square(0, 1, room4).unwrap();
    map.set_grid_square(2, 1, portal).unwrap();
    map
}

/// A function that creates a test area 2 map.
///
/// Room formation:
/// ```text
/// [  p  ]
/// [ r 1 ]
/// ```
pub fn test_area_2() -> Map {
    let mut map = Map::new(String::from("Test Area 2"), 2, 1);
    let room = GridSquare::Room(Room::new(
        String::from("Room 1"),
        String::from("This is in test area 2."),
    ));
    let portal = GridSquare::Portal(Portal::new(
        String::from("test_area_portal"),
        String::from("Test Area"),
        (1, 1),
    ));
    map.set_grid_square(1, 0, room).unwrap();
    map.set_grid_square(0, 0, portal).unwrap();
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
        let db = Connection::open(self.path.as_str()).map_err(|_| "Unable to open database.")?;
        let insert = |name: &str, map_json: String| -> Result<(), &'static str> {
            db.execute(
                "INSERT OR IGNORE INTO maps (name, grid) VALUES (?1, ?2)",
                &[name, &map_json],
            )
            .map_err(|_| "Unable to insert map.")?;
            Ok(())
        };
        let map_json =
            serde_json::to_string(&test_area().grid).map_err(|_| "Unable to serialize map.")?;
        let map_json_2 =
            serde_json::to_string(&test_area_2().grid).map_err(|_| "Unable to serialize map.")?;
        insert("Test Area", map_json)?;
        insert("Test Area 2", map_json_2)?;
        db.close().map_err(|_| "Unable to close database.")?;
        Ok(())
    }

    /// Rollback the migration.
    ///
    /// # Returns
    /// * `Result<(), &'static str>` - A result that is Ok if the migration was successful, or Err if not.
    fn down(&self) -> Result<(), &'static str> {
        let db = Connection::open(self.path.as_str()).map_err(|_| "Unable to open database.")?;
        db.execute(
            "DELETE FROM maps WHERE name = ?1 or name = ?2",
            &["Test Area", "Test Area 2"],
        )
        .map_err(|_| "Unable to delete map.")?;
        db.close().map_err(|_| "Unable to close database.")?;
        Ok(())
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
fn handle_migration_error(name: String, e: &str) -> &'static str {
    eprintln!("Migration Error ({}) {}", name, e);
    return "Migration Error"
}

/// A function that runs the migration to create all map related content.
///
/// # Arguments
/// * `path` - A string that is the path to the database.
///
/// # Returns
/// * `Result<(), &'static str>` - A result that is Ok, or an error message.
pub fn migrate_up(path: Option<String>) -> Result<(), &'static str> {
    let path = path.unwrap_or_else(|| String::from(DB_PATH));
    let migration = CreateMapMigration::new(path);
    migration.up().map_err(|e| handle_migration_error(migration.name, e))?;
    let migration = TestArea::new(migration.path);
    migration.up().map_err(|e| handle_migration_error(migration.name, e))?;
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
    let path = path.unwrap_or_else(|| String::from(DB_PATH));
    let migration = TestArea::new(path);
    migration.down().map_err(|e| handle_migration_error(migration.name, e))?;
    let migration = CreateMapMigration::new(migration.path);
    migration.down().map_err(|e| handle_migration_error(migration.name, e))?;
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
        assert_eq!(result, "Migration Error");
    }
}
