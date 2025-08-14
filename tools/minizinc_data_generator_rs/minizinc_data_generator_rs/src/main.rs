use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <num_vec>", args[0]);
        std::process::exit(1);
    }

    let num_vec: usize = args[1].parse().expect("num_vec must be an integer");

    // Manually format arrays to ensure strict DZN compatibility
    let alpha_coeff_str = (0..num_vec).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let beta_coeff_str = (0..num_vec).map(|_| "0.0".to_string()).collect::<Vec<String>>().join(", ");
    let m_idx_str = (0..num_vec).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let n_idx_str = (0..num_vec).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");
    let t_idx_str = (0..num_vec).map(|_| "0".to_string()).collect::<Vec<String>>().join(", ");

    println!("alpha_coeff = [{}];", alpha_coeff_str);
    println!("beta_coeff = [{}];", beta_coeff_str);
    println!("m_idx = [{}];", m_idx_str);
    println!("n_idx = [{}];", n_idx_str);
    println!("t_idx = [{}];", t_idx_str);
}