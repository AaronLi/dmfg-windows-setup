use winreg::RegKey;
use crate::util::UserFolders;

pub fn registry_write_permission_check(folder_path: &RegKey) -> anyhow::Result<()> {
    let current_value = folder_path.get_value::<String, _>(UserFolders::Music.reg_key())?;

    Ok(folder_path.set_value(UserFolders::Music.reg_key(), &current_value)?)
}