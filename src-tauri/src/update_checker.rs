//! Update checker for application updates via GitHub Releases.
//!
//! This module provides functionality to check for new versions of the application
//! by querying the GitHub Releases API.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const GITHUB_REPO: &str = "woxqaq/project-zombiod-save-auto-backup";
const GITHUB_API: &str = "https://api.github.com";

/// GitHub release information from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub html_url: String,
    pub body: String,
    pub published_at: String,
    pub prerelease: bool,
}

/// Update check result sent to the frontend.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
    pub published_at: String,
}

/// Checks for updates via GitHub API.
///
/// # Returns
/// `Result<UpdateInfo, String>` - Update information or error message
///
/// # Behavior
/// - Fetches the latest release from GitHub
/// - Compares with current version from Cargo.toml
/// - Skips pre-releases
/// - Returns update info if a newer version is available
pub async fn check_for_updates() -> Result<UpdateInfo, String> {
    let current_version = get_current_version();
    let client = reqwest::Client::builder()
        .user_agent("pz-backup-tool")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("{}/repos/{}/releases/latest", GITHUB_API, GITHUB_REPO);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "GitHub API returned error: {}",
            response.status()
        ));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Skip pre-releases
    if release.prerelease {
        return Ok(UpdateInfo {
            has_update: false,
            current_version,
            latest_version: release.tag_name,
            release_url: release.html_url,
            release_notes: release.body,
            published_at: release.published_at,
        });
    }

    let latest_version = release
        .tag_name
        .strip_prefix('v')
        .unwrap_or(&release.tag_name);

    let has_update = compare_versions(&current_version, latest_version) == Ordering::Less;

    Ok(UpdateInfo {
        has_update,
        current_version,
        latest_version: latest_version.to_string(),
        release_url: release.html_url,
        release_notes: release.body,
        published_at: release.published_at,
    })
}

/// Compares two version strings (semantic versioning).
///
/// # Arguments
/// * `current` - Current version string
/// * `latest` - Latest version string
///
/// # Returns
/// `Ordering` - Less if current < latest, Greater if current > latest, Equal if same
fn compare_versions(current: &str, latest: &str) -> Ordering {
    let current_parts: Vec<&str> = current.split('.').collect();
    let latest_parts: Vec<&str> = latest.split('.').collect();

    let max_len = current_parts.len().max(latest_parts.len());

    for i in 0..max_len {
        let current = current_parts.get(i).and_then(|s| s.parse::<u32>().ok());
        let latest = latest_parts.get(i).and_then(|s| s.parse::<u32>().ok());

        match (current, latest) {
            (Some(c), Some(l)) => {
                if c != l {
                    return c.cmp(&l);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => continue,
        }
    }

    Ordering::Equal
}

/// Gets the current application version from Cargo.toml.
///
/// # Returns
/// `String` - Current version (e.g., "0.1.0")
pub fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_versions_equal() {
        assert_eq!(compare_versions("1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_versions("0.1.0", "0.1.0"), Ordering::Equal);
    }

    #[test]
    fn test_compare_versions_less() {
        assert_eq!(compare_versions("0.1.0", "0.2.0"), Ordering::Less);
        assert_eq!(compare_versions("1.0.0", "1.0.1"), Ordering::Less);
        assert_eq!(compare_versions("0.1.0", "1.0.0"), Ordering::Less);
    }

    #[test]
    fn test_compare_versions_greater() {
        assert_eq!(compare_versions("0.2.0", "0.1.0"), Ordering::Greater);
        assert_eq!(compare_versions("1.0.1", "1.0.0"), Ordering::Greater);
        assert_eq!(compare_versions("1.0.0", "0.1.0"), Ordering::Greater);
    }

    #[test]
    fn test_compare_versions_different_lengths() {
        assert_eq!(compare_versions("1.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.0.0", "1.0"), Ordering::Equal);
        assert_eq!(compare_versions("1.0", "1.0.1"), Ordering::Less);
        assert_eq!(compare_versions("1.0.1", "1.0"), Ordering::Greater);
    }

    #[test]
    fn test_get_current_version() {
        let version = get_current_version();
        // Version should be in format x.y.z
        assert!(version.contains('.'));
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);
        parts.iter().for_each(|p| {
            p.parse::<u32>().expect("Version part should be a number");
        });
    }
}
