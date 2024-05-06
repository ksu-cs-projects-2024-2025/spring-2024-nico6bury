use crate::squares::SquareGrid;


/// Struct for handling processing for
/// Constrained Room Growth
pub struct CRG {
	squares: Option<SquareGrid>,
}//end struct CRG

impl Default for CRG {
	fn default() -> Self {
		Self {
			squares: None,
		}//end struct construction
	}//end default()
}//end impl Default for CRG

impl CRG {
	pub fn squares(&self) -> &Option<SquareGrid> {&self.squares}

	/// Sets this object to use a particular collection of squares.
	pub fn set_squares(&mut self, squares: SquareGrid) {self.squares = Some(squares)}
	/// Convenience function for setting squares. 
	pub fn with_squares(mut self, squares: SquareGrid) -> CRG {self.squares = Some(squares); self}
	/// Convenience function for getting a reference to squares.
	pub fn get_squares(&self) -> Option<&SquareGrid> {match &self.squares {Some(sqrs) => Some(sqrs), None => None}}
	/// Returns SquareGrid object with ownership, setting self.squares to None.
	pub fn pop_squares(&mut self) -> Option<SquareGrid> {let squares = self.squares.clone(); self.squares = None; squares}

	/// Randomly adds room starts to squares.  
	/// Changes will be made to self.squares to reflect these changes.
	/// 
	/// If squares is None, or something else goes wrong, then no changes will
	/// be made. If squares is not changed, this function will return Err.
	/// 
	/// Optionally, you can specify the number of starts to be produced.
	/// If rooms is None, then a random number of room starts will be generated.
	pub fn add_random_room_starts(&mut self, rooms: Option<usize>) -> Result<(),String> {
		Err(format!("Not implemented yet."))
	}//end add_random_room_starts(self, rooms)

	/// Grows rooms from room starts in squares.  
	/// Changes will be made to self.squares to reflect these changes.  
	/// 
	/// This function only handles rectangular constrained growth.
	/// 
	/// If squares is None, or something else goes wrong, then no changes will
	/// be made. If squares is not changed, this function will return Err.
	pub fn grow_rooms_from_starts(&mut self) -> Result<(),String> {
		Err(format!("Not implemented yet."))
	}//end grow_rooms_from_starts(self)

	/// Grows rooms in l shapes after they've been expanded
	/// from starts.
	/// 
	/// This function might have unpredictible effects if rooms
	/// are not as expected.
	/// 
	/// If squares is None, or something else goes wrong, then no changes will
	/// be made. If squares is not changed, this function will return Err.
	pub fn grow_rooms_l_growth(&mut self) -> Result<(),String> {
		Err(format!("Not implemented yet."))
	}//end grow_rooms_l_growth(self)

	/// Calculates a connectivity number that represents
	/// minimum connectivity. This number represents the
	/// minimum number of rooms one must travel through to
	/// traverse the map.
	pub fn calculate_connectivity(&mut self) -> usize {
		todo!();
	}//end calculate_connectivity(self)

	/// Changes the number of doors throughout the map
	/// in order to make sure connectivity is at least equal
	/// to min_connectivity.
	/// 
	/// If this function is successful, changes will be
	/// reflected in squares. If something goes wrong, then
	/// no changes will be made, and this function will
	/// return Err.
	pub fn enforce_connectivity(&mut self, min_connectivity: usize) -> Result<(),String> {
		Err(format!("Not implemented yet."))
	}//end enforce connectivity(self, min_connectivity)
}//end impl for CRG

/// Constrained Room Growth Classification (based on Color)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CRGC {
	Door,
	Empty,
	Floor,
	RoomStart,
	Stairs,
	Wall,
	Other((u8,u8,u8)),
}//end enum CRGC

#[allow(dead_code)]
impl CRGC {
	pub fn classify(color: (u8,u8,u8)) -> CRGC {
		match color {
			(0,0,0) => CRGC::Wall,
			(255,255,255) => CRGC::Empty,
			(0,255,0) => CRGC::Stairs,
			(255,0,0) => CRGC::RoomStart,
			(0,0,255) => CRGC::Door,
			(230,230,230) => CRGC::Floor,
			_ => CRGC::Other(color),
		}//end matching color
	}//end classify()

	pub fn color(&self) -> (u8,u8,u8) {
		match self {
			CRGC::Door => (0,0,255),
			CRGC::Empty => (255,255,255),
			CRGC::Floor => (230,230,230),
			CRGC::RoomStart => (255,0,0),
			CRGC::Stairs => (0,255,0),
			CRGC::Wall => (0,0,0),
			CRGC::Other(c) => *c,
		}//end matching self
	}//end color()

}//end impl for CRGC