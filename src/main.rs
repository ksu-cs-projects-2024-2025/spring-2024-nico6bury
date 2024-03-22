use gui::GUI;

mod gui;

fn main() {
    // struct that contains all GUI elements
    let mut gui = GUI::default();

    // make gui visible and start program
    gui.show();
    gui.switch_tab(1);
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
                "CaveGen:Canvas:Update" => {
                    gui.update_cave_canvas();
                    println!("Told cave canvas to update");
                },
                "CaveGen:CA:RunGeneration" => {
                    match gui.get_cave_canvas_squareularization() {
                        Some(square_info) => {
                            let square_width = square_info.0;
                            let square_height = square_info.1;
                            let squares = square_info.2;
                            println!("Go some squareularization info from the GUI. Getting ready to run some CA generations.");

                            // TODO: Run those squares through the CA

                            // return out squares back to gui
                            gui.set_cave_canvas_squareularization(&(square_width, square_height, squares));
                            println!("Finished CA generations and sent those squareularizations back to the GUI to display.");
                        },
                        None => {println!("Couldn't get square info from cave gen canvas. We can't start doing CA like this.");}
                    };
                },
                _ => {
                    println!("Value not recognized: {}", val);
                },
            }//end matching received value
        }//end if we received a message
    }//end main app loop
}//end main method
