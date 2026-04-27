#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("resources/knight.ico", "1")
        .set("InternalName", "DUNGEONCRAWL.EXE")
        .set("RegisteredOrganization", "Rip City Technologies")
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.set("CompanyName", "Rip City Technologies"); 
    res.set("ProductName", "Dungeon Crawl");
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="asInvoker" uiAccess="true"/>
                </requestedPrivileges>
            </security>
        </trustInfo>
    </assembly>
    "#);
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}