#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::MiniZincEnvironment;
    use crate::ffi_bindings::{minizinc_gc_lock, minizinc_gc_unlock};
    use std::sync::Once;
    use crate::coverage_report;

    static INIT: Once = Once::new();

    pub fn setup() {
        INIT.call_once(|| {
            // Initialize logging or other test-wide setup
            println!("---> Initializing MiniZinc GC lock <---");
            unsafe {
                minizinc_gc_lock();
            }
        });
    }

    #[test]
    fn test_get_version_string() {
        setup();
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        setup();
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }

    #[test]
    fn test_parse_string() {
        setup();
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
        setup();
        let env = MiniZincEnvironment::new().unwrap();
        let model_code = "var int: x; constraint x > 5; solve minimize x;";

        // For now, we only parse the model. Solving will be added later.
        let model = env.parse_string(model_code);
        assert!(model.is_ok());

        let solver_instance_ptr = env.get_solver_instance();
        assert!(!solver_instance_ptr.is_null());

        // The following lines are commented out as solving is not yet fully implemented
        // let next_status = env.solver_instance_next(solver_instance_ptr);
        // println!("Next solution status: {}", next_status);
        // assert_eq!(next_status, 1); // Assuming 1 means a solution was found

        // let x_value = env.get_solution_value_int("x");
        // println!("Value of x: {}", x_value);
        // assert_eq!(x_value, 6);

        // env.solver_instance_print_solution(solver_instance_ptr);
    }
}