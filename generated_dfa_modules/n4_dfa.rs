use regex::Regex;

pub fn matches_n4(text: &str) -> bool {
    let pattern = r"^(n40|n42|n444|n4444|n48|n4add_0|n4addn1|n4addn2|n4addn3|n4addn4|n4addn5|n4addp1|n4addp2|n4addp3|n4addp4|n4addp5|n4cmp_0|n4cmpn1|n4cmpn2|n4cmpn3|n4cmpn4|n4cmpn5|n4cmpp1|n4cmpp2|n4cmpp3|n4cmpp4|n4cmpp5|n4divn1|n4divn2|n4divn3|n4divn4|n4divn5|n4divp1|n4divp2|n4divp3|n4divp4|n4divp5|n4gcd_0|n4gcdn1|n4gcdn2|n4gcdn3|n4gcdn4|n4gcdn5|n4gcdp1|n4gcdp2|n4gcdp3|n4gcdp4|n4gcdp5|n4max_0|n4maxn1|n4maxn2|n4maxn3|n4maxn4|n4maxn5|n4maxp1|n4maxp2|n4maxp3|n4maxp4|n4maxp5|n4min_0|n4minn1|n4minn2|n4minn3|n4minn4|n4minn5|n4minp1|n4minp2|n4minp3|n4minp4|n4minp5|n4mul_0|n4muln1|n4muln2|n4muln3|n4muln4|n4muln5|n4mulp1|n4mulp2|n4mulp3|n4mulp4|n4mulp5|n4partialdivn1|n4partialdivn2|n4partialdivn4|n4partialdivp1|n4partialdivp2|n4partialdivp4|n4pow_0|n4powp1|n4powp2|n4powp3|n4powp4|n4powp5|n4remn1|n4remn2|n4remn3|n4remn4|n4remn5|n4remp1|n4remp2|n4remp3|n4remp4|n4remp5|n4sub_0|n4subn1|n4subn2|n4subn3|n4subn4|n4subn5|n4subp1|n4subp2|n4subp3|n4subp4|n4subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
