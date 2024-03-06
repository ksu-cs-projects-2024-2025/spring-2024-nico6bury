use std::{cell::RefCell, rc::Rc};

use fltk::{app::{self, Sender}, button::Button, draw::{draw_line, draw_point, set_draw_color, set_line_style, LineStyle}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Scroll, Tile}, prelude::{DisplayExt, GroupExt, ImageExt, SurfaceDevice, ValuatorExt, WidgetBase, WidgetExt}, surface::ImageSurface, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};

use crate::gui::gui_utils::get_default_tab_padding;

pub struct CaveGenGroup {
	whole_tab_group: Tile,
	cave_canvas_scroll: Scroll,
	pub cave_canvas_frame: Frame,
	pub cave_canvas_image: Rc<RefCell<ImageSurface>>,
	pub level_cur_buf: TextBuffer,
	pub level_tot_buf: TextBuffer,
	pub squares_width_counter: Counter,
	pub squares_height_counter: Counter,
	pub squares_pixel_diameter_counter: Counter,
}//end struct CaveGenGroup

impl Default for CaveGenGroup {
	fn default() -> Self {
		let default_image_sur = ImageSurface::new(10,10, false);
		let cave_gen_group = CaveGenGroup {
			whole_tab_group: Default::default(),
			cave_canvas_scroll: Default::default(),
			cave_canvas_frame: Default::default(),
			cave_canvas_image: Rc::from(RefCell::from(default_image_sur)),
			level_cur_buf: Default::default(),
			level_tot_buf: Default::default(),
			squares_width_counter: Default::default(),
			squares_height_counter: Default::default(),
			squares_pixel_diameter_counter: Default::default(),
		};
		cave_gen_group.whole_tab_group.end();
		cave_gen_group.cave_canvas_scroll.end();
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
		self.whole_tab_group.set_frame(FrameType::FlatBox);

		// exterior group for canvas and scroll to fix border issues
		let mut cave_canvas_group = Group::default()
			.with_pos(0, self.whole_tab_group.y())
			.with_size(self.whole_tab_group.width() / 3, self.whole_tab_group.height() * 2 / 3);
		cave_canvas_group.end();
		cave_canvas_group.set_frame(FrameType::FlatBox);
		self.whole_tab_group.add(&cave_canvas_group);

		// scrollable container for size-locked canvas
		self.cave_canvas_scroll = Scroll::default()
			.with_pos(cave_canvas_group.x(), cave_canvas_group.y())
			.with_size(cave_canvas_group.width(), cave_canvas_group.height());
		self.cave_canvas_scroll.end();
		self.cave_canvas_scroll.set_frame(FrameType::BorderBox);
		cave_canvas_group.add(&self.cave_canvas_scroll);

		// size-locked canvas for drawing
		self.cave_canvas_frame = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + get_default_tab_padding(), self.cave_canvas_scroll.y() + get_default_tab_padding())
			.with_size(100,100)
			.with_label("Canvas thingy");
		self.cave_canvas_frame.set_frame(FrameType::BorderBox);
		self.cave_canvas_scroll.add(&self.cave_canvas_frame);

		// image display part of canvas
		self.update_image_size_and_drawing();

		// exterior vertical flex for canvas setting stuff
		let mut exterior_canvas_setting_flex = Flex::default()
			.with_pos(self.cave_canvas_scroll.x() + self.cave_canvas_scroll.width(), self.whole_tab_group.y())
			.with_size(self.whole_tab_group.width() / 3, self.whole_tab_group.height() / 2);
		exterior_canvas_setting_flex.end();
		exterior_canvas_setting_flex.set_type(FlexType::Column);
		exterior_canvas_setting_flex.set_frame(FrameType::BorderBox);
		self.whole_tab_group.add(&exterior_canvas_setting_flex);

		// set up all controls within exterior_canvas_settings_flex
		self.initialize_canvas_settings(&mut exterior_canvas_setting_flex, msg_sender);
		
