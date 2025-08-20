use regex::Regex;

pub fn matches_cg(text: &str) -> bool {
    let pattern = r"^(cg_space_key|cga|cgaffinetransform|cgb2jm8pwzkeeixq66kbmybr6np61mggl7xusmljvcrw|cgcolorspace|cgcontext|cgdataprovider|cgdisplaybounds|cgdisplaycopydisplaymode|cgdisplaycreateuuidfromdisplayid|cgdisplaymodegetpixelheight|cgdisplaymodegetpixelwidth|cgdisplaymoderelease|cgeyw1h59tabujgj4nr0jdqioulzyqnhnxpiw1rjukznpbfsfkzrelvpocq|cgfloat|cgfont|cggetactivedisplaylist|cgglyph|cgi|cgkeycode|cglocalconst|cgraph|cgrect|cgsize|cgsmainconnectionid|cgssetwindowbackgroundblurradius|cgvde2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
