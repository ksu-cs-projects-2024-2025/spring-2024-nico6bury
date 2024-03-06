use std::{cell::RefCell, rc::Rc};

use fltk::{app::{self, Sender}, button::Button, draw::{draw_line, draw_point, set_draw_color, set_line_style, LineStyle}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Scroll, Tile}, prelude::{DisplayExt, GroupExt, ImageExt, SurfaceDevice, ValuatorExt, WidgetBase, WidgetExt}, surface::ImageSurface, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};

use crate::gui::gui_utils::get_default_tab_padding;

pub struct CaveGenGroup {
	ux_whole_tab_group: Tile,
	ux_cave_canvas_scroll: Scroll,
	pub ux_cave_canvas_frame: Frame,
	pub ux_cave_canvas_image: Rc<RefCell<ImageSurface>>,
	pub ux_level_cur_buf: TextBuffer,
	pub ux_level_tot_buf: TextBuffer,
	pub ux_squares_width_counter: Counter,
	pub ux_squares_height_counter: Counter,
	pub ux_squares_pixel_diameter_counter: Counter,
}//end struct CaveGenGroup

impl Default for CaveGenGroup {
	fn default() -> Self {
		let default_image_sur = ImageSurface::new(10,10, false);
		let cave_gen_group = CaveGenGroup {
			ux_whole_tab_group: Default::default(),
			ux_cave_canvas_scroll: Default::default(),
			ux_cave_canvas_frame: Default::default(),
			ux_cave_canvas_image: Rc::from(RefCell::from(default_image_sur)),
			ux_level_cur_buf: Default::default(),
			ux_level_tot_buf: Default::default(),
			ux_squares_width_counter: Default::default(),
			ux_squares_height_counter: Default::default(),
			ux_squares_pixel_diameter_counter: Default::default(),
		};
		cave_gen_group.ux_whole_tab_group.end();
		cave_gen_group.ux_cave_canvas_scroll.end();
		cave_gen_group
	}//end default()
}//end impl Default for CaveGenGroup

impl CaveGenGroup {
	/// # initialize(&mut self, msg_sender)
	/// This function does all necessary initial setup.  
	/// Call it once after declaring the CaveGenGroup object.
	pub fn initialize(&mut self, msg_sender: &Sender<String>) {
		// let resizable_frame = Frame::default()
		// 	.with_pos(self.whole_tab_group.x(), self.whole_tab_group.y())
		// 	.with_size(self.whole_tab_group.width(), self.whole_tab_group.height());
		// self.whole_tab_group.resizable(&resizable_frame);
		self.ux_whole_tab_group.set_frame(FrameType::FlatBox);

		// exterior group for canvas and scroll to fix border issues
		let mut ux_cave_canvas_group = Group::default()
			.with_pos(0, self.ux_whole_tab_group.y())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() * 2 / 3);
		ux_cave_canvas_group.end();
		ux_cave_canvas_group.set_frame(FrameType::FlatBox);
		self.ux_whole_tab_group.add(&ux_cave_canvas_group);

		// scrollable container for size-locked canvas
		self.ux_cave_canvas_scroll = Scroll::default()
			.with_pos(ux_cave_canvas_group.x(), ux_cave_canvas_group.y())
			.with_size(ux_cave_canvas_group.width(), ux_cave_canvas_group.height());
		self.ux_cave_canvas_scroll.end();
		self.ux_cave_canvas_scroll.set_frame(FrameType::BorderBox);
		ux_cave_canvas_group.add(&self.ux_cave_canvas_scroll);

		// size-locked canvas for drawing
		self.ux_cave_canvas_frame = Frame::default()
			.with_pos(self.ux_cave_canvas_scroll.x() + get_default_tab_padding(), self.ux_cave_canvas_scroll.y() + get_default_tab_padding())
			.with_size(100,100)
			.with_label("Canvas thingy");
		self.ux_cave_canvas_frame.set_frame(FrameType::BorderBox);
		self.ux_cave_canvas_scroll.add(&self.ux_cave_canvas_frame);