		// exterior vertical flex for CA controls
		let mut exterior_cellular_automata_controls_flex = Flex::default()
			.with_pos(exterior_canvas_setting_flex.x(), exterior_canvas_setting_flex.y() + exterior_canvas_setting_flex.height())
			.with_size(self.whole_tab_group.width() / 3, self.whole_tab_group.height() / 2);
		exterior_cellular_automata_controls_flex.end();
		exterior_cellular_automata_controls_flex.set_type(FlexType::Column);
		exterior_cellular_automata_controls_flex.set_frame(FrameType::BorderBox);
		self.whole_tab_group.add(&exterior_cellular_automata_controls_flex);

		// exterior vertical flex for canvas drawing stuff
		let mut exterior_canvas_drawing_setting_flex = Flex::default()
			.with_pos(self.cave_canvas_scroll.x(), self.cave_canvas_scroll.y() + self.cave_canvas_scroll.height())
			.with_size(self.whole_tab_group.width() / 3, self.whole_tab_group.height() - cave_canvas_group.height());
		exterior_canvas_drawing_setting_flex.end();
		exterior_canvas_drawing_setting_flex.set_type(FlexType::Column);
		exterior_canvas_drawing_setting_flex.set_frame(FrameType::BorderBox);
		self.whole_tab_group.add(&exterior_canvas_drawing_setting_flex);

