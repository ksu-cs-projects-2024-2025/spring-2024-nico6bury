use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};

use crate::squares::SquareGrid;


/// Struct for handling processing for
/// Constrained Room Growth
pub struct CRG {
	squares: Option<SquareGrid>,
	rng: ThreadRng,
}//end struct CRG

impl Default for CRG {
	fn default() -> Self {
		Self {
			squares: None,
			rng: rand::thread_rng(),
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
		match &mut self.squares {
			Some(squares) => {
				let room_num = match rooms {
					Some(r) => r,
					None => {
						let row_col_avg = squares.rows() + squares.cols() / 2;
						let r_min = 0.max(row_col_avg / 4);
						let r_max = row_col_avg;
						self.rng.gen_range(r_min..=r_max)
					}};
				
				// get list of possible squares to generate room_starts on
				let empty_list = {
					let mut empt_sqr_vec = Vec::new();
					for col in 0..*squares.cols() {
						for row in 0..*squares.rows() {
							match squares.get(&row,&col) {
								Some(sqr) => {
									match CRGC::classify(*sqr.color()) {
										CRGC::Empty => empt_sqr_vec.push((row,col)),
										_ => (),
									}//end matching color class
								}, None => println!("Couldn't access square at row:{} and col:{} while placing random starts", row, col)
							}//end matching whether we got the square
						}//end looping over row indices
					}//end looping over col indices
					empt_sqr_vec
				};

				// check that we have enough empty spaces for all room starts
				if empty_list.len() < room_num { return Err(format!("Room starts cannot be placed because we want to add {} starts, but starts can only be places on Empty squares, and there are only {} Empty squares!", room_num, empty_list.len())) }

				// randomly get [room_num] indices from empty_list
				for (row,col) in empty_list.choose_multiple(&mut self.rng, room_num) {
					match squares.get_mut(row, col) {
						Some(square) => square.set_color(CRGC::RoomStart.color()),
						None => println!("Couldn't get square at row:{} and col:{} for some reason.", row, col),
					}//end matching whether we can access the square to set color
				}//end looping over row and column indices to place a room start
				
				Ok(())
			}, None => Err(format!("Could not add random starts to squares because squares is None."))
		}//end matching whether we can access squares
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