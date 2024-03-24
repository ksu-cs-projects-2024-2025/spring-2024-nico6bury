
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Square {
	x: usize,
	y: usize,
	width: usize,
	height: usize,
	color: (u8,u8,u8),
}//end struct Square

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

	// /// Allows easy setting of color with new Square.
	// pub fn with_color(mut self, color: (u8,u8,u8)) -> Square { self.color = color; return self; }

	/// Sets the color of this square
	pub fn set_color(&mut self, color: (u8,u8,u8)) { self.color = color; }
	
	pub fn x(&self) -> &usize { &self.x }
	pub fn y(&self) -> &usize { &self.y }
	pub fn width(&self) -> &usize { &self.width }
	pub fn height(&self) -> &usize { &self.height }
	pub fn color(&self) -> &(u8,u8,u8) { &self.color }
}//end impl for Square

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
pub struct SquareGrid {
	squares: Vec<Square>,
	img_width: usize,
	img_height: usize,
}//end struct SquareGrid

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
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Square> { self.squares.iter_mut() }
	pub fn img_width(&self) -> &usize { &self.img_width }
	pub fn img_height(&self) -> &usize { &self.img_height }
}//end impl for SquareGrid
