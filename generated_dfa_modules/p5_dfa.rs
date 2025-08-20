use regex::Regex;

pub fn matches_p5(text: &str) -> bool {
    let pattern = r"^(p50|p50kbase|p50kedit|p5add_0|p5addn1|p5addn2|p5addn3|p5addn4|p5addn5|p5addp1|p5addp2|p5addp3|p5addp4|p5addp5|p5cmp_0|p5cmpn1|p5cmpn2|p5cmpn3|p5cmpn4|p5cmpn5|p5cmpp1|p5cmpp2|p5cmpp3|p5cmpp4|p5cmpp5|p5divn1|p5divn2|p5divn3|p5divn4|p5divn5|p5divp1|p5divp2|p5divp3|p5divp4|p5divp5|p5gcd_0|p5gcdn1|p5gcdn2|p5gcdn3|p5gcdn4|p5gcdn5|p5gcdp1|p5gcdp2|p5gcdp3|p5gcdp4|p5gcdp5|p5max_0|p5maxn1|p5maxn2|p5maxn3|p5maxn4|p5maxn5|p5maxp1|p5maxp2|p5maxp3|p5maxp4|p5maxp5|p5min_0|p5minn1|p5minn2|p5minn3|p5minn4|p5minn5|p5minp1|p5minp2|p5minp3|p5minp4|p5minp5|p5mul_0|p5muln1|p5muln2|p5muln3|p5muln4|p5muln5|p5mulp1|p5mulp2|p5mulp3|p5mulp4|p5mulp5|p5partialdivn1|p5partialdivn5|p5partialdivp1|p5partialdivp5|p5pow_0|p5powp1|p5powp2|p5powp3|p5powp4|p5powp5|p5remn1|p5remn2|p5remn3|p5remn4|p5remn5|p5remp1|p5remp2|p5remp3|p5remp4|p5remp5|p5sub_0|p5subn1|p5subn2|p5subn3|p5subn4|p5subn5|p5subp1|p5subp2|p5subp3|p5subp4|p5subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
