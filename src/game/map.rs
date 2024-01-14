//! # Map
//! Module that represents a location in the game world.
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json;

/// A struct that represents a map in the game world.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Map {
    /// The name of the map. Value must be unique.
    pub name: String,
    /// A grid of rooms and portals in the game world.
    pub grid: Vec<Vec<Option<GridSquare>>>,
}

impl Map {
    /// Constructor for the Map struct.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the map.
    /// * `x` - An i32 that is the number of rooms in the x direction.
    /// * `y` - An i32 that is the number of rooms in the y direction.
    ///
    /// # Returns
    /// * `Map` - A new Map.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let map = map::Map::new(String::from("Test Area"), 3, 3);
    /// assert_eq!(map.name, "Test Area");
    /// ```
    pub fn new(name: String, rows: i32, cols: i32) -> Map {
        let mut grid = vec![];
        // Create a grid of rooms.
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push(None);
            }
            grid.push(row);
        }
        Map { name, grid }
    }

    /// A safe way to get a room from the map.
    ///
    /// # Arguments
    /// * `row` - An i32 that is the row coordinate of the room.
    /// * `col` - An i32 that is the col coordinate of the room.
    ///
    /// # Returns
    /// * `Option<Room>` - An option that is the room at the given coordinates, or None.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let room = map::GridSquare::Room(
    ///     map::Room::new(
    ///         String::from("Test Room"),
    ///         String::from("This is a test room.")
    ///     )
    /// );
    /// let mut map = map::Map::new(String::from("Test Area"), 3, 3);
    /// map.set_grid_square(1, 1, room);
    /// let result = map.get_grid_square(1, 1);
    /// assert!(result.is_some());
    /// let result = map.get_grid_square(0, 0);
    /// assert!(result.is_none());
    /// let result = map.get_grid_square(-1, -1);
    /// assert!(result.is_none());
    /// let result = map.get_grid_square(3, 3);
    /// assert!(result.is_none());
    /// ```
    pub fn get_grid_square(&self, row: i32, col: i32) -> Option<&GridSquare> {
        if col < 0 || row < 0 {
            return None;
        }
        // We can safely assume these are positive numbers based on the check above.
        let col = col as usize;
        let row = row as usize;
        if self.grid.len() <= row || self.grid[0].len() <= col {
            return None;
        }
        let grid_square = &self.grid[row][col];
        match grid_square {
            Some(r) => Some(&r),
            None => None,
        }
    }

    /// A safe way to set a room in the map.
    ///
    /// # Arguments
    /// * `row` - An usize that is the row coordinate of the room.
    /// * `col` - An usize that is the col coordinate of the room.
    ///
    /// # Returns
    /// * `Result<(), &str>` - A result that is Ok, or an error message.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let room = map::GridSquare::Room(map::Room::new(String::from("Test Room"), String::from("This is a test room.")));
    /// let mut map = map::Map::new(String::from("Test Area"), 3, 3);
    /// map.set_grid_square(1, 1, room);
    /// let result = map.get_grid_square(1, 1);
    /// assert!(result.is_some());
    /// ```
    pub fn set_grid_square(
        &mut self,
        row: usize,
        col: usize,
        grid_square: GridSquare,
    ) -> Result<(), &str> {
        if self.grid.len() < row || self.grid[row].len() < col {
            return Err("Index out of bounds.");
        }
        self.grid[row][col] = Some(grid_square);
        Ok(())
    }
}

/// A struct that represents a location in the game world.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Room {
    /// The name of the room.
    pub name: String,
    /// The description of the room.
    pub description: String,
}

impl Room {
    /// Constructor for the Room struct.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the room.
    /// * `description` - A string that is the description of the room.
    ///
    /// # Returns
    /// * `Room` - A new Room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let room = map::Room::new(String::from("Test Room"), String::from("This is a test room."));
    /// ```
    pub fn new(name: String, description: String) -> Room {
        Room { name, description }
    }
}

