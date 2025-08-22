pub fn is_option_vec_empty<T>(opt_vec: &Option<Vec<T>>) -> bool {
    opt_vec.as_ref().map_or(true, |v| v.is_empty())
}

pub fn extend_option_vec<T>(target: &mut Option<Vec<T>>, source: Option<Vec<T>>) {
    if let Some(src_vec) = source {
        target.get_or_insert_with(Vec::new).extend(src_vec);
    }
}
