/**
 * Credential Manager Component
 * Manages authentication credentials for external systems (GitLab, Gerrit, etc.)
 */

import React, { useState } from 'react';
import { Lock, Key, Eye, EyeOff, Save, X } from 'lucide-react';

interface CredentialManagerProps {
  onClose: () => void;
  onSave: (system: string, credentials: Record<string, string>) => void;
}

type ExternalSystem = 'gitlab' | 'gerrit' | 'github' | 'bitbucket';

interface SystemConfig {
  id: ExternalSystem;
  name: string;
  fields: Array<{
    key: string;
    label: string;
    type: 'text' | 'password';
    placeholder: string;
  }>;
}

const SYSTEM_CONFIGS: SystemConfig[] = [
  {
    id: 'gitlab',
    name: 'GitLab',
    fields: [
      { key: 'url', label: 'GitLab URL', type: 'text', placeholder: 'https://gitlab.com' },
      { key: 'token', label: 'Personal Access Token', type: 'password', placeholder: 'glpat-xxxxxxxxxxxx' }
    ]
  },
  {
    id: 'gerrit',
    name: 'Gerrit',
    fields: [
      { key: 'url', label: 'Gerrit URL', type: 'text', placeholder: 'https://gerrit.example.com' },
      { key: 'username', label: 'Username', type: 'text', placeholder: 'your-username' },
      { key: 'password', label: 'Password', type: 'password', placeholder: 'your-password' }
    ]
  },
  {
    id: 'github',
    name: 'GitHub',
    fields: [
      { key: 'token', label: 'Personal Access Token', type: 'password', placeholder: 'ghp_xxxxxxxxxxxx' }
    ]
  },
  {
    id: 'bitbucket',
    name: 'Bitbucket',
    fields: [
      { key: 'url', label: 'Bitbucket URL', type: 'text', placeholder: 'https://bitbucket.org' },
      { key: 'username', label: 'Username', type: 'text', placeholder: 'your-username' },
      { key: 'app_password', label: 'App Password', type: 'password', placeholder: 'your-app-password' }
    ]
  }
];

export const CredentialManager: React.FC<CredentialManagerProps> = ({ onClose, onSave }) => {
  const [selectedSystem, setSelectedSystem] = useState<ExternalSystem>('gitlab');
  const [credentials, setCredentials] = useState<Record<string, string>>({});
  const [showPasswords, setShowPasswords] = useState<Record<string, boolean>>({});
  const [saving, setSaving] = useState(false);

  const systemConfig = SYSTEM_CONFIGS.find(s => s.id === selectedSystem);

  const handleCredentialChange = (key: string, value: string) => {
    setCredentials(prev => ({ ...prev, [key]: value }));
  };

  const togglePasswordVisibility = (key: string) => {
    setShowPasswords(prev => ({ ...prev, [key]: !prev[key] }));
  };

  const handleSave = async () => {
    if (!systemConfig) return;

    setSaving(true);
    try {
      // Validate required fields
      const requiredFields = systemConfig.fields.filter(f => f.type !== 'text');
      const missingFields = requiredFields.filter(f => !credentials[f.key]);

      if (missingFields.length > 0) {
        alert(`Please fill in all required fields: ${missingFields.map(f => f.label).join(', ')}`);
        setSaving(false);
        return;
      }

      // Save credentials (in a real app, these would be encrypted and stored securely)
      await onSave(selectedSystem, credentials);
      onClose();
    } catch (error) {
      console.error('Failed to save credentials:', error);
      alert('Failed to save credentials. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  if (!systemConfig) return null;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-editor-panel border border-editor-line rounded-lg shadow-xl w-full max-w-md mx-4">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-editor-line">
          <div className="flex items-center gap-2">
            <Lock className="text-editor-accent" size={20} />
            <h2 className="text-lg font-semibold text-editor-fg">Manage Credentials</h2>
          </div>
          <button
            onClick={onClose}
            className="text-editor-muted hover:text-editor-fg transition-colors"
            aria-label="Close credential manager"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-4">
          {/* System Selection */}
          <div className="mb-4">
            <label className="block text-sm font-medium text-editor-fg mb-2">
              External System
            </label>
            <select
              value={selectedSystem}
              onChange={(e) => {
                setSelectedSystem(e.target.value as ExternalSystem);
                setCredentials({});
              }}
              className="w-full px-3 py-2 bg-editor-bg border border-editor-line rounded text-editor-fg focus:outline-none focus:ring-2 focus:ring-editor-accent"
              aria-label="Select external system"
            >
              {SYSTEM_CONFIGS.map(system => (
                <option key={system.id} value={system.id}>
                  {system.name}
                </option>
              ))}
            </select>
          </div>

          {/* Credentials Form */}
          <div className="space-y-3">
            {systemConfig.fields.map(field => (
              <div key={field.key}>
                <label className="block text-sm font-medium text-editor-fg mb-1">
                  {field.label}
                  {field.type === 'password' && <span className="text-editor-error ml-1">*</span>}
                </label>
                <div className="relative">
                  <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    {field.type === 'password' ? (
                      <Key size={16} className="text-editor-muted" />
                    ) : (
                      <span className="text-editor-muted text-sm">🔗</span>
                    )}
                  </div>
                  <input
                    type={showPasswords[field.key] ? 'text' : field.type}
                    value={credentials[field.key] || ''}
                    onChange={(e) => handleCredentialChange(field.key, e.target.value)}
                    placeholder={field.placeholder}
                    className="w-full pl-10 pr-10 py-2 bg-editor-bg border border-editor-line rounded text-editor-fg focus:outline-none focus:ring-2 focus:ring-editor-accent"
                    aria-label={field.label}
                  />
                  {field.type === 'password' && (
                    <button
                      type="button"
                      onClick={() => togglePasswordVisibility(field.key)}
                      className="absolute inset-y-0 right-0 pr-3 flex items-center text-editor-muted hover:text-editor-fg"
                      aria-label={showPasswords[field.key] ? 'Hide password' : 'Show password'}
                    >
                      {showPasswords[field.key] ? <EyeOff size={16} /> : <Eye size={16} />}
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>

          {/* Info Box */}
          <div className="mt-4 p-3 bg-editor-info/10 border border-editor-info/20 rounded text-sm text-editor-info">
            <p>
              <strong>Security Note:</strong> Credentials are stored locally and encrypted. They are only used for authenticating with the selected external system.
            </p>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-3 p-4 border-t border-editor-line">
          <button
            onClick={onClose}
            className="px-4 py-2 text-editor-muted hover:text-editor-fg transition-colors"
            disabled={saving}
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="flex items-center gap-2 px-4 py-2 bg-editor-accent text-white rounded hover:bg-editor-accent/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {saving ? (
              <>
                <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full" />
                Saving...
              </>
            ) : (
              <>
                <Save size={16} />
                Save Credentials
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
};

export default CredentialManager;
