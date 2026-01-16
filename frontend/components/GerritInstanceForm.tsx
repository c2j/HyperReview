import React, { useState } from 'react';
import { Plus, Server, CheckCircle2, AlertCircle, Loader2, Eye, EyeOff } from 'lucide-react';
import { simpleGerritService, SimpleCreateParams } from '../services/gerrit-simple-service';

interface GerritInstanceFormProps {
  onClose: () => void;
  onSuccess: () => void;
}

const GerritInstanceForm: React.FC<GerritInstanceFormProps> = ({ onClose, onSuccess }) => {
  const [formData, setFormData] = useState<SimpleCreateParams>({
    name: '',
    url: '',
    username: '',
    password: ''
  });
  const [showPassword, setShowPassword] = useState(false);
  const [isTestingConnection, setIsTestingConnection] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  const [connectionTest, setConnectionTest] = useState<{
    success: boolean;
    version?: string;
    message?: string;
  } | null>(null);

  const validateForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!formData.name.trim()) {
      errors.name = 'Instance name is required';
    } else if (formData.name.length > 100) {
      errors.name = 'Instance name must be less than 100 characters';
    }

    if (!formData.url.trim()) {
      errors.url = 'URL is required';
    } else if (!formData.url.startsWith('https://')) {
      errors.url = 'URL must use HTTPS protocol';
    } else {
      try {
        new URL(formData.url);
      } catch {
        errors.url = 'Invalid URL format';
      }
    }

    if (!formData.username.trim()) {
      errors.username = 'Username is required';
    } else if (formData.username.length > 255) {
      errors.username = 'Username must be less than 255 characters';
    }

    if (!formData.password.trim()) {
      errors.password = 'Password is required';
    } else if (formData.password.length < 6) {
      errors.password = 'Password must be at least 6 characters';
    }

    setValidationErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleFieldChange = (field: keyof SimpleCreateParams, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    setConnectionTest(null);
    if (validationErrors[field]) {
      setValidationErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[field];
        return newErrors;
      });
    }
  };

  const handleTestConnection = async () => {
    if (!validateForm()) {
      return;
    }

    setIsTestingConnection(true);
    setConnectionTest(null);

    try {
      const result = await simpleGerritService.createInstance(formData);
      if (result) {
        setConnectionTest({
          success: true,
          message: 'Connection successful'
        });
      } else {
        setConnectionTest({
          success: false,
          message: 'Connection failed'
        });
      }
    } catch (error) {
      setConnectionTest({
        success: false,
        message: (error as Error).message
      });
    } finally {
      setIsTestingConnection(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    setIsCreating(true);

    try {
      const result = await simpleGerritService.createInstance(formData);
      if (result) {
        onSuccess();
        onClose();
      } else {
        alert('Failed to create Gerrit instance. Please try again.');
      }
    } catch (error) {
      console.error('Failed to create instance:', error);
      alert('Failed to create instance: ' + (error as Error).message);
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">
          Instance Name
        </label>
        <div className="relative">
          <Server
            className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
            size={16}
          />
          <input
            type="text"
            value={formData.name}
            onChange={(e) => handleFieldChange('name', e.target.value)}
            placeholder="e.g. Production Gerrit"
            className={`w-full bg-editor-line/50 border rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none transition-colors ${
              validationErrors.name
                ? 'border-red-500 focus:border-red-500'
                : 'border-editor-line focus:border-editor-accent'
            }`}
            autoFocus
          />
        </div>
        {validationErrors.name && (
          <div className="text-[10px] text-red-400 mt-1 flex items-center gap-1">
            <AlertCircle size={10} />
            {validationErrors.name}
          </div>
        )}
      </div>

      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">
          Gerrit URL
        </label>
        <input
          type="url"
          value={formData.url}
          onChange={(e) => handleFieldChange('url', e.target.value)}
          placeholder="https://gerrit.example.com"
          className={`w-full bg-editor-line/50 border rounded px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none transition-colors ${
            validationErrors.url
              ? 'border-red-500 focus:border-red-500'
              : 'border-editor-line focus:border-editor-accent'
          }`}
        />
        {validationErrors.url && (
          <div className="text-[10px] text-red-400 mt-1 flex items-center gap-1">
            <AlertCircle size={10} />
            {validationErrors.url}
          </div>
        )}
        <div className="text-[10px] text-gray-500 mt-1">
          Must use HTTPS protocol for secure connections
        </div>
      </div>

      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">
          Username
        </label>
        <input
          type="text"
          value={formData.username}
          onChange={(e) => handleFieldChange('username', e.target.value)}
          placeholder="Your Gerrit username"
          className={`w-full bg-editor-line/50 border rounded px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none transition-colors ${
            validationErrors.username
              ? 'border-red-500 focus:border-red-500'
              : 'border-editor-line focus:border-editor-accent'
          }`}
        />
        {validationErrors.username && (
          <div className="text-[10px] text-red-400 mt-1 flex items-center gap-1">
            <AlertCircle size={10} />
            {validationErrors.username}
          </div>
        )}
      </div>

      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">
          HTTP Password Token
        </label>
        <div className="relative">
          <input
            type={showPassword ? 'text' : 'password'}
            value={formData.password}
            onChange={(e) => handleFieldChange('password', e.target.value)}
            placeholder="Your Gerrit HTTP password token"
            className={`w-full bg-editor-line/50 border rounded px-3 py-2 pr-9 text-sm text-white placeholder-gray-500 focus:outline-none transition-colors ${
              validationErrors.password
                ? 'border-red-500 focus:border-red-500'
                : 'border-editor-line focus:border-editor-accent'
            }`}
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300 transition-colors"
          >
            {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
          </button>
        </div>
        {validationErrors.password && (
          <div className="text-[10px] text-red-400 mt-1 flex items-center gap-1">
            <AlertCircle size={10} />
            {validationErrors.password}
          </div>
        )}
        <div className="text-[10px] text-gray-500 mt-1">
          Get your HTTP password from Gerrit Settings → HTTP Password
        </div>
      </div>

      {connectionTest && (
        <div className={`p-3 rounded border flex items-start gap-2 ${
          connectionTest.success
            ? 'bg-green-500/10 border-green-500/30 text-green-400'
            : 'bg-red-500/10 border-red-500/30 text-red-400'
        }`}>
          {connectionTest.success ? (
            <CheckCircle2 size={16} className="flex-shrink-0 mt-0.5" />
          ) : (
            <AlertCircle size={16} className="flex-shrink-0 mt-0.5" />
          )}
          <div className="flex-1">
            <div className="text-xs font-medium">
              {connectionTest.success ? 'Connection Successful' : 'Connection Failed'}
            </div>
            {connectionTest.message && (
              <div className="text-[10px] mt-0.5">{connectionTest.message}</div>
            )}
          </div>
        </div>
      )}

      <div className="flex gap-2 pt-3 border-t border-editor-line mt-1">
        <button
          type="button"
          onClick={onClose}
          className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={handleTestConnection}
          disabled={isTestingConnection || isCreating}
          className="px-4 py-1.5 rounded text-xs bg-editor-line text-white hover:bg-editor-line/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center gap-2"
        >
          {isTestingConnection ? (
            <>
              <Loader2 size={14} className="animate-spin" />
              Testing...
            </>
          ) : (
            'Test Connection'
          )}
        </button>
        <button
          type="submit"
          disabled={isCreating || isTestingConnection}
          className="flex-1 px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center justify-center gap-2"
        >
          {isCreating ? (
            <>
              <Loader2 size={14} className="animate-spin" />
              Creating...
            </>
          ) : (
            <>
              <Plus size={14} />
              Create Instance
            </>
          )}
        </button>
      </div>
    </form>
  );
};

export default GerritInstanceForm;
