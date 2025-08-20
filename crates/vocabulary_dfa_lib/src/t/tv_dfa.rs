use regex::Regex;

pub fn matches_tv(text: &str) -> bool {
    let pattern = r"^(tv_and_video|tv_nsec|tv_sec|tvaluereader|tvaluewriter|tvc_activation_epoch|tvc_activation_slot|tvcf6b1trz353zkuhbjinzkkzjmihxmbahjdjnyw1sq|tvisible_name|tvmonitor|tvos|tvu_config|tvu_filter|tvu_peers1|tvu_port|tvu_protocols|tvu_quic_port|tvu_receive_threads|tvu_retransmit_threads|tvu_shred_sigverify_threads|tvu_sigverify_threads|tvu_sockets|tvu_threads|tvu_udp|tvuconfig|tvureceivethreadsarg|tvuretransmitthreadsarg|tvushredsigverifythreadsarg|tvusockets)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
