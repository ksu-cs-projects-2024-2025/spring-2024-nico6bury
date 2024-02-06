use gui::GUI;

mod gui;

fn main() {
    // struct that contains all GUI elements
    let mut gui = GUI::default();

    // make gui visible and start program
    gui.show();
    while gui.wait() {
        if let Some(val) = gui.msg_receiver.recv() {
            // Todo: match message
            match val.as_str() {
                "MenuChoice::Choice1" => {
                    gui.switch_tab(0);
                },
                "MenuChoice::Choice2" => {
                    gui.switch_tab(1);
                },
                _ => {
                    println!("Value not recognized: {}", val);
                },
            }//end matching received value
        }//end if we received a message
    }//end main app loop
}//end main method
