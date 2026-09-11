pub enum RemoteDirectory {
    Root,
    Libraries,
    Fonts,
    Trophy,
    Custom(String),
}

impl RemoteDirectory {
    pub fn path(&self) -> &str {
        match self {
            RemoteDirectory::Root => "/",
            RemoteDirectory::Libraries => "/system/common/lib/",
            RemoteDirectory::Fonts => "",  // TBA
            RemoteDirectory::Trophy => "", // TBA
            RemoteDirectory::Custom(path) => path,
        }
    }
}