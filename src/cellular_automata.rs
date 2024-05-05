use crate::squares::SquareGrid;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
pub struct CA {
	/// Moore neighborhood size.
	/// Cells within this distance are considered for neighborhood.
	pub neighborhood_size: usize,
	/// Number of wall cells in neighborhood to turn to wall.
	pub neighborhood_threshold: usize,
	generations_so_far: usize,
	squares: Option<SquareGrid>,
}//end struct CA

impl Default for CA {
	/// default size 1 and threshold 5
	fn default() -> Self {
		Self { neighborhood_size: 1, neighborhood_threshold: 5, generations_so_far: 0, squares: None }
	}//end default()
}//end impl Deafult default for CA

#[allow(dead_code)]
impl CA {
	/// Instantiates object with custom parameters, 
	/// though no squares.
	pub fn new(neighborhood_size: usize, neighborhood_threshold: usize) -> CA {CA{neighborhood_size, neighborhood_threshold, generations_so_far: 0, squares: None }}

	pub fn generations_so_far(&self) -> &usize {&self.generations_so_far}
	pub fn squares(&self) -> &Option<SquareGrid> {&self.squares}

	/// Sets this object to use a particular collection 
	/// of squares, resetting [generations_so_far].
	pub fn set_squares(&mut self, squares: SquareGrid) {self.squares = Some(squares); self.generations_so_far = 0; }
	/// Convenience function for setting squares.
	pub fn with_squares(mut self, squares: SquareGrid) -> CA {self.squares = Some(squares); self.generations_so_far = 0; self}
	/// convenience function for getting a reference to squares
	pub fn get_squares(&self) -> &Option<SquareGrid> { &self.squares }
	/// Returns SquareGrid object with ownership, setting self.sqaures to None
	pub fn pop_squares(&mut self) -> Option<SquareGrid> { let squares = self.squares.clone(); self.squares = None; squares }

	/// Counts the number of neighbors with target CA Color/Classification.  
	/// It might be noted that this count does not include the value at [row], [col].  
	/// - If bounds row/column's neighborhood is partially cut off by bounds, 
	/// then count will be a bit higher. In cases of something going wrong 
	/// despite bounds checking, a message might be println!()-ed, but 
	/// the function shouldn't panic or return None.  
	/// - The function should only return None if self.squares is None.  
	/// - Since it is assumed that this function will be called with target == CAC::Wall,  
	/// it is set up so that outermost cells are likely to stay walls.
	fn neighbor_count(&self, row: usize, col: usize, target: CAC) -> Option<usize> {
		match &self.squares {
			Some(squares) => {
				let mut target_count = 0;
		
				// get bounds for valid row and cols, doing bounds checking
				// squares out of bounds are counted as targets
				let low_row: usize;
				if row < self.neighborhood_size { low_row = 0; target_count += 3; } else { low_row = row - self.neighborhood_size; }
				let low_col: usize;
				if col < self.neighborhood_size { low_col = 0; target_count += 3; } else { low_col = col - self.neighborhood_size; }
				let hih_row: usize;
				if row + self.neighborhood_size > *squares.rows() - 1 { hih_row = *squares.rows() - 1; target_count += 3; }
				else { hih_row = row + self.neighborhood_size; }
				let hih_col: usize;// = col + self.neighborhood_size;
				if col + self.neighborhood_size > *squares.cols() - 1 { hih_col = *squares.cols() - 1; target_count += 3; }
				else { hih_col = col + self.neighborhood_size; }

				// Count number of neighbors which CAC::classify(color) == target
				for col_idx in low_col..(hih_col + 1) {
					for row_idx in low_row..(hih_row + 1) {
						if row_idx == row && col_idx == col { continue; }
						match squares.get(&row_idx, &col_idx) {
							Some(square) => if CAC::classify(*square.color()) == target { target_count += 1; },
							None => println!("Somehow we had an invalid index when counting neighborhoods even though we pre-checked that."),}
					}//end looping over row indices in neighborhood of row, col
				}//end looping over col indices in neighborhood of row, col
		
				Some(target_count)
			}, None => {None}}
	}//end neighbor_count

	/// Creates a vector parallel to squares.  
	/// Needless to say, this function returns None if self.squares is None.  
	/// This vector provides the number of neighbors with target CAC of every square at once.  
	/// This is achieved through repeated calls to [neighbor_count()]
	fn all_neighbor_count(&self, target: CAC) -> Option<Vec<usize>> {
		match &self.squares {
			Some(squares) => {
				let mut neighbor_count_all = Vec::new();

				for row_idx in 0..*squares.rows() {
					for col_idx in 0..*squares.cols() {
						match self.neighbor_count(row_idx, col_idx, target) {
							Some(count) => neighbor_count_all.push(count),
							None => return None, }
					}//end looping over cols
				}//end looping over rows

				Some(neighbor_count_all)
			}, None => None,}
	}//end all_neighbor_count

	/// Runs a single generation of cellular automata with the given settings.  
	/// Changes will be made to self.squares to reflect these changes.
	/// 
	/// If squares is None, no changes will be made, and false will be returned.
	pub fn run_generation(&mut self) -> bool {
		match &self.squares {
			Some(squares) => {
				// get neighbor count for each square
				match self.all_neighbor_count(CAC::Wall) {
					Some(parallel_neighbor_count) => {
						let mut cur_squares = squares.clone();

						for (i, square) in cur_squares.iter_mut().enumerate() {
							let this_neighbor_count = parallel_neighbor_count[i];
							// set color based on neighbor count
							let this_cac = CAC::classify(*square.color());
							if this_cac == CAC::Floor || this_cac == CAC::Wall {
								if this_neighbor_count >= self.neighborhood_threshold {
									square.set_color(CAC::Wall.color());
								} else { square.set_color(CAC::Floor.color()); }
							}//end if this is already a wall or floor
						}//end changing color of each square based on neighbor count

						// variable maintenance
						self.squares = Some(cur_squares);
						self.generations_so_far += 1;

						true
					}, None => false, }
			}, None => false,}
	}//end run_generation
}//end impl for CA

/// CA Color/Classification
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum CAC {
	Wall,
	Floor,
	Stairs,
	Other,
}//end enum CAC

#[allow(dead_code)]
impl CAC {
	pub fn classify(color: (u8,u8,u8)) -> CAC {
		match color {
			(0,0,0) => CAC::Wall,
			(255,255,255) => CAC::Floor,
			(0,255,0) => CAC::Stairs,
			_ => CAC::Other,
		}//end matching color to CAC
	}//end classify

	pub fn color(&self) -> (u8,u8,u8) {
		match self {
			CAC::Wall => (0,0,0),
			CAC::Floor => (255,255,255),
			CAC::Stairs => (0,255,0),
			CAC::Other => (50,50,50),
		}//end matching CAC to color
	}//end color()

	pub fn colors_vec() -> Vec<(u8,u8,u8)> {
		let mut color_vec = Vec::new();
		color_vec.push((0,0,0));
		color_vec.push((255,255,255));
		color_vec.push((0,255,0));
		color_vec
	}
}//end impl for CAC