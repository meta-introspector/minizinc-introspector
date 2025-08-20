use regex::Regex;

pub fn matches_n1(text: &str) -> bool {
    let pattern = r"^(n10|n100|n1024|n104|n11|n111|n1111|n12|n123|n125|n15|n16|n19di8zxmdgb7anwdafdtoymrvr|n1add_0|n1addn1|n1addn2|n1addn3|n1addn4|n1addn5|n1addp1|n1addp2|n1addp3|n1addp4|n1addp5|n1cmp_0|n1cmpn1|n1cmpn2|n1cmpn3|n1cmpn4|n1cmpn5|n1cmpp1|n1cmpp2|n1cmpp3|n1cmpp4|n1cmpp5|n1divn1|n1divn2|n1divn3|n1divn4|n1divn5|n1divp1|n1divp2|n1divp3|n1divp4|n1divp5|n1gcd_0|n1gcdn1|n1gcdn2|n1gcdn3|n1gcdn4|n1gcdn5|n1gcdp1|n1gcdp2|n1gcdp3|n1gcdp4|n1gcdp5|n1max_0|n1maxn1|n1maxn2|n1maxn3|n1maxn4|n1maxn5|n1maxp1|n1maxp2|n1maxp3|n1maxp4|n1maxp5|n1min_0|n1minn1|n1minn2|n1minn3|n1minn4|n1minn5|n1minp1|n1minp2|n1minp3|n1minp4|n1minp5|n1mul_0|n1muln1|n1muln2|n1muln3|n1muln4|n1muln5|n1mulp1|n1mulp2|n1mulp3|n1mulp4|n1mulp5|n1partialdivn1|n1partialdivp1|n1pow_0|n1pown1|n1pown2|n1pown3|n1pown4|n1pown5|n1powp1|n1powp2|n1powp3|n1powp4|n1powp5|n1remn1|n1remn2|n1remn3|n1remn4|n1remn5|n1remp1|n1remp2|n1remp3|n1remp4|n1remp5|n1sub_0|n1subn1|n1subn2|n1subn3|n1subn4|n1subn5|n1subp1|n1subp2|n1subp3|n1subp4|n1subp5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
