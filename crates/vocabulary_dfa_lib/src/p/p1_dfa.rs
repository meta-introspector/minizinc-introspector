use regex::Regex;

pub fn matches_p1(text: &str) -> bool {
    let pattern = r"^(p100|p1024|p125|p12_der|p12_to_ssh_pubkey|p13|p14|p1545|p16|p16_0|p16_1|p16_2|p16_3|p16h|p16l|p175|p18|p1_confirm|p1_k|p1_modif|p1_non_confirm|p1_unmodif|p1aceho1derpubkey11111111111111111111111111|p1add_0|p1addn1|p1addn2|p1addn3|p1addn4|p1addn5|p1addp1|p1addp2|p1addp3|p1addp4|p1addp5|p1cmp_0|p1cmpn1|p1cmpn2|p1cmpn3|p1cmpn4|p1cmpn5|p1cmpp1|p1cmpp2|p1cmpp3|p1cmpp4|p1cmpp5|p1divn1|p1divn2|p1divn3|p1divn4|p1divn5|p1divp1|p1divp2|p1divp3|p1divp4|p1divp5|p1gcd_0|p1gcdn1|p1gcdn2|p1gcdn3|p1gcdn4|p1gcdn5|p1gcdp1|p1gcdp2|p1gcdp3|p1gcdp4|p1gcdp5|p1max_0|p1maxn1|p1maxn2|p1maxn3|p1maxn4|p1maxn5|p1maxp1|p1maxp2|p1maxp3|p1maxp4|p1maxp5|p1min_0|p1minn1|p1minn2|p1minn3|p1minn4|p1minn5|p1minp1|p1minp2|p1minp3|p1minp4|p1minp5|p1mul_0|p1muln1|p1muln2|p1muln3|p1muln4|p1muln5|p1mulp1|p1mulp2|p1mulp3|p1mulp4|p1mulp5|p1partialdivn1|p1partialdivp1|p1pow_0|p1pown1|p1pown2|p1pown3|p1pown4|p1pown5|p1powp1|p1powp2|p1powp3|p1powp4|p1powp5|p1qdbumruo5unow5wbkd66m8p7j|p1remn1|p1remn2|p1remn3|p1remn4|p1remn5|p1remp1|p1remp2|p1remp3|p1remp4|p1remp5|p1sub_0|p1subn1|p1subn2|p1subn3|p1subn4|p1subn5|p1subp1|p1subp2|p1subp3|p1subp4|p1subp5|p1vn1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
