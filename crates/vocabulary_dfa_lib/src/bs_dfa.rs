use regex::Regex;

pub fn matches_bs(text: &str) -> bool {
    let pattern = r"^(bs17704|bs58_decode|bs_out|bs_run|bs_shape|bs_transmit|bscr|bscscan|bse|bsem|bsemi|bsend|bset|bset_error|bshd|bsi|bsim|bsime|bsklkan1wm4hvhprdsjosmqsg2j8tq5xp2s2dads6ni4|bsksunvenxarabrl77ufan1gi7unvemqadcbhsjun6tu|bsl|bso|bsol|bsolb|bsolh|bsolhs|bsolhsu|bsolhsub|bson_file_path|bson_log_trail|bsondeserialization|bsonserialization|bsps|bsub|bsum|bsums|bsv|bsv_0|bsv_10|bsv_20|bsv_30|bsv_40|bsv_50|bsv_60|bsv_70|bsv_80|bsv_90|bsym)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
