
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
/// Represents an atomic rectangle of pixels for use in cellular automata generations.
/// In this case, [x] and [y] refer to the x and y of ther pixel in the upper left corner 
/// of the [Square] in regards to the full image this [Square] is a part of.
pub struct Square {
	x: usize,
	y: usize,
	width: usize,
	height: usize,
	color: (u8,u8,u8),
}//end struct Square

#[allow(dead_code)]
impl Square {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> Square {
		Square {
			x,
			y,
			width,
			height,
			color: (0,255,255)
		}//end struct init
	}//end new()

	/// Allows easy setting of color with new Square.
	pub fn with_color(mut self, color: (u8,u8,u8)) -> Square { self.color = color; return self; }

	/// Sets the color of this square
	pub fn set_color(&mut self, color: (u8,u8,u8)) { self.color = color; }
	
	pub fn x(&self) -> &usize { &self.x }
	pub fn y(&self) -> &usize { &self.y }
	pub fn width(&self) -> &usize { &self.width }
	pub fn height(&self) -> &usize { &self.height }
	pub fn color(&self) -> &(u8,u8,u8) { &self.color }
}//end impl for Square

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
/// Represents all the [Squares] that represent the groups within a full image.  
/// Once loaded into a SquareGrid, the record of image dimensions can't be changed, nor can the 
/// positioning of squares.  
/// This is so that a record is always kept of the dimensions of the image when the Square data 
/// was originally taken.
pub struct SquareGrid {
	squares: Vec<Square>,
	img_width: usize,
	img_height: usize,
	rows: usize,
	cols: usize,
}//end struct SquareGrid

#[allow(dead_code)]
impl SquareGrid {
	/// - The only way to create a SquareGrid is with a vec of Squares and the dimensions of the original image.  
	/// - When a SquareGrid is created, the bounds of each Square is checked ot ensure that no Squares exceed
	/// the bounds of the given image dimensions.  
	/// - Since the bounds of Square objects cannot change after initialization, this means
	/// that all bounds in every Square object within a SquareGrid should be valid for the given image dimensions.  
	/// ## Err Conditions
	/// This function returns Err() if:  
	/// - a square is found whose bounds exceed image dimensions  
	/// ## Not Checked
	/// This function does not check if:  
	/// - squares have bounds that overlap with each other
	/// - squares completely cover all pixels within image dimensions
	pub fn from_squares(squares: Vec<Square>, img_width: usize, img_height: usize) -> Result<SquareGrid, (Square, String)> {
		// keep track of unique x values within squares, number of cols
		let mut x_col_track: Vec<usize> = Vec::new();
		// keep track of unique y values within squares, number of rows
		let mut y_row_track: Vec<usize> = Vec::new();

		for square in &squares {
			// add new x to x_col_track
			if !x_col_track.contains(&&square.x) { x_col_track.push(square.x); }
			// add new y to y_row_track
			if !y_row_track.contains(&&square.y) { y_row_track.push(square.y); }
			// do bounds checking to ensure this square fits within img_width and img_height
			if square.x + square.width > img_width || square.y + square.height > img_height {
				let complaint = format!("Square with x:{0}, y:{1}, w:{2}, h:{3} has invalid bounds.\nFurthest x,y reach of square is ({4},{5}), while image has width of {6} and height of {7}.\nThe color of this square is {8:?}.", square.x, square.y, square.width, square.height, square.x + square.width, square.y + square.height, img_width, img_height, square.color);
				return Err((*square, complaint));
			}//end if this square has invalid bounds
		}//end looping through all squares

		let col_count = x_col_track.len();
		let row_count = y_row_track.len();

		Ok(SquareGrid {
			squares,
			img_width,
			img_height,
			rows: row_count,
			cols: col_count,
		})//end struct init
	}//end from_squares

	/// Allows immutable iteration through squares
	pub fn iter(&self) -> std::slice::Iter<'_, Square> { self.squares.iter() }
	/// Allows mutable iteration through squares
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Square> { self.squares.iter_mut() }
	/// Width of original image when square info was taken.
	pub fn img_width(&self) -> &usize { &self.img_width }
	/// Height of original image when square info was taken.
	pub fn img_height(&self) -> &usize { &self.img_height }
	/// Number of rows of squares.  
	/// Calculated as the number of unique y values of squares.
	pub fn rows(&self) -> &usize { &self.rows }
	/// Number of cols of squares.  
	/// Calculated as the number of unique x values of squares.
	pub fn cols(&self) -> &usize { &self.cols }

	// TODO: Add checking to ensure get functions won't panic
	// TODO: Add functions to get number of total rows and columns in SquareGrid
	/// Gets a reference to the square at the specified location
	pub fn get(&self, row: &usize, col: &usize) -> &Square { &self.squares[col * self.img_width + row] }
	/// Gets a mutable reference to the square at the specified location
	pub fn get_mut(&mut self, row: &usize, col: &usize) -> &mut Square { &mut self.squares[col * self.img_width + row] }
}//end impl for SquareGrid
