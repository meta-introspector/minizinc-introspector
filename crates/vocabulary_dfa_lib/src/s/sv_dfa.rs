use regex::Regex;

pub fn matches_sv(text: &str) -> bool {
    let pattern = r"^(svar|svbr|svc_arc|svc_key|svc_port|svc_target_port|svclassify|svd_decomposition|svd_flip|svd_flip_1d|sve|sve2|sve2p1|svector|sven|svg__mime|svg_builder|svg_path|svg_size|svgattributes|svgopenpreview|svgopenpreviewtotheside|svgpathbuilder|svgpreview|svgpreviewmode|svgsize|svlint|svm_bridge|svm_concurrent|svm_inspect_account|svm_integration|svm_metrics_accumulation|svm_output|svm_transactions|svmbase|svmtestentry|svmtestenvironment|svmvalidparams|svn36yvapplysa8kok3qucy14zxdnqknywyuh1f4ok1|svregress|svt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
