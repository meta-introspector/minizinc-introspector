use regex::Regex;

pub fn matches_nc(text: &str) -> bool {
    let pattern = r"^(nc0|nc2|nc3|nc_button_pressed|nc_dim|nc_shape|nc_str|nca|ncached|ncalculation|ncancelling|ncandidates|ncap|ncar|ncaro|ncaron|ncategory|ncb|nccalcsize|nccalcsize_params|nccc|nccc123456eee|ncccc|nccccc|ncccccˇ|nccl_id|nccˇ|nccˇc|nce|nced|ncedi|ncedil|nchain|nchars|nchecksum|nchidlren|nchild|nchpwq|nchunk|nclap|nclassifications|ncleared|nclf|nclient|nclose|nclosing|nclusters|ncname|ncode|ncodes|ncoerced|ncollated|ncolor|ncols_padded|ncols_x|ncols_y|ncombined|ncombinedcfgs|ncommand|ncommission|ncomp0|ncomp1|ncompany|ncon|nconfigs|nconfiguration|nconfigure|ncong|ncongd|ncongdo|ncongdot|nconsole|nconversation|nconverting|ncool|ncorrect|ncould|ncpu|ncrates|ncreate|ncredits|ncsubseq|nction|ncu|ncucumber|ncup|ncursesw|ncursor|ncy|ncˇc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
