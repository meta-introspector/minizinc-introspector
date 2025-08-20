use regex::Regex;

pub fn matches_ii(text: &str) -> bool {
    let pattern = r"^(ii_1nddbo2ezvkylo2c2n5wbkaj|ii_at|ii_buffer|ii_path|ii_ty|ii_ty_span|iiaaa|iibuildstate|iieu|iiiin|iiiint|iiin|iiint|iill|iimsn|iin|iinf|iinfi|iinfin|iinm|iio|iiot|iiota|iiref|iirfilter|iitem|iittxm5a83mt9730hb8xim7gyeju47kiesw2dan8vnj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
