
import React, { useState } from 'react';
import { Upload, GitPullRequest, FileText, Database, Shield, Plus, FileSpreadsheet, X } from 'lucide-react';
import { useTranslation } from '../i18n';

interface NewTaskModalProps {
  onClose: () => void;
  onImport: (id: string) => void;
  onCreate: (task: { title: string; type: string; files: string[] }) => void;
  initialTab?: 'import' | 'create';
}

const NewTaskModal: React.FC<NewTaskModalProps> = ({ onClose, onImport, onCreate, initialTab = 'import' }) => {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<'import' | 'create'>(initialTab);

  // Import State
  const [importValue, setImportValue] = useState('');
  const [isDragging, setIsDragging] = useState(false);

  // Create State
  const [createTitle, setCreateTitle] = useState('');
  const [createType, setCreateType] = useState('code');
  const [filesInput, setFilesInput] = useState(''); // 文本输入的文件列表
  const [fileList, setFileList] = useState<string[]>([]); // 解析后的文件列表

  // 解析文件输入（支持文本框输入，每行一个文件路径）
  const parseFilesInput = (input: string): string[] => {
    return input
      .split('\n')
      .map(line => line.trim())
      .filter(line => line.length > 0);
  };

  // 处理文件输入变化
  const handleFilesInputChange = (value: string) => {
    setFilesInput(value);
    setFileList(parseFilesInput(value));
  };

  // 处理 CSV/Excel 文件上传（模拟，实际需要后端支持）
  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const content = event.target?.result as string;
      // 简单的 CSV 解析：假设第一列是文件路径
      const lines = content.split('\n');
      const filePaths = lines
        .slice(1) // 跳过标题行
        .map(line => {
          const parts = line.split(',');
          return parts[0]?.trim().replace(/^["']|["']$/g, '') || '';
        })
        .filter(path => path.length > 0);

      setFilesInput(filePaths.join('\n'));
      setFileList(filePaths);
    };
    reader.readAsText(file);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = () => {
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    setImportValue("patch-from-file-v2.diff");
  };

  // 删除单个文件
  const removeFile = (index: number) => {
    const newFiles = fileList.filter((_, i) => i !== index);
    setFileList(newFiles);
    setFilesInput(newFiles.join('\n'));
  };

  return (
    <div className="flex flex-col gap-4">
      {/* Tab Header */}
      <div className="flex border-b border-editor-line mb-2">
          <button
              onClick={() => setActiveTab('import')}
              className={`flex-1 py-2 text-xs font-bold uppercase transition-colors border-b-2 ${activeTab === 'import' ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}>
              {t('modal.new_task.tab_import')}
          </button>
          <button
              onClick={() => setActiveTab('create')}
              className={`flex-1 py-2 text-xs font-bold uppercase transition-colors border-b-2 ${activeTab === 'create' ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}>
              {t('modal.new_task.tab_create')}
          </button>
      </div>

      {activeTab === 'import' && (
        <div className="flex flex-col gap-4 animate-fade-in">
             <div>
                <label className="text-xs text-gray-400 mb-2 block font-medium">{t('modal.import_task.label')}</label>
                <div className="relative">
                    <GitPullRequest className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
                    <input
                    type="text"
                    value={importValue}
                    onChange={(e) => setImportValue(e.target.value)}
                    placeholder="e.g. PR#2899, GH-123, or https://gitlab.com/..."
                    className="w-full bg-editor-line/50 border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
                    autoFocus
                    />
                </div>
            </div>

            <div className="flex items-center gap-2 my-1">
                <div className="h-[1px] bg-editor-line flex-1"></div>
                <span className="text-[10px] text-gray-500 uppercase font-bold">{t('modal.import_task.or')}</span>
                <div className="h-[1px] bg-editor-line flex-1"></div>
            </div>

            <div
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
                onDrop={handleDrop}
                className={`border-2 border-dashed rounded-lg p-6 flex flex-col items-center justify-center gap-3 transition-all cursor-pointer group
                    ${isDragging ? 'border-editor-accent bg-editor-accent/10' : 'border-editor-line hover:border-editor-accent/50 hover:bg-editor-line/30'}`}
            >
                <div className={`p-3 rounded-full ${isDragging ? 'bg-editor-accent text-white' : 'bg-editor-line text-gray-400 group-hover:text-editor-accent'}`}>
                    <Upload size={20} />
                </div>
                <div className="text-center">
                    <span className="text-xs text-editor-fg block font-medium">{t('modal.import_task.drag')}</span>
                    <span className="text-[10px] text-gray-500 block mt-1">{t('modal.import_task.supports')}</span>
                </div>
            </div>

            <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-1">
                <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.import_task.cancel')}</button>
                <button
                onClick={() => importValue && onImport(importValue)}
                disabled={!importValue}
                className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm"
                >
                {t('modal.import_task.import')}
                </button>
            </div>
        </div>
      )}

      {activeTab === 'create' && (
         <div className="flex flex-col gap-4 animate-fade-in">
            <div>
                <label className="text-xs text-gray-400 mb-1 block font-medium">{t('modal.create_task.label_title')}</label>
                <input
                type="text"
                value={createTitle}
                onChange={e => setCreateTitle(e.target.value)}
                placeholder="e.g. Q3 Performance Review"
                className="w-full bg-editor-line/50 border border-editor-line rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-editor-accent transition-colors placeholder-gray-600"
                autoFocus
                />
            </div>

            <div>
                <label className="text-xs text-gray-400 mb-2 block font-medium">{t('modal.create_task.label_type')}</label>
                <div className="grid grid-cols-3 gap-3">
                    <div
                        onClick={() => setCreateType('code')}
                        className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${createType === 'code' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                        <FileText size={20} />
                        <span className="text-xs font-medium">{t('modal.create_task.type_code')}</span>
                    </div>
                    <div
                        onClick={() => setCreateType('sql')}
                        className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${createType === 'sql' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                        <Database size={20} />
                        <span className="text-xs font-medium">{t('modal.create_task.type_sql')}</span>
                    </div>
                    <div
                        onClick={() => setCreateType('security')}
                        className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${createType === 'security' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                        <Shield size={20} />
                        <span className="text-xs font-medium">{t('modal.create_task.type_security')}</span>
                    </div>
                </div>
            </div>

            {/* 文件清单输入区域 */}
            <div>
                <label className="text-xs text-gray-400 mb-1 block font-medium">文件清单（可选）</label>

                {/* 文件上传按钮 */}
                <div className="flex items-center gap-2 mb-2">
                    <label className="flex items-center gap-2 px-3 py-1.5 bg-editor-line/30 border border-editor-line rounded cursor-pointer hover:bg-editor-line/50 hover:border-editor-accent/50 transition-all text-xs text-gray-400 hover:text-gray-300">
                        <FileSpreadsheet size={14} />
                        <span>上传 CSV/Excel</span>
                        <input
                            type="file"
                            accept=".csv,.xlsx,.xls"
                            onChange={handleFileUpload}
                            className="hidden"
                        />
                    </label>
                    <span className="text-[10px] text-gray-600">或</span>
                    <span className="text-[10px] text-gray-600">直接输入文件路径（每行一个）</span>
                </div>

                {/* 文本输入框 */}
                <textarea
                    value={filesInput}
                    onChange={(e) => handleFilesInputChange(e.target.value)}
                    placeholder="src/main.ts&#10;src/components/Header.tsx&#10;src/utils/helpers.ts"
                    className="w-full bg-editor-line/50 border border-editor-line rounded px-3 py-2 text-xs text-white placeholder-gray-600 focus:outline-none focus:border-editor-accent transition-colors resize-none"
                    rows={4}
                />

                {/* 已添加的文件列表 */}
                {fileList.length > 0 && (
                    <div className="mt-2 p-2 bg-editor-line/20 rounded border border-editor-line/30">
                        <div className="text-[10px] text-gray-500 mb-1 flex items-center justify-between">
                            <span>已添加 {fileList.length} 个文件</span>
                            <button
                                onClick={() => {
                                    setFilesInput('');
                                    setFileList([]);
                                }}
                                className="text-editor-error hover:underline"
                            >
                                清空全部
                            </button>
                        </div>
                        <div className="max-h-24 overflow-y-auto space-y-1">
                            {fileList.map((file, index) => (
                                <div
                                    key={index}
                                    className="flex items-center gap-2 text-xs text-gray-400 group"
                                >
                                    <span className="flex-1 truncate font-mono">{file}</span>
                                    <button
                                        onClick={() => removeFile(index)}
                                        className="opacity-0 group-hover:opacity-100 text-gray-600 hover:text-editor-error transition-all"
                                    >
                                        <X size={12} />
                                    </button>
                                </div>
                            ))}
                        </div>
                    </div>
                )}
            </div>

            <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-2">
                <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.create_task.cancel')}</button>
                <button
                    onClick={() => {
                        if (createTitle) onCreate({ title: createTitle, type: createType, files: fileList });
                    }}
                    disabled={!createTitle}
                    className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 font-medium transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
                    <Plus size={14} /> {t('modal.create_task.create')}
                </button>
            </div>
         </div>
      )}
    </div>
  );
};

export default NewTaskModal;
