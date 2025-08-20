use regex::Regex;

pub fn matches_tc(text: &str) -> bool {
    let pattern = r"^(tc1|tc26|tc_name|tc_process|tc_result|tc_vers|tca|tcar|tcaro|tcaron|tcase|tce|tced|tcedi|tcedil|tcftyperef|tcgetpgrp|tck|tcl|tco_jump_handler|tcojmp|tcolumncodec|tcomplexscorecombiner|tconst|tconstr|tcopy|tcopying|tcorrect|tcp_accept|tcp_accept_addr|tcp_accept_addr_definition|tcp_accept_definition|tcp_arguments|tcp_buffered_output_port|tcp_buffered_output_port_definition|tcp_close|tcp_close_definition|tcp_connect|tcp_connect_definition|tcp_deferaccept|tcp_input_port|tcp_input_port_definition|tcp_linger|tcp_listen|tcp_listen_definition|tcp_listener_set_non_blocking|tcp_listener_set_non_blocking_definition|tcp_listeners|tcp_nodelay_listener|tcp_output_port|tcp_output_port_definition|tcp_port|tcp_ports|tcp_quickack|tcp_reader|tcp_retries|tcp_stream_set_non_blocking|tcp_stream_set_non_blocking_definition|tcp_writer|tcpcounter|tcpfilterorpolicy|tcplistener_into_incoming|tcproute|tcproutebackend|tcproutebackendreference|tcproutes|tcprouteset|tcpstreamsplitter|tcustomscorer|tcustomsegmentscorer|tcy|tcycles)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
