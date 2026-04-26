#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("resources/knight.ico", "1")
        .set("InternalName", "DUNGEONCRAWL.EXE")
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}