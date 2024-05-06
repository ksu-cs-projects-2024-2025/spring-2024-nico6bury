use std::{cell::RefCell, rc::Rc};

use fltk::{app::{self, Sender}, button::Button, dialog, draw::{draw_line, draw_point, draw_rect_fill, set_draw_color, set_line_style, LineStyle}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Scroll, Tile}, prelude::{DisplayExt, GroupExt, ImageExt, SurfaceDevice, ValuatorExt, WidgetBase, WidgetExt}, surface::ImageSurface, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};
use nice_map_generator::squares::SquareGrid;

use super::gui_utils::{get_default_tab_padding, squareularization_color_square, ux_squareularize_canvas, ListBox, SquareStairDisplay};


/// # enum DrawState
/// This enum represents the current drawing state for the canvas.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum DrawState {
	Wall,
	Floor,
	Stair,
	Empty,
	Door,
	RoomStart,
	Disabled,
}//end enum DrawState

impl DrawState {
	fn get_color_vec() -> Vec<Color> {
		let mut color_vec = Vec::new();
		color_vec.push(Color::Red);
		color_vec.push(Color::Blue);
		color_vec.push(Color::Black);
		color_vec.push(Color::White);
		color_vec.push(Color::Green);
		color_vec.push(Color::Light1);
		color_vec
	}
	
	fn get_color_vec_u8() -> Vec<(u8,u8,u8)> {
		DrawState::get_color_vec().iter().map(|elem| elem.to_rgb()).collect()
	}
}

pub struct RoomGenGroup {
	ux_whole_tab_group: Tile,
	ux_canvas_scroll: Scroll,
	ux_canvas_frame: Frame,
	ux_canvas_image: Rc<RefCell<ImageSurface>>,
	ux_draw_state: Rc<RefCell<DrawState>>,
	ux_brush_size: Rc<RefCell<i32>>,
	ux_last_square_grid: Rc<RefCell<Option<SquareGrid>>>,
	ux_level_cur_buf: TextBuffer,
	ux_level_tot_buf: TextBuffer,
	ux_squares_width_counter: Counter,
	ux_squares_height_counter: Counter,
	ux_squares_pixel_diameter_counter: Counter,
	ux_sub_pixel_scale: usize,
	ux_stairs_list: Rc<RefCell<ListBox<SquareStairDisplay>>>,
	ux_draw_frame_ref: Rc<RefCell<Frame>>,
}//end struct RoomGenGroup

impl Default for RoomGenGroup {
	fn default() -> Self {
		let default_image_sur = ImageSurface::new(10,10,false);
		let build_gen_group = RoomGenGroup {
			ux_whole_tab_group: Default::default(),
			ux_canvas_scroll: Default::default(),
			ux_canvas_frame: Default::default(),
			ux_canvas_image: Rc::from(RefCell::from(default_image_sur)),
			ux_draw_state: Rc::from(RefCell::from(DrawState::Disabled)),
			ux_brush_size: Rc::from(RefCell::from(1)),
			ux_last_square_grid: Rc::from(RefCell::from(None)),
			ux_level_cur_buf: Default::default(),
			ux_level_tot_buf: Default::default(),
			ux_squares_width_counter: Default::default(),
			ux_squares_height_counter: Default::default(),
			ux_squares_pixel_diameter_counter: Default::default(),
			ux_sub_pixel_scale: 1,
			ux_stairs_list: Rc::from(RefCell::from(ListBox::new(0,0,10,10,10))),
			ux_draw_frame_ref: Rc::from(RefCell::from(Frame::default())),
		};//end struct construction
		build_gen_group.ux_whole_tab_group.end();
		build_gen_group.ux_canvas_scroll.end();
		build_gen_group
	}//end default()
}//end impl Default for RoomGenGroup

impl RoomGenGroup {
	pub fn initialize(&mut self, msg_sender: &Sender<String>) {
		self.ux_whole_tab_group.set_frame(FrameType::FlatBox);

		// exterior group for canvas and scroll to fix border issues
		let mut ux_canvas_group = Group::default()
			.with_pos(0, self.ux_whole_tab_group.y())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() * 2 / 3);
		ux_canvas_group.end();
		ux_canvas_group.set_frame(FrameType::FlatBox);
		self.ux_whole_tab_group.add(&ux_canvas_group);

