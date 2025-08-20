use regex::Regex;

pub fn matches_ec(text: &str) -> bool {
    let pattern = r"^(ec2_endpoint_for|ec2_url|ec2instancemetadata|ec4rs|ec6wiaqduvc5azzpq4gaz6glsrjvw9mavwvrcpdogarm|ec_error|ec_param|ec_parameters|ec_params|ec_point|ec_token|eca6zf6jjrjqsyypkbhf3n32mtzur4n2wl4qiiacpcl|ecar|ecaro|ecaron|ecdh|ecdsa_nistp256_sha256|ecdsa_nistp384_sha384|ecdsa_nistp521_sha512|ecdsa_sha1_legacy|ecdsapublickey|ecdsasig|ecf_default|ecgroup|echelon|echidna|echo2|echo3|echo_ai|echo_app|echo_handler|echo_output|echo_script|echo_value|echochambererror|echochambers|echochambers_agent|echochambers_api_key|echoed|echoresponse|eci|ecir|ecirc|eckey|eclosure|ecm0359818|ecmyluxezattgckj4igao14c1c6ovr5uwulbgftvxrcz|ecoimpl|ecol|ecolo|ecology|ecolon|ecommerce|econnreset|economic|economicsociallayercomponents|ecoutez|ecparam|ecparameters|ecparams|ecpoint|ecpointempty|ecprivatekey|ecpublickey|ecr|ecs_enabled|ecs_provider|ecscontainer|ecscredentialsprovider|ecsecstoken|ectwwjftapkfuzfe|ecy|ecznx8cqvu3oqog)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
