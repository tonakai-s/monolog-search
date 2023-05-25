use std::error::Error;
use time::Time;

pub fn parse_time_type(time: &str) -> Result<Time, Box<dyn Error>> {
    let mut iterator = time.split(":");

    Ok(Time::from_hms(
        iterator.next().expect("Error parsing hour.").parse::<u8>()?,
        iterator.next().unwrap_or("00").parse::<u8>()?,
        iterator.next().unwrap_or("00").parse::<u8>()?
    )?)
}

pub fn get_time(monolog_info: &str) -> Result<&str, Box<dyn Error>> {
    let start_byte = monolog_info.find('[').unwrap() + 1;
    let end_byte = monolog_info.find(']').unwrap();

    let datetime: &str = &monolog_info[start_byte..end_byte];
    let time_start_byte = datetime.find('T').unwrap() + 1;
    Ok(&datetime[time_start_byte..time_start_byte + 8])
}