use regex::Regex;

pub fn matches_ib(text: &str) -> bool {
    let pattern = r"^(ibaction|ibactionattr|ibeamcursor|ibeamcursorforverticallayout|ibex|ibg5bo|ibindctx|ibizan|iblank|ibmcloud_classic|ibmcloud_classic_instance_id|ibmcloud_classic_local_hostname|ibmcloud_instance_id|ibmcloud_local_hostname|iboutlet|iboutletattr|iboutletcollection|iboutletcollectionattr|ibrl|ibus)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
