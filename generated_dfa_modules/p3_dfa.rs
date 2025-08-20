use regex::Regex;

pub fn matches_p3(text: &str) -> bool {
    let pattern = r"^(p3125|p32|p3add_0|p3addn1|p3addn2|p3addn3|p3addn4|p3addn5|p3addp1|p3addp2|p3addp3|p3addp4|p3addp5|p3cmp_0|p3cmpn1|p3cmpn2|p3cmpn3|p3cmpn4|p3cmpn5|p3cmpp1|p3cmpp2|p3cmpp3|p3cmpp4|p3cmpp5|p3divn1|p3divn2|p3divn3|p3divn4|p3divn5|p3divp1|p3divp2|p3divp3|p3divp4|p3divp5|p3gcd_0|p3gcdn1|p3gcdn2|p3gcdn3|p3gcdn4|p3gcdn5|p3gcdp1|p3gcdp2|p3gcdp3|p3gcdp4|p3gcdp5|p3max_0|p3maxn1|p3maxn2|p3maxn3|p3maxn4|p3maxn5|p3maxp1|p3maxp2|p3maxp3|p3maxp4|p3maxp5|p3min_0|p3minn1|p3minn2|p3minn3|p3minn4|p3minn5|p3minp1|p3minp2|p3minp3|p3minp4|p3minp5|p3mul_0|p3muln1|p3muln2|p3muln3|p3muln4|p3muln5|p3mulp1|p3mulp2|p3mulp3|p3mulp4|p3mulp5|p3partialdivn1|p3partialdivn3|p3partialdivp1|p3partialdivp3|p3pow_0|p3powp1|p3powp2|p3powp3|p3powp4|p3powp5|p3remn1|p3remn2|p3remn3|p3remn4|p3remn5|p3remp1|p3remp2|p3remp3|p3remp4|p3remp5|p3sub_0|p3subn1|p3subn2|p3subn3|p3subn4|p3subn5|p3subp1|p3subp2|p3subp3|p3subp4|p3subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
