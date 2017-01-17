#![feature(field_init_shorthand)]
// #![feature(proc_macro)]


#[macro_use]
extern crate serde_derive; 
// extern crate serde_json;

extern crate chrono;
 
extern crate clap; 

use clap::{App, Arg, SubCommand};
use chrono::NaiveDate;

// type Date = chrono::Date<chrono::UTC>;

#[allow(dead_code)]
enum HabitState {
    Good,
    Ok,
    Sufficient,
    Late
}

#[derive(Serialize, Deserialize, Debug)] 
// #[derive(Debug)] 
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
// #[derive(Debug)] 
struct Habit {
    name: String,
    done: Vec<NaiveDate>,
    // test: NaiveDate,
    days: Days,

    // skipped_today: bool,
    // hide_until: Option<Date>
}

impl Habit {
    fn new(name: &str, days: Days) -> Habit {
        Habit { 
            name: name.to_string(), 
            days: days, 
            done: vec![]
        }
    }
}

macro_rules! p {
    ($expression:expr) => (
        println!("{:?}", $expression);
    )
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
    }
    else if let Some(rm_args) = args.subcommand_matches("rm") {
        let name = rm_args.value_of("NAME").unwrap(); // required arg
        println!("Removing habit {}", name); // TODO!: actually do it
    }
    else {
        p!("Existing habits: "); // TODO!: print saved ones...
    }

    // for argument in std::env::args() {
    //     println!("{}", argument);
    // }
}
