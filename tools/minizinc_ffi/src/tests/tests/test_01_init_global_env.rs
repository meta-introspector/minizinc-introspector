#[cfg(test)]
mod test_01_init_global_env {
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_01_init_global_env() {
        let _env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        println!("Test 01: Initialized global environment.");
    }
}
