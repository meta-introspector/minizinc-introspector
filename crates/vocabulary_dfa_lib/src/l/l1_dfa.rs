use regex::Regex;

pub fn matches_l1(text: &str) -> bool {
    let pattern = r"^(l1013|l103|l105|l10605|l1063|l1067|l1096|l1101|l1103|l1113|l1116|l1123|l115|l116|l118|l120|l1200|l1204|l121|l1217|l1233|l1238|l1239|l1245|l1258|l126|l128|l1288|l13|l130|l133|l1335|l134|l135|l1350|l136|l1362|l139|l1391|l13b|l13bchat|l13bcode|l14|l140|l1403|l141|l1429|l1446|l145|l1454|l147|l148|l1487|l1510|l1525|l1538|l1539|l1540|l1541|l1544|l1545|l1566|l1576|l1592|l160|l161|l1619|l164|l167|l1690|l1692|l16c1|l171|l174|l176|l178|l18|l181|l184|l1881|l189|l19|l193|l194|l195|l198|l1_memberships|l1_reg|l1r_l2loss_svc|l1r_lr|l1regularizedl2losssvc|l1regularizedlogistic)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
