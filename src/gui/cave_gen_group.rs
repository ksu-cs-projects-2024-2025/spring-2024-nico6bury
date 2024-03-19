use std::{cell::RefCell, rc::Rc};

use fltk::{app::{self, Sender}, button::Button, draw::{draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Scroll, Tile}, prelude::{DisplayExt, GroupExt, ImageExt, SurfaceDevice, ValuatorExt, WidgetBase, WidgetExt}, surface::ImageSurface, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};

use crate::gui::gui_utils::get_default_tab_padding;

/// # enum DrawState
/// This enum represents the current drawing state for the canvas.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum DrawState {
	/// indicates user is drawing wall
	Wall,
	/// indicates user is drawing floor (like erasure)
	Floor,
	/// indicates user is placing a stair/level connection
	Stair,
	/// indicates user is not allowed to draw anything
	Disabled,
}//end enum DrawState

pub struct CaveGenGroup {
	ux_whole_tab_group: Tile,
	ux_cave_canvas_scroll: Scroll,
	ux_cave_canvas_frame: Frame,
	ux_cave_canvas_image: Rc<RefCell<ImageSurface>>,
	ux_cave_canvas_draw_state: Rc<RefCell<DrawState>>,
	ux_cave_canvas_brush_size: Rc<RefCell<i32>>,
	ux_level_cur_buf: TextBuffer,
	ux_level_tot_buf: TextBuffer,
	ux_squares_width_counter: Counter,
	ux_squares_height_counter: Counter,
	ux_squares_pixel_diameter_counter: Counter,
	ux_sub_pixel_scale: i32,
}//end struct CaveGenGroup

impl Default for CaveGenGroup {
	fn default() -> Self {
		let default_image_sur = ImageSurface::new(10,10, false);
		let cave_gen_group = CaveGenGroup {
			ux_whole_tab_group: Default::default(),
			ux_cave_canvas_scroll: Default::default(),
			ux_cave_canvas_frame: Default::default(),
			ux_cave_canvas_image: Rc::from(RefCell::from(default_image_sur)),
			ux_cave_canvas_draw_state: Rc::from(RefCell::from(DrawState::Disabled)),
			ux_cave_canvas_brush_size: Rc::from(RefCell::from(1)),
			ux_level_cur_buf: Default::default(),
			ux_level_tot_buf: Default::default(),
			ux_squares_width_counter: Default::default(),
			ux_squares_height_counter: Default::default(),
			ux_squares_pixel_diameter_counter: Default::default(),
			ux_sub_pixel_scale: 8,
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

		// set up all controls within ux_exterior_canvas_settings_flex
		self.initialize_canvas_settings(&mut ux_exterior_canvas_setting_flex, msg_sender);

		// exterior vertical flex for CA controls
		let mut ux_exterior_cellular_automata_controls_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x(), ux_exterior_canvas_setting_flex.y() + ux_exterior_canvas_setting_flex.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() / 2);
		ux_exterior_cellular_automata_controls_flex.end();
		ux_exterior_cellular_automata_controls_flex.set_type(FlexType::Column);
		ux_exterior_cellular_automata_controls_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_cellular_automata_controls_flex);

		// set up all controls within ux_exterior_cellular_automata_controls_flex
		self.initialize_cellular_automata_settings(&mut ux_exterior_cellular_automata_controls_flex, msg_sender);

