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
						let r_min = 0.max(row_col_avg / 8);
						let r_max = row_col_avg / 2;
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
		/*
		Essentially, algorithm needs to do following:
		Identify all room starts, placing them in struct that saves locations
		of each room generated from each start as a rectangle pointing to x,y,w,h
		During each iteration of growth, get list of squares adjacent to room.
		For each square adjacent to room, determine number of rooms adjacent,
		by saving a "claim" based on room coordinates
		For each adjacent square, if only one claim, add those squares to room claimant
		if adjacent square has more than one claimant, then convert to wall, and tell all
		claiming rooms to stop growing.
		This continues until all rooms have been told to stop growing.

		Thus, our room struct needs to save:
		- x,y,w,h of current room rectangle
		- initial x,y of room start (for identification purposes)
		- whether or not room is currently allowed to grow
		We also should implement functions:
		- get list of square coords adjacent to room
		- consume iter of square_claim to expand room
		We also need a square_claim struct to save:
		- x,y of current square
		- list of references to rooms which hold a claim
		 */
		
		match &self.squares {
			Some(squares) => {
				// create a list of Rooms from all room starts
				let mut room_starts = {
					let mut room_starts = Vec::new();
					for row in 0..*squares.rows() {
						for col in 0..*squares.cols() {
							match squares.get(&row, &col) {
								Some(square) => {
									if CRGC::classify(*square.color()) == CRGC::RoomStart {
										room_starts.push(RoomFromStart::from_square_coord(col, row, room_starts.len()));
									}//end if we found a room start
								}, None => println!("Couldn't get square at row:{}, col:{}", row, col)
							}//end matching whether we can acces square
						}//end looping over col indices
					}//end looping over row indices
					room_starts
				};

				// continue growing rooms while some are allowed to grow
				while room_starts.iter().filter(|elem| elem.allowed_growth).count() > 0 {
					// create list of claims for all squares adjacent
					// to rooms with allowed growth
					let mut all_claims = Vec::new();
					for room in room_starts.iter().filter(|elem| elem.allowed_growth) {
						let corner_claims = room.get_corner_coords(squares.cols() - 1, squares.rows() - 1);
						for claim in RoomFromStart::corner_coords_to_square(&corner_claims, room.index) {
							SquareClaim::update_claims(claim.x, claim.y, &mut all_claims, room.index);
						}//end looping over claims on corner coordinates
					}//end looping over each room

					// create parallel list to room_starts,
					// room -> (Iter<SquareClaims>)
					for room in room_starts.iter_mut() {
						let room_index = room.index.clone();
						let claims = all_claims.iter().filter(|elem| elem.claimants_indices.contains(&room_index)).cloned();
						let allow_growth = claims.clone().filter(|elem| elem.claimants_indices.len() > 1).count() == 0;
						room.allowed_growth = allow_growth;
						if room.allowed_growth {
							let prev = (room.x, room.y, room.w, room.h);
							room.consume_claims(claims);
							let cur = (room.x, room.y, room.w, room.h);
							if prev == cur {
								room.allowed_growth = false;
							}//end if room is trying to grow but can't
						}//end if room is allowed to grow
					}//end looping over rooms

					// keep repeating this process until nothing is able to grow anymore
				}//end looping while at least one element is allowed to grow

				// update squares, draw room floors and walls.
				let mut squares_clone = squares.clone();
				for room in room_starts {
					// Paint squares within rooms as Floor
					{for col in room.x..(room.x + room.w) {
						for row in room.y..(room.y + room.h) {
							match squares_clone.get_mut(&row, &col) {
								Some(square) => {
									match CRGC::classify(*square.color()) {
										CRGC::Empty | CRGC::Other(_) | CRGC::RoomStart => {
											square.set_color(CRGC::Floor.color());
										}, _ => ()
									}//end matching color classification of square
								}, None => println!("Couldn't access square at row:{}, col:{}, painting floor", row, col)
							}//emd matching whether we can get squares
						}//end looping through row indices
					}}//looping through col indices
					
					// paint borders of rooms as walls
					{let border_coords = 
						RoomFromStart::corner_coords_to_square(&room.get_corner_coords(squares.cols() - 1, squares.rows() - 1), room.index);
						for (row,col) in border_coords.iter().map(|elem| (elem.y, elem.x)) {
							match squares_clone.get_mut(&row, &col) {
								Some(square) => {
									match CRGC::classify(*square.color()) {
										CRGC::Empty | CRGC::Other(_) => {
											square.set_color(CRGC::Wall.color());
										}, _ => ()
									}//end making sure this square is empty before painting wall
								}, None => println!("Couldn't access square at row:{}, col:{}, painting wall", row, col)
							}//end matching whether we accessed square properly
						}//end looping over coords which border room
					}//end painting border squares
				}//end looping through rooms we've grown from room starts

				// update squares with the changes we've made
				self.squares = Some(squares_clone);

				Ok(())
			}, None => Err(format!("No Squares Set"))
		}//end matching whether we have squares
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

/// Struct to hold some information about rectangular rooms,
/// used as helper struct when growing rooms from RoomStarts.
#[derive(Clone, Copy, Debug, PartialOrd)]
struct RoomFromStart {
	x: usize,
	y: usize,
	w: usize,
	h: usize,
	index: usize,
	init_x: usize,
	init_y: usize,
	allowed_growth: bool,
}//end struct Room

