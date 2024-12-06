use std::path::{Path, PathBuf};
use strum::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum UserFolders {
    Desktop,
    Music,
    Pictures,
    Video,
    Documents
}

impl UserFolders {
    pub(crate) fn reg_key(&self) -> &'static str {
        match self {
            UserFolders::Desktop => "Desktop",
            UserFolders::Music => "My Music",
            UserFolders::Pictures => "My Pictures",
            UserFolders::Video => "My Video",
            UserFolders::Documents => "Personal"
        }
    }

    pub(crate) fn default_reg_value(&self) -> String {
        self.path(&PathBuf::from(r"%USERPROFILE%\")).display().to_string()
    }

    pub(crate) fn path(&self, user_profile: &Path) -> PathBuf {
        user_profile.join(match self {
            UserFolders::Desktop => "Desktop",
            UserFolders::Music => "Music",
            UserFolders::Pictures => "Pictures",
            UserFolders::Video => "Videos",
            UserFolders::Documents => "Documents"
        })
    }

    pub(crate) fn overrides_entries(&self) -> &'static str {
        match self {
            UserFolders::Desktop => "{754AC886-DF64-4CBA-86B5-F7FBF4FBCEF5}",
            UserFolders::Music => "{A0C69A99-21C8-4671-8703-7934162FCF1D}",
            UserFolders::Pictures => "{0DDD015D-B06C-45D5-8C4C-F59713854639}",
            UserFolders::Video => "{35286A68-3C57-41A1-BBB1-0EAE73D76C95}",
            UserFolders::Documents => "{F42EE2D3-909F-4907-8871-4C22FC0BF756}"
        }
    }
}