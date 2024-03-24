
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
}//end struct SquareGrid

#[allow(dead_code)]
impl SquareGrid {
	pub fn from_squares(squares: Vec<Square>, img_width: usize, img_height: usize) -> SquareGrid {
		// TODO: Parse squares to ensure everything fits within img_bounds
		SquareGrid {
			squares,
			img_width,
			img_height,
		}//end struct init
	}//end from_squares

	/// Allows immutable iteration through squares
	pub fn iter(&self) -> std::slice::Iter<'_, Square> { self.squares.iter() }
	/// Allows mutable iteration through squares
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Square> { self.squares.iter_mut() }
	/// Width of original image when square info was taken.
	pub fn img_width(&self) -> &usize { &self.img_width }
	/// Height of original image when square info was taken.
	pub fn img_height(&self) -> &usize { &self.img_height }

	/// Gets a reference to the square at the specified location
	pub fn get(&self, row: &usize, col: &usize) -> &Square { &self.squares[col * self.img_width + row] }
	/// Gets a mutable reference to the square at the specified location
	pub fn get_mut(&mut self, row: &usize, col: &usize) -> &mut Square { &mut self.squares[col * self.img_width + row] }
}//end impl for SquareGrid
