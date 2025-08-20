use regex::Regex;

pub fn matches_od(text: &str) -> bool {
    let pattern = r"^(oda|odas|odash|odbl|odbla|odblac|odd_definition|odd_extension|odd_fft|odd_field|odd_half_spin_repr|odd_number_of_pairs|odd_ones|odd_shreds|odd_slots|odd_spin_repr|oddball|oddlength|oddlensig|oddness|ode|oder|odi|odic|odin|odio|odir|odiv|odo|odoc|odometer|odot|odp|odr|ods|odso|odsol|odsold|odt|odyssey_01_homer_128kb|odyssey_2409_librivox)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
