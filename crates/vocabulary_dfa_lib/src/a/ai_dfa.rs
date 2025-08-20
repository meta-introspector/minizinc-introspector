use regex::Regex;

pub fn matches_ai(text: &str) -> bool {
    let pattern = r"^(ai2|ai21|ai21j2grandeinstruct|ai21j2jumboinstruct|ai21j2mid|ai21j2midv1|ai21j2ultra|ai21j2ultrav1|ai21j2ultrav1_8k|ai21jamba15largev1|ai21jamba15miniv1|ai21jambainstructv1|ai64|ai_editor|ai_model_id|ai_placeholder|ai_prefix|ai_reactor|ai_response|ai_tools|aic|aidrop|aie|aiedit|aieditor|aient|aies|aiff|aigle|ailab|ailes|ailuropoda|ailurus|aimez|aiming|aimodel|aimodelcreation|ainternal_features|aio|aip|aiqueryinterface|air_cards|air_output_path|airbnb|aircraft|airdrop_lamports|airdropname|airdropping|airdrops|airdropservice|aireactiongenerator|airedale|aireport|airflow|airforce|airliner|airship|ais|aisql|aistudio|aizasy)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
