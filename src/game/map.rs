//! # Map
//! Module that represents a location in the game world.

/// A struct that represents a map in the game world.
#[derive(Debug)]
pub struct Map {
    /// The name of the map. Value must be unique.
    pub name: String,
    /// A grid of rooms in the game world.
    rooms: Vec<Vec<Option<Room>>>
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
        for _ in 0..x {
            let mut row = vec![];
            for _ in 0..y {
                row.push(None);
            }
            rooms.push(row);
        }
        Map {
            name,
            rooms
        }
    }

    /// A safe way to get a room from the map.
    ///
    /// # Arguments
    /// * `x` - An usize that is the x coordinate of the room.
    /// * `y` - An usize that is the y coordinate of the room.
    ///
    /// # Returns
    /// * `Option<Room>` - An option that is the room at the given coordinates, or None.
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
    pub fn get_room(&self, x: usize, y: usize) -> Option<&Room> {
        if self.rooms.len() < x || self.rooms[x].len() < y {
            return None
        }
        let room = &self.rooms[x][y];
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
        if self.rooms.len() < x || self.rooms[x].len() < y {
            return Err("Index out of bounds.")
        }
        self.rooms[x][y] = Some(room);
        Ok(())
    }
}

/// A struct that represents a location in the game world.
#[derive(Debug)]
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
