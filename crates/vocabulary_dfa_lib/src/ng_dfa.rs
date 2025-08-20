use regex::Regex;

pub fn matches_ng(text: &str) -> bool {
    let pattern = r"^(ngateway|ngeneration|ngeq|ngeqq|ngeqs|ngeqsl|ngeqsla|ngeqslan|ngeqslant|nges|ngf|ngg|ngggg|nggggg|ngggggˇ|ngh|nghij|ngiven|nglobal|nglossary|ngme7hgbt6tajn1f6yuccngpqt5cvstndzuvljq4jwa|ngo|ngoodbye|ngoose|ngptyvby9p5l6aofr7blqii|ngram3|ngram_charidx_iterator|ngram_items|ngram_len|ngram_tokenizer|ngramfeature|ngramlist|ngramlistintoiterator|ngramtokenstream|ngrandchild|ngraph1|ngraph2|ngs|ngsi|ngsim|ngt|ngtr|ngtv|nguest|nguidelines)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
