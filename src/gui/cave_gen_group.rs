use std::{cell::RefCell, rc::Rc};

use fltk::{app::{self, Sender}, button::Button, dialog, draw::{draw_line, draw_point, set_draw_color, set_line_style, LineStyle}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Scroll, Tile}, prelude::{DisplayExt, GroupExt, ImageExt, SurfaceDevice, ValuatorExt, WidgetBase, WidgetExt}, surface::ImageSurface, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};

use crate::{cellular_automata::CAC, gui::gui_utils::get_default_tab_padding, squares::SquareGrid};

use super::gui_utils::{squareularization_color_square, squareularization_color_squares, squareularization_get_dominant_color, squareularization_get_rgb_pixels, squareularization_split_img_to_squares, ux_squareularize_canvas, ListBox, SquareStairDisplay};

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
	/// contains last SquareGrid, plus vec with row, col coords of all stairs we found
	ux_last_square_grid: Rc<RefCell<Option<SquareGrid>>>,
	ux_level_cur_buf: TextBuffer,
	ux_level_tot_buf: TextBuffer,
	ux_squares_width_counter: Counter,
	ux_squares_height_counter: Counter,
	ux_squares_pixel_diameter_counter: Counter,
	ux_sub_pixel_scale: usize,
	ux_ca_neighborhood_size_counter: Counter,
	ux_ca_neighborhood_thresh_counter: Rc<RefCell<Counter>>,
	ux_ca_generations_to_run_counter: Counter,
	ux_stairs_list: Rc<RefCell<ListBox<SquareStairDisplay>>>,
	ux_wall_frame_ref: Rc<RefCell<Frame>>,
	ux_floor_frame_ref: Rc<RefCell<Frame>>,
	ux_stairs_frame_ref: Rc<RefCell<Frame>>,
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
			ux_last_square_grid: Rc::from(RefCell::from(None)),
			ux_level_cur_buf: Default::default(),
			ux_level_tot_buf: Default::default(),
			ux_squares_width_counter: Default::default(),
			ux_squares_height_counter: Default::default(),
			ux_squares_pixel_diameter_counter: Default::default(),
			ux_sub_pixel_scale: 1,
			ux_ca_neighborhood_size_counter: Default::default(),
			ux_ca_neighborhood_thresh_counter: Default::default(),
			ux_ca_generations_to_run_counter: Default::default(),
			ux_stairs_list: Rc::from(RefCell::from(ListBox::new(0, 0, 10, 10, 10))),
			ux_wall_frame_ref: Rc::from(RefCell::from(Frame::default())),
			ux_floor_frame_ref: Rc::from(RefCell::from(Frame::default())),
			ux_stairs_frame_ref: Rc::from(RefCell::from(Frame::default())),
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
		ux_exterior_level_connections_flex.set_frame(FrameType::FlatBox);
		self.ux_whole_tab_group.add(&ux_exterior_level_connections_flex);

		// set up all controls within ux_exterior_level_connections_flex
		self.initialize_level_connection_settings(&mut ux_exterior_level_connections_flex, &msg_sender);

		// image display part of canvas
		self.update_image_size_and_drawing();
	}//end initialize()

	/// # initialize_canvas_settings(self, ux_exterior_flex)
	/// Helper method of initialize() to setup widgets within the exterior canvas settings flex.
	fn initialize_canvas_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		// label for this section
		let ux_canvas_setting_section_label = Frame::default().with_label("Canvas Settings");
		ux_exterior_flex.add(&ux_canvas_setting_section_label);
		
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
		self.ux_squares_width_counter.set_value(100.0);
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
		self.ux_squares_height_counter.set_value(100.0);
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
		self.ux_squares_pixel_diameter_counter.set_value(4.0);
		self.ux_squares_pixel_diameter_counter.set_minimum(1.0);
		self.ux_squares_pixel_diameter_counter.set_maximum(100.0);
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
		// label for this section
		let ux_drawing_setting_section_label = Frame::default().with_label("Drawing Settings");
		ux_exterior_flex.add(&ux_drawing_setting_section_label);
		
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
			.with_label("Disabled");
		ux_wall_activation_frame.set_color(Color::Red);
		ux_wall_activation_frame.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_wall_activation_frame);

		let mut ux_floor_activation_frame = Frame::default()
			.with_pos(ux_wall_activation_frame.x() + ux_wall_activation_frame.width(), ux_interior_flex_1.y())
			.with_size(ux_interior_flex_1.width() / 3, ux_interior_flex_1.height())
			.with_label("Activated");
		ux_floor_activation_frame.set_color(Color::DarkGreen);
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

		// set up controls for choosing brush size
		let ux_brush_size_label = Frame::default()
			.with_pos(ux_interior_flex_2.x(), ux_interior_flex_2.y() + ux_interior_flex_2.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height() / 4)
			.with_label("Set Brush Width, based on Canvas scale")
			.with_align(Align::Center);
		ux_exterior_flex.add(&ux_brush_size_label);

		let mut ux_brush_size_counter = Counter::default()
			.with_pos(ux_brush_size_label.x(), ux_brush_size_label.y() + ux_brush_size_label.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height()  / 4);
		ux_brush_size_counter.set_value(2.0);
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
		self.ux_cave_canvas_draw_state = Rc::from(RefCell::from(DrawState::Floor));

		// set handlers for all the buttons
		self.ux_wall_frame_ref = Rc::from(RefCell::from(ux_wall_activation_frame));
		self.ux_floor_frame_ref = Rc::from(RefCell::from(ux_floor_activation_frame));
		self.ux_stairs_frame_ref = Rc::from(RefCell::from(ux_stair_activation_frame));
		let wall_frame_ref = &self.ux_wall_frame_ref; //Rc::from(RefCell::from(ux_wall_activation_frame));
		let floor_frame_ref = &self.ux_floor_frame_ref; //Rc::from(RefCell::from(ux_floor_activation_frame));
		let stairs_frame_ref = &self.ux_stairs_frame_ref; //Rc::from(RefCell::from(ux_stair_activation_frame));

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
		.with_label("Cellular Automata Controls");
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
		self.ux_ca_neighborhood_thresh_counter = Rc::from(RefCell::from(ux_neighbor_threshold_counter));

		let ux_iterations_label = Frame::default().with_label("Iterations to Run");
		ux_interior_flex_3.add(&ux_iterations_label);

		let mut ux_iterations_counter = Counter::default();
		ux_iterations_counter.set_value(2.0);
		ux_iterations_counter.set_bounds(1.0, 100.0);
		ux_iterations_counter.set_precision(0);
		ux_iterations_counter.set_step(1.0, 5);
		ux_iterations_counter.set_type(CounterType::Normal);
		ux_interior_flex_3.add(&ux_iterations_counter);
		self.ux_ca_generations_to_run_counter = ux_iterations_counter;

		// add handler to counters to ensure bounds are updated
		let ux_neighbor_threshold_ref = self.ux_ca_neighborhood_thresh_counter.clone();
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

		self.ux_ca_neighborhood_size_counter = ux_neighbor_closeness_counter;
		

		// button for actually starting generation
		let mut ux_run_ca_btn = Button::default().with_label("Run Generation");
		ux_run_ca_btn.emit(msg_sender.clone(), "CaveGen:CA:RunGeneration".to_string());
		ux_exterior_flex.add(&ux_run_ca_btn);
	}//end initialize_cellular_automata_settings()

	/// This function, as a helper function for initialize(), sets up widgets for level connections
	fn initialize_level_connection_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		// set up label for this section
		let ux_level_connection_settings_section_label = Frame::default()
			.with_size(ux_exterior_flex.width(), 50)
			.with_label("Level Connection Settings");
		ux_exterior_flex.add(&ux_level_connection_settings_section_label);


		let mut ux_level_connection_list: ListBox<SquareStairDisplay> = ListBox::new(
			ux_level_connection_settings_section_label.x(),
			ux_level_connection_settings_section_label.y() + ux_level_connection_settings_section_label.h(),
			ux_exterior_flex.width() - 5,
			ux_exterior_flex.height() - 200,
			30
		);
		ux_level_connection_list.set_label_size(15);
		// end debug test value entry
		ux_exterior_flex.add_resizable(ux_level_connection_list.get_scroll_ref());
		self.ux_stairs_list = Rc::from(RefCell::from(ux_level_connection_list));

		let mut ux_level_connection_add_btn = Button::default()
			.with_size(ux_exterior_flex.width(), 50)
			.with_label("Add Level Connection");
		ux_exterior_flex.add(&ux_level_connection_add_btn);

		ux_level_connection_add_btn.set_callback({
			move |b| {
				// dialog::choice2(b.x(), b.y(), "txt", "yes", "no", "");
				// dialog::input(b.x(), b.y(), "Add stairs", "placeholder");
				dialog::message(b.x(), b.y(), "placeholder, not yet implemented");
			}//end moving for callback
		});

		let mut ux_level_connection_edit_btn = Button::default()
			.with_size(ux_exterior_flex.width(), 50)
			.with_label("Edit Level Connection");
		ux_exterior_flex.add(&ux_level_connection_edit_btn);

		ux_level_connection_edit_btn.set_callback({
			move |b| {
				dialog::message(b.x(), b.y(), "Placeholder, not yet implemented");
			}//end moving for callback
		});

		let mut ux_level_connection_remove_btn = Button::default()
			.with_size(ux_exterior_flex.width(), 50)
			.with_label("Remove Level Connection");
		ux_exterior_flex.add(&ux_level_connection_remove_btn);

		ux_level_connection_remove_btn.emit(msg_sender.clone(), String::from("CaveGen:Stairs:Remove"));

		ux_exterior_flex.fixed(&ux_level_connection_settings_section_label, 50);
		ux_exterior_flex.fixed(&ux_level_connection_add_btn, 50);
		ux_exterior_flex.fixed(&ux_level_connection_edit_btn, 50);
		ux_exterior_flex.fixed(&ux_level_connection_remove_btn, 50);
	}//end initialize_level_connection_settings(&mut self, ux_exterior_flex)

	pub fn get_cave_gen_stairs_selected(&self) -> Vec<String> {
		let stairs_list_ref = &self.ux_stairs_list;
		let stairs_list_ref_clone = stairs_list_ref.clone();
		let stairs_list_borrow = stairs_list_ref_clone.as_ref().borrow();
		stairs_list_borrow.get_selected_elements().into_iter().map(|val| format!("{}", val)).collect()
	}//end get_cave_gen_stairs-selected(self)(

	/// Removes selected stairs from the list and canvas
	pub fn remove_cave_gen_stairs_selected(&mut self) {
		let last_square_stair_ref = &self.ux_last_square_grid;
		let last_square_stair_ref_clone = last_square_stair_ref.clone();
		let mut last_square_stair_borrow = last_square_stair_ref_clone.as_ref().borrow_mut();
		match last_square_stair_borrow.as_mut() {
			Some(squares) => {
				let stairs_list_ref = &self.ux_stairs_list;
				let stairs_list_ref_clone = stairs_list_ref.clone();
				let mut stairs_list_borrow = stairs_list_ref_clone.as_ref().borrow_mut();
				let stairs_list_selected_elements = stairs_list_borrow.get_selected_elements();
				let mut squares_to_recolor = Vec::new();
				for selected_element in stairs_list_selected_elements {
					match squares.get_mut(&selected_element.row_idx, &selected_element.col_idx) {
						Some(this_square) => {
							this_square.set_color((0,0,0));
							squares_to_recolor.push(this_square.clone());
						},
						None => println!("Couldn't access square {:?} while removing stairs from list.", selected_element)
					}//end matching whether we can get this square
				}//end looping over each selected element
				let canvas_ref = &self.ux_cave_canvas_image;
				let canvas_ref_clone = canvas_ref.clone();
				let canvas_borrow = canvas_ref_clone.borrow();
				squareularization_color_square(&canvas_borrow, squares_to_recolor.iter());
				self.ux_cave_canvas_frame.redraw();
				stairs_list_borrow.remove_selected_elements()
			},
			None => println!("We don't have last squares and stairs?"),
		}//end matching whether we had last squarularization
		// TODO: Also remove the relevant squares
	}//end remove_cave_gen_stairs_selected(self)

	/// # update_image_size_and_drawing(&mut self)
	/// This function creates/updates the canvas surface for drawing cave stuff with the right size.  
	/// Note: Code for drawing things on the canvas is heavily adapted from the FLTK paint example that is
	/// part of the fltk-rs repo. My implementation is heavily adapted, but that was my reference point for
	/// figuring out how the handle and draw function are generally supposed to work.
	fn update_image_size_and_drawing(&mut self) {
		let canvas_surface = ImageSurface::new(self.ux_cave_canvas_frame.width(), self.ux_cave_canvas_frame.height(), false);
		
		ImageSurface::push_current(&canvas_surface);
		// TODO: Redo filling to not reset previous work, probably by copying drawings out of old surface image, maybe by using fltk::draw_image or fltk::draw_rbg and limiting size of image? If changing resolution, might need to grid-ify first
		fltk::draw::draw_rect_fill(0,0,self.ux_cave_canvas_frame.width(), self.ux_cave_canvas_frame.height(), Color::Black);
		ImageSurface::pop_current();

		self.ux_cave_canvas_image = Rc::from(RefCell::from(canvas_surface));

		let pixel_scale = self.ux_squares_pixel_diameter_counter.value() as i32;
		let pixel_scale_ref = Rc::from(RefCell::from(pixel_scale));
		let sub_pixel_scale = Rc::from(RefCell::from(self.ux_sub_pixel_scale));
		let brush_size_ref = &self.ux_cave_canvas_brush_size;
		let draw_state = &self.ux_cave_canvas_draw_state;
		let surface_ref = &self.ux_cave_canvas_image;
		let stairs_list_ref = &self.ux_stairs_list;
		let last_square_grid_ref = &self.ux_last_square_grid;

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
			let last_square_grid = last_square_grid_ref.clone();
			let stairs_list_ref_clone = stairs_list_ref.clone();
			move |f, ev| {
				let surface = surface.as_ref().borrow();
				let pixel_scale = {pixel_scale_clone.as_ref().borrow().clone()};
				let sub_pixel_scale_ref = {sub_pixel_scale_clone.as_ref().borrow().clone()};
				let brush_size = {brush_size_clone.as_ref().borrow().clone()};
				let draw_state_ref = {draw_state.as_ref().borrow().clone()};
				// update draw color and size based on draw state
				let draw_color = match draw_state_ref {
					DrawState::Wall => Color::Black,
					DrawState::Floor => Color::White,
					DrawState::Stair => Color::Green,
					DrawState::Disabled => Color::White,
				};
				let draw_size = match draw_state_ref {
					DrawState::Wall => pixel_scale * brush_size,
					DrawState::Floor => pixel_scale * brush_size,
					DrawState::Stair => pixel_scale,
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
						draw_point(x - f.x(), y - f.y());
						set_line_style(LineStyle::Solid, 0);
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
						set_line_style(LineStyle::Solid, 0);
						ImageSurface::pop_current();
						f.redraw();
						true
					},
					Event::Released => {
						let pixel_scale = pixel_scale as usize;
						let mut last_square_grid_clone = last_square_grid.as_ref().borrow_mut();
						let mut stairs_list_ref = stairs_list_ref_clone.as_ref().borrow_mut();
						if let Some(squares) = ux_squareularize_canvas(&surface, Some(&CAC::colors_vec()), &pixel_scale, &sub_pixel_scale_ref) {
							let stairs_list = CaveGenGroup::ux_get_stair_coord_list(&squares);
							CaveGenGroup::ux_update_stairs_list(stairs_list.clone(), &mut stairs_list_ref);
							*last_square_grid_clone = Some(squares);
						}//end if we our squares properly

						f.redraw();
						true
					}
					_ => false
				}//end matching event
			}//end handle move
		});

		// initializes ux_last_square_grid_and_stairs_list so our cache works before user draws anything.
		let _ = self.get_squareularization();
		// make sure gui get drawn on first init stuff
		self.ux_cave_canvas_scroll.redraw();
		self.ux_cave_canvas_frame.redraw();
	}//end update_image_size_and_height(prev_w, prev_h)

	fn ux_update_stairs_list(stairs_list: Vec<SquareStairDisplay>, stairs_list_box: &mut ListBox<SquareStairDisplay>) {
		stairs_list_box.clear_elements();
		stairs_list_box.set_elements(stairs_list);
	}//end ux_update_stairs_list()

	/// Gets a (row, col) list of coordinates pointing to every square in squares which
	/// is classified as a stair via color and CA::CAC::classify().
	fn ux_get_stair_coord_list(squares: &SquareGrid) -> Vec<SquareStairDisplay> {
		let mut stairs_list = Vec::new();
		for row in 0..*squares.rows() {
			for col in 0..*squares.cols() {
				match squares.get(&row, &col) {
					Some(square) => {
						if CAC::Stairs == CAC::classify(*square.color()) {
							stairs_list.push(SquareStairDisplay {square: *square, row_idx: row, col_idx: col});
						}//end if we found a stair
					},
					None => println!("Failed to get an index when counting stairs?"),
				}//end matching whether we got the index
			}//end looping over cols
		}//end looping over rows
		return stairs_list;
	}//end ux_get_stair_coord_list(squares)

	/// Gets squareularized grid and returns that grid, 
	/// including dominant color for each square.
	pub fn get_squareularization(&self) -> Option<SquareGrid> {
		let canvas_ref = &self.ux_cave_canvas_image;
		let canvas_ref_clone = canvas_ref.clone();
		let canvas_borrow = canvas_ref_clone.as_ref().borrow();
		
		match squareularization_get_rgb_pixels(&canvas_borrow) {
			Some(image_and_pixels) => {
				let image = image_and_pixels.0;
				let pixels = image_and_pixels.1;
				let img_width = image.width() as usize;
				let img_height = image.height() as usize;
				let pixel_scale = self.ux_squares_pixel_diameter_counter.value() as usize;
				let sub_pixel_scale = self.ux_sub_pixel_scale;
				let square_scale = pixel_scale * sub_pixel_scale;
				let square_width = square_scale;//img_width / square_scale;
				let square_height = square_scale;//img_height / square_scale;

				match squareularization_split_img_to_squares(&img_width, &img_height, &square_width, &square_height) {
					Some(mut squares) => {
						squareularization_get_dominant_color(&mut squares, Some(&CAC::colors_vec()), &pixels, &img_width, &square_width, &square_height);
						// let stairs = CaveGenGroup::ux_get_stair_coord_list(&squares);
						let last_square_grid_and_stair_list_ref = &self.ux_last_square_grid;
						let last_square_grid_and_stair_list_ref_clone = last_square_grid_and_stair_list_ref.clone();
						let mut last_square_grid_and_stair_list_borrow = last_square_grid_and_stair_list_ref_clone.as_ref().borrow_mut();
						*last_square_grid_and_stair_list_borrow = Some(squares.clone());
						Some(squares)
					},
					None => {
						println!("Couldn't get squareularization. It is likely that a SquareGrid could not be parsed.");
						None
					}}},
			None => None,
		}//end matching squareularization result
	}//end get_squareularization(&mut self)

	/// Gets the last SquareGrid we generated. A new SquareGrid is generated whenever
	/// the user draws on the canvas, so it should theoretically be up to date.  
	/// A new SquareGrid is generated when this struct is initialized as well, so this function
	/// should theoretically always give an up-to-date squaregrid.
	pub fn get_last_squareularization(&self) -> Option<SquareGrid> {
		let last_squares_borrow = {
			let last_squares_ref = &self.ux_last_square_grid;
			let last_squares_ref_clone = last_squares_ref.clone();
			let last_square_clone = last_squares_ref_clone.as_ref().borrow().clone();
			last_square_clone};
		match last_squares_borrow {
			Some(squares) => Some(squares),
			None => None,
		}//end matching existence of last squareularization
	}//end get_last_squareularization()

	/// Sets the canvas based on a squareularization.  
	/// The color from square_info is set to the square in question.  
	/// This function might panic under a variety of circumstances. 
	/// See [CaveGenGroup]::[squareularization_color_squares()] 
	/// for more information, as calls to that function are the 
	/// main reason for panics.
	pub fn set_squareularization(&mut self, squares: &SquareGrid) {
		let canvas_ref = &self.ux_cave_canvas_image;
		let canvas_ref_clone = canvas_ref.clone();
		let canvas_borrow = canvas_ref_clone.borrow();

		squareularization_color_squares(&canvas_borrow, squares,&false);
		self.ux_cave_canvas_frame.redraw();

		// update last squareularization
		// let stairs = CaveGenGroup::ux_get_stair_coord_list(squares);
		let last_stairs_ref = &self.ux_last_square_grid;
		let last_stairs_ref_clone = last_stairs_ref.clone();
		let mut last_stairs_borrow = last_stairs_ref_clone.as_ref().borrow_mut();
		*last_stairs_borrow = Some(squares.clone());
	}//end set_squareularization(&mut self, square_info)

	/// # update_canvas(&mut self)
	/// This function updates the size of the drawing canvas based on user settings. 
	pub fn update_canvas(&mut self) {
		let diameter_counter = self.ux_squares_pixel_diameter_counter.value();
		let squares_width = self.ux_squares_width_counter.value();
		let squares_height = self.ux_squares_height_counter.value();
		let pixels_width = squares_width * diameter_counter;
		let pixels_height = squares_height * diameter_counter;
		self.ux_cave_canvas_frame.set_size(pixels_width as i32, pixels_height as i32);
		let stair_list_ref = &self.ux_stairs_list;
		let stair_list_ref_clone = stair_list_ref.clone();
		let mut stair_list_borrow = stair_list_ref_clone.as_ref().borrow_mut();
		stair_list_borrow.clear_elements();
		self.update_image_size_and_drawing();
	}//end update_canvas(self)

	/// gets CA settings for cave canvas.
	/// Returns neighborhood size, threshold, and generations to run
	pub fn get_cave_canvas_ca_settings(&self) -> (usize,usize,usize) {
		let size = self.ux_ca_neighborhood_size_counter.value() as usize;
		let thresh = self.ux_ca_neighborhood_thresh_counter.as_ref().borrow().value() as usize;
		let iterations = self.ux_ca_generations_to_run_counter.value() as usize;
		(size, thresh, iterations)
	}//end get_cave_canvas_ca_settings()
}//end impl for CaveGenGroup

widget_extends!(CaveGenGroup, Tile, ux_whole_tab_group);