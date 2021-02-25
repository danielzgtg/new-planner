use std::fs;
use crate::models::{Batch, Course};

pub struct LoadedRaw {
    course_code: String,
    raw: String,
}

fn parse_data(raw: &LoadedRaw) -> Course {
    let mut batches = vec!();
    let mut learning_descs = vec!();
    let mut latest_time = "0";
    for line in raw.raw.lines().skip_while(|x| x.starts_with('+')) {
        if line.len() == 0 { continue }
        let time;
        let desc;
        {
            let mut split = line.splitn(3, ' ');
            match split.next().expect("state") {
                "+" => panic!("Expected to have studied in order"),
                "-" => {},
                _ => panic!("Invalid state"),
            }
            time = match split.next().expect("time") {
                "0" => None,
                x => Some(x),
            };
            desc = split.next().expect("desc");
            split.next().unwrap_none();
        }
        match time {
            None => learning_descs.push(desc),
            Some(time) => {
                assert!(time.chars().all(|x| x.is_numeric()), "Time should be numeric");
                assert!(time > latest_time, "Items must be listed in chronological order");
                latest_time = time;
                batches.push(Batch {
                    evaluation_time: time,
                    evaluation_desc: desc,
                    learning_descs
                });
                learning_descs = vec!();
            },
        }
    }
    learning_descs.first().expect_none("Must have exam after all the studying");
    batches.first().expect("You probably don't have empty courses");
    Course {
        course_code: &raw.course_code,
        batches,
    }
}

pub fn load_data(raws: &[LoadedRaw]) -> Vec<Course> {
    raws.iter().map(|x| parse_data(x)).collect()
}

pub fn read_data() -> Vec<LoadedRaw> {
    fs::read_dir("data")
        .expect("Can't list data directory")
        .map(|x| x.expect("Error in directory entry"))
        .map(|entry| {
            let path = entry.path();
            match path.extension() {
                Some(x) => if x != "txt" { panic!("Alien file in data directory") },
                _ => panic!("Extensionless file in data directory"),
            }
            let course_code = path.file_stem()
                .expect("Can't get file stem")
                .to_str()
                .expect("Invalid file name");
            assert!(course_code.chars().all(|x| x.is_ascii_alphanumeric()), "Invalid course code");
            LoadedRaw {
                course_code: course_code.to_owned(),
                raw: fs::read_to_string(path).expect("Can't read data file"),
            }
        }).collect()
}
