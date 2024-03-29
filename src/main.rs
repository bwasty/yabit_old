#![feature(field_init_shorthand)]

#[macro_use]
extern crate serde_derive; 
extern crate serde_json;
extern crate chrono;
extern crate clap; 

#[macro_use] extern crate enum_primitive;
extern crate num;
// use num::FromPrimitive;

use std::fs::File;
use std::error::Error;
use std::time::SystemTime;

use clap::{App, Arg, SubCommand};
use chrono::{NaiveDate, Datelike, Duration};

macro_rules! p {
    ($expression:expr) => (
        println!("{:?}", $expression);
    )
}

enum_from_primitive! {
#[derive(Debug, PartialEq)]
    enum HabitState {
        VeryGood = 1,
        Good = 2,
        OK = 3,
        Sufficient = 4,
        Late = 5,
        Unknown = 6,
    }
}
use HabitState::*;

// enum HabitStatus {
//     Done,
//     Skipped,
//     Hidden
// }

#[derive(Serialize, Deserialize, Debug)] 
struct Days {
    good: i32,
    ok: i32,
    sufficient: i32,
}
impl Days {
    fn new(good: i32, ok: i32, sufficient: i32) -> Days {
        assert!(good < ok && ok < sufficient);
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
            skipped: vec![],
        }
    }

    #[allow(dead_code)]
    /// state when habit not done on date
    fn state(&self, date: &NaiveDate) -> HabitState {
        let diff = self.days_since_done(date);
        let days = &self.days;
        match (days.good - diff, days.ok - diff, days.sufficient - diff) {
            (x, _, _) if x >  1 => VeryGood,
            (x, _, _) if x == 1 => Good,
            (x, _, _) if x == 0 => OK,
            (_, y, _) if y >  0 => OK,
            (_, y, _) if y == 0 => Sufficient,
            (_, _, z) if z >  0 => Sufficient,
            (_, _, z) if z <= 0 => Late,
            _ => unreachable!()
        }
    }

    fn days_since_done(&self, date: &NaiveDate) -> i32 {
        p!(date);
        p!(today());
        assert!(date >= &today()); // TODO: support dates in the past
        // TODO: replace last with last before date
        let diff = *date - *self.done.last().unwrap(); // TODO: handle unwrap
        diff.num_days() as i32
    }

    /// Days until the state 'decays'
    #[allow(dead_code)]
    fn days_left(&self) -> i32 {
        let today = today();
        let diff = self.days_since_done(&today);
        match self.state(&(today - Duration::days(1))) {
            VeryGood | Good     => self.days.good - diff,
            OK                  => self.days.ok - diff,
            Sufficient | Late   => self.days.sufficient - diff,
            Unknown             => 0 // TODO!: what makes sense here?
        }
    }

    /// Difference in days to HabitState
    // fn diff_to_state(diff: i32) {
    //     // TODO: state() enough??
    // }

    /// Average of the last `averaging_limit` habit executions
    #[allow(dead_code)]
    fn avg_state(&self, averaging_limit: usize) -> HabitState {
        let latest_dates: Vec<_> = self.done.iter()
            .rev() // latest executions
            .take(averaging_limit + 1) // add one for comparing against the last execution before
            .collect(); 
        let diffs: Vec<_> = latest_dates.windows(2).map(|window| {
            let done = *window[0];
            let done_before = *window[1];
            (done - done_before).num_days()
        }).collect(); 
        if diffs.len() == 0 {
            return Unknown
        }
        let avg = diffs.iter().sum::<i64>() as f32 / diffs.len() as f32;
        match avg {
            _ if avg <= self.days.good as f32 => Good,
            _ if avg <= self.days.ok as f32 => OK,
            _ if avg <= self.days.sufficient as f32 => Sufficient,
            _ => Late
        }
    }
}

#[allow(dead_code)]
fn tomorrow() -> NaiveDate {
    today() + Duration::days(1)
}

#[derive(Debug)]
struct Habits {
    pub habits: Vec<Habit>,
    pub averaging_window: i32
}
impl Habits {
    fn new() -> Habits {
        Habits { habits: vec![], averaging_window: 5 }
    }

