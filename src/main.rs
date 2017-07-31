
#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate meval;

use gtk::{ButtonExt, Inhibit, OrientableExt, WidgetExt};
use gtk::Orientation::{Vertical, Horizontal};
use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

// Define the structure of the model.
#[derive(Clone)]
pub struct Model {
    expression: String,
}

// The messages that can be sent to the update function.
#[derive(Msg)]
pub enum Msg {
    Quit,
    Operator(String),
    Calculate,
    Clear,
    OnNumber(i32),
}

#[widget]
impl Widget for Win {
    // The initial model.
    fn model() -> Model {
        Model {
            expression: String::new(),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg, model: &mut Model) {
        match event {
            OnNumber(n) => model.expression = format!("{}{}", model.expression, n),
            Operator(op) => model.expression = format!("{}{}", model.expression, op),
            Calculate => {

                let evaluated = meval::eval_str(&model.expression);
                match evaluated {
                    Ok(number) => model.expression = number.to_string(),
                    Err(e) => model.expression = e.to_string(),
                }
            }
            Clear => model.expression = String::new(),
            Quit => gtk::main_quit(),
            _ => println!("not implemented yet"),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,

                gtk::Box {
                    gtk::Label {
                        text: &model.expression.to_string(),
                    },
                },
                
                gtk::Box {

                    orientation: Vertical,

                    gtk::Box {
                        orientation: Horizontal,
                        
                        gtk::Button {
                            clicked => OnNumber(1),
                            label: "1",
                        },
                        gtk::Button {
                            clicked => OnNumber(2),
                            label: "2",
                        },
                        gtk::Button {
                            clicked => OnNumber(3),
                            label: "3",
                        },
                        
                    },

                    gtk::Box {
                        orientation: Horizontal,
                        
                        gtk::Button {
                            clicked => OnNumber(4),
                            label: "4",
                        },
                        gtk::Button {
                            clicked => OnNumber(5),
                            label: "5",
                        },
                        gtk::Button {
                            clicked => OnNumber(6),
                            label: "6",
                        },
                        
                    },

                    gtk::Box {
                        orientation: Horizontal,
                        
                        gtk::Button {
                            clicked => OnNumber(7),
                            label: "7",
                        },
                        gtk::Button {
                            clicked => OnNumber(8),
                            label: "8",
                        },
                        gtk::Button {
                            clicked => OnNumber(9),
                            label: "9",
                        },
                        
                    },
                    
                    gtk::Box {
                        orientation: Horizontal,
                        
                        gtk::Button {
                            clicked => Operator("+".to_string()),
                            label: "+",
                        },
                        gtk::Button {
                            clicked => Operator("-".to_string()),
                            label: "-",
                        },
                        
                    },
                    gtk::Box {
                        orientation: Horizontal,
                        
                        gtk::Button {
                            clicked => Operator("+".to_string()),
                            label: "*",
                        },
                        gtk::Button {
                            clicked => Operator("-".to_string()),
                            label: "/",
                        },
                        
                    },
                    gtk::Button {
                            clicked => Calculate,
                            label: "=",
                    },
                    gtk::Button {
                        clicked => Clear,
                        label: "clear",
                    }

                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}