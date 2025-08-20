use regex::Regex;

pub fn matches_gc(text: &str) -> bool {
    let pattern = r"^(gc8xnhu6nnriwt9rbewi7ptosx4yanlyxak9gtbb8vah|gc_audit|gc_calls|gc_collect|gc_collection|gc_dep|gc_drop_mut|gc_grow_factor|gc_images|gc_interval|gc_keep_used_patch|gc_len|gc_logs|gc_loop|gc_maybe|gc_metadata|gc_not_required|gc_opts|gc_patch|gc_profile|gc_replace|gc_shadowed_roots|gc_sources|gc_thread|gc_threshold|gc_unused_patches|gc_visit_children|gc_workspace|gca|gcancellable|gccgoflags|gccllvmsomething|gcda|gcda_files|gcda_path|gced|gcedi|gcedil|gcflags|gcgdjwjrtgcbsjlbdpjahsrzp7flc6t3y9x5tdt0vzgo6vxpilmw|gch|gchar|gcir|gcirc|gclient|gcmetadata|gcno|gcno_files|gcno_path|gcopts|gcov|gcp_backoff_multiplier|gcp_hostname|gcp_initial_retry_interval_ms|gcp_ip_external_0|gcp_ip_local_0|gcp_location|gcp_machine_type|gcp_max_overloaded_retries|gcp_max_rate_limit_retries|gcp_max_retries|gcp_max_retry_interval_ms|gcp_project_id|gcp_vertex_ai_doc_url|gcplocation|gcpointertype|gcpvertexaierror|gcpvertexaimodel|gcs_compstr|gcs_cursorpos|gcs_eternal_ai_base_url|gcs_resultstr|gcy|gczjb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
