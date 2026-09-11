#[derive(Debug)]
pub struct DLC {
    pub id: String,

    // Installed ac.pkg
    pub package_path: String,

    // Present only if decrypted DLC is currently mounted
    pub mounted_path: Option<String>,
}