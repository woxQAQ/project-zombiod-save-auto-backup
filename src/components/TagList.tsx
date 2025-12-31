import type { Tag } from "../types/tags";
import { shouldUseLightText } from "../types/tags";

interface TagListProps {
  tags: Tag[];
  onRemoveTag?: (tagName: string) => void;
  editable?: boolean;
}

/**
 * TagList component
 * Displays a list of colored tags with optional remove button
 */
export const TagList: React.FC<TagListProps> = ({ tags, onRemoveTag, editable = false }) => {
  if (tags.length === 0) {
    return null;
  }

  return (
    <div className="flex flex-wrap gap-1.5 mt-2">
      {tags.map((tag) => (
        <span
          key={tag.name}
          className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
          style={{
            backgroundColor: tag.color,
            color: shouldUseLightText(tag.color) ? "#FFFFFF" : "#000000",
          }}
        >
          {tag.name}
          {editable && onRemoveTag && (
            <button
              type="button"
              onClick={() => onRemoveTag(tag.name)}
              className="hover:bg-white/20 rounded-full p-0.5 transition-colors"
              aria-label={`Remove ${tag.name} tag`}
            >
              <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                className="w-3 h-3"
              >
                <title>Remove</title>
                <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          )}
        </span>
      ))}
    </div>
  );
};