		// exterior vertical flex for canvas drawing stuff
		let mut ux_exterior_canvas_drawing_setting_flex = Flex::default()
			.with_pos(self.ux_cave_canvas_scroll.x(), self.ux_cave_canvas_scroll.y() + self.ux_cave_canvas_scroll.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() - ux_cave_canvas_group.height());
		ux_exterior_canvas_drawing_setting_flex.end();
		ux_exterior_canvas_drawing_setting_flex.set_type(FlexType::Column);
		ux_exterior_canvas_drawing_setting_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_canvas_drawing_setting_flex);

		// set up all controls within ux_exterior_canvas_drawing_setting_flex
		self.initialize_drawing_settings(&mut ux_exterior_canvas_drawing_setting_flex);

		// exterior vertical flex for level connections stuff
		let mut ux_exterior_level_connections_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x() + ux_exterior_canvas_setting_flex.width(), ux_exterior_canvas_setting_flex.y())
			.with_size(self.ux_whole_tab_group.width() - (self.ux_cave_canvas_scroll.width() + ux_exterior_canvas_setting_flex.width()), self.ux_whole_tab_group.height());
		ux_exterior_level_connections_flex.end();
		ux_exterior_level_connections_flex.set_type(FlexType::Column);
		ux_exterior_level_connections_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_level_connections_flex);

		// image display part of canvas
		self.update_image_size_and_drawing();
	}//end initialize()

	/// # initialize_canvas_settings(self, ux_exterior_flex)
	/// Helper method of initialize() to setup widgets within the exterior canvas settings flex.
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

	/// # initialize_drawing_settings(self, ux_exterior_flex)
	/// 
	/// This function, as a helper function for initialize(), sets up widgets for drawing settings flex.
	fn initialize_drawing_settings(&mut self, ux_exterior_flex: &mut Flex) {
		// flex for holding active/inactive identifiers
		let mut ux_interior_flex_1 = Flex::default()
			.with_pos(ux_exterior_flex.x(), ux_exterior_flex.y())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height() / 4);
		ux_interior_flex_1.end();
		ux_interior_flex_1.set_type(FlexType::Row);
		ux_interior_flex_1.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_flex_1);

		// flex for holding drawing mode buttons
		let mut ux_interior_flex_2 = Flex::default()
			.with_pos(ux_interior_flex_1.x(), ux_interior_flex_1.y() + ux_interior_flex_1.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height() / 4);
		ux_interior_flex_2.end();
		ux_interior_flex_2.set_type(FlexType::Row);
		ux_interior_flex_2.set_frame(FrameType::FlatBox);
		ux_exterior_flex.add(&ux_interior_flex_2);

		// set up frames to show whether each drawing mode is active
		let mut ux_wall_activation_frame = Frame::default()
			.with_pos(ux_interior_flex_1.x(), ux_interior_flex_1.y())
			.with_size(ux_interior_flex_1.width() / 3, ux_interior_flex_1.height())
			.with_label("Activated");
		ux_wall_activation_frame.set_color(Color::DarkGreen);
		ux_wall_activation_frame.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_wall_activation_frame);

		let mut ux_floor_activation_frame = Frame::default()
			.with_pos(ux_wall_activation_frame.x() + ux_wall_activation_frame.width(), ux_interior_flex_1.y())
			.with_size(ux_interior_flex_1.width() / 3, ux_interior_flex_1.height())
			.with_label("Disabled");
		ux_floor_activation_frame.set_color(Color::Red);
		ux_floor_activation_frame.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_floor_activation_frame);

		let mut ux_stair_activation_frame = Frame::default()
			.with_pos(ux_floor_activation_frame.x() + ux_floor_activation_frame.width(), ux_interior_flex_1.y())
			.with_size(ux_interior_flex_1.width() / 3, ux_interior_flex_1.height())
			.with_label("Disabled");
		ux_stair_activation_frame.set_color(Color::Red);
		ux_stair_activation_frame.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_stair_activation_frame);

		// set up buttons to choose between different drawing modes
		let mut ux_draw_wall_btn = Button::default()
			.with_pos(ux_interior_flex_2.x(), ux_interior_flex_2.y())
			.with_size(ux_interior_flex_2.width() / 3, ux_interior_flex_2.height())
			.with_label("Draw Wall");
		ux_draw_wall_btn.set_color(Color::Black);
		ux_draw_wall_btn.set_label_color(Color::White);
		ux_interior_flex_2.add(&ux_draw_wall_btn);
		

		let mut ux_draw_floor_btn = Button::default()
			.with_pos(ux_draw_wall_btn.x() + ux_draw_wall_btn.width(), ux_interior_flex_2.y())
			.with_size(ux_interior_flex_2.width() / 3, ux_interior_flex_2.height())
			.with_label("Draw Floor");
		ux_draw_floor_btn.set_color(Color::White);
		ux_interior_flex_2.add(&ux_draw_floor_btn);

		let mut ux_draw_stairs_btn = Button::default()
			.with_pos(ux_draw_floor_btn.x() + ux_draw_floor_btn.width(), ux_interior_flex_2.y())
			.with_size(ux_interior_flex_2.width() / 3, ux_interior_flex_2.height())
			.with_label("Draw Stairs");
		ux_draw_stairs_btn.set_color(Color::Green);
		ux_interior_flex_2.add(&ux_draw_stairs_btn);

		// set up controls for choosing 
		let ux_brush_size_label = Frame::default()
			.with_pos(ux_interior_flex_2.x(), ux_interior_flex_2.y() + ux_interior_flex_2.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height() / 4)
			.with_label("Set Brush Width, based on Canvas scale")
			.with_align(Align::Center);
		ux_exterior_flex.add(&ux_brush_size_label);

		let mut ux_brush_size_counter = Counter::default()
			.with_pos(ux_brush_size_label.x(), ux_brush_size_label.y() + ux_brush_size_label.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height()  / 4);
		ux_brush_size_counter.set_value(1.0);
		ux_brush_size_counter.set_minimum(1.0);
		ux_brush_size_counter.set_maximum(20.0);
		ux_brush_size_counter.set_precision(0);
		ux_brush_size_counter.set_step(1.0, 2);
		ux_brush_size_counter.set_type(CounterType::Simple);
		ux_exterior_flex.add(&ux_brush_size_counter);

		// set handler for the brush size counter, in order to update self.ux_cave_canvas_brush_size
		ux_brush_size_counter.handle({
			let brush_size_ref = self.ux_cave_canvas_brush_size.clone();
			move |c, ev| {
				let mut brush_size = brush_size_ref.as_ref().borrow_mut();
				match ev {
					// Updating brush size everytime the counter is clicked, whether brush size changed or not, is quite jank.
					// However, if there's a better event to use, then that is not clear from the FLTK docs. 
					// Event::Activate does not seem to be invoked when the counter is used, so that's out.
					// This implementation does assume that the counter cannot have its value changed through key presses. 
					Event::Push => {
						*brush_size = c.value() as i32;
						true
					}
					_ => false
				}
			}
		});

		// update state of draw_state
		self.ux_cave_canvas_draw_state = Rc::from(RefCell::from(DrawState::Wall));

		// set handlers for all the buttons
		let wall_frame_ref = Rc::from(RefCell::from(ux_wall_activation_frame));
		let floor_frame_ref = Rc::from(RefCell::from(ux_floor_activation_frame));
		let stairs_frame_ref = Rc::from(RefCell::from(ux_stair_activation_frame));

		ux_draw_wall_btn.handle({
			let draw_state = self.ux_cave_canvas_draw_state.clone();
			let wall_frame_ref = wall_frame_ref.clone();
			let floor_frame_ref = floor_frame_ref.clone();
			let stairs_frame_ref = stairs_frame_ref.clone();
			move |_b, ev| {
				let mut draw_state = draw_state.as_ref().borrow_mut();
				let mut wall_frame_ref = wall_frame_ref.as_ref().borrow_mut();
				let mut floor_frame_ref = floor_frame_ref.as_ref().borrow_mut();
				let mut stairs_frame_ref = stairs_frame_ref.as_ref().borrow_mut();
				match ev {
					Event::Push => {
						*draw_state = DrawState::Wall;
						wall_frame_ref.set_label("Activated");
						wall_frame_ref.set_color(Color::DarkGreen);
						floor_frame_ref.set_label("Disabled");
						floor_frame_ref.set_color(Color::Red);
						stairs_frame_ref.set_label("Disabled");
						stairs_frame_ref.set_color(Color::Red);
						true
					}
					_ => false
				}
			}
		});

		ux_draw_floor_btn.handle({
			let draw_state = self.ux_cave_canvas_draw_state.clone();
			let wall_frame_ref = wall_frame_ref.clone();
			let floor_frame_ref = floor_frame_ref.clone();
			let stairs_frame_ref = stairs_frame_ref.clone();
			move |_b, ev| {
				let mut draw_state = draw_state.as_ref().borrow_mut();
				let mut wall_frame_ref = wall_frame_ref.as_ref().borrow_mut();
				let mut floor_frame_ref = floor_frame_ref.as_ref().borrow_mut();
				let mut stairs_frame_ref = stairs_frame_ref.as_ref().borrow_mut();
				match ev {
					Event::Push => {
						*draw_state = DrawState::Floor;
						wall_frame_ref.set_label("Disabled");
						wall_frame_ref.set_color(Color::Red);
						floor_frame_ref.set_label("Activated");
						floor_frame_ref.set_color(Color::DarkGreen);
						stairs_frame_ref.set_label("Disabled");
						stairs_frame_ref.set_color(Color::Red);
						true
					}
					_ => false
				}
			}
		});

		ux_draw_stairs_btn.handle({
			let draw_state = self.ux_cave_canvas_draw_state.clone();
			let wall_frame_ref = wall_frame_ref.clone();
			let floor_frame_ref = floor_frame_ref.clone();
			let stairs_frame_ref = stairs_frame_ref.clone();
			move |_b, ev| {
				let mut draw_state = draw_state.as_ref().borrow_mut();
				let mut wall_frame_ref = wall_frame_ref.as_ref().borrow_mut();
				let mut floor_frame_ref = floor_frame_ref.as_ref().borrow_mut();
				let mut stairs_frame_ref = stairs_frame_ref.as_ref().borrow_mut();
				match ev {
					Event::Push => {
						*draw_state = DrawState::Stair;
						wall_frame_ref.set_label("Disabled");
						wall_frame_ref.set_color(Color::Red);
						floor_frame_ref.set_label("Disabled");
						floor_frame_ref.set_color(Color::Red);
						stairs_frame_ref.set_label("Activated");
						stairs_frame_ref.set_color(Color::DarkGreen);
						true
					}
					_ => false
				}
			}
		});
	}//end initialize_drawing_settings

	/// # initiliaze_cellular_automata_settings(self, ux_exterior_flex)
	/// 
	/// This function, as a helper function for initialize(), sets up widgets for CA settings flex.
	fn initialize_cellular_automata_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		let ux_settings_label = Frame::default()
		.with_label("Cellular Automata Controls\n(Requires Squareularity)");
			// .with_size(ux_exterior_flex.width(), 50)
			// .with_pos(ux_exterior_flex.x(), ux_exterior_flex.y())
		ux_exterior_flex.add(&ux_settings_label);

		// add the spacer flexes
		let mut ux_interior_flex_1 = Flex::default().with_type(FlexType::Row);
		ux_interior_flex_1.end();
		ux_exterior_flex.add(&ux_interior_flex_1);
		let mut ux_interior_flex_2 = Flex::default().with_type(FlexType::Row);
		ux_interior_flex_2.end();
		ux_exterior_flex.add(&ux_interior_flex_2);
		let mut ux_interior_flex_3 = Flex::default().with_type(FlexType::Row);
		ux_interior_flex_3.end();
		ux_exterior_flex.add(&ux_interior_flex_3);

		// add the counter controls
		let ux_neighbor_closeness_label = Frame::default().with_label("CA Neighbor Closeness");
		ux_interior_flex_1.add(&ux_neighbor_closeness_label);

		let mut ux_neighbor_closeness_counter = Counter::default();
		ux_neighbor_closeness_counter.set_value(1.0);
		ux_neighbor_closeness_counter.set_bounds(1.0, 10.0);
		ux_neighbor_closeness_counter.set_precision(0);
		ux_neighbor_closeness_counter.set_step(1.0, 1);
		ux_neighbor_closeness_counter.set_type(CounterType::Simple);
		ux_interior_flex_1.add(&ux_neighbor_closeness_counter);

		let ux_neighbor_threshold_label = Frame::default().with_label("CA Neighbor Threshold");
		ux_interior_flex_2.add(&ux_neighbor_threshold_label);

		let mut ux_neighbor_threshold_counter = Counter::default();
		ux_neighbor_threshold_counter.set_value(5.0);
		ux_neighbor_threshold_counter.set_bounds(1.0, 100.0);
		ux_neighbor_threshold_counter.set_precision(0);
		ux_neighbor_threshold_counter.set_step(1.0, 1);
		ux_neighbor_threshold_counter.set_type(CounterType::Simple);
		ux_interior_flex_2.add(&ux_neighbor_threshold_counter);

		let ux_iterations_label = Frame::default().with_label("Iterations to Run");
		ux_interior_flex_3.add(&ux_iterations_label);

		let mut ux_iterations_counter = Counter::default();
		ux_iterations_counter.set_value(1.0);
		ux_iterations_counter.set_bounds(1.0, 100.0);
		ux_iterations_counter.set_precision(0);
		ux_iterations_counter.set_step(1.0, 5);
		ux_iterations_counter.set_type(CounterType::Normal);
		ux_interior_flex_3.add(&ux_iterations_counter);

		// add handler to counters to ensure bounds are updated
		let ux_neighbor_threshold_ref = Rc::from(RefCell::from(ux_neighbor_threshold_counter));
		ux_neighbor_closeness_counter.handle({
			let ux_neighbor_threshold_ref = ux_neighbor_threshold_ref.clone();
			move |c, ev| {
				match ev {
					Event::Push => {
						let new_closeness_val = c.value() as i32;
						// formula is (2c+1)^2 - 1
						let closeness_max_neighbors = ((new_closeness_val*2)+1)*((new_closeness_val*2)+1)-1;
						let mut ux_neighbor_threshold = ux_neighbor_threshold_ref.as_ref().borrow_mut();
						ux_neighbor_threshold.set_maximum(closeness_max_neighbors as f64);
						if ux_neighbor_threshold.value() as i32 > closeness_max_neighbors {ux_neighbor_threshold.set_value(closeness_max_neighbors as f64);}
						true
					},
					_ => false
				}
			}
		});

		// button for actually starting generation
		let mut ux_run_ca_btn = Button::default().with_label("Run Generation");
		ux_run_ca_btn.emit(msg_sender.clone(), "CaveGen:CA:RunGeneration".to_string());
	}//end initialize_cellular_automata_settings()

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
		let sub_pixel_scale = Rc::from(RefCell::from(self.ux_sub_pixel_scale));
		let brush_size_ref = &self.ux_cave_canvas_brush_size;
		let draw_state = &self.ux_cave_canvas_draw_state;
		let surface_ref = &self.ux_cave_canvas_image;

		self.ux_cave_canvas_frame.draw( {
			let surface = surface_ref.clone();
			move |f| {
				let surface = surface.borrow();
				let mut img = surface.image().unwrap();
				img.draw(f.x(), f.y(), f.w(), f.h());
			}
		});

		self.ux_cave_canvas_frame.handle( {
			let mut x = 0;
			let mut y = 0;
			let surface = surface_ref.clone();
			let pixel_scale_clone = pixel_scale_ref.clone();
			let sub_pixel_scale_clone = sub_pixel_scale.clone();
			let brush_size_clone = brush_size_ref.clone();
			let draw_state = draw_state.clone();
			move |f, ev| {
				let surface = surface.as_ref().borrow_mut();
				let pixel_scale = pixel_scale_clone.as_ref().borrow();
				let sub_pixel_scale_ref = sub_pixel_scale_clone.as_ref().borrow();
				let brush_size = brush_size_clone.as_ref().borrow();
				let draw_state_ref = draw_state.as_ref().borrow();
				// update draw color and size based on draw state
				let draw_color = match *draw_state_ref {
					DrawState::Wall => Color::Black,
					DrawState::Floor => Color::White,
					DrawState::Stair => Color::Green,
					DrawState::Disabled => Color::White,
				};
				let draw_size = match *draw_state_ref {
					DrawState::Wall => *pixel_scale * *brush_size,
					DrawState::Floor => *pixel_scale * *brush_size,
					DrawState::Stair => *pixel_scale,
					DrawState::Disabled => 0,
				};
				match ev {
					Event::Push => {
						ImageSurface::push_current(&surface);
						set_draw_color(draw_color);
						set_line_style(LineStyle::Solid, draw_size);
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
						set_draw_color(draw_color);
						set_line_style(LineStyle::Solid, draw_size);
						let coords = app::event_coords();
						draw_line(x - f.x(), y - f.y(), coords.0 - f.x(), coords.1 - f.y());
						x = coords.0;
						y = coords.1;
						ImageSurface::pop_current();
						f.redraw();
						true
					},
					Event::Released => {
						CaveGenGroup::ux_squareularize_canvas(&surface, f, &pixel_scale, &sub_pixel_scale_ref);
						true
					}
					_ => false
				}//end matching event
			}//end handle move
		});

		self.ux_cave_canvas_scroll.redraw();
		self.ux_cave_canvas_frame.redraw();
	}//end update_image_size_and_height(prev_w, prev_h)

	/// This function takes a mutable reference to an image surface and performs operations on it
	/// to squareularize it.  
	/// This function will return false if some part of this process is unsuccessful, or true if things went okay.  
	/// Arguments:  
	/// - canvas - the image surface you want to squareularize  
	/// - pixel_scale - how many pixels are in one grid square
	/// - sub_pixel_scale - scale of sub-grid within each grid square
	fn ux_squareularize_canvas(canvas: &ImageSurface, canvas_frame: &mut Frame, pixel_scale: &i32, sub_pixel_scale: &i32) -> bool {
		if let Some(cur_img) = canvas.image() {
			match cur_img.depth() {
				fltk::enums::ColorDepth::Rgb8 => {
					// // soft todo: optimization throughout this function
					let raw_pixel_data = cur_img.to_rgb_data();

					// convert out pixel Vec<u8> into a Vec<u8,u8,u8> for RGB
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

					// figure out list of bounds for squares in our grid
					let square_scale = pixel_scale * sub_pixel_scale;
					let square_width = cur_img.width() / square_scale;
					let square_height = cur_img.height() / square_scale;
					// format of (x, y, Color), assume square_width and square_height, fill in color later
					let mut squares: Vec<(i32, i32, (u8,u8,u8))> = Vec::new();
					for mut x in (0..cur_img.width()).step_by(square_width as usize) {
						// Squares at edges might overlap, but they won't be out of bounds
						if x + square_width > cur_img.width() {x = cur_img.width() - square_width;}
						for mut y in (0..cur_img.height()).step_by(square_height as usize) {
							// Squares at edges might overlap, but they won't be out of bounds
							if y + square_height > cur_img.height() {y = cur_img.height() - square_height;}
							squares.push((x,y,Color::Cyan.to_rgb()));
						}//end looping over all potential y values for sub-squares
					}//end looping over all potential x values for sub-squares

					// figure out dominant color in each square 
					for square in &mut squares {
						// figure out dominant color here, set square.2 to that
						// color_counts1 and color_counts2 are parallel
						let mut color_counts1: Vec<(u8,u8,u8)> = Vec::new();
						let mut color_counts2: Vec<u64> = Vec::new();
						for y in square.1..(square_height + square.1) {
							for x in square.0..(square_width + square.0) {
								let this_overall_index = (y * cur_img.width()) + x;
								let this_rgb = rgb_pixel_vec[this_overall_index as usize];
								let this_color = (this_rgb.0, this_rgb.1, this_rgb.2);
								if let Some(color_index) = color_counts1.iter().position(|&c| c == this_color) {
									color_counts2[color_index] += 1;
								} else {
									color_counts1.push(this_color);
									color_counts2.push(1);
								}//end else we need to add new entry to color counts
							}//end looping over all x values within square
						}//end looping over all y values within square
						
						// check to see which color has the highest count
						let mut running_most = ((40,40,40), 0);
						for (i, count) in color_counts2.iter().enumerate() {
							if *count > running_most.1 { running_most = (color_counts1[i], color_counts2[i]); }
						}//end getting the color that's most common from color counts
						*square = (square.0, square.1, running_most.0);
					}//end figuring out which color is dominant

					// paint dominant color to entire square using the canvas
					ImageSurface::push_current(&canvas);
					draw_rect_fill(0, 0, cur_img.width(), cur_img.height(), Color::Magenta);
					for square in &mut squares {
						let a = (square.0, square.1, square_width, square_height);
						let c = Color::from_rgb(square.2.0, square.2.1, square.2.2);
						draw_rect_fill(a.0, a.1, a.2, a.3, c);
					}//end painting dominant color to entirety of each square
					ImageSurface::pop_current();
					canvas_frame.redraw();

					println!("Performed Squareularization");
					true
				},
				_ => return false,
			}
		} else {return false;}
	}//end squareularize_canvas(canvas)

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
	}//end update_canvas(self)
}//end impl for CaveGenGroup

widget_extends!(CaveGenGroup, Tile, ux_whole_tab_group);