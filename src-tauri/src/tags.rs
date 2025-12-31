//! Tag management system for Project Zomboid save backup/restore.
//!
//! This module provides:
//! - Tag data structures (name, color)
//! - Tag association with backups and saves
//! - Tag database persistence (JSON format)
//! - Tag CRUD operations

use crate::config::{get_config_dir, ConfigError};
use crate::file_ops::FileOpsError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Tag database file name.
const TAGS_DB_FILE_NAME: &str = "tags.json";

/// Tag data structure with name and color.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    /// Tag name (unique identifier)
    pub name: String,
    /// Tag color (hex color code like #FF5733)
    pub color: String,
}

/// Tag association type for different targets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum TagTarget {
    /// Backup tag association
    Backup { save_name: String, backup_name: String },
    /// Save tag association
    Save { relative_path: String },
}

/// Tag database containing all tags and associations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsDatabase {
    /// All defined tags (unique by name)
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Tag associations with targets
    #[serde(default)]
    pub associations: Vec<TagAssociation>,
}

impl Default for TagsDatabase {
    fn default() -> Self {
        TagsDatabase {
            tags: Vec::new(),
            associations: Vec::new(),
        }
    }
}

/// Tag association linking targets to tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagAssociation {
    /// Target object (backup or save)
    pub target: TagTarget,
    /// Associated tag names
    #[serde(default)]
    pub tag_names: Vec<String>,
}

/// Error type for tag operations.
#[derive(Debug)]
pub enum TagsError {
    /// File operation error
    FileOp(FileOpsError),
    /// JSON serialization/deserialization error
    Json(serde_json::Error),
    /// Tag not found
    TagNotFound(String),
    /// Invalid color format
    InvalidColor(String),
    /// Duplicate tag name
    DuplicateTag(String),
}

impl From<FileOpsError> for TagsError {
    fn from(err: FileOpsError) -> Self {
        TagsError::FileOp(err)
    }
}

impl From<ConfigError> for TagsError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::FileOp(e) => TagsError::FileOp(e),
            _ => TagsError::FileOp(FileOpsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Config error: {}", err),
            ))),
        }
    }
}

impl From<serde_json::Error> for TagsError {
    fn from(err: serde_json::Error) -> Self {
        TagsError::Json(err)
    }
}

impl std::fmt::Display for TagsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagsError::FileOp(err) => write!(f, "File operation error: {}", err),
            TagsError::Json(err) => write!(f, "JSON error: {}", err),
            TagsError::TagNotFound(name) => write!(f, "Tag not found: {}", name),
            TagsError::InvalidColor(color) => write!(f, "Invalid color format: {}", color),
            TagsError::DuplicateTag(name) => write!(f, "Tag already exists: {}", name),
        }
    }
}

impl std::error::Error for TagsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TagsError::FileOp(err) => Some(err),
            TagsError::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl Serialize for TagsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Result type for tag operations.
pub type TagsResult<T> = Result<T, TagsError>;

/// Result type for tag operations used in Tauri commands (serialized).
pub type TagsResultT<T> = Result<T, String>;

/// Returns the full path to the tags database file.
pub fn get_tags_db_path() -> TagsResult<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(TAGS_DB_FILE_NAME))
}

/// Loads the tags database from disk.
///
/// # Returns
/// `TagsResult<TagsDatabase>` - Loaded database, or default if file doesn't exist
///
/// # Behavior
/// - If tags.json exists, loads and parses it
/// - If tags.json doesn't exist, returns default empty database
/// - If tags.json is corrupted, returns error
pub fn load_tags_db() -> TagsResult<TagsDatabase> {
    let db_path = get_tags_db_path()?;

    if !db_path.exists() {
        // Tags database doesn't exist yet, return default
        return Ok(TagsDatabase::default());
    }

    let content = fs::read_to_string(&db_path)
        .map_err(FileOpsError::Io)?;

    let db: TagsDatabase = serde_json::from_str(&content)?;

    Ok(db)
}

/// Saves the tags database to disk.
///
/// # Arguments
/// * `db` - Database to save
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
///
/// # Behavior
/// - Creates config directory if it doesn't exist
/// - Overwrites existing tags.json
/// - Writes formatted JSON for readability
pub fn save_tags_db(db: &TagsDatabase) -> TagsResult<()> {
    let db_path = get_tags_db_path()?;

    // Create config directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(FileOpsError::Io)?;
    }

    // Serialize to formatted JSON
    let json = serde_json::to_string_pretty(db)?;

    // Write to file
    fs::write(&db_path, json)
        .map_err(FileOpsError::Io)?;

    Ok(())
}

