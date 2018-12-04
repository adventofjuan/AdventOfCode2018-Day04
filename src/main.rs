extern crate chrono;
extern crate indexmap;

use indexmap::map::IndexMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use chrono::{NaiveDateTime, Timelike};

enum Action {
    BeginShift(i32),
    WakeUp,
    FallAsleep,
    Error,
}

fn parse_shift_start(val: &str) -> i32 {
    let split = val.split(" ").collect::<Vec<&str>>();
    let id = split[0].parse::<i32>().expect("Could not parse id");
    id
}

fn read_file_to_vec(file: BufReader<&File>)
    -> IndexMap<NaiveDateTime, Action> {

    let mut res = IndexMap::new();
    for line in file.lines() {
        let l = line.unwrap();
        let timestamp = NaiveDateTime::parse_from_str(
                                           &l[1..17],
                                            "%Y-%m-%d %H:%M"
                                        ).expect("Could not parse time");
        let action = match &l[19..24] {
            "wakes" => Action::WakeUp,
            "falls" => Action::FallAsleep,
            "Guard" => Action::BeginShift(parse_shift_start(&l[26..])),
            _       => Action::Error,
        };

        res.insert(timestamp, action);
    }
    res
}

fn part01(values: &mut IndexMap<NaiveDateTime, Action>) {
    let mut sleep_times = IndexMap::new();

    let mut cur_guard = 0;
    let mut asl_start = 0;

    let mut max_minutes_sum= 0;
    let mut max_id = 0;

    values.sort_keys();

    for (time, event) in values.iter() {
        print!("[{}] ", time);
        match event {
            Action::BeginShift(id) => {
                print!("Guard #{} starts", *id);
                cur_guard = *id;
            },
            Action::WakeUp => {
                print!("Wake up");
                let c_sleep = sleep_times.entry(cur_guard).or_insert([0;60]);
                for min in asl_start..time.time().minute() {
                    c_sleep[min as usize] += 1;
                }
            },
            Action::FallAsleep => {
                print!("Fall asleep");
                asl_start = time.time().minute();
            },
            Action::Error => println!("Oop"),
        };
        print!("\n");
    }

    for (id, min_array) in sleep_times.iter() {
        let minutes_sum:i32 = min_array.iter().sum();
        if minutes_sum > max_minutes_sum {
            max_minutes_sum = minutes_sum;
            max_id = *id;
        }
    }

    let mut max_minute = 0;
    let mut max_dur = 0;
    let minutes_arr = sleep_times.get(&max_id).unwrap();
    for i in 0..minutes_arr.len() {
        if minutes_arr[i] > max_dur {
            max_minute = i as i32;
            max_dur = minutes_arr[i];
        }
    }

    println!("Part1 result = {}", max_id * max_minute);
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let mut values = read_file_to_vec(file);

    part01(&mut values);
}