		// scrollable container for size-locked canvas
		self.ux_canvas_scroll = Scroll::default()
			.with_pos(ux_canvas_group.x(), ux_canvas_group.y())
			.with_size(ux_canvas_group.width(), ux_canvas_group.height());
		self.ux_canvas_scroll.end();
		self.ux_canvas_scroll.set_frame(FrameType::BorderBox);
		ux_canvas_group.add(&self.ux_canvas_scroll);

		// size-locked canvas for drawing
		self.ux_canvas_frame = Frame::default()
			.with_pos(self.ux_canvas_scroll.x() + get_default_tab_padding(), self.ux_canvas_scroll.y() + get_default_tab_padding())
			.with_size(100,100)
			.with_label("Canvas thingy");
		self.ux_canvas_frame.set_frame(FrameType::BorderBox);
		self.ux_canvas_scroll.add(&self.ux_canvas_frame);

		// exterior vertical flex for canvas setting stuff
		let mut ux_exterior_canvas_setting_flex = Flex::default()
			.with_pos(self.ux_canvas_scroll.x() + self.ux_canvas_scroll.width(), self.ux_whole_tab_group.y())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() / 2);
		ux_exterior_canvas_setting_flex.end();
		ux_exterior_canvas_setting_flex.set_type(FlexType::Column);
		ux_exterior_canvas_setting_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_canvas_setting_flex);

		// set up all controls within ux_exterior_canvas_settings_flex
		self.initialize_canvas_settings(&mut ux_exterior_canvas_setting_flex, msg_sender);

		// exterior vertical flex for build gen algo controls
		let mut ux_exterior_build_gen_controls_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x(), ux_exterior_canvas_setting_flex.y() + ux_exterior_canvas_setting_flex.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() / 2);
		ux_exterior_build_gen_controls_flex.end();
		ux_exterior_build_gen_controls_flex.set_type(FlexType::Column);
		ux_exterior_build_gen_controls_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_build_gen_controls_flex);

		// set up all controls within ux_exterior_build_gen_controls_flex
		self.initialize_build_gen_controls(&mut ux_exterior_build_gen_controls_flex, msg_sender);

		// exterior vertical flex for canvas drawing stuff
		let mut ux_exterior_canvas_drawing_setting_flex = Flex::default()
			.with_pos(self.ux_canvas_scroll.x(), self.ux_canvas_scroll.y() + self.ux_canvas_scroll.height())
			.with_size(self.ux_whole_tab_group.width() / 3, self.ux_whole_tab_group.height() - ux_canvas_group.height());
		ux_exterior_canvas_drawing_setting_flex.end();
		ux_exterior_canvas_drawing_setting_flex.set_type(FlexType::Column);
		ux_exterior_canvas_drawing_setting_flex.set_frame(FrameType::BorderBox);
		self.ux_whole_tab_group.add(&ux_exterior_canvas_drawing_setting_flex);

		// set up all controls within ux_exterior_canvas-drawing_setting_flex
		self.initialize_drawing_settings(&mut ux_exterior_canvas_drawing_setting_flex);

		// exterior vertical flex for level connections stuff
		let mut ux_exterior_level_connections_flex = Flex::default()
			.with_pos(ux_exterior_canvas_setting_flex.x() + ux_exterior_canvas_setting_flex.width(), ux_exterior_canvas_setting_flex.y())
			.with_size(self.ux_whole_tab_group.width() - (self.ux_canvas_scroll.width() + ux_exterior_canvas_setting_flex.width()), self.ux_whole_tab_group.height());
		ux_exterior_level_connections_flex.end();
		ux_exterior_level_connections_flex.set_type(FlexType::Column);
		ux_exterior_level_connections_flex.set_frame(FrameType::FlatBox);
		self.ux_whole_tab_group.add(&ux_exterior_level_connections_flex);

		// set up all controls within ux_exterior_level_connections_flex
		self.initialize_level_connection_settings(&mut ux_exterior_level_connections_flex, &msg_sender);

		// image display part of canvas
		self.update_image_size_and_drawing();
	}//end initialize()

	fn initialize_canvas_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		ux_exterior_flex.begin();
		
		// label for this section
		let _ux_canvas_setting_section_label = Frame::default().with_label("Canvas Settings");

		// interior level number horizontal flex 1
		let mut ux_interior_level_number_hor_flex_1 = Flex::default()
			.with_pos(ux_exterior_flex.x(), ux_exterior_flex.y())
			.with_size(ux_exterior_flex.width(), 50);
		ux_interior_level_number_hor_flex_1.end();
		ux_interior_level_number_hor_flex_1.set_type(FlexType::Row);
		ux_interior_level_number_hor_flex_1.set_frame(FrameType::FlatBox);

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
		// ux_exterior_flex.add(&ux_interior_canvas_size_hor_flex_1);

		let mut ux_interior_canvas_size_hor_flex_2 = Flex::default()
			.with_pos(ux_interior_canvas_size_hor_flex_1.x(), ux_interior_canvas_size_hor_flex_1.y() + ux_interior_canvas_size_hor_flex_1.height())
			.with_size(ux_interior_canvas_size_hor_flex_1.width(), 50);
		ux_interior_canvas_size_hor_flex_2.end();
		ux_interior_canvas_size_hor_flex_2.set_type(FlexType::Row);
		ux_interior_canvas_size_hor_flex_2.set_frame(FrameType::FlatBox);
		// ux_exterior_flex.add(&ux_interior_canvas_size_hor_flex_2);

		ux_exterior_flex.end();

		// level number stuff
		let ux_level_label_frame = Frame::default()
			.with_pos(self.ux_canvas_scroll.x() + self.ux_canvas_scroll.width(), self.ux_canvas_scroll.y())
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
			.with_pos(self.ux_canvas_scroll.x() + self.ux_canvas_scroll.width(), ux_level_down_btn.y() + ux_level_down_btn.height() + get_default_tab_padding())
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
		ux_update_canvas_button.emit(msg_sender.clone(), "RoomGen:Canvas:Update".to_string());

		// update cave canvas frame based on default values in Counters
		let new_width = self.ux_squares_width_counter.value() * self.ux_squares_pixel_diameter_counter.value();
		let new_height = self.ux_squares_height_counter.value() * self.ux_squares_pixel_diameter_counter.value();
		self.ux_canvas_frame.set_size(new_width as i32, new_height as i32);
	}//end initialize_canvas_settings()

	fn initialize_drawing_settings(&mut self, ux_exterior_flex: &mut Flex) {
		// label for this section
		let ux_drawing_setting_section_label = Frame::default().with_label("Drawing Settings").with_align(Align::Center);
		ux_exterior_flex.add(&ux_drawing_setting_section_label);
		ux_exterior_flex.fixed(&ux_drawing_setting_section_label, 25);
		
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
		let mut ux_draw_doors_btn = Button::default()
			.with_label("Doors");
		ux_draw_doors_btn.set_color(Color::Blue);
		ux_draw_doors_btn.set_label_color(Color::White);
		ux_draw_doors_btn.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_draw_doors_btn);

		let mut ux_draw_room_start_btn = Button::default()
			.with_label("Room Start");
		ux_draw_room_start_btn.set_color(Color::Red);
		ux_draw_room_start_btn.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_draw_room_start_btn);

		let mut ux_draw_empty_btn = Button::default()
			.with_label("Erase");
		ux_draw_empty_btn.set_color(Color::White);
		ux_draw_empty_btn.set_frame(FrameType::FlatBox);
		ux_interior_flex_1.add(&ux_draw_empty_btn);

		// set up buttons to choose between different drawing modes
		let mut ux_draw_wall_btn = Button::default()
			.with_label("Wall");
		ux_draw_wall_btn.set_color(Color::Black);
		ux_draw_wall_btn.set_label_color(Color::White);
		ux_interior_flex_2.add(&ux_draw_wall_btn);
		

		let mut ux_draw_floor_btn = Button::default()
			.with_label("Floor");
		ux_draw_floor_btn.set_color(Color::Light1);
		ux_interior_flex_2.add(&ux_draw_floor_btn);

		let mut ux_draw_stairs_btn = Button::default()
			.with_label("Stairs");
		ux_draw_stairs_btn.set_color(Color::Green);
		ux_interior_flex_2.add(&ux_draw_stairs_btn);

		// draw state label frame
		let ux_draw_activation_frame = Frame::default()
			.with_label("No Mode Selected");
		ux_exterior_flex.add(&ux_draw_activation_frame);
		ux_exterior_flex.fixed(&ux_draw_activation_frame, 20);

		// set up controls for choosing brush size
		let mut ux_interior_flex_3 = Flex::default()
			.with_type(FlexType::Row);
		ux_interior_flex_3.end();
		ux_exterior_flex.add(&ux_interior_flex_3);

		let ux_brush_size_label = Frame::default()
			.with_pos(ux_interior_flex_2.x(), ux_interior_flex_2.y() + ux_interior_flex_2.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height() / 4)
			.with_label("Brush Width")
			.with_align(Align::Center);
		ux_interior_flex_3.add(&ux_brush_size_label);
		ux_interior_flex_3.fixed(&ux_brush_size_label, 120);

		let mut ux_brush_size_counter = Counter::default()
			.with_pos(ux_brush_size_label.x(), ux_brush_size_label.y() + ux_brush_size_label.height())
			.with_size(ux_exterior_flex.width(), ux_exterior_flex.height()  / 4);
		ux_brush_size_counter.set_value(2.0);
		ux_brush_size_counter.set_minimum(1.0);
		ux_brush_size_counter.set_maximum(20.0);
		ux_brush_size_counter.set_precision(0);
		ux_brush_size_counter.set_step(1.0, 2);
		ux_brush_size_counter.set_type(CounterType::Simple);
		ux_interior_flex_3.add(&ux_brush_size_counter);

		// set handler for the brush size counter, in order to update self.ux_cave_canvas_brush_size
		ux_brush_size_counter.handle({
			let brush_size_ref = self.ux_brush_size.clone();
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
		self.ux_draw_state = Rc::from(RefCell::from(DrawState::Disabled));
		self.ux_draw_frame_ref = Rc::from(RefCell::from(ux_draw_activation_frame));

		// set handlers for all the buttons
		let draw_frame_ref = &self.ux_draw_frame_ref;
		let draw_state_ref = &self.ux_draw_state;

		ux_draw_doors_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::Door;
				draw_frame_ref.set_label("Draw Doors");
			}//end closure
		});

		ux_draw_empty_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::Empty;
				draw_frame_ref.set_label("Draw Empty Space");
			}//end closure
		});

		ux_draw_floor_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::Floor;
				draw_frame_ref.set_label("Draw Floor");
			}//end closure
		});

		ux_draw_room_start_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::RoomStart;
				draw_frame_ref.set_label("Draw Room Starts");
			}//end closure
		});

		ux_draw_stairs_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::Stair;
				draw_frame_ref.set_label("Draw Stairs");
			}//end closure
		});

		ux_draw_wall_btn.set_callback({
			let draw_frame_ref = draw_frame_ref.clone();
			let draw_state_ref = draw_state_ref.clone();
			move |_| {
				let mut draw_state_ref = draw_state_ref.as_ref().borrow_mut();
				let mut draw_frame_ref = draw_frame_ref.as_ref().borrow_mut();
				*draw_state_ref = DrawState::Wall;
				draw_frame_ref.set_label("Draw Walls");
			}//end closure
		});

	}

	fn initialize_build_gen_controls(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		let ux_settings_label = Frame::default().with_label("Algorithm Controls");
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

		// add room starts controls
		let mut ux_room_start_help_btn = Button::default()
			.with_label("Help");
		ux_interior_flex_1.add(&ux_room_start_help_btn);
		ux_interior_flex_1.fixed(&ux_room_start_help_btn, 70);
		ux_room_start_help_btn.handle({
			move |_, ev| {
				match ev {
					Event::Released => {
						dialog::message_default("Help dialog test for room starts.");
						true
					},
					_ => false,
				}//end matching events
			}//end closure
		});

		let mut ux_room_start_place_btn = Button::default()
			.with_label("Place Room Starts");
		ux_room_start_place_btn.emit(msg_sender.clone(), String::from("RoomGen:RoomStarts"));
		ux_interior_flex_1.add(&ux_room_start_place_btn);

		// add constrained room growth controls
		let mut ux_room_growth_help_btn = Button::default()
			.with_label("Help");
		ux_interior_flex_2.add(&ux_room_growth_help_btn);
		ux_interior_flex_2.fixed(&ux_room_growth_help_btn, 70);
		ux_room_growth_help_btn.handle({
			move |_, ev| {
				match ev {
					Event::Released => {
						dialog::message_default("Help dialog test for constrained growth of rooms.");
						true
					},
					_ => false
				}//end matching events
			}//end closure
		});

		let mut ux_room_growth_start_btn = Button::default()
			.with_label("Constrained Growth");
		ux_room_growth_start_btn.emit(msg_sender.clone(), String::from("RoomGen:ConstrainedGrowth"));
		ux_interior_flex_2.add(&ux_room_growth_start_btn);

		// add connectivity controls
		let mut ux_connectivity_btn = Button::default()
			.with_label("Connectivity");
		ux_connectivity_btn.emit(msg_sender.clone(), String::from("RoomGen:Connectivity"));
		ux_interior_flex_3.add(&ux_connectivity_btn);

		let mut ux_connectivity_help_btn = Button::default()
			.with_label("Help");
		ux_interior_flex_3.add(&ux_connectivity_help_btn);
		ux_interior_flex_3.fixed(&ux_connectivity_help_btn, 70);
		ux_connectivity_help_btn.handle({
			move |_, ev| {
				match ev {
					Event::Released => {
						dialog::message_default("Help dialog test for connectivity settings.");
						true
					},
					_ => false,
				}//end matching events
			}//end closure
		});

		let mut ux_connectivity_limit_counter = Counter::default()
			.with_type(CounterType::Simple);
		ux_connectivity_limit_counter.set_value(5.0);
		ux_connectivity_limit_counter.set_bounds(1.0, 30.0);
		ux_connectivity_limit_counter.set_precision(0);
		ux_connectivity_limit_counter.set_step(1.0, 5);
		ux_interior_flex_3.add(&ux_connectivity_limit_counter);
	}	

	fn initialize_level_connection_settings(&mut self, ux_exterior_flex: &mut Flex, msg_sender: &Sender<String>) {
		// et up label for this section
		let ux_level_connection_settings_section_label = Frame::default().with_label("Level Connection Settings");
		ux_exterior_flex.add(&ux_level_connection_settings_section_label);

		let mut ux_level_connection_list: ListBox<SquareStairDisplay> = ListBox::new(
			ux_level_connection_settings_section_label.x(),
			ux_level_connection_settings_section_label.y() + ux_level_connection_settings_section_label.h(),
			ux_exterior_flex.width() - 5,
			ux_exterior_flex.height() - 200,
			30
		);
		ux_level_connection_list.set_label_size(15);
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

		ux_level_connection_remove_btn.emit(msg_sender.clone(), String::from("RoomGen:Stairs:Remove"));

		ux_exterior_flex.fixed(&ux_level_connection_settings_section_label, 50);
		ux_exterior_flex.fixed(&ux_level_connection_add_btn, 50);
		ux_exterior_flex.fixed(&ux_level_connection_edit_btn, 50);
		ux_exterior_flex.fixed(&ux_level_connection_remove_btn, 50);
	}

	fn update_image_size_and_drawing(&mut self) {
		let canvas_surface = ImageSurface::new(self.ux_canvas_frame.width(), self.ux_canvas_frame.height(), false);

		ImageSurface::push_current(&canvas_surface);
		draw_rect_fill(0,0,self.ux_canvas_frame.width(), self.ux_canvas_frame.height(), Color::White);
		ImageSurface::pop_current();

		self.ux_canvas_image = Rc::from(RefCell::from(canvas_surface));

		let pixel_scale = self.ux_squares_pixel_diameter_counter.value() as i32;
		let pixel_scale_ref = Rc::from(RefCell::from(pixel_scale));
		let sub_pixel_scale = Rc::from(RefCell::from(self.ux_sub_pixel_scale));
		let brush_size_ref = &self.ux_brush_size;
		let draw_state = &self.ux_draw_state;
		let surface_ref = &self.ux_canvas_image;
		let stairs_list_ref = &self.ux_stairs_list;
		let last_square_grid_ref = &self.ux_last_square_grid;

		self.ux_canvas_frame.draw({
			let surface = surface_ref.clone();
			move |f| {
				let surface = surface.borrow();
				let mut img = surface.image().unwrap();
				img.draw(f.x(), f.y(), f.w(), f.h());
			}
		});

		self.ux_canvas_frame.handle({
			let mut x = 0;
			let mut y = 0;
			let surface = surface_ref.clone();
			let pixel_scale = pixel_scale_ref.clone();
			let sub_pixel_scale = sub_pixel_scale.clone();
			let brush_size = brush_size_ref.clone();
			let draw_state = draw_state.clone();
			let last_square_grid = last_square_grid_ref.clone();
			let stairs_list = stairs_list_ref.clone();
			move |f, ev| {
				let surface = surface.as_ref().borrow();
				let pixel_scale = {pixel_scale.as_ref().borrow().clone()};
				let sub_pixel_scale = {sub_pixel_scale.as_ref().borrow().clone()};
				let brush_size = {brush_size.as_ref().borrow().clone()};
				let draw_state = {draw_state.as_ref().borrow().clone()};
				let draw_color = match draw_state {
					DrawState::Wall => Color::Black,
					DrawState::Floor => Color::Light1,
					DrawState::Stair => Color::Green,
					DrawState::Empty => Color::White,
					DrawState::Door => Color::Blue,
					DrawState::RoomStart => Color::Red,
					DrawState::Disabled => Color::White,
				};
				let draw_size = match draw_state {
					DrawState::Stair => pixel_scale,
					DrawState::Disabled => 0,
					_ => pixel_scale * brush_size,
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
					}//end drag event
					Event::Released => {
						let pixel_scale = pixel_scale as usize;
						let mut last_square_grid = last_square_grid.as_ref().borrow_mut();
						let mut stairs_list = stairs_list.as_ref().borrow_mut();
						if let Some(squares) = ux_squareularize_canvas(&surface, Some(&DrawState::get_color_vec_u8()), &pixel_scale, &sub_pixel_scale) {
							let stair_vec = Self::ux_get_stair_coord_list(&squares);
							Self::ux_update_stairs_list(stair_vec, &mut stairs_list);
							*last_square_grid = Some(squares);
						}//end if we get our squares properly
						
						f.redraw();
						true
					}
					_ => false
				}//end matching events
			}//end handle move
		});

		// make sure gui gets drawn on first init stuff
		self.ux_canvas_scroll.redraw();
		self.ux_canvas_frame.redraw();
	}//end update_image_size_and_drawing()

	fn ux_get_stair_coord_list(squares: &SquareGrid) -> Vec<SquareStairDisplay> {
		let mut stairs_list = Vec::new();
		for row in 0..*squares.rows() {
			for col in 0..*squares.cols() {
				match squares.get(&row, &col) {
					Some(square) => {
						if *square.color() == (0,255,0) {
							stairs_list.push(SquareStairDisplay {square: *square, row_idx: row, col_idx: col});
						}//end if we found a stair
					},
					None => println!("Failed to get an index when counting stairs?"),
				}//end matching whether we got the index
			}//end looping over cols
		}//end looping over rows
		return stairs_list;
	}//end ux_get_stair_coord_list()

	fn ux_update_stairs_list(stairs_list: Vec<SquareStairDisplay>, stairs_list_box: &mut ListBox<SquareStairDisplay>) {
		stairs_list_box.clear_elements();
		stairs_list_box.set_elements(stairs_list);
	}//end ux_update_stairs_list(s)

	pub fn get_stairs_selected(&self) -> Vec<String> {
		let stairs_list_ref = &self.ux_stairs_list;
		let stairs_list_ref_clone = stairs_list_ref.clone();
		let stairs_list_borrow = stairs_list_ref_clone.as_ref().borrow();
		stairs_list_borrow.get_selected_elements().into_iter().map(|val| format!("{}", val)).collect()
	}//end get_stairs_selected

	pub fn remove_stairs_selected(&mut self) {
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
							this_square.set_color((255,255,255));
							squares_to_recolor.push(this_square.clone());
						},
						None => println!("Couldn't access square {:?} while removing stairs from list.", selected_element)
					}//end matching whether we can get this square
				}//end looping over each selected element
				let canvas_ref = &self.ux_canvas_image;
				let canvas_ref_clone = canvas_ref.clone();
				let canvas_borrow = canvas_ref_clone.borrow();
				squareularization_color_square(&canvas_borrow, squares_to_recolor.iter());
				self.ux_canvas_frame.redraw();
				stairs_list_borrow.remove_selected_elements()
			},
			None => println!("We don't have last squares and stairs?"),
		}//end matching whether we had last squarularization
	}

	pub fn update_canvas(&mut self) {
		let diameter_counter = self.ux_squares_pixel_diameter_counter.value();
		let squares_width = self.ux_squares_width_counter.value();
		let squares_height = self.ux_squares_height_counter.value();
		let pixels_width = squares_width * diameter_counter;
		let pixels_height = squares_height * diameter_counter;
		self.ux_canvas_frame.set_size(pixels_width as i32, pixels_height as i32);
		let stair_list_ref = &self.ux_stairs_list;
		let stair_list_ref_clone = stair_list_ref.clone();
		let mut stair_list_borrow = stair_list_ref_clone.as_ref().borrow_mut();
		stair_list_borrow.clear_elements();
		self.update_image_size_and_drawing();
	}//end update_canvas

}//end impl for RoomGenGroup

widget_extends!(RoomGenGroup, Tile, ux_whole_tab_group);