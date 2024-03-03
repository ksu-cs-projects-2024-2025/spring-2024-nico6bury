
use fltk::{app::{self, App, Receiver, Sender}, button::Button, enums::{FrameType, Shortcut}, group::{Group, Tabs}, menu::{self, SysMenuBar}, prelude::{GroupExt, MenuExt, WidgetExt}, window::Window};

use self::{cave_gen_group::CaveGenGroup, gui_utils::{get_default_menu_height, get_default_tab_padding, get_default_win_height, get_default_win_width}};

mod gui_utils;
mod cave_gen_group;

pub struct GUI {
	/// application struct everything runs in
	app: App,
	/// main window of application
	main_window: Window,
	/// sends messages for events
	pub msg_sender: Sender<String>,
	/// receives messages for events
	pub msg_receiver: Receiver<String>,
	top_menu: SysMenuBar,

	/// contains all the tabs
	tab_container: Tabs,
	/// tab contains general settings for generation and what to use
	gen_setting_tab: Group,
	/// tab contains settings for cave generation, allows specification of input map
	cave_gen_tab: CaveGenGroup,
	/// tab contains settings for room-based structure generation, allows specification of room map
	room_gen_tab: Group,
	/// tab contains settings for where structures should be in relation to each other
	multi_gen_tab: Group,
	/// tab contains output image of map, displayed using some sort of canvas drawing in all likelihood
	output_img_tab: Group,
}//end struct GUI

impl Default for GUI {
	fn default() -> GUI {
		let (s, r) = app::channel();
		let mut gui = GUI {
			app: App::default(),
			main_window: Window::default(),
			msg_sender: s,
			msg_receiver: r,
			top_menu: SysMenuBar::default(),
			tab_container: Tabs::default(),
			gen_setting_tab: Group::default(),
			cave_gen_tab: CaveGenGroup::default(),
			room_gen_tab: Group::default(),
			multi_gen_tab: Group::default(),
			output_img_tab: Group::default(),
		};//end struct construction
		gui.main_window.end();
		gui.top_menu.end();
		gui.tab_container.end();
		gui.gen_setting_tab.end();
		gui.cave_gen_tab.end();
		gui.room_gen_tab.end();
		gui.multi_gen_tab.end();
		gui.output_img_tab.end();
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
		self.main_window.set_size(get_default_win_width(), get_default_win_height());
		self.main_window.make_resizable(true);
		self.main_window.set_label("spring-2024-cis598");
		self.main_window.end();

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
		self.top_menu.set_size(get_default_win_width(), get_default_menu_height());
		self.top_menu.set_frame(FrameType::FlatBox);
		self.top_menu.end();
		self.main_window.add(&self.top_menu);
		
		// set up all the emitters
		self.top_menu.add_emit(
			"&File/Choice1...\t",
			Shortcut::Ctrl | 'n',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Choice1".to_string(),
		);
		self.top_menu.add_emit(
			"&File/Choice2...\t",
			Shortcut::Ctrl | 'o',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			"MenuChoice::Choice2".to_string(),
		);
		self.top_menu.add_emit(
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
		self.tab_container = Tabs::default()
			.with_size(get_default_win_width(), get_default_win_height() - get_default_menu_height())
			.with_pos(0, get_default_menu_height());
		self.tab_container.auto_layout();
		self.tab_container.end();
		self.main_window.add(&self.tab_container);

		// first tab settings
		self.gen_setting_tab = Group::default()
			.with_pos(0, self.tab_container.y() + get_default_tab_padding())
			.with_size(self.tab_container.width(), self.tab_container.height())
			.with_label("General Settings");
		self.gen_setting_tab.end();
		self.tab_container.add(&self.gen_setting_tab);

		let test_button = Button::default()
			.with_size(50, 50)
			.with_pos(100, 100)
			.with_label("test button in tab 1");
		self.gen_setting_tab.add(&test_button);

		// second tab settings
		self.cave_gen_tab = CaveGenGroup::default()
			.with_pos(0, self.tab_container.y() + get_default_tab_padding())
			.with_size(self.tab_container.width(), self.tab_container.height())
			.with_label("Cave Generation");
		self.cave_gen_tab.end();
		self.tab_container.add(&*self.cave_gen_tab);
		self.cave_gen_tab.initialize(&self.msg_sender);

		// third tab settings
		self.room_gen_tab = Group::default()
			.with_pos(0, self.tab_container.y() + get_default_tab_padding())
			.with_size(self.tab_container.width(), self.tab_container.height())
			.with_label("Structure Generation");
		self.room_gen_tab.end();
		self.tab_container.add(&self.room_gen_tab);

		// fourth tab settings
		self.multi_gen_tab = Group::default()
			.with_pos(0, self.tab_container.y() + get_default_tab_padding())
			.with_size(self.tab_container.width(), self.tab_container.height())
			.with_label("Relative Placement");
		self.multi_gen_tab.end();
		self.tab_container.add(&self.multi_gen_tab);

		// fifth tab settings
		self.output_img_tab = Group::default()
			.with_pos(0, self.tab_container.y() + get_default_tab_padding())
			.with_size(self.tab_container.width(), self.tab_container.height())
			.with_label("Output");
		self.output_img_tab.end();
		self.tab_container.add(&self.output_img_tab);

	}//end initialize_tabs(&mut self)

	/// # switch_tab(&mut self, tab_idx)
	/// 
	/// Switches the currently visisble tab to the specified one.  
	/// If the desired index does not correspond to a tab, then
	/// a message will be displayed in the console.
	pub fn switch_tab(&mut self, tab_idx:u8){
		let cur_vis_val = self.tab_container.value();
		if cur_vis_val.is_none() {
			println!("No tab currently selected. Something is wrong.");
			return;
		}//end if no tab is selected
		let cur_vis = cur_vis_val.unwrap();

		match tab_idx {
			0 => {
				if cur_vis.is_same(&self.gen_setting_tab) {return;}
				self.tab_container.set_value(&self.gen_setting_tab).expect("Should be able to set vis setting tab.");
			},
			1 => {
				if cur_vis.is_same(&*self.cave_gen_tab) {return;}
				self.tab_container.set_value(&*self.cave_gen_tab).expect("Should be able to set vis cave tab.");
			},
			2 => {
				if cur_vis.is_same(&self.room_gen_tab) {return;}
				self.tab_container.set_value(&self.room_gen_tab).expect("Should be able to set vis room tab.");
			},
			3 => {
				if cur_vis.is_same(&self.multi_gen_tab) {return;}
				self.tab_container.set_value(&self.multi_gen_tab).expect("Should be able to set vis multi tab.");
			},
			4 => {
				if cur_vis.is_same(&self.output_img_tab) {return;}
				self.tab_container.set_value(&self.output_img_tab).expect("Should be able to set vis output tab.");
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
		self.main_window.show();
		self.main_window.maximize();
	}//end show(&mut self)

	/// # wait(&self)
	/// 
	/// This method wraps app.wait().  
	/// To run the main app loop, use while(gui.wait()){}.
	pub fn wait(&self) -> bool {
		self.app.wait()
	}//end wait(&self)

	pub fn redraw_tabs(&mut self) {
		self.tab_container.redraw();
		self.gen_setting_tab.redraw();
		self.cave_gen_tab.redraw();
		self.room_gen_tab.redraw();
		self.multi_gen_tab.redraw();
		self.output_img_tab.redraw();
	}//end redraw_tabs(self)

	pub fn update_cave_canvas(&mut self) {
		self.cave_gen_tab.update_canvas();
	}//end update_cave_canvas
}//end impl for GUI