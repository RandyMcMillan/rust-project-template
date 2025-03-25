use std::env;
//#[cfg_attr(nightly, feature(nightly))]
#[cfg(feature = "nightly")]
use lazy_static::lazy_static;
#[cfg(feature = "nightly")]
lazy_static! {
    static ref BINARY_NAME: String = {
        match env::var("USER") {
            Ok(name) => name,
            Err(_) => "rustup-project-default".to_string(), // Provide a default value
        }
    };
}

#[cfg(not(feature = "nightly"))]
static BINARY_NAME: &str = "not-nightly";

fn get_binary_name() -> String {
    match env::var("CARGO_BIN_NAME") {
        Ok(name) => name,
        Err(_) => "rust-project-template-default".to_string(), // Provide a default value
    }
}

pub fn config_path(file: &str) -> String {
    match std::env::consts::OS {
        "linux" | "macos" => format!(
            "{}/.config/{}/{}",
            std::env::var("HOME").unwrap(),
            //get_binary_name(),
            //*BINARY_NAME+"bin_name",
            //"project-template-".to_owned()+&BINARY_NAME.clone()+"-bin_name",
            "project-template-".to_owned()+&BINARY_NAME.clone()+"-bin_name",
            file
        ),
        "windows" => format!(
            "{}\\{}\\{}",
            std::env::var("APPDATA").unwrap(),
            get_binary_name(),
            file
        ),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_config_path() {
        assert_eq!(
            config_path("config.toml"),
            format!(
                "{}\\{}\\config.toml",
                std::env::var("APPDATA").unwrap(),
                BINARY_NAME
            )
        );
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn test_unix_config_path() {
        assert_eq!(
            config_path("config.toml"),
            format!(
                "{}/.config/{}/config.toml",
                std::env::var("HOME").unwrap(),
                BINARY_NAME,
            )
        );
    }

    #[test]
    #[should_panic]
    #[cfg(any(
        target_os = "ios",
        target_os = "android",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    fn test_ios_config_path() {
        config_path("config.toml");
    }
}
