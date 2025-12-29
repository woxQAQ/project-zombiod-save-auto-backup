import { useEffect } from "react";

interface UpdateInfo {
  has_update: boolean;
  current_version: string;
  latest_version: string;
  release_url: string;
  release_notes: string;
  published_at: string;
}

interface UpdateAvailableModalProps {
  isOpen: boolean;
  onClose: () => void;
  updateInfo: UpdateInfo | null;
}

export const UpdateAvailableModal: React.FC<UpdateAvailableModalProps> = ({
  isOpen,
  onClose,
  updateInfo,
}) => {
  // Handle ESC key press
  useEffect(() => {
    if (!isOpen) return;

    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    };

    window.addEventListener("keydown", handleEsc);
    return () => window.removeEventListener("keydown", handleEsc);
  }, [isOpen, onClose]);

  if (!isOpen || !updateInfo) return null;

  const handleDownload = () => {
    window.open(updateInfo.release_url, "_blank");
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-[#1a1a1a] border border-gray-800 rounded-lg shadow-xl w-full max-w-lg mx-4">
        {/* Header */}
        <div className="p-6 border-b border-gray-800">
          <div className="flex items-center space-x-3">
            <div className="bg-primary/20 p-2 rounded-lg">
              <svg
                className="w-6 h-6 text-primary"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
              >
                <title>Download icon</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                />
              </svg>
            </div>
            <div>
              <h2 className="text-xl font-semibold text-foreground">Update Available</h2>
              <p className="text-sm text-gray-400">
                Version {updateInfo.latest_version} is ready to install
              </p>
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="p-6 space-y-4">
          {/* Version comparison */}
          <div className="bg-gray-900 border border-gray-800 rounded-lg p-4">
            <p className="text-sm text-gray-400">Current version: {updateInfo.current_version}</p>
            <p className="text-sm text-primary font-medium">
              Latest version: {updateInfo.latest_version}
            </p>
          </div>

          {/* Release notes */}
          <div>
            <h3 className="text-sm font-medium text-foreground mb-2">What's New</h3>
            <div className="bg-gray-900 border border-gray-800 rounded-lg p-4 max-h-48 overflow-y-auto">
              <div className="text-sm text-gray-300 whitespace-pre-wrap">
                {updateInfo.release_notes || "No release notes available."}
              </div>
            </div>
          </div>

          {/* Published date */}
          <p className="text-xs text-gray-500">
            Published: {new Date(updateInfo.published_at).toLocaleDateString()}
          </p>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-3 p-6 border-t border-gray-800">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-foreground rounded-lg transition-colors"
          >
            Remind Me Later
          </button>
          <button
            type="button"
            onClick={handleDownload}
            className="px-6 py-2 bg-primary hover:bg-red-700 text-white rounded-lg transition-colors"
          >
            Download Update
          </button>
        </div>
      </div>
    </div>
  );
};
