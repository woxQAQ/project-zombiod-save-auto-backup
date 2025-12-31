import { useEffect, useState } from "react";
import { useTags } from "../hooks/useTags";
import type { Tag, TagTarget } from "../types/tags";
import { TagList } from "./TagList";
import { TagPicker } from "./TagPicker";

interface TagEditorProps {
  isOpen: boolean;
  target: TagTarget | null;
  currentTags: Tag[];
  availableTags: Tag[];
  onSave: (tagNames: string[]) => Promise<void>;
  onCancel: () => void;
}

/**
 * TagEditor component
 * Modal dialog for editing tags on a backup or save
 */
export const TagEditor: React.FC<TagEditorProps> = ({
  isOpen,
  target,
  currentTags,
  availableTags,
  onSave,
  onCancel,
}) => {
  const { createTag, loadAllTags } = useTags();
  const [selectedTags, setSelectedTags] = useState<Tag[]>(currentTags);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Reset selected tags when dialog opens or currentTags change
  useEffect(() => {
    if (isOpen) {
      setSelectedTags(currentTags);
      setError(null);
    }
  }, [isOpen, currentTags]);

  if (!isOpen || !target) {
    return null;
  }

  const targetLabel =
    target.type === "backup" ? `${target.saveName} / ${target.backupName}` : target.relativePath;

  const handleTagToggle = (tag: Tag) => {
    setSelectedTags((prev) => {
      const isSelected = prev.some((t) => t.name === tag.name);
      if (isSelected) {
        return prev.filter((t) => t.name !== tag.name);
      } else {
        return [...prev, tag];
      }
    });
  };

  const handleCreateTag = async (name: string, color: string) => {
    try {
      await createTag(name, color);
      // Reload all tags to get the updated list
      await loadAllTags();
      // Auto-select the newly created tag
      const newTag = { name, color };
      if (!selectedTags.some((t) => t.name === name)) {
        setSelectedTags((prev) => [...prev, newTag]);
      }
    } catch (_err) {
      setError("Failed to create tag");
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      setError(null);
      const tagNames = selectedTags.map((t) => t.name);
      await onSave(tagNames);
      onCancel();
    } catch (_err) {
      setError("Failed to save tags");
    } finally {
      setSaving(false);
    }
  };

  const handleRemoveSelected = (tagName: string) => {
    setSelectedTags((prev) => prev.filter((t) => t.name !== tagName));
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <button
        type="button"
        className="absolute inset-0 bg-black/70"
        onClick={onCancel}
        aria-label="Close dialog"
      />

      {/* Modal */}
      <div className="relative bg-gray-900 border border-gray-700 rounded-lg shadow-xl max-w-md w-full mx-4 max-h-[80vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="px-6 py-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold text-white">Edit Tags</h2>
          <p className="text-sm text-gray-400 mt-1 truncate">{targetLabel}</p>
        </div>

        {/* Content */}
        <div className="px-6 py-4 overflow-y-auto flex-1 space-y-4">
          {/* Currently selected tags */}
          {selectedTags.length > 0 && (
            <div>
              <p className="text-sm text-gray-400 mb-2">Selected tags:</p>
              <TagList tags={selectedTags} editable onRemoveTag={handleRemoveSelected} />
            </div>
          )}

          {/* Tag picker */}
          <TagPicker
            availableTags={availableTags}
            selectedTags={selectedTags}
            onTagToggle={handleTagToggle}
            onCreateTag={handleCreateTag}
            disabled={saving}
          />

          {/* Error message */}
          {error && (
            <div className="text-sm text-red-400 bg-red-900/20 border border-red-900 rounded px-3 py-2">
              {error}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-gray-700 flex justify-end gap-3">
          <button
            type="button"
            onClick={onCancel}
            disabled={saving}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded font-medium transition-colors disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleSave}
            disabled={saving}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-medium transition-colors disabled:opacity-50 flex items-center gap-2"
          >
            {saving && (
              <svg
                className="animate-spin h-4 w-4"
                viewBox="0 0 24 24"
                fill="none"
                aria-label="Loading"
              >
                <title>Loading</title>
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                />
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            )}
            Save Tags
          </button>
        </div>
      </div>
    </div>
  );
};
