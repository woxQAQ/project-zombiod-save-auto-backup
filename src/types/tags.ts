/**
 * Tag type definitions for the backup tool.
 */

/**
 * Tag with name and color.
 */
export interface Tag {
  name: string;
  color: string; // hex color like "#FF5733"
}

/**
 * Tag target type - can be either a backup or a save.
 */
export type TagTarget =
  | { type: "backup"; saveName: string; backupName: string }
  | { type: "save"; relativePath: string };

/**
 * Tag association linking a target to tag names.
 */
export interface TagAssociation {
  target: TagTarget;
  tagNames: string[];
}

/**
 * Tags database containing all tags and associations.
 */
export interface TagsDatabase {
  tags: Tag[];
  associations: TagAssociation[];
}

/**
 * Helper function to create a backup tag target.
 */
export function backupTarget(saveName: string, backupName: string): TagTarget {
  return { type: "backup", saveName, backupName };
}

/**
 * Helper function to create a save tag target.
 */
export function saveTarget(relativePath: string): TagTarget {
  return { type: "save", relativePath };
}

/**
 * Checks if a tag target is a backup target.
 */
export function isBackupTarget(
  target: TagTarget,
): target is Extract<TagTarget, { type: "backup" }> {
  return target.type === "backup";
}

/**
 * Checks if a tag target is a save target.
 */
export function isSaveTarget(target: TagTarget): target is Extract<TagTarget, { type: "save" }> {
  return target.type === "save";
}

/**
 * Default tag colors for quick selection.
 */
export const DEFAULT_TAG_COLORS = [
  { name: "Red", value: "#EF4444" },
  { name: "Orange", value: "#F97316" },
  { name: "Yellow", value: "#EAB308" },
  { name: "Green", value: "#22C55E" },
  { name: "Blue", value: "#3B82F6" },
  { name: "Purple", value: "#A855F7" },
  { name: "Pink", value: "#EC4899" },
  { name: "Gray", value: "#6B7280" },
];

/**
 * Determines if text should be light or dark based on background color.
 * Returns true if text should be light (white), false if text should be dark (black).
 */
export function shouldUseLightText(backgroundColor: string): boolean {
  // Remove # if present
  const hex = backgroundColor.replace("#", "");

  // Parse RGB values
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);

  // Calculate relative luminance (W3C formula)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

  // Use light text for dark backgrounds (luminance < 0.5)
  return luminance < 0.5;
}

/**
 * Validates a hex color string.
 */
export function isValidHexColor(color: string): boolean {
  return /^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/.test(color);
}

/**
 * Converts a tag target to a string key for caching/indexing.
 */
export function tagTargetToKey(target: TagTarget): string {
  if (isBackupTarget(target)) {
    return `backup:${target.saveName}:${target.backupName}`;
  } else {
    return `save:${target.relativePath}`;
  }
}
