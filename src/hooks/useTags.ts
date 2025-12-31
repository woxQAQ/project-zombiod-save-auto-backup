/**
 * Tag management hook for the backup tool.
 */

import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import type { Tag } from "../types/tags";

/**
 * Tag management hook.
 */
export function useTags() {
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Loads all tags from the database.
   */
  const loadAllTags = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const allTags: Tag[] = await invoke("get_all_tags_command");
      setTags(allTags);
    } catch (err) {
      console.error("Failed to load tags:", err);
      setError("Failed to load tags");
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Creates a new tag.
   */
  const createTag = useCallback(
    async (name: string, color: string) => {
      try {
        setLoading(true);
        setError(null);
        await invoke("create_tag_command", { name, color });
        // Reload tags after creating
        await loadAllTags();
      } catch (err) {
        console.error("Failed to create tag:", err);
        setError("Failed to create tag");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [loadAllTags],
  );

  /**
   * Deletes a tag.
   */
  const deleteTag = useCallback(
    async (name: string) => {
      try {
        setLoading(true);
        setError(null);
        await invoke("delete_tag_command", { name });
        // Reload tags after deleting
        await loadAllTags();
      } catch (err) {
        console.error("Failed to delete tag:", err);
        setError("Failed to delete tag");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [loadAllTags],
  );

  return {
    tags,
    loading,
    error,
    loadAllTags,
    createTag,
    deleteTag,
  };
}

/**
 * Tag operations for backups.
 */
export function useBackupTags() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Gets tags for a backup.
   */
  const getBackupTags = useCallback(
    async (saveName: string, backupName: string): Promise<Tag[]> => {
      try {
        setLoading(true);
        setError(null);
        const tags: Tag[] = await invoke("get_backup_tags_command", {
          saveName,
          backupName,
        });
        return tags;
      } catch (err) {
        console.error("Failed to get backup tags:", err);
        setError("Failed to get backup tags");
        return [];
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  /**
   * Adds tags to a backup.
   */
  const addBackupTags = useCallback(
    async (saveName: string, backupName: string, tagNames: string[]) => {
      try {
        setLoading(true);
        setError(null);
        await invoke("add_tags_to_backup_command", {
          saveName,
          backupName,
          tags: tagNames,
        });
      } catch (err) {
        console.error("Failed to add backup tags:", err);
        setError("Failed to add backup tags");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  /**
   * Removes tags from a backup.
   */
  const removeBackupTags = useCallback(
    async (saveName: string, backupName: string, tagNames: string[]) => {
      try {
        setLoading(true);
        setError(null);
        await invoke("remove_tags_from_backup_command", {
          saveName,
          backupName,
          tags: tagNames,
        });
      } catch (err) {
        console.error("Failed to remove backup tags:", err);
        setError("Failed to remove backup tags");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  return {
    loading,
    error,
    getBackupTags,
    addBackupTags,
    removeBackupTags,
  };
}

/**
 * Tag operations for saves.
 */
export function useSaveTags() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Gets tags for a save.
   */
  const getSaveTags = useCallback(async (relativePath: string): Promise<Tag[]> => {
    try {
      setLoading(true);
      setError(null);
      const tags: Tag[] = await invoke("get_save_tags_command", {
        relativePath,
      });
      return tags;
    } catch (err) {
      console.error("Failed to get save tags:", err);
      setError("Failed to get save tags");
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Adds tags to a save.
   */
  const addSaveTags = useCallback(async (relativePath: string, tagNames: string[]) => {
    try {
      setLoading(true);
      setError(null);
      await invoke("add_tags_to_save_command", {
        relativePath,
        tags: tagNames,
      });
    } catch (err) {
      console.error("Failed to add save tags:", err);
      setError("Failed to add save tags");
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Removes tags from a save.
   */
  const removeSaveTags = useCallback(async (relativePath: string, tagNames: string[]) => {
    try {
      setLoading(true);
      setError(null);
      await invoke("remove_tags_from_save_command", {
        relativePath,
        tags: tagNames,
      });
    } catch (err) {
      console.error("Failed to remove save tags:", err);
      setError("Failed to remove save tags");
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    loading,
    error,
    getSaveTags,
    addSaveTags,
    removeSaveTags,
  };
}
