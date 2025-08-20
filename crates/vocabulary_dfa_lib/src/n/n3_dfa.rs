use regex::Regex;

pub fn matches_n3(text: &str) -> bool {
    let pattern = r"^(n3125|n32|n333|n3333|n3add_0|n3addn1|n3addn2|n3addn3|n3addn4|n3addn5|n3addp1|n3addp2|n3addp3|n3addp4|n3addp5|n3cmp_0|n3cmpn1|n3cmpn2|n3cmpn3|n3cmpn4|n3cmpn5|n3cmpp1|n3cmpp2|n3cmpp3|n3cmpp4|n3cmpp5|n3d26crxd99tptm8uo2ouzkhsiq6eq|n3divn1|n3divn2|n3divn3|n3divn4|n3divn5|n3divp1|n3divp2|n3divp3|n3divp4|n3divp5|n3gcd_0|n3gcdn1|n3gcdn2|n3gcdn3|n3gcdn4|n3gcdn5|n3gcdp1|n3gcdp2|n3gcdp3|n3gcdp4|n3gcdp5|n3max_0|n3maxn1|n3maxn2|n3maxn3|n3maxn4|n3maxn5|n3maxp1|n3maxp2|n3maxp3|n3maxp4|n3maxp5|n3min_0|n3minn1|n3minn2|n3minn3|n3minn4|n3minn5|n3minp1|n3minp2|n3minp3|n3minp4|n3minp5|n3mul_0|n3muln1|n3muln2|n3muln3|n3muln4|n3muln5|n3mulp1|n3mulp2|n3mulp3|n3mulp4|n3mulp5|n3partialdivn1|n3partialdivn3|n3partialdivp1|n3partialdivp3|n3pow_0|n3powp1|n3powp2|n3powp3|n3powp4|n3powp5|n3remn1|n3remn2|n3remn3|n3remn4|n3remn5|n3remp1|n3remp2|n3remp3|n3remp4|n3remp5|n3sub_0|n3subn1|n3subn2|n3subn3|n3subn4|n3subn5|n3subp1|n3subp2|n3subp3|n3subp4|n3subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