		// exterior vertical flex for level connections stuff
		let mut exterior_level_connections_flex = Flex::default()
			.with_pos(exterior_canvas_setting_flex.x() + exterior_canvas_setting_flex.width(), exterior_canvas_setting_flex.y())
			.with_size(self.whole_tab_group.width() - (self.cave_canvas_scroll.width() + exterior_canvas_setting_flex.width()), self.whole_tab_group.height());
		exterior_level_connections_flex.end();
		exterior_level_connections_flex.set_type(FlexType::Column);
		exterior_level_connections_flex.set_frame(FrameType::BorderBox);
		self.whole_tab_group.add(&exterior_level_connections_flex);

		
	}//end initialize()

	/// # initialize_canvas_settings(self, exterior_flex)
	/// Helper method of initialize() to handle controls within the exterior canvas settings flex.
	fn initialize_canvas_settings(&mut self, exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		// interior level number horizontal flex 1
		let mut interior_level_number_hor_flex_1 = Flex::default()
			.with_pos(exterior_flex.x(), exterior_flex.y())
			.with_size(exterior_flex.width(), 50);
		interior_level_number_hor_flex_1.end();
		interior_level_number_hor_flex_1.set_type(FlexType::Row);
		interior_level_number_hor_flex_1.set_frame(FrameType::FlatBox);
		exterior_flex.add(&interior_level_number_hor_flex_1);

		// interior level number horizontal flex 2
		let mut interior_level_number_hor_flex_2 = Flex::default()
			.with_pos(interior_level_number_hor_flex_1.x(), interior_level_number_hor_flex_1.y() + interior_level_number_hor_flex_1.height())
			.with_size(interior_level_number_hor_flex_1.width(), 50);
		interior_level_number_hor_flex_2.end();
		interior_level_number_hor_flex_2.set_type(FlexType::Row);
		interior_level_number_hor_flex_2.set_frame(FrameType::FlatBox);
		exterior_flex.add(&interior_level_number_hor_flex_2);

		// interior canvas size horizontal flex 1
		let mut interior_canvas_size_hor_flex_1 = Flex::default()
			.with_pos(interior_level_number_hor_flex_2.x(), interior_level_number_hor_flex_2.y() + interior_level_number_hor_flex_2.height())
			.with_size(interior_level_number_hor_flex_2.width(), 50);
		interior_canvas_size_hor_flex_1.end();
		interior_canvas_size_hor_flex_1.set_type(FlexType::Row);
		interior_canvas_size_hor_flex_1.set_frame(FrameType::FlatBox);
		exterior_flex.add(&interior_canvas_size_hor_flex_1);

		let mut interior_canvas_size_hor_flex_2 = Flex::default()
			.with_pos(interior_canvas_size_hor_flex_1.x(), interior_canvas_size_hor_flex_1.y() + interior_canvas_size_hor_flex_1.height())
			.with_size(interior_canvas_size_hor_flex_1.width(), 50);
		interior_canvas_size_hor_flex_2.end();
		interior_canvas_size_hor_flex_2.set_type(FlexType::Row);
		interior_canvas_size_hor_flex_2.set_frame(FrameType::FlatBox);
		exterior_flex.add(&interior_canvas_size_hor_flex_2);

		// level number stuff
		let level_label_frame = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + self.cave_canvas_scroll.width(), self.cave_canvas_scroll.y())
			.with_label("Level")
			.with_size(30, 20)
			.with_align(Align::Center);
		interior_level_number_hor_flex_1.add(&level_label_frame);

		self.level_cur_buf = TextBuffer::default();
		self.level_cur_buf.set_text("1");
		let mut level_cur_label_txt = TextDisplay::default()
			.with_pos(level_label_frame.x() + level_label_frame.width(), level_label_frame.y())
			.with_size(20,20);
		level_cur_label_txt.set_buffer(self.level_cur_buf.clone());
		interior_level_number_hor_flex_1.add(&level_cur_label_txt);
		
		let level_out_of_label_frame = Frame::default()
			.with_pos(level_cur_label_txt.x() + level_cur_label_txt.width(), level_cur_label_txt.y())
			.with_size(25,20)
			.with_label(" out of ");
		interior_level_number_hor_flex_1.add(&level_out_of_label_frame);
		
		self.level_tot_buf = TextBuffer::default();
		self.level_tot_buf.set_text("3");
		let mut level_total_label_txt = TextEditor::default()
			.with_pos(level_out_of_label_frame.x() + level_out_of_label_frame.width(), level_out_of_label_frame.y())
			.with_size(20,20);
		level_total_label_txt.set_buffer(self.level_tot_buf.clone());
		interior_level_number_hor_flex_1.add(&level_total_label_txt);
		
		let level_down_btn = Button::default()
			.with_pos(level_label_frame.x(), level_label_frame.y() + level_label_frame.height())
			.with_size(25, 25)
			.with_label("@line");
		interior_level_number_hor_flex_2.add(&level_down_btn);
		let level_up_btn = Button::default()
			.with_pos(level_down_btn.x() + level_down_btn.width() , level_down_btn.y())
			.with_size(25,25)
			.with_label("@+");
		interior_level_number_hor_flex_2.add(&level_up_btn);

		// stuff for setting size/resolution of squares
		let square_size_label = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + self.cave_canvas_scroll.width(), level_down_btn.y() + level_down_btn.height() + get_default_tab_padding())
			.with_size(90, 25)
			.with_label("Level Size (in squares)")
			.with_align(Align::Inside);
		interior_canvas_size_hor_flex_1.add(&square_size_label);

		self.squares_width_counter = Counter::default()
			.with_pos(square_size_label.x(), square_size_label.y() + square_size_label.height())
			.with_size(50, 25)
			.with_label("Width")
			.with_align(Align::Top);
		self.squares_width_counter.set_value(50.0);
		self.squares_width_counter.set_minimum(3.0);
		self.squares_width_counter.set_maximum(1000.0);
		self.squares_width_counter.set_precision(0);
		self.squares_width_counter.set_step(1.0, 10);
		interior_canvas_size_hor_flex_2.add(&self.squares_width_counter);

		self.squares_height_counter = Counter::default()
			.with_pos(self.squares_width_counter.x() + self.squares_width_counter.width(), self.squares_width_counter.y())
			.with_size(50, 25)
			.with_label("Height")
			.with_align(Align::Top);
		self.squares_height_counter.set_value(50.0);
		self.squares_height_counter.set_minimum(3.0);
		self.squares_height_counter.set_maximum(1000.0);
		self.squares_height_counter.set_precision(0);
		self.squares_height_counter.set_step(1.0, 10);
		interior_canvas_size_hor_flex_2.add(&self.squares_height_counter);

		// pixel scale
		let squares_pixel_diameter_label = Frame::default()
			.with_pos(self.squares_width_counter.x(), self.squares_width_counter.y() + self.squares_width_counter.height())
			.with_size(50, 25)
			.with_label("Scale (Pixel Diameter per Square)");
		exterior_flex.add(&squares_pixel_diameter_label);

		self.squares_pixel_diameter_counter = Counter::default()
			.with_pos(squares_pixel_diameter_label.x(), squares_pixel_diameter_label.y() + squares_pixel_diameter_label.height())
			.with_size(50, 25)
			.with_align(Align::TopLeft);
		self.squares_pixel_diameter_counter.set_value(2.0);
		self.squares_pixel_diameter_counter.set_minimum(1.0);
		self.squares_pixel_diameter_counter.set_maximum(30.0);
		self.squares_pixel_diameter_counter.set_precision(0);
		self.squares_pixel_diameter_counter.set_step(1.0, 10);
		self.squares_pixel_diameter_counter.set_type(CounterType::Simple);
		exterior_flex.add(&self.squares_pixel_diameter_counter);

		// button for updating canvas
		let mut update_canvas_button = Button::default()
			.with_pos(self.squares_pixel_diameter_counter.x(), self.squares_pixel_diameter_counter.y() + self.squares_pixel_diameter_counter.height())
			.with_size(100, 25)
			.with_label("Clear Canvas and Update Size/Scale");
		exterior_flex.add(&update_canvas_button);
		// TODO: Rework this to activate entirely within this file and remove need for msg_sender for this functionallity
		update_canvas_button.emit(msg_sender.clone(), "CaveGen:Canvas:Update".to_string());
	}//initialize_canvas_settings

	/// # update_image_size_and_drawing(&mut self)
	/// This function creates/updates the canvas surface for drawing cave stuff with the right size.  
	fn update_image_size_and_drawing(&mut self) {
		let canvas_surface = ImageSurface::new(self.cave_canvas_frame.width(), self.cave_canvas_frame.height(), false);
		
		ImageSurface::push_current(&canvas_surface);
		// TODO: Redo filling to not reset previous work, probably by copying drawings out of old surface image, maybe by using fltk::draw_image or fltk::draw_rbg and limiting size of image? If changing resolution, might need to grid-ify first
		fltk::draw::draw_rect_fill(0,0,self.cave_canvas_frame.width(), self.cave_canvas_frame.height(), Color::White);
		ImageSurface::pop_current();

		self.cave_canvas_image = Rc::from(RefCell::from(canvas_surface));

		let pixel_scale = self.squares_pixel_diameter_counter.value() as i32;
		let pixel_scale_ref = Rc::from(RefCell::from(pixel_scale));

		self.cave_canvas_frame.draw( {
			let surface = self.cave_canvas_image.clone();
			move |f| {
				let surface = surface.borrow();
				let mut img = surface.image().unwrap();
				img.draw(f.x(), f.y(), f.w(), f.h());
			}
		});

		self.cave_canvas_frame.handle( {
			let mut x = 0;
			let mut y = 0;
			let surface = self.cave_canvas_image.clone();
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
		let diameter_counter = self.squares_pixel_diameter_counter.value();
		let squares_width = self.squares_width_counter.value();
		let squares_height = self.squares_height_counter.value();
		let pixels_width = squares_width * diameter_counter;
		let pixels_height = squares_height * diameter_counter;
		self.cave_canvas_frame.set_size(pixels_width as i32, pixels_height as i32);
		self.update_image_size_and_drawing();
		self.cave_canvas_scroll.redraw();
		self.cave_canvas_frame.redraw();
	}//end update_canvas(self)
}//end impl for CaveGenGroup

widget_extends!(CaveGenGroup, Tile, whole_tab_group);