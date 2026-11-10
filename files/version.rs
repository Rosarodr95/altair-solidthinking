//! Static build info for Altair Solidthinking.

pub const BUILD_VERSION: &str  = "v1.5.3";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "34cd7bd4d742";
pub const BUILD_TIMESTAMP: u64 = 1731054404;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Altair Solidthinking", BUILD_VERSION, BUILD_CHANNEL)
}
