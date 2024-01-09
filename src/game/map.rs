//! # Map
//! Module that represents a location in the game world.

/// A struct that represents a map in the game world.
#[derive(Debug)]
pub struct Map {
    /// The name of the map. Value must be unique.
    pub name: String,
    /// A grid of rooms in the game world.
    grid: Vec<Vec<Option<Room>>>
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
    pub fn new(name: String, x: i32, y: i32) -> Map {
        let mut rooms = vec![];
        // Create a grid of rooms.
        for _ in 0..y {
            let mut row = vec![];
            for _ in 0..x {
                row.push(None);
            }
            rooms.push(row);
        }
        Map {
            name,
            grid: rooms
        }
    }

    /// A safe way to get a room from the map.
    ///
    /// # Arguments
    /// * `x` - An i32 that is the x coordinate of the room.
    /// * `y` - An i32 that is the y coordinate of the room.
    ///
    /// # Returns
    /// * `Option<Room>` - An option that is the room at the given coordinates, or None.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let room = map::Room::new(String::from("Test Room"), String::from("This is a test room."));
    /// //Room formation:
    /// //x [] x
    /// //[][][]
    /// //x [] x
    /// let mut map = map::Map::new(String::from("Test Area"), 3, 3);
    /// map.set_room(1, 1, room);
    /// let result = map.get_room(1, 1);
    /// assert!(result.is_some());
    /// let result = map.get_room(0, 0);
    /// assert!(result.is_none());
    /// let result = map.get_room(-1, -1);
    /// assert!(result.is_none());
    /// let result = map.get_room(3, 3);
    /// assert!(result.is_none());
    /// ```
    pub fn get_room(&self, x: i32, y: i32) -> Option<&Room> {
        if x < 0 || y < 0 {
            return None
        }
        // We can safely assume these are positive numbers based on the check above.
        let x = x as usize;
        let y = y as usize;
        if self.grid.len() <= y || self.grid[0].len() <= x {
            return None
        }
        let room = &self.grid[x][y];
        match room {
            Some(r) => Some(&r),
            None => None,
        }
    }

    /// A safe way to set a room in the map.
    ///
    /// # Arguments
    /// * `x` - An usize that is the x coordinate of the room.
    /// * `y` - An usize that is the y coordinate of the room.
    ///
    /// # Returns
    /// * `Result<(), &str>` - A result that is Ok, or an error message.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::map;
    ///
    /// let room = map::Room::new(String::from("Test Room"), String::from("This is a test room."));
    /// let mut map = map::Map::new(String::from("Test Area"), 3, 3);
    /// map.set_room(1, 1, room);
    /// let result = map.get_room(1, 1);
    /// assert!(result.is_some());
    /// ```
    pub fn set_room(&mut self, x: usize, y: usize, room: Room) -> Result<(), &str> {
        if self.grid.len() < x || self.grid[x].len() < y {
            return Err("Index out of bounds.")
        }
        self.grid[x][y] = Some(room);
        Ok(())
    }
}

/// A struct that represents a location in the game world.
#[derive(Debug, PartialEq)]
pub struct Room {
    /// The name of the room.
    pub name: String,
    /// The description of the room.
    pub description: String
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
        Room {
            name,
            description
        }
    }
}

/// A portal is a struct that teleports a player to another map at a set of coordinates.
/// Portals are one way, and are not visible to the player.
#[derive(Debug, PartialEq)]
pub struct Portal {
    /// Name of the portal
    pub name: String,
    /// Map name where the user is traveling to.
    pub target: String,
    /// Coordinates where the user is traveling to in the map.
    pub location: (i32, i32),
}

impl Portal {
    /// Constructor for the Portal struct.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the portal.
    /// * `target` - A string that is the name of the map the portal is targeting.
    /// * `location` - A tuple of i32s that is the coordinates of the portal.
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

/// A grid square is a struct that represents a square on the map grid.
#[derive(Debug, PartialEq)]
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
        GridSquare::Portal(Portal::new(String::from($name), String::from($target), $location))
    };
}

pub fn test_area() -> Map {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the grid room macro.
    #[test]
    fn create_a_grid_room() {
        let room = room!("Test Room", "This is a test room.");
        assert_eq!(GridSquare::Room(Room::new(String::from("Test Room"), String::from("This is a test room."))), room);
    }

    /// Test the grid portal macro.
    #[test]
    fn create_a_grid_portal() {
        let portal = portal!("Test Portal", "Test Area", (1, 1));
        assert_eq!(GridSquare::Portal(Portal::new(String::from("Test Portal"), String::from("Test Area"), (1, 1))), portal);
    }
}