/// Validates a hex color string.
///
/// # Arguments
/// * `color` - Color string to validate
///
/// # Returns
/// `TagsResult<()>` - Ok(()) if valid, Err otherwise
///
/// # Behavior
/// - Accepts formats: #RGB, #RRGGBB, #RRGGBBAA
fn validate_color(color: &str) -> TagsResult<()> {
    let color = color.trim();

    if !color.starts_with('#') {
        return Err(TagsError::InvalidColor(color.to_string()));
    }

    let hex_part = &color[1..];
    let len = hex_part.len();

    // Valid lengths: 3 (RGB), 6 (RRGGBB), 8 (RRGGBBAA)
    if len != 3 && len != 6 && len != 8 {
        return Err(TagsError::InvalidColor(color.to_string()));
    }

    // Check all characters are valid hex
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(TagsError::InvalidColor(color.to_string()));
    }

    Ok(())
}

/// Creates a new tag.
///
/// # Arguments
/// * `name` - Tag name (must be unique)
/// * `color` - Tag color (hex format like #FF5733)
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn create_tag(name: String, color: String) -> TagsResult<()> {
    // Validate color format
    validate_color(&color)?;

    let mut db = load_tags_db()?;

    // Check for duplicate tag name
    if db.tags.iter().any(|t| t.name == name) {
        return Err(TagsError::DuplicateTag(name));
    }

    // Add new tag
    db.tags.push(Tag { name, color });

    save_tags_db(&db)
}

/// Deletes a tag and removes all its associations.
///
/// # Arguments
/// * `name` - Tag name to delete
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn delete_tag(name: String) -> TagsResult<()> {
    let mut db = load_tags_db()?;

    // Check if tag exists
    if !db.tags.iter().any(|t| t.name == name) {
        return Err(TagsError::TagNotFound(name));
    }

    // Remove tag
    db.tags.retain(|t| t.name != name);

    // Remove tag from all associations
    for association in &mut db.associations {
        association.tag_names.retain(|t| t != &name);
    }

    // Clean up empty associations
    db.associations.retain(|a| !a.tag_names.is_empty());

    save_tags_db(&db)
}

/// Returns all defined tags.
///
/// # Returns
/// `TagsResult<Vec<Tag>>` - List of all tags
pub fn get_all_tags() -> TagsResult<Vec<Tag>> {
    let db = load_tags_db()?;
    Ok(db.tags)
}

/// Helper to find or create an association for a target.
fn find_association_mut<'a>(db: &'a mut TagsDatabase, target: &TagTarget) -> Option<&'a mut TagAssociation> {
    db.associations.iter_mut().find(|a| &a.target == target)
}

/// Adds tags to a backup.
///
/// # Arguments
/// * `save_name` - Save name
/// * `backup_name` - Backup name
/// * `tags` - Tag names to add
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn add_tags_to_backup(save_name: &str, backup_name: &str, tags: Vec<String>) -> TagsResult<()> {
    if tags.is_empty() {
        return Ok(());
    }

    let mut db = load_tags_db()?;
    let target = TagTarget::Backup {
        save_name: save_name.to_string(),
        backup_name: backup_name.to_string(),
    };

    // Validate all tags exist
    for tag in &tags {
        if !db.tags.iter().any(|t| &t.name == tag) {
            return Err(TagsError::TagNotFound(tag.clone()));
        }
    }

    // Find or create association
    let association = match find_association_mut(&mut db, &target) {
        Some(a) => a,
        None => {
            db.associations.push(TagAssociation {
                target: target.clone(),
                tag_names: Vec::new(),
            });
            db.associations.last_mut().unwrap()
        }
    };

    // Add tags (avoid duplicates)
    for tag in tags {
        if !association.tag_names.contains(&tag) {
            association.tag_names.push(tag);
        }
    }

    save_tags_db(&db)
}

/// Removes tags from a backup.
///
/// # Arguments
/// * `save_name` - Save name
/// * `backup_name` - Backup name
/// * `tags` - Tag names to remove
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn remove_tags_from_backup(save_name: &str, backup_name: &str, tags: Vec<String>) -> TagsResult<()> {
    if tags.is_empty() {
        return Ok(());
    }

    let db = load_tags_db();
    if db.is_err() {
        return Ok(()); // If we can't load db, no tags to remove
    }
    let mut db = db.unwrap();

    let target = TagTarget::Backup {
        save_name: save_name.to_string(),
        backup_name: backup_name.to_string(),
    };

    if let Some(association) = find_association_mut(&mut db, &target) {
        association.tag_names.retain(|t| !tags.contains(t));
    }

    // Clean up empty associations
    db.associations.retain(|a| !a.tag_names.is_empty());

    save_tags_db(&db)
}

