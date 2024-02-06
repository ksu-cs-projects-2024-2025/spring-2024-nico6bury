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
            println!("{}", val);
        }//end if we received a message
    }//end main app loop
}//end main method
