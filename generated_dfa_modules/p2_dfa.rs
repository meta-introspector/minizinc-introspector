use regex::Regex;

pub fn matches_p2(text: &str) -> bool {
    let pattern = r"^(p20|p223aerbxx4jtwdvaaaafqcthwtcxxlk5z6zvpmfdsdyhzkf2waaaieahp41|p243|p25|p27|p270|p271|p272|p279|p2_extend|p2_k|p2_modif|p2_more|p2_unmodif|p2add_0|p2addn1|p2addn2|p2addn3|p2addn4|p2addn5|p2addp1|p2addp2|p2addp3|p2addp4|p2addp5|p2c|p2c_att|p2c_pos|p2cmp_0|p2cmpn1|p2cmpn2|p2cmpn3|p2cmpn4|p2cmpn5|p2cmpp1|p2cmpp2|p2cmpp3|p2cmpp4|p2cmpp5|p2divn1|p2divn2|p2divn3|p2divn4|p2divn5|p2divp1|p2divp2|p2divp3|p2divp4|p2divp5|p2gcd_0|p2gcdn1|p2gcdn2|p2gcdn3|p2gcdn4|p2gcdn5|p2gcdp1|p2gcdp2|p2gcdp3|p2gcdp4|p2gcdp5|p2max_0|p2maxn1|p2maxn2|p2maxn3|p2maxn4|p2maxn5|p2maxp1|p2maxp2|p2maxp3|p2maxp4|p2maxp5|p2min_0|p2minn1|p2minn2|p2minn3|p2minn4|p2minn5|p2minp1|p2minp2|p2minp3|p2minp4|p2minp5|p2mul_0|p2muln1|p2muln2|p2muln3|p2muln4|p2muln5|p2mulp1|p2mulp2|p2mulp3|p2mulp4|p2mulp5|p2partialdivn1|p2partialdivn2|p2partialdivp1|p2partialdivp2|p2pow_0|p2powp1|p2powp2|p2powp3|p2powp4|p2powp5|p2remn1|p2remn2|p2remn3|p2remn4|p2remn5|p2remp1|p2remp2|p2remp3|p2remp4|p2remp5|p2sub_0|p2subn1|p2subn2|p2subn3|p2subn4|p2subn5|p2subp1|p2subp2|p2subp3|p2subp4|p2subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
