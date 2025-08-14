use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <num_vars> <num_values> <num_partitions> <num_self_systems>", args[0]);
        std::process::exit(1);
    }

    let num_vars: usize = args[1].parse().expect("num_vars must be an integer");
    let num_values: usize = args[2].parse().expect("num_values must be an integer");
    let num_partitions: usize = args[3].parse().expect("num_partitions must be an integer");
    let num_self_systems: usize = args[4].parse().expect("num_self_systems must be an integer");

    // Manually format arrays to ensure strict DZN compatibility
    // These arrays will now be sized by num_vars
    let alpha_coeff_str = (0..num_vars).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let beta_coeff_str = (0..num_vars).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let m_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let n_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let t_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");

    // Generate placeholder Gödel numbers
    let godel_numbers_of_self_str = (0..num_self_systems).map(|i| (i + 1000).to_string()).collect::<Vec<String>>().join(", ");

    // Manually format arrays to ensure strict DZN compatibility
    // These arrays will now be sized by num_vars
    let alpha_coeff_str = (0..num_vars).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let beta_coeff_str = (0..num_vars).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let m_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let n_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let t_idx_str = (0..num_vars).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");

    println!("num_vars = {};", num_vars);
    println!("num_values = {};", num_values);
    println!("num_partitions = {};", num_partitions);
    println!("alpha_coeff = [{}];", alpha_coeff_str);
    println!("beta_coeff = [{}];", beta_coeff_str);
    println!("m_idx = [{}];", m_idx_str);
    println!("n_idx = [{}];", n_idx_str);
    println!("t_idx = [{}];", t_idx_str);
    println!("godel_numbers_of_self = [{}];", godel_numbers_of_self_str);
}