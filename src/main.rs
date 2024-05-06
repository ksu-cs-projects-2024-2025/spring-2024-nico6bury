
use gui::GUI;
use nice_map_generator::{cellular_automata::CA, export};

mod gui;

fn main() {
    // struct that contains all GUI elements
    let mut gui = GUI::default();

    // make gui visible and start program
    gui.show();
    gui.switch_tab(2);
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
                "RoomGen:Canvas:Update" => {
                    gui.update_room_canvas();
                    println!("Told build canvas to update");
                },
                "CaveGen:CA:RunGeneration" => {
                    match gui.get_cave_canvas_squareularization() {
                        Some(squares) => {
                            println!("Go some squareularization info from the GUI. Getting ready to run some CA generations.");

                            let ca_info = gui.get_cave_canvas_ca_settings();
                            // TODO: Run those squares through the CA
                            let mut ca_runner = CA::new(ca_info.0, ca_info.1).with_squares(squares);

                            for _ in 0..ca_info.2 {
                                ca_runner.run_generation();
                                match ca_runner.get_squares() {
                                    Some(changed_squares) => {
                                        // return out squares back to gui
                                        gui.set_cave_canvas_squareularization(changed_squares);
                                        
                                        // TODO: Find a way to make gui update on each generation

                                        println!("Finished CA generation {} and sent squareularization back to the GUI to display.", ca_runner.generations_so_far());
                                    }, None => println!("CA Gen Failed or couldn't get squares."),
                                }//end matching based on squares

                            }//end running for each generation

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
                "RoomGen:Stairs:Remove" => {
                    let selected_elem_list = gui.get_room_gen_stairs_selected();
                    if selected_elem_list.len() > 0 {
                        let message = format!("Are you sure you want to remove the selected level connections?\nThe following level connections will be lost:\n{}", selected_elem_list.join(", "));
                        if GUI::yes_no(&message) {
                            gui.remove_room_gen_stairs_selected();
                        }//end if we're clear to go ahead and remove those things
                    } else {
                        GUI::message("No Level Connections Selected. Please select connections to remove.");
                    }//end else we just need to say that there are no level connections selected
                },
                "Export:PNG" | "Export:JPEG" | "Export:BMP" | "Export:WEBP" => {
                    let img_format = val.split(":").last().unwrap_or("PNG").to_lowercase();
                    let cave_room_choice = GUI::choice_dialog("Do you want to save a cave map or a room map?", "Cave", "Room");
                    if let Some(cave_room_choice) = cave_room_choice {
                        if let Some(mut pathbuf) = GUI::save_img_dialog(&img_format) {
                            pathbuf.set_extension(img_format.to_lowercase());
                            match cave_room_choice {
                                "Cave" => {
                                    if let Some(squares) = gui.get_cave_canvas_squareularization() {
                                        let square_img = export::generate_img_from_map(&squares);
                                        square_img.save(pathbuf).expect("Failed to save img");
                                    }//end if we can get a SquareGrid from the cave canvas
                                }, "Room" => {
                                    if let Some(squares) = gui.get_room_canvas_squareularization() {
                                        let square_img = export::generate_img_from_map(&squares);
                                        square_img.save(pathbuf).expect("Failed to save img");
                                    }
                                }, _ => println!("Unrecognized Cave or Room choice: {}", cave_room_choice),
                            }//end matching user choice
                        }//end if we have a file path to save to
                    }//end if user is deciding between cave and room map to save
                },
                _ => {
                    println!("Value not recognized: {}", val);
                },
            }//end matching received value
        }//end if we received a message
    }//end main app loop
}//end main method
