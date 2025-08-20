use regex::Regex;

pub fn matches_nr(text: &str) -> bool {
    let pattern = r"^(nr_inputs|nra|nranking|nrar|nrarr|nrarrc|nrarrw|nrdoc|nre|nreasoning|nrecent|nrecover|nrecursive|nrecv|nreduced|nreduction|nrelevant|nremoving|nreplaced_line|nreplacements|nreq|nreset|nresize|nresolve|nresults|nreturn|nreturns|nrevc|nri|nrig|nrigh|nrighta|nrightar|nrightarr|nrightarro|nrightarrow|nrld|nrmse|nrnfvbesv9fem5kfmscxjsgelrskcv55drtavdy5azpnsp|nrof_lines|nrole|nrows_dst|nrows_split|nrows_x|nrows_y|nrrr|nrrrr|nrtr|nrtri|nrtrie|nrule|nrules|nruns|nrustdoc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