/// Returns all tags for a backup.
///
/// # Arguments
/// * `save_name` - Save name
/// * `backup_name` - Backup name
///
/// # Returns
/// `TagsResult<Vec<Tag>>` - List of tags
pub fn get_backup_tags(save_name: &str, backup_name: &str) -> TagsResult<Vec<Tag>> {
    let db = load_tags_db()?;
    let target = TagTarget::Backup {
        save_name: save_name.to_string(),
        backup_name: backup_name.to_string(),
    };

    let association = match db.associations.iter().find(|a| &a.target == &target) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    let mut result = Vec::new();
    for tag_name in &association.tag_names {
        if let Some(tag) = db.tags.iter().find(|t| &t.name == tag_name) {
            result.push(tag.clone());
        }
    }

    Ok(result)
}

/// Adds tags to a save.
///
/// # Arguments
/// * `relative_path` - Save relative path
/// * `tags` - Tag names to add
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn add_tags_to_save(relative_path: &str, tags: Vec<String>) -> TagsResult<()> {
    if tags.is_empty() {
        return Ok(());
    }

    let mut db = load_tags_db()?;
    let target = TagTarget::Save {
        relative_path: relative_path.to_string(),
    };

    // Validate all tags exist
    for tag in &tags {
        if !db.tags.iter().any(|t| &t.name == tag) {
            return Err(TagsError::TagNotFound(tag.clone()));
        }
    }

    // Find or create association
    let association = match find_association_mut(&mut db, &target) {
        Some(a) => a,
        None => {
            db.associations.push(TagAssociation {
                target: target.clone(),
                tag_names: Vec::new(),
            });
            db.associations.last_mut().unwrap()
        }
    };

    // Add tags (avoid duplicates)
    for tag in tags {
        if !association.tag_names.contains(&tag) {
            association.tag_names.push(tag);
        }
    }

    save_tags_db(&db)
}

/// Removes tags from a save.
///
/// # Arguments
/// * `relative_path` - Save relative path
/// * `tags` - Tag names to remove
///
/// # Returns
/// `TagsResult<()>` - Ok(()) on success
pub fn remove_tags_from_save(relative_path: &str, tags: Vec<String>) -> TagsResult<()> {
    if tags.is_empty() {
        return Ok(());
    }

    let db = load_tags_db();
    if db.is_err() {
        return Ok(());
    }
    let mut db = db.unwrap();

    let target = TagTarget::Save {
        relative_path: relative_path.to_string(),
    };

    if let Some(association) = find_association_mut(&mut db, &target) {
        association.tag_names.retain(|t| !tags.contains(t));
    }

    // Clean up empty associations
    db.associations.retain(|a| !a.tag_names.is_empty());

    save_tags_db(&db)
}

