use regex::Regex;

pub fn matches_p4(text: &str) -> bool {
    let pattern = r"^(p43|p4add_0|p4addn1|p4addn2|p4addn3|p4addn4|p4addn5|p4addp1|p4addp2|p4addp3|p4addp4|p4addp5|p4cmp_0|p4cmpn1|p4cmpn2|p4cmpn3|p4cmpn4|p4cmpn5|p4cmpp1|p4cmpp2|p4cmpp3|p4cmpp4|p4cmpp5|p4divn1|p4divn2|p4divn3|p4divn4|p4divn5|p4divp1|p4divp2|p4divp3|p4divp4|p4divp5|p4gcd_0|p4gcdn1|p4gcdn2|p4gcdn3|p4gcdn4|p4gcdn5|p4gcdp1|p4gcdp2|p4gcdp3|p4gcdp4|p4gcdp5|p4max_0|p4maxn1|p4maxn2|p4maxn3|p4maxn4|p4maxn5|p4maxp1|p4maxp2|p4maxp3|p4maxp4|p4maxp5|p4min_0|p4minn1|p4minn2|p4minn3|p4minn4|p4minn5|p4minp1|p4minp2|p4minp3|p4minp4|p4minp5|p4mul_0|p4muln1|p4muln2|p4muln3|p4muln4|p4muln5|p4mulp1|p4mulp2|p4mulp3|p4mulp4|p4mulp5|p4partialdivn1|p4partialdivn2|p4partialdivn4|p4partialdivp1|p4partialdivp2|p4partialdivp4|p4pow_0|p4powp1|p4powp2|p4powp3|p4powp4|p4powp5|p4qkeuzdkitvpyad|p4remn1|p4remn2|p4remn3|p4remn4|p4remn5|p4remp1|p4remp2|p4remp3|p4remp4|p4remp5|p4sub_0|p4subn1|p4subn2|p4subn3|p4subn4|p4subn5|p4subp1|p4subp2|p4subp3|p4subp4|p4subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
