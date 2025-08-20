use regex::Regex;

pub fn matches_n5(text: &str) -> bool {
    let pattern = r"^(n52|n555|n5555|n5678|n56789|n5add_0|n5addn1|n5addn2|n5addn3|n5addn4|n5addn5|n5addp1|n5addp2|n5addp3|n5addp4|n5addp5|n5cmp_0|n5cmpn1|n5cmpn2|n5cmpn3|n5cmpn4|n5cmpn5|n5cmpp1|n5cmpp2|n5cmpp3|n5cmpp4|n5cmpp5|n5divn1|n5divn2|n5divn3|n5divn4|n5divn5|n5divp1|n5divp2|n5divp3|n5divp4|n5divp5|n5gcd_0|n5gcdn1|n5gcdn2|n5gcdn3|n5gcdn4|n5gcdn5|n5gcdp1|n5gcdp2|n5gcdp3|n5gcdp4|n5gcdp5|n5max_0|n5maxn1|n5maxn2|n5maxn3|n5maxn4|n5maxn5|n5maxp1|n5maxp2|n5maxp3|n5maxp4|n5maxp5|n5min_0|n5minn1|n5minn2|n5minn3|n5minn4|n5minn5|n5minp1|n5minp2|n5minp3|n5minp4|n5minp5|n5mul_0|n5muln1|n5muln2|n5muln3|n5muln4|n5muln5|n5mulp1|n5mulp2|n5mulp3|n5mulp4|n5mulp5|n5partialdivn1|n5partialdivn5|n5partialdivp1|n5partialdivp5|n5pow_0|n5powp1|n5powp2|n5powp3|n5powp4|n5powp5|n5remn1|n5remn2|n5remn3|n5remn4|n5remn5|n5remp1|n5remp2|n5remp3|n5remp4|n5remp5|n5sub_0|n5subn1|n5subn2|n5subn3|n5subn4|n5subn5|n5subp1|n5subp2|n5subp3|n5subp4|n5subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