/// Returns all tags for a save.
///
/// # Arguments
/// * `relative_path` - Save relative path
///
/// # Returns
/// `TagsResult<Vec<Tag>>` - List of tags
pub fn get_save_tags(relative_path: &str) -> TagsResult<Vec<Tag>> {
    let db = load_tags_db()?;
    let target = TagTarget::Save {
        relative_path: relative_path.to_string(),
    };

    let association = match db.associations.iter().find(|a| &a.target == &target) {
        Some(a) => a,
        None => return Ok(Vec::new()),
    };

    let mut result = Vec::new();
    for tag_name in &association.tag_names {
        if let Some(tag) = db.tags.iter().find(|t| &t.name == tag_name) {
            result.push(tag.clone());
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::TempDir;

    /// Helper to create a temporary config directory for testing
    fn setup_temp_config_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[test]
    fn test_validate_color_valid() {
        assert!(validate_color("#FF5733").is_ok());
        assert!(validate_color("#F53").is_ok());
        assert!(validate_color("#FF5733AA").is_ok());
        assert!(validate_color("#abc").is_ok());
    }

    #[test]
    fn test_validate_color_invalid() {
        assert!(validate_color("FF5733").is_err()); // Missing #
        assert!(validate_color("#FF5").is_err()); // Invalid length
        assert!(validate_color("#FF57333").is_err()); // Invalid length
        assert!(validate_color("#GG5733").is_err()); // Invalid hex
    }

    #[test]
    #[serial]
    fn test_tags_database_default() {
        let db = TagsDatabase::default();
        assert_eq!(db.tags.len(), 0);
        assert_eq!(db.associations.len(), 0);
    }

    #[test]
    #[serial]
    fn test_create_and_get_tag() {
        // This test uses the actual config directory, so we need to clean up
        let result = create_tag("important".to_string(), "#FF0000".to_string());
        assert!(result.is_ok());

        let tags = get_all_tags().unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "important");
        assert_eq!(tags[0].color, "#FF0000");

        // Clean up
        let _ = delete_tag("important".to_string());
    }

    #[test]
    #[serial]
    fn test_create_duplicate_tag_fails() {
        let _ = create_tag("test".to_string(), "#FF0000".to_string());
        let result = create_tag("test".to_string(), "#00FF00".to_string());
        assert!(matches!(result, Err(TagsError::DuplicateTag(_))));

        // Clean up
        let _ = delete_tag("test".to_string());
    }

    #[test]
    #[serial]
    fn test_delete_tag() {
        let _ = create_tag("to_delete".to_string(), "#FF0000".to_string());
        assert_eq!(get_all_tags().unwrap().len(), 1);

        let result = delete_tag("to_delete".to_string());
        assert!(result.is_ok());
        assert_eq!(get_all_tags().unwrap().len(), 0);
    }

    #[test]
    #[serial]
    fn test_delete_nonexistent_tag_fails() {
        let result = delete_tag("nonexistent".to_string());
        assert!(matches!(result, Err(TagsError::TagNotFound(_))));
    }

    #[test]
    #[serial]
    fn test_add_and_get_backup_tags() {
        let _ = create_tag("important".to_string(), "#FF0000".to_string());
        let _ = create_tag("test".to_string(), "#00FF00".to_string());

        let result = add_tags_to_backup("Survival", "backup1.tar.gz", vec![
            "important".to_string(),
            "test".to_string(),
        ]);
        assert!(result.is_ok());

        let tags = get_backup_tags("Survival", "backup1.tar.gz").unwrap();
        assert_eq!(tags.len(), 2);

        // Clean up
        let _ = delete_tag("important".to_string());
        let _ = delete_tag("test".to_string());
    }

    #[test]
    #[serial]
    fn test_add_and_get_save_tags() {
        let _ = create_tag("main".to_string(), "#0000FF".to_string());

        let result = add_tags_to_save("Survival/MySave", vec!["main".to_string()]);
        assert!(result.is_ok());

        let tags = get_save_tags("Survival/MySave").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "main");

        // Clean up
        let _ = delete_tag("main".to_string());
    }

    #[test]
    #[serial]
    fn test_remove_backup_tags() {
        let _ = create_tag("tag1".to_string(), "#FF0000".to_string());
        let _ = create_tag("tag2".to_string(), "#00FF00".to_string());

        let _ = add_tags_to_backup("Survival", "backup1.tar.gz", vec![
            "tag1".to_string(),
            "tag2".to_string(),
        ]);

        let result = remove_tags_from_backup("Survival", "backup1.tar.gz", vec!["tag1".to_string()]);
        assert!(result.is_ok());

        let tags = get_backup_tags("Survival", "backup1.tar.gz").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "tag2");

        // Clean up
        let _ = delete_tag("tag1".to_string());
        let _ = delete_tag("tag2".to_string());
    }

    #[test]
    #[serial]
    fn test_remove_save_tags() {
        let _ = create_tag("tag1".to_string(), "#FF0000".to_string());
        let _ = create_tag("tag2".to_string(), "#00FF00".to_string());

        let _ = add_tags_to_save("Survival/MySave", vec![
            "tag1".to_string(),
            "tag2".to_string(),
        ]);

        let result = remove_tags_from_save("Survival/MySave", vec!["tag1".to_string()]);
        assert!(result.is_ok());

        let tags = get_save_tags("Survival/MySave").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "tag2");

        // Clean up
        let _ = delete_tag("tag1".to_string());
        let _ = delete_tag("tag2".to_string());
    }

    #[test]
    #[serial]
    fn test_tag_serialization() {
        let tag = Tag {
            name: "test".to_string(),
            color: "#FF5733".to_string(),
        };

        let json = serde_json::to_string(&tag).unwrap();
        let parsed: Tag = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.color, "#FF5733");
    }
}
