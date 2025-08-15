#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::MiniZincEnvironment;
    use crate::coverage_report;

    #[test]
    fn test_get_version_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }

    #[test]
    fn test_parse_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
    }

    #[test]
    fn test_solve_and_extract_int() {
        let env = MiniZincEnvironment::new().unwrap();
        let model_code = "var int: x; constraint x > 5; solve minimize x;";
        let args: Vec<&str> = Vec::new();

        let run_status = env.run_model(model_code, &args);
        println!("Run status: {}", run_status);
        assert_eq!(run_status, 0); // Assuming 0 means success

        let si_ptr = env.get_solver_instance();
        assert!(!si_ptr.is_null());

        let next_status = env.solver_instance_next(si_ptr);
        println!("Next solution status: {}", next_status);
        assert_eq!(next_status, 1); // Assuming 1 means a solution was found

        let x_value = env.get_solution_value_int(si_ptr, "x");
        println!("Value of x: {}", x_value);
        assert_eq!(x_value, 6);

        env.solver_instance_print_solution(si_ptr);
    }
}