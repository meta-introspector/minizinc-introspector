use regex::Regex;

pub fn matches_tf(text: &str) -> bool {
    let pattern = r"^(tf2qmybnfbo0z0lo9hsvjstytgeye1nrn7w|tf32|tf_capitalized|tf_explanation|tf_factor|tf_id|tf_idf|tf_idf_logic|tf_idf_params|tf_idf_vectorization_test_file_1|tf_idf_vectorization_test_file_2|tf_idf_vectorization_test_file_3|tf_idf_vectorization_test_file_4|tf_idf_vectorization_test_file_5|tf_max|tf_max_u|tf_mean|tf_num_bits|tf_std|tf_sum|tf_total|tf_upper|tfailed|tfandpositionrecorder|tfidf_path|tfidf_result|tfidf_results|tfidf_state|tfidflogic|tfidfmethod|tfidfparams|tfidfs|tfield|tfirst|tfoobar|tfoot|tfor|tfr|tfrom|tfruit|tftextembedder|tfunc|tfvars|tfw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
