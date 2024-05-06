use std::slice::Iter;

use fltk::{button::Button, draw::{draw_rect_fill, draw_rect_with_color}, enums::{Align, Color, Event, FrameType}, group::{self, Flex, Pack, Scroll}, prelude::{GroupExt, ImageExt, SurfaceDevice, WidgetBase, WidgetExt}, surface::ImageSurface, widget::Widget, widget_extends};
use grid::Grid;
use nice_map_generator::squares::{Square, SquareGrid};

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

#[allow(dead_code)]
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
		// let ieh = u_to_i(&self.element_height);
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

#[allow(dead_code)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SquareStairDisplay {
	pub square: Square,
	pub row_idx: usize,
	pub col_idx: usize,
}//end struct SquareStairDisplay

impl Default for SquareStairDisplay {
    fn default() -> Self {
		Self { square: Square::new(0, 0, 0, 0), row_idx: Default::default(), col_idx: Default::default() }
	}//end default()
} //end struct SquareStairDisplay

impl std::fmt::Display for SquareStairDisplay {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Row: {}, Col: {}", self.row_idx, self.col_idx)
	}//end fmt
}//end impl Display for SquareStairDisplay

	/// This function takes a mutable reference to an image surface and performs operations on it
	/// to squareularize it.  
	/// This function will return false if some part of this process is unsuccessful, or true if things went okay.  
	/// It is recommended to call redraw() on the frame holding [canvas] after calling this method.
	/// Arguments:  
	/// - canvas - the image surface you want to squareularize
	/// - preferred_colors - an optional list of colors to be preferred above others when determining colors
	/// - pixel_scale - how many pixels are in one grid square
	/// - sub_pixel_scale - scale of sub-grid within each grid square
	pub fn ux_squareularize_canvas(canvas: &ImageSurface, preferred_colors: Option<&Vec<(u8,u8,u8)>>, pixel_scale: &usize, sub_pixel_scale: &usize) -> Option<SquareGrid> {
		match squareularization_get_rgb_pixels(canvas) {
			Some(image_and_pixels) => {
				let image = image_and_pixels.0;
				let pixels = image_and_pixels.1;
				let img_width = image.width() as usize;
				let img_height = image.height() as usize;


				// figure out list of bounds for squares in our grid
				let square_scale = (pixel_scale * sub_pixel_scale) as usize;
				let square_width = square_scale;//img_width / square_scale;
				let square_height = square_scale;//img_height / square_scale;
				// format of (x, y, Color), assume square_width and square_height, fill in color later
				match squareularization_split_img_to_squares(&img_width, &img_height, &square_width, &square_height) {
					Some(mut squares) => {
						// figure out dominant color in each square, replacing color value in vec
						squareularization_get_dominant_color(&mut squares, preferred_colors, &pixels, &img_width, &square_width, &square_height);
		
						// paint dominant color to entire square using the canvas
						squareularization_color_squares(canvas, &squares, &false);
		
						Some(squares)
					},
					None => {
						println!("Couldn't split img to squares. It is likely that a SquareGrid could not be created from Square Vec.");
						None
					}
				}
			},
			None => {println!("Squareularization Failed. Couldn't get image from canvas, or pixels weren't in RGB color depth 3."); None },
		}//end matching image get result
	}//end ux_squareularize_canvas(canvas)

	/// Helper function for ux_squareularize_canvas
	/// 
	/// This function assumes that canvas contains an RGB image.  
	/// It returns None if:  
	/// - we can't get an image from canvas
	/// - the image we get isn't an RGB image with color depth 3
	/// It returns Some with a tuple containing:
	/// - the image grabbed from canvas
	/// - a Vector containing the RGB value of each pixel
	pub fn squareularization_get_rgb_pixels(canvas: &ImageSurface) -> Option<(fltk::image::RgbImage, Vec<(u8,u8,u8)>)> {
		if let Some(cur_img) = canvas.image() {
			match cur_img.depth() {
				fltk::enums::ColorDepth::Rgb8 => {
					let raw_pixel_data = cur_img.to_rgb_data();

					// convert our pixel R,G,B,R,G,B into RGB,RGB
					let mut rgb_pixel_vec = Vec::new();
					let mut rgb_trio = Vec::new();
					for val in raw_pixel_data {
						rgb_trio.push(val);
						if rgb_trio.len() >= 3 {
							let r = rgb_trio[0];
							let g = rgb_trio[1];
							let b = rgb_trio[2];
							rgb_pixel_vec.push((r,g,b));
							rgb_trio.clear();
						}//end if we're ready to push
					}//end packing raw pixel data into rgb data

					Some((cur_img, rgb_pixel_vec))
				},
				_ => return None,
			}//end matching to correct color depths
		} else {return None;}
	}//end squareularization_get_rgb_pixels(canvas)

	/// Helper function for ux_squareularize_canvas()
	/// - This function, given the dimensions of a larger image and smaller square within that image, 
	/// splits up the image into squares of roughly the dimensions provided.  
	/// - This is returned as a vector containing the x and y coordinate of the 
	/// upper left corner of each square.  
	/// Each element also has a tuple with an rgb value,
	/// to be used in later processing.
	/// - There are cases when the image cannot be split evenly into squares of the same size.
	/// In such a case, the squares along the bottom or right edge of the image will overlap slightly.
	pub fn squareularization_split_img_to_squares(img_width: &usize, img_height: &usize, square_width: &usize, square_height: &usize) -> Option<SquareGrid> {
		let mut squares: Vec<Square> = Vec::new();
		// format of (x, y, Color), assume square_width and square_height, fill in color later
		for mut y in (0..*img_height).step_by(*square_height) {
			// Squares at edges might overlap, but they won't be out of bounds
			if y + square_height > *img_height {y = img_height - square_height;}
			for mut x in (0..*img_width).step_by(*square_width) {
				// Squares at edges might overlap, but they won't be out of bounds
				if x + square_width > *img_width {x = img_width - square_width;}
				let square = Square::new(x,y, *square_width, *square_height);
				squares.push(square);
			}//end looping over all potential x values for sub-squares
		}//end looping over all potential y values for sub-squares
		match SquareGrid::from_squares(squares, *img_width, *img_height) {
			Ok(square_grid) => {
				Some(square_grid)
			},
			Err(err_info) => {
				println!("Recieved an error when converting squares to SquareGrid. Printing it below:");
				println!("{}", err_info.1);
				None
			}}
	}//end squareularization_split_img_to_squares

	/// Helper function for ux_squareularize_canvas
	/// Determines a dominant color in each square, recording
	/// this information in squares. 
	/// ## Bias:
	/// This function has a bias towards certain colors and will count them as being
	/// x10 more dominant. Colors meaningful to CA are given bias. Colors close to CA colors
	/// will be converted.  
	/// Stairs a given a large bias due to conversion strength of wall and floor.
	/// ## Panics:
	/// - If this function panics, it is mostly likely a result of
	/// the bounds of a square exceeding image bounds, causing the
	/// function to attempt accessing a pixel that doesn't exist.  
	/// This should not happen when using SquareGrids though.
	pub fn squareularization_get_dominant_color(squares: &mut SquareGrid, preferred_colors: Option<&Vec<(u8,u8,u8)>>, pixels: &Vec<(u8,u8,u8)> ,img_width: &usize, square_width: &usize, square_height: &usize) {
		/*
		square.0 refers to square width, square.1 refers to square height
		 */
		for square in squares.iter_mut() {
			// figure out dominant color here, set square.2 to that
			// color_counts1 and color_counts2 are parallel
			let mut color_counts_color: Vec<(u8,u8,u8)> = Vec::new();
			let mut color_counts_count: Vec<u64> = Vec::new();
			for y in *square.y()..(square_height + square.y()) {
				for x in *square.x()..(square_width + square.x()) {
					let this_overall_index = (y * img_width) + x;
					let this_rgb = pixels[this_overall_index];
					let mut this_color = (this_rgb.0, this_rgb.1, this_rgb.2);
					
					if let Some(pcs) = preferred_colors {
						if !pcs.contains(&this_rgb) {
							let pcs_dists = {
								let mut tmp_dif_vec = Vec::new();
								for color in pcs {
									let r_diff = this_rgb.0 as f32 - color.0 as f32;
									let g_diff = this_rgb.1 as f32 - color.1 as f32;
									let b_diff = this_rgb.2 as f32 - color.2 as f32;
									let t_diff = (r_diff.abs() + g_diff.abs() + b_diff.abs()) / 3.;
									tmp_dif_vec.push(t_diff);
								}//end getting total average difference to each color
								tmp_dif_vec
							};
							let mut closest = (0, 1000.);
							for (idx, pcs_dist) in pcs_dists.iter().enumerate() {
								if *pcs_dist < closest.1 {
									closest = (idx, *pcs_dist);
								}//end if we have a better closest value
							}//end finding closest of preferred colors
							if closest.1 <= 150. && this_rgb != (255,0,255) { if let Some(color) = pcs.get(closest.0) { this_color = *color; } }
						}//end if this color is not preferred
					}//end if we're applying a bias

					if let Some(color_index) = color_counts_color.iter().position(|&c| c == this_color) {
						color_counts_count[color_index] += 1;
					} else {
						color_counts_color.push(this_color);
						color_counts_count.push(1);
					}//end else we need to add new entry to color counts
				}//end looping over all x values within square
			}//end looping over all y values within square

			// bias for CA colors
			for (i, color) in color_counts_color.iter().enumerate() {
				match *color {
					(0,255,0) => color_counts_count[i] *= 500,
					(255,0,255) => color_counts_count[i] *= 0,
					_ => {},
				}//end matching bias to colors
			}//end applying bias to color counts

			// check to see which color has the highest count
			let mut running_most = ((40,40,40), 0);
			for (i, count) in color_counts_count.iter().enumerate() {
				if *count > running_most.1 { running_most = (color_counts_color[i], color_counts_count[i]); }
			}//end getting the color that's most common from color counts
			square.set_color(running_most.0);
		}//end figuring out which color is dominant
	}//end squareularization_get_dominant_color()

	/// Helper function for [ux_squareularize_canvas()]
	/// - This function, given an ImageSurface and Vec of squares 
	/// within that image, paints the color within the squares 
	/// vec to that square.  
	/// - If [use_debug_color] is true, then the whole [canvas] will be 
	/// painted magenta before painting anything, in order to 
	/// visually show if any space was missed.
	/// - Because this function accesses fltk drawing functions and
	/// must do type conversions, there are a few potential panics
	/// that could happen.
	/// - It is recommended to call [redraw()] on the frame holding 
	/// canvas after calling this function.
	/// ## Panics:
	/// - type conversion of [square_width] or [square_height] to i32
	/// - type conversion of [squares]\[i\].0 or [squares]\[i\].1 to i32
	/// - debug drawing tries to calculate image dimensions by 
	/// getting the last element of [squares] and adding [square_width] 
	/// and [square_height] appropriately. If squares cannot 
	/// be accessed, the type conversion to i32 fails, or 
	/// the dimensions calculated exceed the image bounds, a 
	/// panic might happen.
	pub fn squareularization_color_squares(canvas: &ImageSurface, squares: &SquareGrid, use_debug_color: &bool) {
		ImageSurface::push_current(&canvas);
		if *use_debug_color {
			// paint magenta to entire canvas as debugging
			draw_rect_fill(0, 0, *squares.img_width() as i32, *squares.img_height() as i32, Color::Magenta);
		}//end if we're doing a debug fill
		// paint dominant color to entire square using the canvas
		for square in squares.iter() {
			let a = (*square.x() as i32, *square.y() as i32, *square.width() as i32, *square.height() as i32);
			let c = Color::from_rgb(square.color().0, square.color().1, square.color().2);
			draw_rect_fill(a.0, a.1, a.2, a.3, c);
			if *use_debug_color { draw_rect_with_color(a.0, a.1, a.2, a.3, Color::Magenta); }
		}//end painting dominant color to entirety of each square
		ImageSurface::pop_current();
	}//end squareularization_color_squares()

	/// Function similar to squareularization_color_squares, this function allows you
	/// to only color a select few squares instead of redoing the whole canvas.
	pub fn squareularization_color_square(canvas: &ImageSurface, squares: Iter<Square>) {
		ImageSurface::push_current(&canvas);
		for square in squares {
			let coords = (*square.x() as i32, *square.y() as i32, *square.width() as i32, *square.height() as i32);
			let color = Color::from_rgb(square.color().0, square.color().1, square.color().2);
			draw_rect_fill(coords.0, coords.1, coords.2, coords.3, color);
		}//end looping over each square
		ImageSurface::pop_current();
	}//end squareularization_color_squares