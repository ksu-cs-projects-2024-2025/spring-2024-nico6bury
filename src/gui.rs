
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use fltk::{app::{self, App, Receiver, Sender}, button::Button, dialog, enums::{Align, FrameType, Shortcut}, group::{Group, Tabs}, menu::{self, Choice, SysMenuBar}, prelude::{GroupExt, MenuExt, WidgetExt}, window::Window};
use nice_map_generator::squares::SquareGrid;

use self::{room_gen_group::RoomGenGroup, cave_gen_group::CaveGenGroup, gui_utils::{get_default_menu_height, get_default_tab_padding, get_default_win_height, get_default_win_width}};

mod gui_utils;
mod cave_gen_group;
mod room_gen_group;

pub struct GUI {
	/// application struct everything runs in
	app: App,
	/// main window of application
	ux_main_window: Window,
	/// sends messages for events
	msg_sender: Sender<String>,
	/// receives messages for events
	pub msg_receiver: Receiver<String>,
	ux_top_menu: SysMenuBar,

	/// contains all the tabs
	ux_tab_container: Tabs,
	/// tab contains general settings for generation and what to use
	ux_gen_setting_tab: Group,
	/// tab contains settings for cave generation, allows specification of input map
	ux_cave_gen_tab: CaveGenGroup,
	/// tab contains settings for room-based structure generation, allows specification of room map
	ux_room_gen_tab: RoomGenGroup,
	/// tab contains settings for where structures should be in relation to each other
	ux_multi_gen_tab: Group,
	/// tab contains output image of map, displayed using some sort of canvas drawing in all likelihood
	ux_output_img_tab: Group,
}//end struct GUI

impl Default for GUI {
	fn default() -> GUI {
		let (s, r) = app::channel();
		let mut gui = GUI {
			app: App::default(),
			ux_main_window: Window::default(),
			msg_sender: s,
			msg_receiver: r,
			ux_top_menu: SysMenuBar::default(),
			ux_tab_container: Tabs::default(),
			ux_gen_setting_tab: Group::default(),
			ux_cave_gen_tab: CaveGenGroup::default(),
			ux_room_gen_tab: RoomGenGroup::default(),
			ux_multi_gen_tab: Group::default(),
			ux_output_img_tab: Group::default(),
		};//end struct construction
		gui.ux_main_window.end();
		gui.ux_top_menu.end();
		gui.ux_tab_container.end();
		gui.ux_gen_setting_tab.end();
		gui.ux_cave_gen_tab.end();
		gui.ux_room_gen_tab.end();
		gui.ux_multi_gen_tab.end();
		gui.ux_output_img_tab.end();
		gui.initialize();
		return gui;
	}//end default()
}//end impl Default for GUI

impl GUI {
	/// # initialize(&mut self)
	/// 
	/// This method should be called first to setup the GUI.  
	/// It will also call other initialize functions.
	fn initialize(&mut self) {
		// set up main window properties
		self.ux_main_window.set_size(get_default_win_width(), get_default_win_height());
		self.ux_main_window.make_resizable(true);
		self.ux_main_window.set_label("spring-2024-cis598");
		self.ux_main_window.end();

		println!("Main Initialization finished.");
		// run other initialization functions
		self.initialize_top_menu();
		println!("Top Menu Initialization finished.");
		self.initialize_tabs();
		println!("Tabs Initialization finished.");
	}//end initialize(&mut self)

	/// # initialize_top_menu(&mut self)
	/// 
	/// This function sets up the state and structure
	/// of buttons in the top menu bar.  
	/// Functions as helper function to initialize().
	fn initialize_top_menu(&mut self){
		// settings for top menu
		self.ux_top_menu.set_size(get_default_win_width(), get_default_menu_height());
		self.ux_top_menu.set_frame(FrameType::FlatBox);
		self.ux_top_menu.end();
		self.ux_main_window.add(&self.ux_top_menu);
		
		// set up all the emitters
		self.ux_top_menu.add_emit(
			"&File/Choice1...\t",
			Shortcut::Ctrl | 'n',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Choice1".to_string(),
		);
		self.ux_top_menu.add_emit(
			"&File/Choice2...\t",
			Shortcut::Ctrl | 'o',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Choice2".to_string(),
		);
		self.ux_top_menu.add_emit(
			"Regen",
			Shortcut::Ctrl | 'r',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::GenerateDistricts".to_string(),
		);
	}//end initialize_top_menu(&mut self)

