use regex::Regex;

pub fn matches_bk(text: &str) -> bool {
    let pattern = r"^(bka|bkar|bkaro|bkarow|bkcpbqqbzqggvnfso5nq8rq4rwwogywjuut9bibjxwnf|bkey|bkfdxijqwzxgtzajqxh7wvehkamwcgsevkrvswffrjpd|bkpt|bks|bktree|bkyz2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
