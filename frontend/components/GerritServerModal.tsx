
import React, { useState, useEffect } from 'react';
import { Globe, User, ShieldCheck, Zap, AlertCircle, Loader2, Link2, Eye, EyeOff, Tag } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getGerritConfig, saveGerritConfig, testGerritConnection } from '../api/client';
import type { GerritServerConfig } from '../api/types/gerrit';

interface GerritServerModalProps {
  onClose: () => void;
  onSuccess: () => void;
}

const GerritServerModal: React.FC<GerritServerModalProps> = ({ onClose, onSuccess }) => {
  const { t } = useTranslation();
  const [config, setConfig] = useState<GerritServerConfig>({
      name: 'Default Gerrit',
      url: '',
      username: '',
      token: '',
      authType: 'http'
  });
  const [loading, setLoading] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);
  const [showToken, setShowToken] = useState(false);

  useEffect(() => {
      setLoading(true);
      getGerritConfig().then(data => {
          if (data) setConfig(data);
      }).finally(() => setLoading(false));
  }, []);

  const handleTest = async () => {
      setTesting(true);
      setTestResult(null);
      try {
          const res = await testGerritConnection(config);
          setTestResult({
              success: true,
              message: t('gerrit.server.success', { version: res.version })
          });
      } catch (e) {
          setTestResult({ success: false, message: t('gerrit.server.test_failed') });
      } finally {
          setTesting(false);
      }
  };

   const handleSave = async () => {
      setTesting(true);
      try {
          await saveGerritConfig(config);
          onSuccess();
      } catch (e) {
          setTestResult({ success: false, message: t('gerrit.server.test_failed') });
      } finally {
          setTesting(false);
      }
  };

  if (loading) return <div className="py-12 flex justify-center"><Loader2 className="animate-spin text-purple-500" /></div>;

  return (
    <div className="flex flex-col gap-5">
       <div className="space-y-4">
        {/* Configuration Name */}
        <div>
            <label className="text-[10px] font-bold text-gray-500 uppercase mb-1.5 block tracking-widest">{t('gerrit.server.config_name')}</label>
            <div className="relative group">
                <Tag className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600 group-focus-within:text-purple-400 transition-colors" size={14} />
                <input
                    value={config.name}
                    onChange={e => setConfig({ ...config, name: e.target.value })}
                    placeholder={t('gerrit.server.config_name_placeholder')}
                    className="w-full bg-editor-bg border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white focus:outline-none focus:border-purple-500 transition-all font-mono"
                />
            </div>
        </div>

        {/* Server URL */}
        <div>
            <label className="text-[10px] font-bold text-gray-500 uppercase mb-1.5 block tracking-widest">{t('gerrit.server.url')}</label>
            <div className="relative group">
                <Globe className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600 group-focus-within:text-purple-400 transition-colors" size={14} />
                <input 
                    value={config.url}
                    onChange={e => setConfig({ ...config, url: e.target.value })}
                    placeholder={t('gerrit.server.url_placeholder')}
                    className="w-full bg-editor-bg border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white focus:outline-none focus:border-purple-500 transition-all font-mono"
                />
            </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
            {/* Username */}
            <div>
                <label className="text-[10px] font-bold text-gray-500 uppercase mb-1.5 block tracking-widest">{t('gerrit.server.username')}</label>
                <div className="relative group">
                    <User className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600 group-focus-within:text-purple-400 transition-colors" size={14} />
                    <input 
                        value={config.username}
                        onChange={e => setConfig({ ...config, username: e.target.value })}
                         placeholder={t('gerrit.server.username_placeholder')}
                        className="w-full bg-editor-bg border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white focus:outline-none focus:border-purple-500 transition-all font-mono"
                    />
                </div>
            </div>
            {/* Auth Type */}
            <div>
                <label className="text-[10px] font-bold text-gray-500 uppercase mb-1.5 block tracking-widest">{t('gerrit.server.auth_type')}</label>
                <select
                    value={config.authType}
                    onChange={e => setConfig({ ...config, authType: e.target.value as any })}
                    className="w-full bg-editor-bg border border-editor-line rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-purple-500 transition-all cursor-pointer"
                >
                    <option value="http">{t('gerrit.server.http_password')}</option>
                    <option value="ssh">{t('gerrit.server.ssh_key')}</option>
                </select>
            </div>
        </div>

        {/* API Token */}
        <div>
            <label className="text-[10px] font-bold text-gray-500 uppercase mb-1.5 block tracking-widest">{t('gerrit.server.token')}</label>
            <div className="relative group">
                <ShieldCheck className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600 group-focus-within:text-purple-400 transition-colors" size={14} />
                <input 
                    type={showToken ? "text" : "password"}
                    value={config.token}
                    onChange={e => setConfig({ ...config, token: e.target.value })}
                        placeholder={t('gerrit.server.password_placeholder')}
                    className="w-full bg-editor-bg border border-editor-line rounded pl-9 pr-10 py-2 text-sm text-white focus:outline-none focus:border-purple-500 transition-all font-mono"
                />
                <button 
                    onClick={() => setShowToken(!showToken)}
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-600 hover:text-gray-400"
                >
                    {showToken ? <EyeOff size={14} /> : <Eye size={14} />}
                </button>
            </div>
            <p className="text-[10px] text-gray-600 mt-1.5 flex items-center gap-1">
                <Zap size={10} /> {t('gerrit.server.token_tip')}
            </p>
        </div>
      </div>

      {testResult && (
          <div className={`p-3 rounded-lg flex items-start gap-2 border animate-fade-in ${testResult.success ? 'bg-green-900/10 border-green-500/30 text-green-400' : 'bg-red-900/10 border-red-500/30 text-red-400'}`}>
              {testResult.success ? <ShieldCheck size={16} className="shrink-0" /> : <AlertCircle size={16} className="shrink-0" />}
              <span className="text-[11px] font-medium">{testResult.message}</span>
          </div>
      )}

      <div className="flex justify-between items-center pt-4 border-t border-editor-line">
          <button 
            onClick={handleTest}
            disabled={testing || !config.url}
            className="flex items-center gap-2 px-3 py-1.5 rounded text-xs text-gray-400 hover:text-white hover:bg-editor-line transition-all disabled:opacity-30"
          >
              {testing ? <Loader2 size={12} className="animate-spin" /> : <Link2 size={12} />} {t('gerrit.server.test')}
          </button>
          <div className="flex gap-2">
            <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-400">{t('gerrit.server.cancel')}</button>
            <button 
                onClick={handleSave}
                disabled={testing || !config.url || !config.token || !config.name}
                className="bg-purple-600 hover:bg-purple-500 text-white px-6 py-1.5 rounded text-xs font-bold transition-colors shadow-lg shadow-purple-900/20 disabled:opacity-50"
            >
                {testing ? t('gerrit.server.testing') : t('gerrit.server.save')}
            </button>
          </div>
      </div>
    </div>
  );
};

export default GerritServerModal;