/// A portal is a struct that teleports a player to another map at a set of coordinates.
/// Portals are one way, and are not visible to the player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Portal {
    /// Name of the portal
    pub name: String,
    /// Map name where the user is traveling to.
    pub target: String,
    /// Coordinates where the user is traveling to in the map. row, col
    pub location: (i32, i32),
}

impl Portal {
    /// Constructor for the Portal struct.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the portal.
    /// * `target` - A string that is the name of the map the portal is targeting.
    /// * `location` - A tuple of i32s that is the coordinates of the portal. (row, col)
    ///
    /// # Returns
    /// * `Portal` - A new Portal.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let portal = map::Portal::new(String::from("Test Portal"), String::from("Test Area"), (1, 1));
    /// assert_eq!(portal.name, "Test Portal");
    /// assert_eq!(portal.target, "Test Area");
    /// assert_eq!(portal.location, (1, 1));
    /// ```
    pub fn new(name: String, target: String, location: (i32, i32)) -> Portal {
        Portal {
            name,
            target,
            location,
        }
    }
}

/// A function that loads maps from the database.
///
/// # Arguments
/// * `map_name` - A string that is the name of the map to load.
///
/// # Returns
/// * `Result<Map, &str>` - A result that is Ok, or an error message.
pub fn load_map(map_name: &str, path: Option<String>) -> Result<Map, &str> {
    let path = match path {
        Some(p) => p,
        None => String::from(crate::DB_PATH),
    };
    let path = path.replace("~", std::env::var("HOME").unwrap().as_str());
    let conn = Connection::open(path.as_str()).map_err(|_| "Unable to open database.")?;
    let mut stmt = conn.prepare("SELECT name, grid FROM maps WHERE name = ?1").map_err(|_| "Unable to prepare statement.")?;
    let mut rows = stmt.query(&[&map_name]).map_err(|_| "Unable to query database.")?;
    let row = match rows.next() {
        Ok(Some(r)) => r,
        Ok(None) => return Err("No map found."),
        Err(_) => return Err("Unable to get row."),
    };
    let name = row.get(0).map_err(|_| "Unable to get name.")?;
    let grid_string: String = row.get(1).map_err(|_| "Unable to get grid.")?;
    let grid: Vec<Vec<Option<GridSquare>>> = serde_json::from_str(grid_string.as_str()).map_err(|_| "Unable to deserialize grid.")?;
    Ok(Map { name, grid })
}

/// A grid square is a struct that represents a square on the map grid.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum GridSquare {
    Room(Room),
    Portal(Portal),
}

/// Macro for creating a grid square that is a room, and takes in two string slices.
#[macro_export]
macro_rules! room {
    ($name:expr, $description:expr) => {
        GridSquare::Room(Room::new(String::from($name), String::from($description)))
    };
}

/// Macro for creating a grid square that is a portal, and takes in two string slices and a tuple
/// of i32s
#[macro_export]
macro_rules! portal {
    ($name:expr, $target:expr, $location:expr) => {
        GridSquare::Portal(Portal::new(
            String::from($name),
            String::from($target),
            $location,
        ))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the grid room macro.
    #[test]
    fn create_a_grid_room() {
        let room = room!("Test Room", "This is a test room.");
        assert_eq!(
            GridSquare::Room(Room::new(
                String::from("Test Room"),
                String::from("This is a test room.")
            )),
            room
        );
    }

    /// Test the grid portal macro.
    #[test]
    fn create_a_grid_portal() {
        let portal = portal!("Test Portal", "Test Area", (1, 1));
        assert_eq!(
            GridSquare::Portal(Portal::new(
                String::from("Test Portal"),
                String::from("Test Area"),
                (1, 1)
            )),
            portal
        );
    }

    #[test]
     fn load_map_test() {
         // Create an in memory database.
         crate::migration::map::migrate_up(Some(String::from("test.db"))).unwrap();
         let map = load_map("Test Area", Some(String::from("test.db"))).unwrap();
         std::fs::remove_file("test.db").unwrap();
         assert_eq!(map.name, "Test Area");
         assert_eq!(map.grid.len(), 3);
     }
}
