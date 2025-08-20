use regex::Regex;

pub fn matches_pg(text: &str) -> bool {
    let pattern = r"^(pg17|pg_backend_pid|pg_id|pg_pool|pg_stat_activity|pg_terminate_backend|pga2|pga3|pga_extract_point|pga_hyperplane|pga_point|pga_test|pga_translation|pgalgebra|pgamv|pgconnectoptions|pgm|pgo_data_dir|pgo_dir|pgo_profile|pgo_works|pgrep|pgrow|pgsslmode|pgvalue|pgvalueref|pgvector|pgvectordistancefunction)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
