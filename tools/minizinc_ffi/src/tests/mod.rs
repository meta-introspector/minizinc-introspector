#[cfg(test)]
mod tests {
    use super::*;

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
}
