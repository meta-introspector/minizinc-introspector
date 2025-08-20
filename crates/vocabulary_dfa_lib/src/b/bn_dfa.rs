use regex::Regex;

pub fn matches_bn(text: &str) -> bool {
    let pattern = r"^(bn0|bn128|bn254x5|bn2d|bn2puayxux6juabaxydkdj5qhbnnmkw8dcgugcyrrfn|bn3|bna|bnb_ledger_balance_of|bnb_ledger_transfer|bnb_rpc|bnb_test|bnbchain|bnd_ty|bne|bneck_conf|bneck_confs|bneq|bnequ|bnequi|bnequiv|bnid2|bnids|bnode0|bnode1|bnode_from_scratch|bnode_from_term|bnode_from_term_ref|bnodeissuer|bnodeprofile|bnos|bnot|bnplzt2tiwhtsvxsbeptp4zui4dkcmz2xr4eikai1v|bns|bntceohky4hcimfvef7fpqeqnuil1xt7j4olhzsyu4tcr809axabclcj5dm1xe5gzruo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
