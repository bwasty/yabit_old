#![feature(field_init_shorthand)]

#[macro_use]
extern crate serde_derive; 
extern crate serde_json;
extern crate chrono;
extern crate clap; 

use std::fs::File;
use std::error::Error;

use clap::{App, Arg, SubCommand};
use chrono::{NaiveDate, Datelike};

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

    #[allow(dead_code)]
    fn state(&self) -> HabitState {
        unimplemented!()
    }
}

fn today() -> NaiveDate {
    let today = chrono::Local::today();
    NaiveDate::from_num_days_from_ce(today.num_days_from_ce())
}

#[derive(Debug)]
struct Habits {
    pub habits: Vec<Habit>
}
impl Habits {
    fn new() -> Habits{
        Habits { habits: vec![] }
    }

    fn load(&mut self) {
        match File::open("habits.json") {
            Ok(file) => {
                // TODO: deserialize
                p!("load...");
                // let mut s = String::new();
                // file.read_to_string(&mut s).unwrap();
                self.habits = serde_json::from_reader(file).unwrap();
            },
            Err(err) => println!("Couldn't open habits.json ({}), creating new one.", err.description())
        }
    }

    fn save(&self) {
        let mut f = File::create("habits.json").unwrap();
        serde_json::to_writer_pretty(&mut f, &self.habits).unwrap()
    }

    fn add(&mut self, name: &str, days: Days) {
        self.habits.push(Habit::new(name, days));
    }

    fn remove(&mut self, name: &str) {
        self.habits.retain(|ref habit| habit.name != name);
    }

    fn done(&mut self, name: &str) {
        let today = today();
        let i = self.index_of(name);
        let habit = self.habits.get_mut(i).unwrap();
        habit.done.push(today);
        habit.done.dedup();

        // done and skipped is mutually exclusive...
        if let Ok(i) = habit.skipped.binary_search(&today) {
            habit.skipped.remove(i);
        }
    }

    fn skip(&mut self, name: &str) {
        let today = today();
        let i = self.index_of(name);
        let habit = self.habits.get_mut(i).unwrap();
        habit.skipped.push(today);
        habit.skipped.dedup();

        // done and skipped is mutually exclusive...
        if let Ok(i) = habit.done.binary_search(&today) {
            habit.done.remove(i);
        }
    }

    fn index_of(&self, name: &str) -> usize {
        self.habits.iter().position(|ref habit| habit.name == name).unwrap()
    }
}

fn main() {
    let mut habits = Habits::new();
    habits.load();

    let required_name_arg = Arg::with_name("NAME").required(true);
    let args = App::new("yabit")
        .subcommand(SubCommand::with_name("add").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("rm").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("done").arg(required_name_arg.clone()))
        .subcommand(SubCommand::with_name("skip").arg(required_name_arg.clone()))
        .get_matches();

    let (command, sub_args) = args.subcommand();
    if command == "" {
        // no subcommand, just print habits
        p!(habits);
        return
    }
    let name = sub_args.unwrap().value_of("NAME").unwrap(); // required arg
    match command {
        "add" => {
            // TODO: days...
            habits.add(name, Days::new(1, 2, 3));
        }
        "rm" => {
            habits.remove(name);
        },
        "done" => {
            habits.done(name);
        },
        "skip" => {
            habits.skip(name);
        }
        _ => ()
    }

    habits.save(); 
}

