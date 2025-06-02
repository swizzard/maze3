use multid::{BoundedIx2, V2, iterators};
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DoorState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Doors {
    pub north: Option<DoorState>,
    pub east: Option<DoorState>,
    pub south: Option<DoorState>,
    pub west: Option<DoorState>,
}

impl Doors {
    fn open_north(&mut self) {
        if self.north.is_some() {
            self.north = Some(DoorState::Open)
        }
    }
    fn open_east(&mut self) {
        if self.east.is_some() {
            self.east = Some(DoorState::Open)
        }
    }
    fn open_south(&mut self) {
        if self.south.is_some() {
            self.south = Some(DoorState::Open)
        }
    }
    fn open_west(&mut self) {
        if self.west.is_some() {
            self.west = Some(DoorState::Open)
        }
    }
    fn close_north(&mut self) {
        if self.north.is_some() {
            self.north = Some(DoorState::Closed)
        }
    }
    fn close_east(&mut self) {
        if self.east.is_some() {
            self.east = Some(DoorState::Closed)
        }
    }
    fn close_south(&mut self) {
        if self.south.is_some() {
            self.south = Some(DoorState::Closed)
        }
    }
    fn close_west(&mut self) {
        if self.west.is_some() {
            self.west = Some(DoorState::Closed)
        }
    }
}