impl PartialEq for RoomFromStart {
	fn eq(&self, other: &Self) -> bool {
		self.init_x == other.init_x && self.init_y == other.init_y
	}//end eq
}//end impl PartialEq for Room

impl RoomFromStart {
	fn from_square_coord(x: usize, y: usize, index: usize) -> RoomFromStart {
		RoomFromStart { x, y, w: 1, h: 1, index, init_x: x, init_y: y, allowed_growth: true }
	}//end from_square_coord(x,y)
	
	/// Returns coords of four adjacent corners.  
	/// It is possible that corner coords might be within Room
	fn get_corner_coords(&self, max_x: usize, max_y: usize) -> Vec<SquareClaim> {
		let mut corners = Vec::new();
		let low_x = self.x.checked_sub(1).unwrap_or(0);
		let low_y = self.y.checked_sub(1).unwrap_or(0);
		let hih_x = max_x.min(self.x + self.w);
		let hih_y = max_y.min(self.y + self.h);
		corners.push(SquareClaim::new(low_x, low_y, self.index));
		corners.push(SquareClaim::new(low_x, hih_y, self.index));
		corners.push(SquareClaim::new(hih_x, low_y, self.index));
		corners.push(SquareClaim::new(hih_x, hih_y, self.index));
		return corners;
	}//end get_corner coords

	fn corner_coords_to_square(corners: &Vec<SquareClaim>, room_index: usize) -> Vec<SquareClaim> {
		let mut min_x = usize::MAX;
		let mut min_y = usize::MAX;
		let mut max_x = 0;
		let mut max_y = 0;
		for corner in corners {
			if corner.x < min_x {min_x = corner.x}
			else if corner.x > max_x {max_x = corner.x}
			if corner.y < min_y {min_y = corner.y}
			else if corner.y > max_y {max_y = corner.y}
		}//end determineing min and max x and y

		let mut square_claims = Vec::new();
		for x in min_x..=max_x {
			let claim1 = SquareClaim::new(x,min_y,room_index);
			let claim2 = SquareClaim::new(x,max_y,room_index);
			if !square_claims.contains(&claim1) {square_claims.push(claim1);}
			if !square_claims.contains(&claim2) {square_claims.push(claim2);}
		}//end looping over x values
		for y in min_y..=max_y {
			let claim1 = SquareClaim::new(min_x,y,room_index);
			let claim2 = SquareClaim::new(max_x,y,room_index);
			if !square_claims.contains(&claim1) {square_claims.push(claim1);}
			if !square_claims.contains(&claim2) {square_claims.push(claim2);}
		}//end looping over y values

		return square_claims;
	}//end corner_coords_to_squares

	fn consume_claims<I: Iterator<Item = SquareClaim>>(&mut self, claims: I) {
		let mut min_x = self.x;
		let mut min_y = self.y;
		let mut max_x = self.x + self.w - 1;
		let mut max_y = self.y + self.h - 1;
		for claim in claims {
			if claim.x < min_x {min_x = claim.x}
			else if claim.x > max_x {max_x = claim.x}
			if claim.y < min_y {min_y = claim.y}
			else if claim.y > max_y {max_y = claim.y}
		}//end looping over claims, getting min and max x and y
		if min_x > max_x || min_y > max_y {
			println!("WHAT. min-max x:{},{} | y:{},{}", min_x, max_x, min_y, max_y)
		}
		self.x = min_x;
		self.y = min_y;
		self.w = max_x - self.x + 1;
		self.h = max_y - self.y + 1;
	}//end consume_claims
}//end impl Room

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct SquareClaim {
	x: usize,
	y: usize,
	claimants_indices: Vec<usize>
}//end struct SquareClaim

impl SquareClaim {
	fn new(x: usize, y: usize, claim_index: usize) -> SquareClaim {
		let mut tmp_vec = Vec::new(); tmp_vec.push(claim_index);
		SquareClaim {x, y, claimants_indices: tmp_vec}
	}//end new(x,y,claim)

	/// Updates a list of claims with a new claim, returning true if the square
	/// provided is already being claimed by another room.
	fn update_claims<'room>(x_new: usize, y_new: usize, claims: &mut Vec<SquareClaim>, room_claim_index: usize) -> bool {
		let mut found_same = false;
		for claim in claims.iter_mut() {
			if claim.x == x_new && claim.y == y_new {
				found_same = true;
				claim.claimants_indices.push(room_claim_index);
			}//end if we found a match
		}//end looping over all claims to look for match
		if !found_same { claims.push(SquareClaim::new(x_new, y_new, room_claim_index)); }
		found_same
	}//end update claims
}//end impl SquareClaim

/// Constrained Room Growth Classification (based on Color)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CRGC {
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
			(140,140,140) => CRGC::Floor,
			_ => CRGC::Other(color),
		}//end matching color
	}//end classify()

	pub fn color(&self) -> (u8,u8,u8) {
		match self {
			CRGC::Door => (0,0,255),
			CRGC::Empty => (255,255,255),
			CRGC::Floor => (140,140,140),
			CRGC::RoomStart => (255,0,0),
			CRGC::Stairs => (0,255,0),
			CRGC::Wall => (0,0,0),
			CRGC::Other(c) => *c,
		}//end matching self
	}//end color()

}//end impl for CRGC