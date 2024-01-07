//! # Room
//! Module that represents a location in the game world.

/// A struct that represents a location in the game world.
pub struct Room {
    /// The name of the room. Value must be unique.
    pub name: String,
    /// The description of the room.
    pub description: String,
    /// The room to the north of this room.
    pub north: Box<Option<Room>>,
    /// The room to the south of this room.
    pub south: Box<Option<Room>>,
    /// The room to the east of this room.
    pub east: Box<Option<Room>>,
    /// The room to the west of this room.
    pub west: Box<Option<Room>>,
}

impl Room {
    /// Room constructor.
    ///
    /// # Arguments
    /// * `name` - A string that is the name of the room. Value must be unique.
    /// * `description` - A string that is the description of the room.
    /// * `north` - A Box<Option<Room>> that is the room to the north of this room.
    /// * `south` - A Box<Option<Room>> that is the room to the south of this room.
    /// * `east` - A Box<Option<Room>> that is the room to the east of this room.
    /// * `west` - A Box<Option<Room>> that is the room to the west of this room.
    ///
    /// # Returns
    /// * `Room` - A new Room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::room;
    ///
    /// let room = room::Room::new(String::from("Room 1"), String::from("This is room 1."));
    /// ```
    pub fn new(name: String, description: String) -> Room {
        Room {
            name,
            description,
            north: Box::new(None),
            south: Box::new(None),
            east: Box::new(None),
            west: Box::new(None),
        }
    }

    /// A facade function that sets the room to the north of this room.
    /// 
    /// # Arguments
    /// * `room` - A Room that is the room to the north of this room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::room;
    ///
    /// let mut room1 = room::Room::new(String::from("Room 1"), String::from("This is room 1."));
    /// let room2 = room::Room::new(String::from("Room 2"), String::from("This is room 2."));
    /// room1.set_north(room2);
    /// ```
    pub fn set_north(&mut self, room: Room) {
        self.north = Box::new(Some(room));
    }

    /// A facade function that sets the room to the south of this room.
    ///
    /// # Arguments
    /// * `room` - A Room that is the room to the south of this room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::room;
    ///
    /// let mut room1 = room::Room::new(String::from("Room 1"), String::from("This is room 1."));
    /// let room2 = room::Room::new(String::from("Room 2"), String::from("This is room 2."));
    /// room1.set_south(room2);
    /// ```
    pub fn set_south(&mut self, room: Room) {
        self.south = Box::new(Some(room));
    }

    /// A facade function that sets the room to the east of this room.
    /// 
    /// # Arguments
    /// * `room` - A Room that is the room to the east of this room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::room;
    ///
    /// let mut room1 = room::Room::new(String::from("Room 1"), String::from("This is room 1."));
    /// let room2 = room::Room::new(String::from("Room 2"), String::from("This is room 2."));
    /// room1.set_east(room2);
    /// ```
    pub fn set_east(&mut self, room: Room) {
        self.east = Box::new(Some(room));
    }

    /// A facade function that sets the room to the west of this room.
    ///
    /// # Arguments
    /// * `room` - A Room that is the room to the west of this room.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::room;
    ///
    /// let mut room1 = room::Room::new(String::from("Room 1"), String::from("This is room 1."));
    /// let room2 = room::Room::new(String::from("Room 2"), String::from("This is room 2."));
    /// room1.set_west(room2);
    /// ```
    pub fn set_west(&mut self, room: Room) {
        self.west = Box::new(Some(room));
    }
}
