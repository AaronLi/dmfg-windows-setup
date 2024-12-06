use crate::registry;
use crate::util::UserFolders;
use anyhow::Error;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};
use strum::IntoEnumIterator;
use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::RegKey;

pub fn remove_onedrive() -> anyhow::Result<()> {
    try_uninstall_onedrive()?;
    try_undo_onedrive_library_overrides()
}

fn try_uninstall_onedrive() -> anyhow::Result<()> {
    let mut uninstall_result = Command::new("winget")
        .arg("uninstall")
        .arg("Microsoft.OneDrive")
        .stdin(Stdio::null())
        // .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let error_code = uninstall_result.wait()?;

    if error_code.success() {
        Ok(())
    }else {
        let stdout_lines = BufReader::new(uninstall_result.stdout.unwrap());

        let mut onedrive_not_installed = false;
        for line in stdout_lines.lines() {
            if line?.ends_with("No installed package found matching input criteria.") {
                onedrive_not_installed = true;
            }
        }

        if onedrive_not_installed {
            Ok(())
        } else {
            Err(Error::msg("Onedrive appears to be installed but could not be uninstalled via winget"))
        }
    }
}

fn try_undo_onedrive_library_overrides() -> anyhow::Result<()> {
    let parent_predef = RegKey::predef(HKEY_CURRENT_USER);

    let shell_folders = parent_predef.open_subkey_with_flags(PathBuf::from(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders"), KEY_ALL_ACCESS)?;

    let current_user_profile = PathBuf::from(env::var("USERPROFILE")?);

    registry::registry_write_permission_check(&shell_folders).map_err(|_|Error::msg("Caller does not have permission to write registry keys"))?;

    for v in UserFolders::iter() {
        let _ = shell_folders.get_value::<String, _>(v.reg_key())?;
    }

    // all values present


    for library in UserFolders::iter() {
        let current_registry_value = shell_folders.get_value::<String, _>(library.reg_key())?;
        if library.default_reg_value() != current_registry_value {
            let actual_path = library.path(&current_user_profile);
            if !fs::exists(&actual_path)? {
                fs::create_dir(&actual_path)?;
                println!("Created {}", actual_path.display());
            }

            shell_folders.set_value(library.reg_key(), &library.default_reg_value())?;
        }
    }
    println!("Reset OneDrive library paths");

    for library in UserFolders::iter() {
        if let Ok(_existing) = shell_folders.get_value::<String, _>(library.overrides_entries()) {
            shell_folders.delete_value(library.overrides_entries())?;
        }
    }
    println!("Removed OneDrive overrides");

    Ok(())
}