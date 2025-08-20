use regex::Regex;

pub fn matches_c7(text: &str) -> bool {
    let pattern = r"^(c7c8odr8oashr5feyrq2tjkaxl18id1dsj2zbkdgl2c2|c7ws9ic7kn9xnclsnomvztvbzurm3rfgdeqn7qjmwnln)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
