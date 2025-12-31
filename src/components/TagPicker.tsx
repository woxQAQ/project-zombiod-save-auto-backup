import { useState } from "react";
import type { Tag } from "../types/tags";
import { DEFAULT_TAG_COLORS, isValidHexColor, shouldUseLightText } from "../types/tags";

interface TagPickerProps {
  availableTags: Tag[];
  selectedTags: Tag[];
  onTagToggle: (tag: Tag) => void;
  onCreateTag?: (name: string, color: string) => void;
  disabled?: boolean;
}

/**
 * TagPicker component
 * Allows selecting/deselecting tags and creating new ones
 */
export const TagPicker: React.FC<TagPickerProps> = ({
  availableTags,
  selectedTags,
  onTagToggle,
  onCreateTag,
  disabled = false,
}) => {
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [newTagName, setNewTagName] = useState("");
  const [newTagColor, setNewTagColor] = useState(DEFAULT_TAG_COLORS[4].value); // Default to blue
  const [customColor, setCustomColor] = useState("");

  const selectedTagNames = new Set(selectedTags.map((t) => t.name));

  const handleCreateTag = (e: React.FormEvent) => {
    e.preventDefault();
    const name = newTagName.trim();
    const color = customColor || newTagColor;

    if (!name) {
      return;
    }

    if (!isValidHexColor(color)) {
      return;
    }

    if (onCreateTag) {
      onCreateTag(name, color);
    }

    // Reset form
    setNewTagName("");
    setCustomColor("");
    setShowCreateForm(false);
  };

  const isTagSelected = (tag: Tag) => selectedTagNames.has(tag.name);

  return (
    <div className="space-y-3">
      {/* Available tags */}
      {availableTags.length > 0 && (
        <div>
          <p className="text-sm text-gray-400 mb-2">Select tags:</p>
          <div className="flex flex-wrap gap-2">
            {availableTags.map((tag) => {
              const selected = isTagSelected(tag);
              return (
                <button
                  key={tag.name}
                  type="button"
                  onClick={() => onTagToggle(tag)}
                  disabled={disabled}
                  className={`
                    px-3 py-1.5 rounded-full text-sm font-medium transition-all
                    ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}
                    ${
                      selected
                        ? "ring-2 ring-offset-2 ring-offset-gray-900 ring-blue-500"
                        : "opacity-70 hover:opacity-100"
                    }
                  `}
                  style={{
                    backgroundColor: tag.color,
                    color: shouldUseLightText(tag.color) ? "#FFFFFF" : "#000000",
                  }}
                >
                  {tag.name}
                  {selected && (
                    <span className="ml-1.5">
                      <svg
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="3"
                        className="w-3 h-3 inline"
                      >
                        <title>Selected</title>
                        <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                      </svg>
                    </span>
                  )}
                </button>
              );
            })}
          </div>
        </div>
      )}

      {/* Create new tag */}
      {onCreateTag && (
        <div>
          {!showCreateForm ? (
            <button
              type="button"
              onClick={() => setShowCreateForm(true)}
              disabled={disabled}
              className={`
                text-sm text-blue-400 hover:text-blue-300 transition-colors
                ${disabled ? "opacity-50 cursor-not-allowed" : ""}
              `}
            >
              + Create new tag
            </button>
          ) : (
            <form onSubmit={handleCreateTag} className="space-y-2">
              <div>
                <label htmlFor="new-tag-name" className="block text-sm text-gray-400 mb-1">
                  Tag name
                </label>
                <input
                  id="new-tag-name"
                  type="text"
                  value={newTagName}
                  onChange={(e) => setNewTagName(e.target.value)}
                  placeholder="e.g., Important"
                  disabled={disabled}
                  className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
                />
              </div>

              <div>
                <span className="block text-sm text-gray-400 mb-1">Tag color</span>
                <div className="flex flex-wrap gap-2 mb-2">
                  {DEFAULT_TAG_COLORS.map((color) => (
                    <button
                      key={color.value}
                      type="button"
                      onClick={() => {
                        setNewTagColor(color.value);
                        setCustomColor("");
                      }}
                      disabled={disabled}
                      className={`
                        w-8 h-8 rounded-full transition-all
                        ${
                          newTagColor === color.value && !customColor
                            ? "ring-2 ring-offset-2 ring-offset-gray-900 ring-white"
                            : "hover:scale-110"
                        }
                        ${disabled ? "opacity-50 cursor-not-allowed" : ""}
                      `}
                      style={{ backgroundColor: color.value }}
                      title={color.name}
                    />
                  ))}
                </div>
                <div className="flex items-center gap-2">
                  <input
                    type="color"
                    value={customColor || newTagColor}
                    onChange={(e) => {
                      setCustomColor(e.target.value);
                    }}
                    disabled={disabled}
                    className="w-10 h-10 rounded cursor-pointer disabled:opacity-50"
                  />
                  <span className="text-sm text-gray-500">Custom color</span>
                </div>
              </div>

              <div className="flex gap-2">
                <button
                  type="submit"
                  disabled={disabled || !newTagName.trim()}
                  className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-900 text-white rounded text-sm font-medium transition-colors disabled:opacity-50"
                >
                  Create
                </button>
                <button
                  type="button"
                  onClick={() => {
                    setShowCreateForm(false);
                    setNewTagName("");
                    setCustomColor("");
                  }}
                  disabled={disabled}
                  className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded text-sm font-medium transition-colors disabled:opacity-50"
                >
                  Cancel
                </button>
              </div>
            </form>
          )}
        </div>
      )}
    </div>
  );
};
