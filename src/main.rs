use std::{io::{self, Write}, error::Error, fs, env};
use time::Time;
use dotenv::dotenv;

mod parser;
mod files;

#[allow(dead_code, unused)]
#[derive(Debug)]
pub struct LogInfos {
    time: Time,
    content: String
}

impl LogInfos {
    fn new(time: &str, content: &str) -> LogInfos {
        let time: Time = parser::parse_time_type(time).expect("Error on parsing the time.");
        let content: String = content.to_string();
        LogInfos { time, content }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    print!("Digite a data desejada (yyyy-mm-dd): ");
    io::stdout().flush().unwrap();
    
    let mut wanted_date: String = String::new();
    io::stdin().read_line(&mut wanted_date)?;

    print!("Digite horário inicial (HH:MM): ");
    io::stdout().flush().unwrap();

    let mut wanted_init_time: String = String::new();
    io::stdin().read_line(&mut wanted_init_time)?;

    let parsed_init_time: Time = match parser::parse_time_type(wanted_init_time.trim()) {
        Ok(init_time) => init_time,
        Err(error) => panic!("An error ocurred on parsing the init time: {:?}", error),
    };

    print!("Digite horário final (HH:MM): ");
    io::stdout().flush().unwrap();

    let mut wanted_end_time: String = String::new();
    io::stdin().read_line(&mut wanted_end_time)?;

    let parsed_end_time: Time = match parser::parse_time_type(wanted_end_time.trim()) {
        Ok(end_time) => end_time,
        Err(error) => panic!("An error ocurred on parsing the end time: {:?}", error)
    };

    let path: String = format!(
        "{}{}{}{}.{}",
        env::var("LOG_DIR_PATH").expect("LOG_DIR_PATH must be set."),
        env::var("LOG_FILE_PREFIX").expect("LOG_FILE_PREFIX must be set."),
        env::var("LOG_FILE_POSTFIX").expect("LOG_FILE_POSTFIX must be set."),
        wanted_date.trim(),
        env::var("LOG_FILE_EXTENSION").expect("LOG_FILE_EXTENSION must be set."),
    );

    let content: String = fs::read_to_string(&path)?;
    let lines: Vec<&str> = content.lines().collect();

    let infos: Vec<LogInfos> = lines.iter().map(|line: &&str| {
        let string_line: Vec<&str> = line.split(": ").collect::<Vec<&str>>();
        let time: &str = parser::get_time(string_line[0]).unwrap();
        LogInfos::new(time, string_line[1])
    }).collect::<Vec<LogInfos>>();

    let mut valid_contents: Vec<&str> = vec![];
    for info in infos.iter() {
        if info.time >= parsed_init_time && info.time <= parsed_end_time {
            println!("{}", info.content);
            valid_contents.push(info.content.as_str());
        }
    }
    files::save_result(valid_contents)?;

    Ok(())
}