impl Default for Doors {
    fn default() -> Self {
        Self {
            north: Some(DoorState::Closed),
            east: Some(DoorState::Closed),
            south: Some(DoorState::Closed),
            west: Some(DoorState::Closed),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Room {
    pub description: String,
    pub doors: Doors,
}

#[derive(Debug, Clone)]
pub struct Maze<const N_ROWS: usize, const N_COLS: usize> {
    pub rooms: V2<Room, N_ROWS, N_COLS>,
    pub current_ix: BoundedIx2<N_ROWS, N_COLS>,
    pub goal: BoundedIx2<N_ROWS, N_COLS>,
}

impl<const N_ROWS: usize, const N_COLS: usize> Maze<N_ROWS, N_COLS> {
    pub fn new() -> Self {
        let ixs = iterators::V2Indices::<N_ROWS, N_COLS>::new();
        let mut rooms: Vec<Room> = Vec::with_capacity(N_ROWS * N_COLS);
        for ix in ixs {
            let r = Room {
                description: format!("room {ix:?}"),
                doors: Doors {
                    north: ix.north().map(|_| DoorState::Closed),
                    east: ix.east().map(|_| DoorState::Closed),
                    south: ix.south().map(|_| DoorState::Closed),
                    west: ix.west().map(|_| DoorState::Closed),
                },
            };
            rooms.push(r);
        }
        Self {
            rooms: V2::new(rooms).unwrap(),
            current_ix: BoundedIx2::new(0, 0).unwrap(),
            goal: BoundedIx2::<N_ROWS, N_COLS>::max(),
        }
    }
    pub fn open_north(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.open_north();
        if let Some(r) = self.rooms.get_mut(ix.north()) {
            r.doors.open_south();
        }
    }
    pub fn open_east(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.open_east();
        if let Some(r) = self.rooms.get_mut(ix.east()) {
            r.doors.open_west();
        }
    }
    pub fn open_south(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.open_south();
        if let Some(r) = self.rooms.get_mut(ix.south()) {
            r.doors.open_north();
        }
    }
    pub fn open_west(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.open_west();
        if let Some(r) = self.rooms.get_mut(ix.west()) {
            r.doors.open_east();
        }
    }
    pub fn close_north(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.close_north();
        if let Some(r) = self.rooms.get_mut(ix.north()) {
            r.doors.close_south();
        }
    }
    pub fn close_east(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.close_east();
        if let Some(r) = self.rooms.get_mut(ix.east()) {
            r.doors.close_west();
        }
    }
    pub fn close_south(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.close_south();
        if let Some(r) = self.rooms.get_mut(ix.south()) {
            r.doors.close_north();
        }
    }
    pub fn close_west(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) {
        self.rooms[ix].doors.close_west();
        if let Some(r) = self.rooms.get_mut(ix.west()) {
            r.doors.close_east();
        }
    }
    pub fn move_north(&mut self) -> bool {
        match self.rooms[self.current_ix].doors.north {
            Some(DoorState::Open) => {
                self.current_ix = self.current_ix.north().unwrap();
                true
            }
            _ => false,
        }
    }
    pub fn move_south(&mut self) -> bool {
        match self.rooms[self.current_ix].doors.south {
            Some(DoorState::Open) => {
                self.current_ix = self.current_ix.south().unwrap();
                true
            }
            _ => false,
        }
    }
    pub fn move_east(&mut self) -> bool {
        match self.rooms[self.current_ix].doors.east {
            Some(DoorState::Open) => {
                self.current_ix = self.current_ix.east().unwrap();
                true
            }
            _ => false,
        }
    }
    pub fn move_west(&mut self) -> bool {
        match self.rooms[self.current_ix].doors.west {
            Some(DoorState::Open) => {
                self.current_ix = self.current_ix.west().unwrap();
                true
            }
            _ => false,
        }
    }
    pub fn is_done(&self) -> bool {
        self.current_ix == self.goal
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Default for Maze<N_ROWS, N_COLS> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use multid::BoundedIx2;

    #[test]
    fn test_new() {
        let m = Maze::<3, 3>::new();
        assert_eq!(
            Doors {
                north: None,
                east: Some(DoorState::Closed),
                south: Some(DoorState::Closed),
                west: None,
            },
            m.rooms[BoundedIx2::<3, 3>::new(0, 0).unwrap()].doors,
            "0, 0"
        );
        assert_eq!(
            Doors {
                north: None,
                east: Some(DoorState::Closed),
                south: Some(DoorState::Closed),
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(0, 1).unwrap()].doors,
            "0, 1"
        );
        assert_eq!(
            Doors {
                north: None,
                east: None,
                south: Some(DoorState::Closed),
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(0, 2).unwrap()].doors,
            "0, 2"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: Some(DoorState::Closed),
                south: Some(DoorState::Closed),
                west: None,
            },
            m.rooms[BoundedIx2::<3, 3>::new(1, 0).unwrap()].doors,
            "1, 0"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: Some(DoorState::Closed),
                south: Some(DoorState::Closed),
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(1, 1).unwrap()].doors,
            "1, 1"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: None,
                south: Some(DoorState::Closed),
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(1, 2).unwrap()].doors,
            "1, 2"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: Some(DoorState::Closed),
                south: None,
                west: None,
            },
            m.rooms[BoundedIx2::<3, 3>::new(2, 0).unwrap()].doors,
            "2,0"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: Some(DoorState::Closed),
                south: None,
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(2, 1).unwrap()].doors,
            "2,1"
        );
        assert_eq!(
            Doors {
                north: Some(DoorState::Closed),
                east: None,
                south: None,
                west: Some(DoorState::Closed),
            },
            m.rooms[BoundedIx2::<3, 3>::new(2, 2).unwrap()].doors,
            "2,2"
        );
    }

    #[test]
    fn test_open_east() {
        let mut m = Maze::<3, 3>::new();
        let ix = BoundedIx2::<3, 3>::new(0, 0).unwrap();
        dbg!(&m.rooms[ix].doors);
        m.open_east(ix);
        assert_eq!(
            Some(DoorState::Open),
            m.rooms[ix].doors.east,
            "original room"
        );
        let ix2 = BoundedIx2::<3, 3>::new(0, 1).unwrap();
        assert_eq!(Some(DoorState::Open), m.rooms[ix2].doors.west, "neighbor");
    }
    #[test]
    fn test_open_west() {
        let mut m = Maze::<3, 3>::new();
        let ix = BoundedIx2::<3, 3>::new(0, 1).unwrap();
        dbg!(&m.rooms[ix].doors);
        m.open_west(ix);
        assert_eq!(
            Some(DoorState::Open),
            m.rooms[ix].doors.west,
            "original room"
        );
        let ix2 = BoundedIx2::<3, 3>::new(0, 0).unwrap();
        assert_eq!(Some(DoorState::Open), m.rooms[ix2].doors.east, "neighbor");
    }
}
