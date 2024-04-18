use fltk::{button::Button, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{self, Flex, FlexType, Pack, Scroll}, prelude::{GroupExt, WidgetBase, WidgetExt}, widget::Widget, widget_extends};
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
pub fn get_default_tab_padding() -> i32 {10}

fn get_default_grid_width() -> i32 {get_default_win_width() - 400}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height() - get_default_tab_padding() - 225}
fn get_max_grid_button_width() -> i32 {30}
fn get_max_grid_button_height() -> i32 {15}

/// # FlexGrid
/// 
/// This struct is meant to be a sort of wrapper around a bunch of buttons and nested flexes in order to mimic a grid of buttons.
/// Note: The code for the FlexGrid custom widget is something I developed in my simple city generator project for CIS536.
/// All the code is written by me.
pub struct FlexGrid {
	/// # outer_flex
	/// The flex containing the flex containing the buttons
	outer_flex: Flex,
	/// # inner_flexes
	/// the flexes contained within the inner flex
	inner_flexes: Vec<Flex>,
}//end struct FlexGrid

#[allow(dead_code)]
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

pub struct ListBox<T > where T : std::fmt::Display {
	pack: Pack,
	scroll: Scroll,
	// outer_widget: Flex,
	elements: Vec<ListItem<T>>,
	element_height: usize,
}//end struct ListBox

impl<T: std::fmt::Display> ListBox<T> {
	/// Creates a new ListBox with the given size.
	pub fn new(x: i32, y: i32, width: i32, height: i32, element_height: usize) -> ListBox<T> {
		// create the pieces with the desired size and fit them together
		// let mut temp_scroll_flex = Flex::default()
		// 	.with_type(FlexType::Row)
		// 	.with_size(width, height)
		// 	.with_pos(x, y);
		// temp_scroll_flex.end();
		// temp_scroll_flex.set_color(Color::Magenta);
		
		let mut temp_scroll = Scroll::new(x,y,width,height, None);
		temp_scroll.end();
		// temp_scroll_flex.add(&temp_scroll);

		// let mut temp_pack_flex = Flex::default()
		// 	.with_type(FlexType::Column)
		// 	.with_size(width - 15, height - 15)
		// 	.with_pos(x, y);
		// temp_pack_flex.end();
		// temp_scroll.add(&temp_pack_flex);
		// temp_pack_flex.set_frame(FrameType::GtkRoundUpFrame);

		let temp_pack = Pack::new(x,y,width, height, None);
		temp_pack.end();
		temp_scroll.add(&temp_pack);
		// temp_pack_flex.add(&temp_pack);
		let temp_elements = Vec::new();

		// set scroll children to resize when it does, partially
		temp_scroll.handle(move |scroll, ev| {
			match ev {
				/*
				Resize is done on Push to enable tracking for Enter.
				Resize is done on Enter in order to compensate for Event::Resize not
				being triggered during some tile resizings.
				 */
				Event::Resize | Event::Fullscreen | Event::Push | Event::Enter | Event::Leave => {
					for child_index in 0..scroll.children() {
						if let Some(mut child) = scroll.child(child_index) {
							child.set_size(scroll.width() - 15, child.height());
							child.redraw();
							// println!("{:?}", ev);
						}//end if we got the child from scroll
					}//end looping over child indices
					scroll.redraw();
					true
				},
				_ => false,
			}//end matching event
		});

		ListBox {
			pack: temp_pack,
			scroll: temp_scroll,
			elements: temp_elements,
			element_height,
			// outer_widget: temp_scroll_flex,
		}//end struct construction
	}//end new

	/// Clears all elements from the list.
	pub fn clear_elements(&mut self) {
		self.elements.clear();
		self.pack.clear();
		self.pack.redraw();
		self.scroll.redraw();
	}//end clear_elements()

	/// Sets the height of each elment of the list.
	pub fn set_element_height(&mut self, element_height: usize) {
		self.element_height = element_height;
		let ieh = u_to_i(&element_height);
		for element in &mut self.elements {
			element.set_size(element.w(), ieh);
			element.redraw();
		}//end resizing each element
		self.pack.redraw();
		self.scroll.redraw();
	}//end set_element_height(self, element_height)

	/// Replaces the current list of elements
	pub fn set_elements(&mut self, option_list: Vec<T>) {
		// create the new list of elements
		let mut temp_elements = Vec::new();
		let ieh = u_to_i(&self.element_height);
		for val in option_list {
			let temp_list_item = ListItem::new(50, self.element_height, val);
			temp_elements.push(temp_list_item);
		}//end creating Frame from each String
		// replace the current list of elements in widget
		self.pack.clear();
		self.elements = temp_elements;
		for element in &self.elements {
			self.pack.add(element.get_frame_ref());
		}//end adding each element to the pack
		let new_height_u = self.elements.len() * self.element_height;
		let new_height_i = u_to_i(&new_height_u);
		self.pack.set_size(self.pack.width(), new_height_i);
		self.pack.redraw();
	}//end set_elements

	pub fn set_label_size(&mut self, label_size: i32) {
		for element in &mut self.elements {
			element.set_label_size(label_size);
		}//end changing label size of each element
	}//end set_label_size(self, label_size)

