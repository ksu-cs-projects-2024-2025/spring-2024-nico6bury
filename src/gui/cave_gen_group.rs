use fltk::{app::Sender, button::Button, enums::{Align, FrameType}, frame::Frame, group::{Group, Scroll}, prelude::{DisplayExt, GroupExt, ValuatorExt, WidgetExt}, text::{TextBuffer, TextDisplay, TextEditor}, valuator::{Counter, CounterType}, widget_extends};

use crate::gui::gui_utils::{get_default_menu_height, get_default_tab_padding};

pub struct CaveGenGroup {
	whole_tab_group: Group,
	cave_canvas_scroll: Scroll,
	pub cave_canvas_frame: Frame,
	pub level_cur_buf: TextBuffer,
	pub level_tot_buf: TextBuffer,
	pub squares_width_counter: Counter,
	pub squares_height_counter: Counter,
	pub squares_pixel_diameter_counter: Counter,
}//end struct CaveGenGroup

impl Default for CaveGenGroup {
	fn default() -> Self {
		let cave_gen_group = CaveGenGroup {
			whole_tab_group: Default::default(),
			cave_canvas_scroll: Default::default(),
			cave_canvas_frame: Default::default(),
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
	pub fn initialize(&mut self, msg_sender: &Sender<String>) {
		// scrollable container for size-locked canvas
		self.cave_canvas_scroll = Scroll::default()
			.with_pos(get_default_tab_padding(), self.whole_tab_group.y() + get_default_menu_height())
			.with_size(self.whole_tab_group.width() / 2, self.whole_tab_group.height() / 2);
		self.cave_canvas_scroll.end();
		self.cave_canvas_scroll.set_frame(FrameType::EmbossedFrame);
		self.whole_tab_group.add(&self.cave_canvas_scroll);

		// size-locked canvas for drawing
		self.cave_canvas_frame = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + get_default_tab_padding(), self.cave_canvas_scroll.y() + get_default_tab_padding())
			.with_size(100,100)
			.with_label("Canvas thingy");
		self.cave_canvas_frame.set_frame(FrameType::BorderBox);
		self.cave_canvas_scroll.add(&self.cave_canvas_frame);

		// level number stuff
		let level_label_frame = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + self.cave_canvas_scroll.width() + get_default_tab_padding(), self.cave_canvas_scroll.y())
			.with_label("Level")
			.with_size(30, 20)
			.with_align(Align::Center);
		self.whole_tab_group.add(&level_label_frame);

		self.level_cur_buf = TextBuffer::default();
		self.level_cur_buf.set_text("1");
		let mut level_cur_label_txt = TextDisplay::default()
			.with_pos(level_label_frame.x() + level_label_frame.width() + get_default_tab_padding(), level_label_frame.y())
			.with_size(20,20);
		level_cur_label_txt.set_buffer(self.level_cur_buf.clone());
		self.whole_tab_group.add(&level_cur_label_txt);
		
		let level_out_of_label_frame = Frame::default()
			.with_pos(level_cur_label_txt.x() + level_cur_label_txt.width(), level_cur_label_txt.y())
			.with_size(25,20)
			.with_label(" out of ");
		self.whole_tab_group.add(&level_out_of_label_frame);
		
		self.level_tot_buf = TextBuffer::default();
		self.level_tot_buf.set_text("3");
		let mut level_total_label_txt = TextEditor::default()
			.with_pos(level_out_of_label_frame.x() + level_out_of_label_frame.width(), level_out_of_label_frame.y())
			.with_size(20,20);
		level_total_label_txt.set_buffer(self.level_tot_buf.clone());
		self.whole_tab_group.add(&level_total_label_txt);
		
		let level_down_btn = Button::default()
			.with_pos(level_label_frame.x(), level_label_frame.y() + level_label_frame.height() + get_default_tab_padding())
			.with_size(25, 25)
			.with_label("@line");
		self.whole_tab_group.add(&level_down_btn);
		let level_up_btn = Button::default()
			.with_pos(level_down_btn.x() + level_down_btn.width() + get_default_tab_padding(), level_down_btn.y())
			.with_size(25,25)
			.with_label("@+");
		self.whole_tab_group.add(&level_up_btn);

		// stuff for setting size/resolution of squares
		let square_size_label = Frame::default()
			.with_pos(self.cave_canvas_scroll.x() + self.cave_canvas_scroll.width() + get_default_tab_padding(), level_down_btn.y() + level_down_btn.height() + get_default_tab_padding())
			.with_size(90, 25)
			.with_label("Level Size (in squares)")
			.with_align(Align::Inside);
		self.whole_tab_group.add(&square_size_label);

		self.squares_width_counter = Counter::default()
			.with_pos(square_size_label.x(), square_size_label.y() + square_size_label.height() + get_default_tab_padding())
			.with_size(50, 25)
			.with_label("Width")
			.with_align(Align::Top);
		self.squares_width_counter.set_value(50.0);
		self.squares_width_counter.set_minimum(3.0);
		self.squares_width_counter.set_maximum(1000.0);
		self.squares_width_counter.set_precision(0);
		self.squares_width_counter.set_step(1.0, 10);
		self.whole_tab_group.add(&self.squares_width_counter);

		self.squares_height_counter = Counter::default()
			.with_pos(self.squares_width_counter.x() + self.squares_width_counter.width() + get_default_tab_padding(), self.squares_width_counter.y())
			.with_size(50, 25)
			.with_label("Height")
			.with_align(Align::Top);
		self.squares_height_counter.set_value(50.0);
		self.squares_height_counter.set_minimum(3.0);
		self.squares_height_counter.set_maximum(1000.0);
		self.squares_height_counter.set_precision(0);
		self.squares_height_counter.set_step(1.0, 10);
		self.whole_tab_group.add(&self.squares_height_counter);

		// pixel scale
		self.squares_pixel_diameter_counter = Counter::default()
			.with_pos(self.squares_width_counter.x(), self.squares_width_counter.y() + self.squares_height_counter.height() +  2 * get_default_tab_padding())
			.with_size(50, 25)
			.with_label("Scale (Pixel Diameter per Square)")
			.with_align(Align::TopLeft);
		self.squares_pixel_diameter_counter.set_value(2.0);
		self.squares_pixel_diameter_counter.set_minimum(1.0);
		self.squares_pixel_diameter_counter.set_maximum(30.0);
		self.squares_pixel_diameter_counter.set_precision(0);
		self.squares_pixel_diameter_counter.set_step(1.0, 10);
		self.squares_pixel_diameter_counter.set_type(CounterType::Simple);
		self.whole_tab_group.add(&self.squares_pixel_diameter_counter);

		// button for updating canvas
		let mut update_canvas_button = Button::default()
			.with_pos(self.squares_pixel_diameter_counter.x(), self.squares_pixel_diameter_counter.y() + self.squares_pixel_diameter_counter.height() + get_default_tab_padding())
			.with_size(100, 25)
			.with_label("Update Canvas");
		self.whole_tab_group.add(&update_canvas_button);
		update_canvas_button.emit(msg_sender.clone(), "CaveGen:Canvas:Update".to_string());
	}//end initialize()

	pub fn update_canvas(&mut self) {
		let diameter_counter = self.squares_pixel_diameter_counter.value();
		let squares_width = self.squares_width_counter.value();
		let squares_height = self.squares_height_counter.value();
		let pixels_width = squares_width * diameter_counter;
		let pixels_height = squares_height * diameter_counter;
		self.cave_canvas_frame.set_size(pixels_width as i32, pixels_height as i32);
		self.cave_canvas_frame.redraw();
	}//end update_canvas(self)
}//end impl for CaveGenGroup

widget_extends!(CaveGenGroup, Group, whole_tab_group);