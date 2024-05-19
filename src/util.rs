pub fn shorten_lv(path: &str) -> String {
    const MARK: &str = "#";

    if path.starts_with("/dev/mapper/") {
        if let Some(lv) = path.split('/').nth(3) {
            let lv = lv.replace("--", MARK);
            let lv_vg: Vec<String> = lv.split('-').map(|x| x.replace(MARK, "-")).collect();
            return format!("/dev/{}/{}", lv_vg[0], lv_vg[1]);
        }
    }

    path.to_string()
}
