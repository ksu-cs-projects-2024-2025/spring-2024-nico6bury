use gui::GUI;

use crate::cellular_automata::CA;

mod gui;
mod squares;
mod cellular_automata;

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
                        Some(squares) => {
                            println!("Go some squareularization info from the GUI. Getting ready to run some CA generations.");

                            let ca_info = gui.get_cave_canvas_ca_settings();
                            // TODO: Run those squares through the CA
                            let mut ca_runner = CA::new(ca_info.0, ca_info.1).with_squares(squares);

                            for _ in 0..ca_info.2 { ca_runner.run_generation(); }

                            match ca_runner.get_squares() {
                                Some(changed_squares) => {
                                    // return out squares back to gui
                                    gui.set_cave_canvas_squareularization(changed_squares);
                                    println!("Finished CA generations and sent those squareularizations back to the GUI to display.");
                                }, None => println!("CA Gen Failed or couldn't get squares."),
                            }//end matching based on squares
                        },
                        None => {println!("Couldn't get square info from cave gen canvas. We can't start doing CA like this.");}
                    };
                },
                "CaveGen:Stairs:Remove" => {
                    let selected_elem_list = gui.get_cave_gen_stairs_selected();
                    if selected_elem_list.len() != 0 {
                        let message = format!("Are you sure you want to remove the selected level connections?\nThe following level connections will be lost:\n{}",
                            selected_elem_list.join(", "));
                        if GUI::yes_no(&message) {
                            gui.remove_cave_gen_stairs_selected();
                        }//end if we're clear to go ahead and remove those things
                        // println!("{}", message_result);
                    } else {
                        GUI::message("No Level Connections Selected. Please select connections to remove.");
                    }//end else we just need to say that there are no level connections selected
                },
                _ => {
                    println!("Value not recognized: {}", val);
                },
            }//end matching received value
        }//end if we received a message
    }//end main app loop
}//end main method
