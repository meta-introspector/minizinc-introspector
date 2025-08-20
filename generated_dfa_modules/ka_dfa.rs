use regex::Regex;

pub fn matches_ka(text: &str) -> bool {
    let pattern = r"^(ka|kacaxb2sgetlbk8d1dcomcggeavypafwsljiid0hynjmx0qesxsfbt|kahan_summation_algorithm|kaiming|kakatoe|kale|kalosm_0_2|kalosm_learning|kalosm_learning_macro|kalosm_parse_macro|kan|kangaroo|kanjinumeric|kanjis|kann|kannada|kansas|kanssa|kap|kapacitor|kapp|kappa_params_file|kappa_params_path|kappa_params_version|kappav|karatsuba|kargs_path|karma|karras|karrassigmaschedule|kart|katanemo|katharostech|katherine|kaudiodevicepropertystreamformat|kaudiohardwarepropertydefaultinputdevice|kaudiohardwarepropertydefaultoutputdevice|kaudioobjectpropertyelementmaster|kaudioobjectpropertyscopeglobal|kaudioobjectpropertyscopeinput|kaudioobjectpropertyscopeoutput|kaudioobjectsystemobject|kaufen|kaum|kawamata|kawamata_log_terminal_threshold|kazakh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