	/// # initialize_tabs(&mut self)
	/// 
	/// This function sets up the state of tab containers 
	/// in the gui.  
	/// Functions as helper function to initialize().
	fn initialize_tabs(&mut self){
		// tab container settings
		self.ux_tab_container = Tabs::default()
			.with_size(get_default_win_width(), get_default_win_height() - get_default_menu_height())
			.with_pos(0, get_default_menu_height());
		self.ux_tab_container.auto_layout();
		self.ux_tab_container.end();
		self.ux_main_window.add(&self.ux_tab_container);

		// first tab settings
		self.ux_gen_setting_tab = Group::default()
			.with_pos(0, self.ux_tab_container.y() + get_default_tab_padding())
			.with_size(self.ux_tab_container.width(), self.ux_tab_container.height())
			.with_label("General Settings");
		self.ux_gen_setting_tab.end();
		self.ux_tab_container.add(&self.ux_gen_setting_tab);

		let ux_test_button = Button::default()
			.with_size(50, 50)
			.with_pos(100, 100)
			.with_label("test button in tab 1");
		self.ux_gen_setting_tab.add(&ux_test_button);

		// second tab settings
		self.ux_cave_gen_tab = CaveGenGroup::default()
			.with_pos(0, self.ux_tab_container.y() + get_default_tab_padding())
			.with_size(self.ux_tab_container.width(), self.ux_tab_container.height())
			.with_label("Cave Generation");
		self.ux_cave_gen_tab.end();
		self.ux_tab_container.add(&*self.ux_cave_gen_tab);
		self.ux_cave_gen_tab.initialize(&self.msg_sender);

		// third tab settings
		self.ux_room_gen_tab = RoomGenGroup::default()
			.with_pos(0, self.ux_tab_container.y() + get_default_tab_padding())
			.with_size(self.ux_tab_container.width(), self.ux_tab_container.height())
			.with_label("Structure Generation");
		self.ux_room_gen_tab.end();
		self.ux_tab_container.add(&*self.ux_room_gen_tab);
		self.ux_room_gen_tab.initialize(&self.msg_sender);

		// fourth tab settings
		self.ux_multi_gen_tab = Group::default()
			.with_pos(0, self.ux_tab_container.y() + get_default_tab_padding())
			.with_size(self.ux_tab_container.width(), self.ux_tab_container.height())
			.with_label("Relative Placement");
		self.ux_multi_gen_tab.end();
		self.ux_tab_container.add(&self.ux_multi_gen_tab);

		// fifth tab settings
		self.ux_output_img_tab = Group::default()
			.with_pos(0, self.ux_tab_container.y() + get_default_tab_padding())
			.with_size(self.ux_tab_container.width(), self.ux_tab_container.height())
			.with_label("Output");
		self.ux_output_img_tab.end();
		self.initialize_output_tab();
		self.ux_tab_container.add(&self.ux_output_img_tab);

	}//end initialize_tabs(&mut self)

	pub fn initialize_output_tab(&mut self) {
		let mut ux_format_choice = Choice::default()
			.with_label("Select Image Format")
			.with_align(Align::Left)
			.with_pos(170, 60)
			.with_size(100,25);
		self.ux_output_img_tab.add(&ux_format_choice);
		ux_format_choice.add_choice("PNG");
		ux_format_choice.add_choice("JPEG");
		ux_format_choice.add_choice("BMP");
		ux_format_choice.add_choice("WEBP");

		let mut ux_export_format = Button::default()
			.with_label("Export")
			.with_align(Align::Center)
			.with_pos(ux_format_choice.x(), ux_format_choice.y() + ux_format_choice.h() + 10)
			.with_size(100, 25);
		self.ux_output_img_tab.add(&ux_export_format);

		let ux_format_choice_ref = Rc::from(RefCell::from(ux_format_choice));
		let ux_format_choice_ref_ref = &ux_format_choice_ref;
		let msg_sender_ref = Rc::from(RefCell::from(self.msg_sender.clone()));
		let msg_sender_ref_ref = &msg_sender_ref;
		ux_export_format.set_callback({
			let ux_format_choice_ref = ux_format_choice_ref_ref.clone();
			let msg_sender_ref = msg_sender_ref_ref.clone();
			move |_| {
				let format_choice = {ux_format_choice_ref.as_ref().borrow().choice()};
				match format_choice {
					Some(choice) => {
						let msg_sender = {msg_sender_ref.as_ref().borrow()};
						match choice.as_str() {
							"PNG" | "JPEG" | "BMP" | "WEBP" => msg_sender.send(format!("Export:{}", choice)),
							_ => println!("Unrecognized output format {}", choice),
						}//end matching choice
					},
					None => (),
				}//end matching chosen choice
			}//end closure
		});

	}//end initialize_output_tab(self)

