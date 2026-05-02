#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or("".to_string()) == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("knight.ico")
            .set_language(0x0409)
            .set_manifest_file("manifest.xml")
            .set("InternalName", "DUNGEONCRAWL.EXE")
            .set("RegisteredOrganization", "Rip City Technologies")
            .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile().unwrap();
    }
}

#[cfg(unix)]
fn main() {
}