pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn near(&self, pos: &Position, distance: u32) -> bool {
        (self.x + distance).gt(&pos.x)
            && (self.x.saturating_sub(distance)).lt(&pos.x)
            && (self.y + distance).gt(&pos.y)
            && (self.y.saturating_sub(distance)).lt(&pos.y)
    }
}

#[cfg(test)]
mod position_tests {
    use std::ops::Not;

    use super::Position;

    #[test]
    fn near_position_should_pass() {
        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 11, y: 9 };
        assert!(pos1.near(&pos2, 2));
        assert!(pos2.near(&pos1, 2));

        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 10, y: 10 };
        assert!(pos1.near(&pos2, 1));
        assert!(pos2.near(&pos1, 1));

        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 55, y: 10 };
        assert!(pos1.near(&pos2, 48));
        assert!(pos2.near(&pos1, 48));
    }

    #[test]
    fn near_position_should_fail() {
        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 12, y: 9 };
        assert!(pos1.near(&pos2, 2).not());
        assert!(pos2.near(&pos1, 2).not());

        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 10, y: 15 };
        assert!(pos1.near(&pos2, 5).not());
        assert!(pos2.near(&pos1, 5).not());

        let pos1 = Position { x: 100, y: 10 };
        let pos2 = Position { x: 10, y: 15 };
        assert!(pos1.near(&pos2, 50).not());
        assert!(pos2.near(&pos1, 50).not());

        let pos1 = Position { x: 10, y: 10 };
        let pos2 = Position { x: 10, y: 10 };
        assert!(pos1.near(&pos2, 0).not());
        assert!(pos2.near(&pos1, 0).not());
    }
}
