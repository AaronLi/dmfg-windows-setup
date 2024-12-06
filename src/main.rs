use std::io::BufRead;
use strum::IntoEnumIterator;
mod registry;
mod util;
mod onedrive;

fn main() -> anyhow::Result<()> {
    onedrive::remove_onedrive()
}

