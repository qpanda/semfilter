use anyhow::Error;

use crate::filter::Settings;

//
// Windows specific code
//

#[cfg(target_family = "windows")]
use anyhow::anyhow;

#[cfg(target_family = "windows")]
pub fn enable_ansi_support(settings: &Settings) -> Result<(), Error> {
    if settings.mode.is_highlight() {
        match ansi_term::enable_ansi_support() {
            Ok(()) => return Ok(()),
            Err(error) => return Err(anyhow!("enabling failed with error code {}", error)),
        }
    }

    Ok(())
}

//
// Unix specific code
//

#[cfg(target_family = "unix")]
pub fn enable_ansi_support(settings: &Settings) -> Result<(), Error> {
    if settings.mode.is_highlight() {
        // ANSI support needs to be enabled only on Windows
    }

    Ok(())
}