	pub fn add_element(&mut self, new_element: T) {
		let new_list_item = ListItem::new(i_to_u(&self.pack.w()), self.element_height, new_element);
		self.pack.add(&new_list_item.frame);
		self.elements.push(new_list_item);
		self.pack.redraw();
	}//end add_element(self, new_element)

	/// Gets vector with references to vals
	pub fn get_elements(&self) -> Vec<&T> {
		let mut temp_vec = Vec::new();
		for elem in &self.elements {
			temp_vec.push(elem.get_val());
		}//end taking from each list item
		temp_vec
	}//end get_elements

	pub fn get_selected_elements(&self) -> Vec<&T> {
		let mut temp_vec = Vec::new();
		for elem in &self.elements {
			if elem.is_selected() {
				temp_vec.push(elem.get_val());
			}//end if this element is selected
		}//end looping over elements
		temp_vec
	}//end get_selected_elements
	
	pub fn get_non_selected_elements(&self) -> Vec<&T> {
		let mut temp_vec = Vec::new();
		for elem in &self.elements {
			if !elem.is_selected() {
				temp_vec.push(elem.get_val());
			}//end if element is not selected
		}//end looping over elements
		temp_vec
	}//end get_non_selected_elements

	pub fn remove_selected_elements(&mut self) where T: Clone {
		let non_selected_elements: Vec<T> = self.get_non_selected_elements().into_iter().cloned().collect();
		self.set_elements(non_selected_elements);
	}//end remove_selected_elements()

	pub fn get_scroll_ref(&self) -> &Scroll { &self.scroll }
	pub fn get_scroll_ref_mut(&mut self) -> &mut Scroll { &mut self.scroll }
}//end impl for ListBox

pub struct ListItem<T > where T: std::fmt::Display {
	val: T,
	frame: Button,
}//end struct ListItem

impl<T: std::fmt::Display> ListItem<T> {
	pub fn new(w: usize, h: usize, val: T) -> ListItem<T> {
		let wi = u_to_i(&w);
		let hi = u_to_i(&h);
		let mut temp_frame = Button::default()
			.with_size(wi,hi)
			.with_label(&format!("{}", val));
		temp_frame.set_color(Color::Inactive);
		temp_frame.set_label_color(Color::White);
		temp_frame.set_frame(FrameType::FlatBox);

		temp_frame.handle({
			move |frame, ev| {
				match ev {
					Event::Push  => {
						let cur_color = frame.color();
						match cur_color {
							Color::Inactive => frame.set_color(Color::Selection),
							Color::Selection => frame.set_color(Color::Inactive),
							_ => frame.set_color(Color::Inactive),
						}
						true
					},
					_ => false,
				}
			}//end move for handling event
		});

		ListItem {
			val,
			frame: temp_frame,
		}//end struct construction
	}//end new()

	/// Changes the value stored in this ListItem.
	pub fn set_val(&mut self, new_val: T) {
		self.val = new_val;
		self.frame.set_label(&format!("{}", self.val));
		self.frame.redraw_label();
	}//end set_val()

	/// Redraws a widget, necessary for resizing and changing positions.
	pub fn redraw(&mut self) { self.frame.redraw(); }
	/// Sets to dimensions width and height.
	pub fn set_size(&mut self, w: i32, h: i32) {
		self.frame.set_size(w, h);
	}//end set_size(self, w, h)
	pub fn x(&self) -> i32 { self.frame.x() }
	pub fn y(&self) -> i32 { self.frame.y() }
	pub fn w(&self) -> i32 { self.frame.w() }
	pub fn h(&self) -> i32 { self.frame.h() }

	pub fn set_label_size(&mut self, label_size: i32) { self.frame.set_label_size(label_size); self.frame.redraw_label(); }

	/// Gets reference to current value stored in ListItem.
	pub fn get_val(&self) -> &T { &self.val }
	/// Gets reference to frame of this ListItem.
	pub fn get_frame_ref(&self) -> &Button { &self.frame }
	/// Gets mutable reference to frame of this ListItem.  
	/// Be careful not to change the label using this function, or things
	/// will be weird.
	pub fn get_frame_ref_mut(&mut self) -> &mut Button { &mut self.frame }
	/// Returns true if frame color is equal to Color::Selection, false otherwise.  
	/// If should be noted that frame color is toggled between Color::Selection and Color::Inactive when frame is clicked.
	pub fn is_selected(&self) -> bool { self.frame.color() == Color::Selection }
}//end impl for ListItem

/// Rounds a usize into an i32. If we can't convert,
	/// returns i32::MAX.
	#[allow(dead_code)]
	fn u_to_i(unsi: &usize) -> i32 {
		match i32::try_from(*unsi) {
			Ok(int) => int,
			Err(_) => i32::MAX,
		}//end matching cast result
	}//end u_to_i

	/// Rounds an int to a usize. If we can't convert,
	/// returns 0.
	#[allow(dead_code)]
	fn i_to_u(int: &i32) -> usize {
		match usize::try_from(*int) {
			Ok(u) => u,
			Err(_) => 0,
		}//end matching result of cast
	}//end i_to_u