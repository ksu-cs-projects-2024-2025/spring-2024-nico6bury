use fltk::{prelude::{WidgetExt, GroupExt, WidgetBase}, button::Button, group::{Flex, self}, widget_extends, enums::Align, widget::Widget};
use grid::Grid;

/// # default window width
/// gives the default width in pixels of the main window
pub fn get_default_win_width() -> i32 {1000}
/// # default window height
/// gives the default height in pixels of the main window
pub fn get_default_win_height() -> i32 {700}
/// # default menu height
/// gives the default height in pixels of the menu bar at the top
/// of the main window.
pub fn get_default_menu_height() -> i32 {30}
/// # default tab padding
/// gives teh default padding amount in pixels between tabs and content
pub fn get_default_tab_padding() -> i32 {0}

fn get_default_grid_width() -> i32 {get_default_win_width() - 400}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height() - get_default_tab_padding() - 225}
fn get_max_grid_button_width() -> i32 {30}
fn get_max_grid_button_height() -> i32 {15}

/// # FlexGrid
/// 
/// This struct is meant to be a sort of wrapper around a bunch of buttons and nested flexes in order to mimic a grid of buttons.
pub struct FlexGrid {
	/// # outer_flex
	/// The flex containing the flex containing the buttons
	outer_flex: Flex,
	/// # inner_flexes
	/// the flexes contained within the inner flex
	inner_flexes: Vec<Flex>,
}//end struct FlexGrid

impl FlexGrid {
	/// # default()
	/// 
	/// constructs the empty FlexGrid
	pub fn default() -> FlexGrid {
		let new_outer_flex = Flex::new(0, get_default_menu_height() + get_default_tab_padding(), get_default_grid_width(), get_default_grid_height(), None);
		new_outer_flex.end();
		FlexGrid {
			outer_flex: new_outer_flex,
			inner_flexes: Vec::new(),
		}//end struct construction
	}//end new()

	pub fn change_button(&mut self, row:usize, col:usize) -> Result<Widget, String> {
		match self.inner_flexes.get_mut(row) {
			Some(v) => {
				match v.child(col as i32) {
					Some(a) => {
						return Ok(a);
					},
					None => return Err("This is an error message".to_string())
				};
			},
			None => return Err("This is an error message".to_string()),
		};
	}

	/// # clear_inner_flexes
	/// 
	/// clears the children of this struct. should hopefully work
	pub fn clear_inner_flexes(&mut self) {
		self.outer_flex.clear();
		self.inner_flexes.clear();
	}//end clear_inner_flexes(&mut self)

	/// #initialize_flex(self, grid)]
	/// 
	/// Sets up the flex-boxes like a grid
	pub fn initialize_flex(&mut self, rows:usize, cols:usize) {
		// set outer flex to be have rows of elements
		self.outer_flex.set_type(group::FlexType::Column);
		self.outer_flex.set_align(Align::LeftTop);
		for _row_index in 0..rows {
			let inner_flex_x = 0;//self.outer_flex.x();
			let inner_flex_y = self.outer_flex.y() + (self.outer_flex.width() / cols as i32);
			let inner_flex_w = get_default_grid_width() / cols as i32;
			let inner_flex_h = get_default_grid_height() / rows as i32;
			let mut inner_flex = Flex::new(inner_flex_x,inner_flex_y,inner_flex_w,inner_flex_h,None);
			inner_flex.set_type(group::FlexType::Row);
			// make flex show up
			self.outer_flex.add(&inner_flex);
			// save flex to struct
			self.inner_flexes.push(inner_flex);
		}//end adding inner flexes
		// println!("{} inner flexes", self.inner_flexes.len());
		// println!("inner flex x:{}", self.inner_flexes.first().unwrap().x());
	}//end initialize_flex(self, grid)

	/// # fill_flex(self, buttons)
	/// fills up the flex with buttons such that the buttons will show up in the flex looking like a grid
	/// 
	/// It should be noted that this function should expect to receive things in the order of col, rows
	pub fn fill_flex(&mut self, buttons:&Grid<Button>) {
		for row_idx in 0..buttons.rows() {
			let this_inner_flex = self.inner_flexes.get_mut(row_idx).unwrap();
			// loop over the current row of buttons
			for button in buttons.iter_row(row_idx) {
				if !button.was_deleted() {
					this_inner_flex.add(button);
				}//end if button wasn't deleted
				else {println!("button was deleted, row {}", row_idx);}
			}//end adding each button in row to inner flex
			this_inner_flex.end();
		}//end looping over each inner flex and adding buttons
		self.outer_flex.end();
	}//end fill_flex
}//end impl for FlexGrid

widget_extends!(FlexGrid, Flex, outer_flex);