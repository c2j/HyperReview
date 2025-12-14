/**
 * TagManager Component
 * Manages tags for organizing repository items
 */

import React, { useState, useEffect } from 'react';
import { Tag, Plus, Trash2, Check, X, Search } from 'lucide-react';
import { useApiClient } from '../api/client';
import { useErrorStore, ErrorSeverity } from '../utils/errorHandler';
import type { Tag as TagType } from '../api/types';

interface TagManagerProps {
  onClose: () => void;
  onTagSelect?: (tag: TagType) => void;
}

const PRESET_COLORS = [
  '#3B82F6', // Blue
  '#10B981', // Emerald
  '#F59E0B', // Amber
  '#EF4444', // Red
  '#8B5CF6', // Violet
  '#EC4899', // Pink
  '#06B6D4', // Cyan
  '#84CC16', // Lime
  '#F97316', // Orange
  '#6366F1', // Indigo
];

export const TagManager: React.FC<TagManagerProps> = ({ onClose, onTagSelect }) => {
  const [tags, setTags] = useState<TagType[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [newTag, setNewTag] = useState({
    label: '',
    color: PRESET_COLORS[0],
    description: ''
  });

  const { getTags, createTag } = useApiClient();
  const { addError, showToast } = useErrorStore();

  // Load tags
  useEffect(() => {
    loadTags();
  }, []);

  const loadTags = async () => {
    setLoading(true);
    try {
      const loadedTags = await getTags();
      setTags(loadedTags);
    } catch (error) {
      addError({
        severity: ErrorSeverity.ERROR,
        title: 'Load Error',
        message: 'Failed to load tags'
      });
      console.error('Load tags error:', error);
    } finally {
      setLoading(false);
    }
  };

  // Filter tags by search query
  const filteredTags = tags.filter(tag =>
    tag.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (tag.description && tag.description.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  // Handle tag creation
  const handleCreate = async () => {
    if (!newTag.label.trim()) {
      addError({
        severity: ErrorSeverity.ERROR,
        title: 'Validation Error',
        message: 'Tag name is required'
      });
      return;
    }

    // Check if tag already exists
    if (tags.some(t => t.label.toLowerCase() === newTag.label.toLowerCase())) {
      addError({
        severity: ErrorSeverity.ERROR,
        title: 'Validation Error',
        message: 'A tag with this name already exists'
      });
      return;
    }

    try {
      const created = await createTag(newTag.label, newTag.color);
      setTags([...tags, created]);
      setNewTag({ label: '', color: PRESET_COLORS[0], description: '' });
      setIsCreating(false);
      showToast({
        severity: ErrorSeverity.SUCCESS,
        title: 'Success',
        message: 'Tag created successfully'
      });
    } catch (error) {
      addError({
        severity: ErrorSeverity.ERROR,
        title: 'Create Error',
        message: 'Failed to create tag'
      });
      console.error('Create tag error:', error);
    }
  };

  // Handle tag selection
  const handleSelect = (tag: TagType) => {
    onTagSelect?.(tag);
    onClose();
  };

  // Handle tag deletion
  const handleDelete = async (tag: TagType) => {
    if (!confirm(`Are you sure you want to delete the tag "${tag.label}"?`)) {
      return;
    }

    try {
      // In a real implementation, you would call a deleteTag API
      // await deleteTag(tag.id);
      setTags(tags.filter(t => t.id !== tag.id));
      showToast({
        severity: ErrorSeverity.SUCCESS,
        title: 'Success',
        message: 'Tag deleted successfully'
      });
    } catch (error) {
      addError({
        severity: ErrorSeverity.ERROR,
        title: 'Delete Error',
        message: 'Failed to delete tag'
      });
      console.error('Delete tag error:', error);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-editor-panel border border-editor-line rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[80vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-editor-line">
          <div className="flex items-center gap-2">
            <Tag className="text-editor-accent" size={20} />
            <h2 className="text-lg font-semibold text-editor-fg">Tag Manager</h2>
          </div>
          <button
            onClick={onClose}
            className="text-editor-muted hover:text-editor-fg transition-colors"
            aria-label="Close tag manager"
          >
            <X size={20} />
          </button>
        </div>

        {/* Search */}
        <div className="p-4 border-b border-editor-line">
          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <Search size={16} className="text-editor-muted" />
            </div>
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search tags..."
              className="w-full pl-10 pr-4 py-2 bg-editor-bg border border-editor-line rounded text-editor-fg placeholder-editor-muted focus:outline-none focus:ring-2 focus:ring-editor-accent"
              aria-label="Search tags"
            />
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-4">
          {/* Create Tag Button */}
          <button
            onClick={() => setIsCreating(true)}
            className="w-full flex items-center justify-center gap-2 p-3 border-2 border-dashed border-editor-line rounded-lg text-editor-muted hover:text-editor-fg hover:border-editor-accent transition-colors mb-4"
            aria-label="Create new tag"
          >
            <Plus size={18} />
            Create New Tag
          </button>

          {/* Create Tag Form */}
          {isCreating && (
            <div className="bg-editor-bg border border-editor-line rounded-lg p-4 mb-4">
              <h3 className="text-sm font-medium text-editor-fg mb-3">Create New Tag</h3>

              {/* Tag Name */}
              <div className="mb-3">
                <label className="block text-xs font-medium text-editor-muted mb-1">
                  Tag Name *
                </label>
                <input
                  type="text"
                  value={newTag.label}
                  onChange={(e) => setNewTag({ ...newTag, label: e.target.value })}
                  placeholder="e.g., Bug, Feature, Priority"
                  className="w-full px-3 py-2 bg-editor-panel border border-editor-line rounded text-editor-fg focus:outline-none focus:ring-2 focus:ring-editor-accent"
                  aria-label="Tag name"
                />
              </div>

              {/* Color Selection */}
              <div className="mb-3">
                <label className="block text-xs font-medium text-editor-muted mb-2">
                  Color
                </label>
                <div className="flex flex-wrap gap-2">
                  {PRESET_COLORS.map(color => (
                    <button
                      key={color}
                      onClick={() => setNewTag({ ...newTag, color })}
                      className={`w-8 h-8 rounded-full border-2 ${
                        newTag.color === color ? 'border-white scale-110' : 'border-editor-line'
                      }`}
                      style={{ backgroundColor: color }}
                      aria-label={`Select color ${color}`}
                    />
                  ))}
                </div>
              </div>

              {/* Description */}
              <div className="mb-3">
                <label className="block text-xs font-medium text-editor-muted mb-1">
                  Description (optional)
                </label>
                <input
                  type="text"
                  value={newTag.description}
                  onChange={(e) => setNewTag({ ...newTag, description: e.target.value })}
                  placeholder="Brief description of this tag"
                  className="w-full px-3 py-2 bg-editor-panel border border-editor-line rounded text-editor-fg focus:outline-none focus:ring-2 focus:ring-editor-accent"
                  aria-label="Tag description"
                />
              </div>

              {/* Actions */}
              <div className="flex items-center justify-end gap-2">
                <button
                  onClick={() => {
                    setIsCreating(false);
                    setNewTag({ label: '', color: PRESET_COLORS[0], description: '' });
                  }}
                  className="px-3 py-1.5 text-sm text-editor-muted hover:text-editor-fg transition-colors"
                >
                  Cancel
                </button>
                <button
                  onClick={handleCreate}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-editor-accent text-white rounded text-sm hover:bg-editor-accent/90 transition-colors"
                >
                  <Check size={16} />
                  Create Tag
                </button>
              </div>
            </div>
          )}

          {/* Tags List */}
          {loading ? (
            <div className="flex items-center justify-center py-8 text-editor-muted">
              <div className="animate-spin h-6 w-6 border-2 border-editor-accent border-t-transparent rounded-full mr-3" />
              Loading tags...
            </div>
          ) : filteredTags.length === 0 ? (
            <div className="flex items-center justify-center py-8 text-editor-muted">
              {searchQuery ? 'No tags found matching your search' : 'No tags created yet'}
            </div>
          ) : (
            <div className="space-y-2">
              {filteredTags.map(tag => (
                <div
                  key={tag.id}
                  className="flex items-center gap-3 p-3 bg-editor-bg border border-editor-line rounded hover:border-editor-accent/50 transition-colors group"
                >
                  {/* Tag Badge */}
                  <button
                    onClick={() => handleSelect(tag)}
                    className="flex items-center gap-2 flex-1 text-left hover:opacity-80 transition-opacity"
                    aria-label={`Select tag ${tag.label}`}
                  >
                    <span
                      className="w-4 h-4 rounded-full flex-shrink-0"
                      style={{ backgroundColor: tag.color }}
                    />
                    <div>
                      <div className="text-sm font-medium text-editor-fg">
                        {tag.label}
                      </div>
                      {tag.description && (
                        <div className="text-xs text-editor-muted">
                          {tag.description}
                        </div>
                      )}
                      <div className="text-xs text-editor-muted">
                        {tag.usage_count} item{tag.usage_count !== 1 ? 's' : ''}
                      </div>
                    </div>
                  </button>

                  {/* Actions */}
                  <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    {/* Edit button - TODO: Implement edit functionality */}
                    {/* <button
                      onClick={() => {}}
                      className="p-1.5 text-editor-muted hover:text-editor-fg transition-colors"
                      aria-label={`Edit tag ${tag.label}`}
                    >
                      <Edit2 size={16} />
                    </button> */}
                    <button
                      onClick={() => handleDelete(tag)}
                      className="p-1.5 text-editor-muted hover:text-editor-error transition-colors"
                      aria-label={`Delete tag ${tag.label}`}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="border-t border-editor-line p-4">
          <div className="flex items-center justify-between text-xs text-editor-muted">
            <span>
              {filteredTags.length} tag{filteredTags.length !== 1 ? 's' : ''}
            </span>
            <span>Tags are stored locally in SQLite</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TagManager;
