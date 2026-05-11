use crate::system_profile::BootProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub profile: BootProfile,
    pub verbose: bool,
    pub self_check: bool,
}

fn parse_profile(raw: &str) -> BootProfile {
    match raw.to_lowercase().as_str() {
        "safe" => BootProfile::Safe,
        "recovery" => BootProfile::Recovery,
        _ => BootProfile::Normal,
    }
}

fn parse_verbose(raw: &str) -> bool {
    matches!(raw, "1" | "true" | "TRUE" | "yes" | "YES")
}

fn parse_args(args: &[String]) -> (Option<BootProfile>, Option<bool>, bool) {
    let mut profile = None;
    let mut verbose = None;
    let mut self_check = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" if i + 1 < args.len() => {
                profile = Some(parse_profile(&args[i + 1]));
                i += 1;
            }
            "--verbose" => {
                verbose = Some(true);
            }
            "--quiet" => {
                verbose = Some(false);
            }
            "--self-check" => {
                self_check = true;
            }
            _ => {}
        }
        i += 1;
    }

    (profile, verbose, self_check)
}

impl RuntimeConfig {
    pub fn from_env_and_args(args: &[String]) -> Self {
        let profile_raw =
            std::env::var("RUST_LINUX_PROFILE").unwrap_or_else(|_| "normal".to_string());
        let verbose_raw = std::env::var("RUST_LINUX_VERBOSE").unwrap_or_else(|_| "0".to_string());

        let mut profile = parse_profile(&profile_raw);
        let mut verbose = parse_verbose(&verbose_raw);

        let (arg_profile, arg_verbose, self_check) = parse_args(args);
        if let Some(p) = arg_profile {
            profile = p;
        }
        if let Some(v) = arg_verbose {
            verbose = v;
        }

        Self {
            profile,
            verbose,
            self_check,
        }
    }

    pub fn from_env() -> Self {
        Self::from_env_and_args(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_profile() {
        assert_eq!(parse_profile("normal"), BootProfile::Normal);
        assert_eq!(parse_profile("safe"), BootProfile::Safe);
        assert_eq!(parse_profile("recovery"), BootProfile::Recovery);
        assert_eq!(parse_profile("weird"), BootProfile::Normal);
    }

    #[test]
    fn parser_verbose() {
        assert!(parse_verbose("1"));
        assert!(parse_verbose("true"));
        assert!(!parse_verbose("0"));
    }

    #[test]
    fn cli_overrides_and_self_check_flag() {
        let args = vec![
            "--profile".to_string(),
            "safe".to_string(),
            "--verbose".to_string(),
            "--self-check".to_string(),
        ];
        let cfg = RuntimeConfig::from_env_and_args(&args);
        assert_eq!(cfg.profile, BootProfile::Safe);
        assert!(cfg.verbose);
        assert!(cfg.self_check);
    }
}