		// exterior vertical flex for canvas setting stuff
		let mut ux_exterior_canvas_setting_flex = Flex::default()
			.with_pos(self.ux_cave_canvas_scroll.x() + self.ux_cave_canvas_scroll.width(), self.ux_whole_tab_group.y())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() / 2);
		ux_exterior_canvas_setting_flex.end();
		ux_exterior_canvas_setting_flex.set_type(FlexType::Column);
		ux_exterior_canvas_setting_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_canvas_setting_flex);

		// set up all controls within exterior_canvas_settings_flex
		self.initialize_canvas_settings(&mut ux_exterior_canvas_setting_flex, msg_sender);

		// image display part of canvas
		self.update_image_size_and_drawing();
		
		// exterior vertical flex for CA controls
		let mut ux_exterior_cellular_automata_controls_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x(), ux_exterior_canvas_setting_flex.y() + ux_exterior_canvas_setting_flex.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() / 2);
		ux_exterior_cellular_automata_controls_flex.end();
		ux_exterior_cellular_automata_controls_flex.set_type(FlexType::Column);
		ux_exterior_cellular_automata_controls_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_cellular_automata_controls_flex);

		// exterior vertical flex for canvas drawing stuff
		let mut ux_exterior_canvas_drawing_setting_flex = Flex::default()
			.with_pos(self.ux_cave_canvas_scroll.x(), self.ux_cave_canvas_scroll.y() + self.ux_cave_canvas_scroll.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() - ux_cave_canvas_group.height());
		ux_exterior_canvas_drawing_setting_flex.end();
		ux_exterior_canvas_drawing_setting_flex.set_type(FlexType::Column);
		ux_exterior_canvas_drawing_setting_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_canvas_drawing_setting_flex);

		// exterior vertical flex for level connections stuff
		let mut ux_exterior_level_connections_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x() + ux_exterior_canvas_setting_flex.width(), ux_exterior_canvas_setting_flex.y())
			.with_size(self.ux_whole_tab_group.width() - (self.ux_cave_canvas_scroll.width() + ux_exterior_canvas_setting_flex.width()), self.ux_whole_tab_group.height());
		ux_exterior_level_connections_flex.end();
		ux_exterior_level_connections_flex.set_type(FlexType::Column);
		ux_exterior_level_connections_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_level_connections_flex);

		
	}//end initialize()

	/// # initialize_canvas_settings(self, exterior_flex)
	/// Helper method of initialize() to handle controls within the exterior canvas settings flex.
	fn initialize_canvas_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		// interior level number horizontal flex 1
		let mut ux_interior_level_number_hor_flex_1 = Flex::default()
			.with_pos(ux_exterior_flex.x(), ux_exterior_flex.y())
			.with_size(ux_exterior_flex.width(), 50);
		ux_interior_level_number_hor_flex_1.end();
		ux_interior_level_number_hor_flex_1.set_type(FlexType::Row);
		ux_interior_level_number_hor_flex_1.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_level_number_hor_flex_1);

		// interior level number horizontal flex 2
		let mut ux_interior_level_number_hor_flex_2 = Flex::default()
			.with_pos(ux_interior_level_number_hor_flex_1.x(), ux_interior_level_number_hor_flex_1.y() + ux_interior_level_number_hor_flex_1.height())
			.with_size(ux_interior_level_number_hor_flex_1.width(), 50);
		ux_interior_level_number_hor_flex_2.end();
		ux_interior_level_number_hor_flex_2.set_type(FlexType::Row);
		ux_interior_level_number_hor_flex_2.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_level_number_hor_flex_2);

		// interior canvas size horizontal flex 1
		let mut ux_interior_canvas_size_hor_flex_1 = Flex::default()
			.with_pos(ux_interior_level_number_hor_flex_2.x(), ux_interior_level_number_hor_flex_2.y() + ux_interior_level_number_hor_flex_2.height())
			.with_size(ux_interior_level_number_hor_flex_2.width(), 50);
		ux_interior_canvas_size_hor_flex_1.end();
		ux_interior_canvas_size_hor_flex_1.set_type(FlexType::Row);
		ux_interior_canvas_size_hor_flex_1.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_canvas_size_hor_flex_1);

		let mut ux_interior_canvas_size_hor_flex_2 = Flex::default()
			.with_pos(ux_interior_canvas_size_hor_flex_1.x(), ux_interior_canvas_size_hor_flex_1.y() + ux_interior_canvas_size_hor_flex_1.height())
			.with_size(ux_interior_canvas_size_hor_flex_1.width(), 50);
		ux_interior_canvas_size_hor_flex_2.end();
		ux_interior_canvas_size_hor_flex_2.set_type(FlexType::Row);
		ux_interior_canvas_size_hor_flex_2.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_canvas_size_hor_flex_2);

		// level number stuff
		let ux_level_label_frame = Frame::default()
			.with_pos(self.ux_cave_canvas_scroll.x() + self.ux_cave_canvas_scroll.width(), self.ux_cave_canvas_scroll.y())
			.with_label("Level")
			.with_size(30, 20)
			.with_align(Align::Center);
		ux_interior_level_number_hor_flex_1.add(&ux_level_label_frame);

		self.ux_level_cur_buf = TextBuffer::default();
		self.ux_level_cur_buf.set_text("1");
		let mut ux_level_cur_label_txt = TextDisplay::default()
			.with_pos(ux_level_label_frame.x() + ux_level_label_frame.width(), ux_level_label_frame.y())
			.with_size(20,20);
		ux_level_cur_label_txt.set_buffer(self.ux_level_cur_buf.clone());
		ux_interior_level_number_hor_flex_1.add(&ux_level_cur_label_txt);
		
		let ux_level_out_of_label_frame = Frame::default()
			.with_pos(ux_level_cur_label_txt.x() + ux_level_cur_label_txt.width(), ux_level_cur_label_txt.y())
			.with_size(25,20)
			.with_label(" out of ");
		ux_interior_level_number_hor_flex_1.add(&ux_level_out_of_label_frame);
		
		self.ux_level_tot_buf = TextBuffer::default();
		self.ux_level_tot_buf.set_text("3");
		let mut ux_level_total_label_txt = TextEditor::default()
			.with_pos(ux_level_out_of_label_frame.x() + ux_level_out_of_label_frame.width(), ux_level_out_of_label_frame.y())
			.with_size(20,20);
		ux_level_total_label_txt.set_buffer(self.ux_level_tot_buf.clone());
		ux_interior_level_number_hor_flex_1.add(&ux_level_total_label_txt);
		
		let ux_level_down_btn = Button::default()
			.with_pos(ux_level_label_frame.x(), ux_level_label_frame.y() + ux_level_label_frame.height())
			.with_size(25, 25)
			.with_label("@line");
		ux_interior_level_number_hor_flex_2.add(&ux_level_down_btn);
		let ux_level_up_btn = Button::default()
			.with_pos(ux_level_down_btn.x() + ux_level_down_btn.width() , ux_level_down_btn.y())
			.with_size(25,25)
			.with_label("@+");
		ux_interior_level_number_hor_flex_2.add(&ux_level_up_btn);

		// stuff for setting size/resolution of squares
		let ux_square_size_label = Frame::default()
			.with_pos(self.ux_cave_canvas_scroll.x() + self.ux_cave_canvas_scroll.width(), ux_level_down_btn.y() + ux_level_down_btn.height() + get_default_tab_padding())
			.with_size(90, 25)
			.with_label("Level Size (in squares)")
			.with_align(Align::Inside);
		ux_interior_canvas_size_hor_flex_1.add(&ux_square_size_label);

		self.ux_squares_width_counter = Counter::default()
			.with_pos(ux_square_size_label.x(), ux_square_size_label.y() + ux_square_size_label.height())
			.with_size(50, 25)
			.with_label("Width")
			.with_align(Align::Top);
		self.ux_squares_width_counter.set_value(50.0);
		self.ux_squares_width_counter.set_minimum(3.0);
		self.ux_squares_width_counter.set_maximum(1000.0);
		self.ux_squares_width_counter.set_precision(0);
		self.ux_squares_width_counter.set_step(1.0, 10);
		ux_interior_canvas_size_hor_flex_2.add(&self.ux_squares_width_counter);

		self.ux_squares_height_counter = Counter::default()
			.with_pos(self.ux_squares_width_counter.x() + self.ux_squares_width_counter.width(), self.ux_squares_width_counter.y())
			.with_size(50, 25)
			.with_label("Height")
			.with_align(Align::Top);
		self.ux_squares_height_counter.set_value(50.0);
		self.ux_squares_height_counter.set_minimum(3.0);
		self.ux_squares_height_counter.set_maximum(1000.0);
		self.ux_squares_height_counter.set_precision(0);
		self.ux_squares_height_counter.set_step(1.0, 10);
		ux_interior_canvas_size_hor_flex_2.add(&self.ux_squares_height_counter);

		// pixel scale
		let ux_squares_pixel_diameter_label = Frame::default()
			.with_pos(self.ux_squares_width_counter.x(), self.ux_squares_width_counter.y() + self.ux_squares_width_counter.height())
			.with_size(50, 25)
			.with_label("Scale (Pixel Diameter per Square)");
		ux_exterior_flex.add(&ux_squares_pixel_diameter_label);

		self.ux_squares_pixel_diameter_counter = Counter::default()
			.with_pos(ux_squares_pixel_diameter_label.x(), ux_squares_pixel_diameter_label.y() + ux_squares_pixel_diameter_label.height())
			.with_size(50, 25)
			.with_align(Align::TopLeft);
		self.ux_squares_pixel_diameter_counter.set_value(8.0);
		self.ux_squares_pixel_diameter_counter.set_minimum(1.0);
		self.ux_squares_pixel_diameter_counter.set_maximum(30.0);
		self.ux_squares_pixel_diameter_counter.set_precision(0);
		self.ux_squares_pixel_diameter_counter.set_step(1.0, 10);
		self.ux_squares_pixel_diameter_counter.set_type(CounterType::Simple);
		ux_exterior_flex.add(&self.ux_squares_pixel_diameter_counter);

		// button for updating canvas
		let mut ux_update_canvas_button = Button::default()
			.with_pos(self.ux_squares_pixel_diameter_counter.x(), self.ux_squares_pixel_diameter_counter.y() + self.ux_squares_pixel_diameter_counter.height())
			.with_size(100, 25)
			.with_label("Clear Canvas and Update Size/Scale");
		ux_exterior_flex.add(&ux_update_canvas_button);
		// TODO: Rework this to activate entirely within this file and remove need for msg_sender for this functionallity
		ux_update_canvas_button.emit(msg_sender.clone(), "CaveGen:Canvas:Update".to_string());

		// update cave canvas frame based on default values in Counters
		let new_width = self.ux_squares_width_counter.value() * self.ux_squares_pixel_diameter_counter.value();
		let new_height = self.ux_squares_height_counter.value() * self.ux_squares_pixel_diameter_counter.value();
		self.ux_cave_canvas_frame.set_size(new_width as i32, new_height as i32);
	}//initialize_canvas_settings

	/// # update_image_size_and_drawing(&mut self)
	/// This function creates/updates the canvas surface for drawing cave stuff with the right size.  
	fn update_image_size_and_drawing(&mut self) {
		let canvas_surface = ImageSurface::new(self.ux_cave_canvas_frame.width(), self.ux_cave_canvas_frame.height(), false);
		
		ImageSurface::push_current(&canvas_surface);
		// TODO: Redo filling to not reset previous work, probably by copying drawings out of old surface image, maybe by using fltk::draw_image or fltk::draw_rbg and limiting size of image? If changing resolution, might need to grid-ify first
		fltk::draw::draw_rect_fill(0,0,self.ux_cave_canvas_frame.width(), self.ux_cave_canvas_frame.height(), Color::White);
		ImageSurface::pop_current();

		self.ux_cave_canvas_image = Rc::from(RefCell::from(canvas_surface));

		let pixel_scale = self.ux_squares_pixel_diameter_counter.value() as i32;
		let pixel_scale_ref = Rc::from(RefCell::from(pixel_scale));

		self.ux_cave_canvas_frame.draw( {
			let surface = self.ux_cave_canvas_image.clone();
			move |f| {
				let surface = surface.borrow();
				let mut img = surface.image().unwrap();
				img.draw(f.x(), f.y(), f.w(), f.h());
			}
		});

		self.ux_cave_canvas_frame.handle( {
			let mut x = 0;
			let mut y = 0;
			let surface = self.ux_cave_canvas_image.clone();
			let pixel_scale_clone = pixel_scale_ref.clone();
			move |f, ev| {
				let surface = surface.borrow_mut();
				let pixel_scale = pixel_scale_clone.borrow();
				match ev {
					Event::Push => {
						ImageSurface::push_current(&surface);
						set_draw_color(Color::Black);
						set_line_style(LineStyle::Solid, *pixel_scale);
						let coords = app::event_coords();
						x = coords.0; // fefwf
						y = coords.1;
						draw_point(x, y);
						ImageSurface::pop_current();
						f.redraw();
						true
					}//end push event
					Event::Drag => {
						ImageSurface::push_current(&surface);
						set_draw_color(Color::Black);
						set_line_style(LineStyle::Solid, *pixel_scale);
						let coords = app::event_coords();
						draw_line(x - f.x(), y - f.y(), coords.0 - f.x(), coords.1 - f.y());
						x = coords.0;
						y = coords.1;
						ImageSurface::pop_current();
						f.redraw();
						true
					}
					_ => false
				}//end matching event
			}//end handle move
		});
	}//end update_image_size_and_height(prev_w, prev_h)

	/// # update_canvas(&mut self)
	/// This function updates the size of the drawing canvas based on user settings. 
	pub fn update_canvas(&mut self) {
		let diameter_counter = self.ux_squares_pixel_diameter_counter.value();
		let squares_width = self.ux_squares_width_counter.value();
		let squares_height = self.ux_squares_height_counter.value();
		let pixels_width = squares_width * diameter_counter;
		let pixels_height = squares_height * diameter_counter;
		self.ux_cave_canvas_frame.set_size(pixels_width as i32, pixels_height as i32);
		self.update_image_size_and_drawing();
		self.ux_cave_canvas_scroll.redraw();
		self.ux_cave_canvas_frame.redraw();
	}//end update_canvas(self)
}//end impl for CaveGenGroup

widget_extends!(CaveGenGroup, Tile, ux_whole_tab_group);