	/// # switch_tab(&mut self, tab_idx)
	/// 
	/// Switches the currently visisble tab to the specified one.  
	/// If the desired index does not correspond to a tab, then
	/// a message will be displayed in the console.
	pub fn switch_tab(&mut self, tab_idx:u8){
		let cur_vis_val = self.ux_tab_container.value();
		if cur_vis_val.is_none() {
			println!("No tab currently selected. Something is wrong.");
			return;
		}//end if no tab is selected
		let cur_vis = cur_vis_val.unwrap();

		match tab_idx {
			0 => {
				if cur_vis.is_same(&self.ux_gen_setting_tab) {return;}
				self.ux_tab_container.set_value(&self.ux_gen_setting_tab).expect("Should be able to set vis setting tab.");
			},
			1 => {
				if cur_vis.is_same(&*self.ux_cave_gen_tab) {return;}
				self.ux_tab_container.set_value(&*self.ux_cave_gen_tab).expect("Should be able to set vis cave tab.");
			},
			2 => {
				if cur_vis.is_same(&*self.ux_room_gen_tab) {return;}
				self.ux_tab_container.set_value(&*self.ux_room_gen_tab).expect("Should be able to set vis room tab.");
			},
			3 => {
				if cur_vis.is_same(&self.ux_multi_gen_tab) {return;}
				self.ux_tab_container.set_value(&self.ux_multi_gen_tab).expect("Should be able to set vis multi tab.");
			},
			4 => {
				if cur_vis.is_same(&self.ux_output_img_tab) {return;}
				self.ux_tab_container.set_value(&self.ux_output_img_tab).expect("Should be able to set vis output tab.");
			},
			_ => {
				println!("Unsupported tab index {}", tab_idx);
			},
		}//end matching desired tab index

		self.redraw_tabs();
	}//end switch_tab(&mut self, tab_idx)

	/// # show(&mut self)
	/// 
	/// This method causes the GUI to become visible.  
	/// It is recommended to call this after the GUI 
	/// has been constructed.
	pub fn show(&mut self){
		self.ux_main_window.show();
		// resize window slightly to force it to recalculate 
		self.force_resize_calc();
		// self.ux_main_window.maximize();
	}//end show(&mut self)

	/// # force_resize_calc(&mut self)
	/// 
	/// Does a couple resize calls on main window that cause widget sizes to be recalculated.  
	/// Size and location of main window should remain the same before and after function call.
	pub fn force_resize_calc(&mut self) {
		self.ux_main_window.resize(self.ux_main_window.x(), self.ux_main_window.y(), self.ux_main_window.width(), self.ux_main_window.height() + 1);
		self.ux_main_window.resize(self.ux_main_window.x(), self.ux_main_window.y(), self.ux_main_window.width(), self.ux_main_window.height());
	}//end force_resize_calc(&mut self)

	/// # wait(&self)
	/// 
	/// This method wraps app.wait().  
	/// To run the main app loop, use while(gui.wait()){}.
	pub fn wait(&self) -> bool {
		self.app.wait()
	}//end wait(&self)

	pub fn redraw_tabs(&mut self) {
		self.ux_tab_container.redraw();
		self.ux_gen_setting_tab.redraw();
		self.ux_cave_gen_tab.redraw();
		self.ux_room_gen_tab.redraw();
		self.ux_multi_gen_tab.redraw();
		self.ux_output_img_tab.redraw();
	}//end redraw_tabs(self)

	pub fn update_cave_canvas(&mut self) {
		self.ux_cave_gen_tab.update_canvas();
	}//end update_cave_canvas

	pub fn update_room_canvas(&mut self) {
		self.ux_room_gen_tab.update_canvas();
	}//end update-build_canvas

	/// Gets representation of grid of color from cave canvas.
	/// First two elements in Vector are width and height of each square.  
	/// Format of vec is x,y coord of upper left of each square, plus color for that square in RGB.  
	/// It should be noted that squares might overlap.
	pub fn get_cave_canvas_squareularization(&mut self) -> Option<SquareGrid> {
		self.ux_cave_gen_tab.get_last_squareularization()
	}//end get_cave_canvas_squareularization()

