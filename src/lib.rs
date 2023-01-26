pub mod calendar {
    pub mod day_1;
    pub mod day_2;
}

pub struct Config {
    pub day_number: u32,
    pub input_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("please provide day number");
        }

        let day_number = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("wrong day number"),
        };
        
        if day_number > 25 || day_number < 1 {
            return Err("please provide day in range 1..25");
        }
        
        let input_path = format!("calendar_inputs/d{day_number}.txt");

        Ok(Config { day_number, input_path })
    }
}