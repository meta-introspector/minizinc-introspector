use regex::Regex;

pub fn matches_n2(text: &str) -> bool {
    let pattern = r"^(n20|n219|n220|n221|n230|n231|n243|n25|n265|n27|n291|n2add_0|n2addn1|n2addn2|n2addn3|n2addn4|n2addn5|n2addp1|n2addp2|n2addp3|n2addp4|n2addp5|n2cmp_0|n2cmpn1|n2cmpn2|n2cmpn3|n2cmpn4|n2cmpn5|n2cmpp1|n2cmpp2|n2cmpp3|n2cmpp4|n2cmpp5|n2divn1|n2divn2|n2divn3|n2divn4|n2divn5|n2divp1|n2divp2|n2divp3|n2divp4|n2divp5|n2gcd_0|n2gcdn1|n2gcdn2|n2gcdn3|n2gcdn4|n2gcdn5|n2gcdp1|n2gcdp2|n2gcdp3|n2gcdp4|n2gcdp5|n2max_0|n2maxn1|n2maxn2|n2maxn3|n2maxn4|n2maxn5|n2maxp1|n2maxp2|n2maxp3|n2maxp4|n2maxp5|n2min_0|n2minn1|n2minn2|n2minn3|n2minn4|n2minn5|n2minp1|n2minp2|n2minp3|n2minp4|n2minp5|n2mul_0|n2muln1|n2muln2|n2muln3|n2muln4|n2muln5|n2mulp1|n2mulp2|n2mulp3|n2mulp4|n2mulp5|n2partialdivn1|n2partialdivn2|n2partialdivp1|n2partialdivp2|n2pow_0|n2powp1|n2powp2|n2powp3|n2powp4|n2powp5|n2remn1|n2remn2|n2remn3|n2remn4|n2remn5|n2remp1|n2remp2|n2remp3|n2remp4|n2remp5|n2sub_0|n2subn1|n2subn2|n2subn3|n2subn4|n2subn5|n2subp1|n2subp2|n2subp3|n2subp4|n2subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