    fn load(&mut self) {
        match File::open("habits.json") {
            Ok(file) => {
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
    let start_time = SystemTime::now();

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

    print_elapsed(&start_time);
}

fn print_elapsed(start_time: &SystemTime) {
    let elapsed = start_time.elapsed().unwrap();
    println!("{}s {:.*}ms", elapsed.as_secs(), 1, elapsed.subsec_nanos() as f64 / 1_000_000.0);
}

fn today() -> NaiveDate {
    let today = chrono::Local::today();
    NaiveDate::from_num_days_from_ce(today.num_days_from_ce())
}


// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_habit_state() {
        let mut h = Habit::new("foo", Days::new(3, 5, 7));
        h.done.push(today());
        assert_eq!(h.state(&today()),                       VeryGood);
        assert_eq!(h.state(&(today() + Duration::days(1))), VeryGood);
        assert_eq!(h.state(&(today() + Duration::days(2))), Good);
        assert_eq!(h.state(&(today() + Duration::days(3))), OK);
        assert_eq!(h.state(&(today() + Duration::days(4))), OK);
        assert_eq!(h.state(&(today() + Duration::days(5))), Sufficient);
        assert_eq!(h.state(&(today() + Duration::days(6))), Sufficient);
        assert_eq!(h.state(&(today() + Duration::days(7))), Late);
        assert_eq!(h.state(&(today() + Duration::days(8))), Late);

        h.days = Days::new(2, 3, 4);
        assert_eq!(h.state(&today()),                       VeryGood);
        assert_eq!(h.state(&(today() + Duration::days(1))), Good);
        assert_eq!(h.state(&(today() + Duration::days(2))), OK);
        assert_eq!(h.state(&(today() + Duration::days(3))), Sufficient);
        assert_eq!(h.state(&(today() + Duration::days(4))), Late);
        assert_eq!(h.state(&(today() + Duration::days(5))), Late);

        h.days = Days::new(1, 2, 3);
        assert_eq!(h.state(&today()),                       Good);
        assert_eq!(h.state(&(today() + Duration::days(1))), OK);
        assert_eq!(h.state(&(today() + Duration::days(2))), Sufficient);
        assert_eq!(h.state(&(today() + Duration::days(3))), Late);
    }

    // #[test]
    #[allow(dead_code)]
    fn test_habit_days_left() {
        let mut h = Habit::new("foo", Days::new(2, 4, 5));
        let t = today();
        h.done.push(t);
        assert_eq!(h.days_left(), 2);
        // h.done[0] = t - Duration::days(1);
        // assert_eq!(h.days_left(), 1);
        // h.done[0] = t - Duration::days(2);
        // assert_eq!(h.days_left(), 0);        
        // h.done[0] = t - Duration::days(3);
        // assert_eq!(h.days_left(), 1);
        // h.done[0] = t - Duration::days(4);
        // assert_eq!(h.days_left(), 0);
        // h.done[0] = t - Duration::days(5);
        // assert_eq!(h.days_left(), 0);
        // h.done[0] = t - Duration::days(6);
        // assert_eq!(h.days_left(), -1);
    }

    #[test]
    #[allow(dead_code)]
    fn test_habit_avg_state() {
        let limit = 5; // averaging limit
        let mut h = Habit::new("foo", Days::new(2, 3, 4));
        let t = today();
        assert_eq!(h.avg_state(limit), Unknown);
        h.done.push(t - Duration::days(1));
        assert_eq!(h.avg_state(limit), Unknown);
        h.done.insert(0, t - Duration::days(3));
        assert_eq!(h.avg_state(limit), Good);
        h.done.insert(0, t - Duration::days(6));
        assert_eq!(h.avg_state(limit), OK);
        h.done.insert(0, t - Duration::days(10));
        assert_eq!(h.avg_state(limit), OK);
        h.done.insert(0, t - Duration::days(15));
        assert_eq!(h.avg_state(limit), Sufficient);
        h.done.insert(0, t - Duration::days(30));
        assert_eq!(h.avg_state(limit), Late);
        assert_eq!(h.avg_state(3), OK);
    }
}

