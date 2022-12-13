pub mod calendar {
    pub mod day_1;
}

pub struct Config {
    pub day_number: u32,
    pub part_1_input_path: String,
    pub part_2_input_path: String,
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
        
        let part_1_input_path = format!("calendar_inputs/day_{day_number}/input-part1.txt");
        let part_2_input_path = format!("calendar_inputs/day_{day_number}/input-part2.txt");

        Ok(Config { day_number, part_1_input_path, part_2_input_path })
    }
}