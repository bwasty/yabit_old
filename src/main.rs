#![feature(field_init_shorthand)]

#[macro_use]
extern crate serde_derive; 
extern crate serde_json;
extern crate chrono;
extern crate clap; 

use std::fs::File;
use std::error::Error;

use clap::{App, Arg, SubCommand};
use chrono::NaiveDate;

macro_rules! p {
    ($expression:expr) => (
        println!("{:?}", $expression);
    )
}

#[allow(dead_code)]
enum HabitState {
    Good,
    Ok,
    Sufficient,
    Late
}

// enum HabitStatus {
//     Done,
//     Skipped,
//     Hidden
// }

#[derive(Serialize, Deserialize, Debug)] 
struct Days {
    good: u16,
    ok: u16,
    sufficient: u16
}
impl Days {
    fn new(good: u16, ok: u16, sufficient: u16) -> Days {
        Days { good, ok, sufficient }
    }
} 

#[derive(Serialize, Deserialize, Debug)] 
struct Habit {
    name: String,
    done: Vec<NaiveDate>,
    skipped: Vec<NaiveDate>,
    days: Days,

    // skipped_today: bool,
    // hide_until: Option<Date>
}

impl Habit {
    fn new(name: &str, days: Days) -> Habit {
        Habit { 
            name: name.to_string(), 
            days: days, 
            done: vec![],
            skipped: vec![]
        }
    }

    fn state(&self) -> HabitState {
        unimplemented!()
    }
}

struct Habits {
    habits: Vec<Habit>
}
impl Habits {
    fn load(&mut self) {
        match File::open("habits.json") {
            Ok(file) => {
                // TODO: deserialize
                p!("load...");
            },
            Err(err) => println!("Couldn't open habits.json ({}), creating new one.", err.description())
        }
    }

    fn save(&self) {

    }

    fn add(name: &str, days: Days) {
        unimplemented!()
    }

    fn remove(name: &str) {
        unimplemented!()
    }

    fn done(name: &str) {
        unimplemented!()
    }

    fn skip(name: &str) {
        unimplemented!()
    }
}

impl Drop for Habits {
    fn drop(&mut self) {
        p!("Drop");
        self.save();
    }
}

fn main() {
    let required_name_arg = Arg::with_name("NAME").required(true);
    let args = App::new("yabit")
        .subcommand(SubCommand::with_name("new").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("rm").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("done").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("skip").arg(required_name_arg.clone()))
        .get_matches();
    if let Some(new_args) = args.subcommand_matches("new") {
        let name = new_args.value_of("NAME").unwrap(); // required arg
        let habit = Habit::new(name, Days::new(1, 2, 3));
        println!("Adding {:?}", habit); // TODO!: actually do it
        println!("{}", serde_json::to_string_pretty(&habit).unwrap())
    }
    else if let Some(rm_args) = args.subcommand_matches("rm") {
        let name = rm_args.value_of("NAME").unwrap(); // required arg
        println!("Removing habit {}", name); // TODO!: actually do it
    }
    else {
        p!("Existing habits: "); // TODO!: print saved ones...
    }

    let (command, sub_args) = args.subcommand();
    let name = sub_args.unwrap().value_of("NAME").unwrap(); // required arg
    // match command {
    //     "new" => 
    // }

    let habits = Habits { habits: vec![] };
}