	pub fn get_room_canvas_squareularization(&mut self) -> Option<SquareGrid> {
		self.ux_room_gen_tab.get_last_squareularization()
	}//end get_room_canvas_squareularization()

	/// Sets the canvas stuff in cave canvas based on [square_info].  
	/// Format for [square_info] should be the same as format for [get_cave_canvas_squareularization()].  
	/// This function calls CaveGenGroup::[squareularization_color_squares()], which might panic under a 
	/// variety of circumstances, mostly from type conversion (usize to i32) or from the canvas size shrinking 
	/// in between getting and setting squareularization. For more information on how the function might panic, 
	/// refer back to [CaveGenGroup]::[squareularization_color_squares()].
	pub fn set_cave_canvas_squareularization(&mut self, squares: &SquareGrid) {
		self.ux_cave_gen_tab.set_squareularization(squares);
	}//end set_cave_canvas_squareularization(self, square_info)

	pub fn set_room_canvas_squareularization(&mut self, squares: &SquareGrid) {
		self.ux_room_gen_tab.set_squareularization(squares);
	}//end set_room_canvas_squareularization

	/// Returns neighborhood size, neighborhood threshold, and generations to run from cave canvas group
	pub fn get_cave_canvas_ca_settings(&self) -> (usize,usize,usize) {
		self.ux_cave_gen_tab.get_cave_canvas_ca_settings()
	}//end get_cave_canvas_ca_settings(self)

	pub fn get_cave_gen_stairs_selected(&self) -> Vec<String> {
		self.ux_cave_gen_tab.get_cave_gen_stairs_selected()
	}//end get_cave_gen_stairs_selected(self)

	pub fn get_room_gen_stairs_selected(&self) -> Vec<String> {
		self.ux_room_gen_tab.get_stairs_selected()
	}

	pub fn remove_cave_gen_stairs_selected(&mut self) {
		self.ux_cave_gen_tab.remove_cave_gen_stairs_selected()
	}//end remove_cave_gen_stairs_selected(self)

	pub fn remove_room_gen_stairs_selected(&mut self) {
		self.ux_room_gen_tab.remove_stairs_selected()
	}

	/// Displays message to user, asking them yes or no.  
	/// If user answers yes, then returns true.  
	/// If user answers no or closes dialog early, returns false.
	pub fn yes_no(text: &str) -> bool {
		match dialog::choice2(0, 0, text, "Yes", "No", "") {
			Some(choice_index) => choice_index == 0,
			None => false,
		}//end matching dialog result
	}//end yes_no(text)

	/// Displays a message box to user.
	pub fn message(text: &str) {
		dialog::message(0, 0, text);
	}//end message(text)

	/// Displays an alert box/error message to the user.
	pub fn alert(text: &str) {
		dialog::beep(dialog::BeepType::Error);
		dialog::alert_default(text);
	}//end alert(text)

	/// Displays dialog for getting a user to give a path where they'll save an image.
	pub fn save_img_dialog(file_extension: &str) -> Option<PathBuf> {
		let mut dialog = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseSaveFile);
		dialog.set_filter(&format!("*.{}", file_extension));
		dialog.set_option(dialog::FileDialogOptions::SaveAsConfirm.union(dialog::FileDialogOptions::Preview).union(dialog::FileDialogOptions::NewFolder));
		dialog.set_title(&format!("Export {} Image", file_extension.to_uppercase()));
		dialog.show();
		
		if "" == dialog.error_message().unwrap_or("".to_string()).replace("No error", "") && "\"\"" != format!("{:?}",dialog.filename()) {
			Some(dialog.filename())
		} else { if dialog.error_message().unwrap_or_default() != "No error" {println!("{}", dialog.error_message().unwrap_or_default());} None}
	}//end save_img_dialog

	/// Displays dialog asking user to choose between two options or cancel.
	/// If the user chooses one of the options, then the text of that option
	/// will be returned.
	pub fn choice_dialog<'a>(prompt: &str, choice1: &'a str, choice2: &'a str) -> Option<&'a str> {
		match dialog::choice2_default(prompt, "cancel", &choice1, &choice2) {
			Some(choice_index) => {
				match choice_index {
					1 => Some(choice1),
					2 => Some(choice2),
					_ => None,
				}//end matching choice index
			}, None => None,
		}//end matching dialog result
	}//end choice_dialog()
}//end impl for GUI