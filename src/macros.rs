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
    use crate::game::map::*;
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
}
