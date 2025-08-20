use regex::Regex;

pub fn matches_nn(text: &str) -> bool {
    let pattern = r"^(nn_build|nn_build_bench|nn_helper|nn_test|nn_test_degenerate|nn_test_empty|nn_test_error|nn_test_random|nn_tests|nnbsp|nneg|nnerror|nnetwork|nnew_line|nni|nnmatch|nnnnn|nnnnnnnnn|nnnoiseless|nnode|nnodes|nnone|nnopqrs|nnotice|nnotification|nns|nnx|nnz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
