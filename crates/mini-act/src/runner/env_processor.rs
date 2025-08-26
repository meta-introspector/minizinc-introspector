use std::collections::HashMap;

pub fn substitute_env_vars(run: &str, env: &Option<HashMap<String, String>>) -> String {
    let mut substituted_run = run.to_string();
    if let Some(env_map) = env {
        let mut changed = true;
        while changed {
            changed = false;
            for (key, value) in env_map {
                let placeholder = format!("${{{{ env.{} }}}}", key);
                if substituted_run.contains(&placeholder) {
                    substituted_run = substituted_run.replace(&placeholder, value);
                    changed = true;
                }
            }
        }
    }
    substituted_run
}
