use regex::Regex;

pub fn matches_rx(text: &str) -> bool {
    let pattern = r"^(rx2|rx_approve|rx_bytes|rx_bytes_delta|rx_cake|rx_chunk|rx_clone|rx_clone2|rx_compressed|rx_dora|rx_drops|rx_drops_delta|rx_end|rx_errs|rx_errs_delta|rx_fifo|rx_fifo_delta|rx_frame|rx_frame_delta|rx_from_worker|rx_guard|rx_key|rx_master|rx_multicast|rx_packets|rx_packets_delta|rx_req|rx_resp|rx_script|rx_start|rx_sub|rx_to_main|rxd|rxvt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
