
import React, { useState, useCallback, useEffect, useMemo } from 'react';
import TitleBar from './components/TitleBar';
import LocalToolBar from './components/LocalToolBar';
import RemoteToolBar from './components/RemoteToolBar';
import LocalTaskTree from './components/LocalTaskTree';
import RemoteTaskTree from './components/RemoteTaskTree';
import DiffView from './components/DiffView';
import LocalRightPanel from './components/LocalRightPanel';
import RemoteRightPanel from './components/RemoteRightPanel';
import ActionBar from './components/ActionBar';
import StatusBar from './components/StatusBar';
import WelcomeView from './components/WelcomeView';
import Modal from './components/Modal';
import OpenRepoModal from './components/OpenRepoModal';
import NewTaskModal from './components/NewTaskModal';
import CommandPalette from './components/CommandPalette';
import SettingsModal from './components/SettingsModal';
import ReviewActionModal, { ReviewType } from './components/ReviewActionModal';
import SubmitReviewModal from './components/SubmitReviewModal';
import TagManagerModal from './components/TagManagerModal';
import SyncStatusModal from './components/SyncStatusModal';
import BranchCompareModal from './components/BranchCompareModal';
import GerritImportModal from './components/GerritImportModal';
import GerritServerModal from './components/GerritServerModal';
import TourGuide from './components/TourGuide';
import { useTranslation } from './i18n';
import { getGerritConfig } from './api/client';

const App: React.FC = () => {
  const { t, fontSize, ligatures, vimMode, language } = useTranslation();
  const [activeTaskId, setActiveTaskId] = useState('1');
  const [notification, setNotification] = useState<string | null>(null);
  const [mode, setMode] = useState<'local' | 'remote'>('local');
  const [isGerritConfigured, setIsGerritConfigured] = useState(false);

  const [isRepoLoaded, setIsRepoLoaded] = useState(false);
  const [selectedRepoPath, setSelectedRepoPath] = useState<string | null>(null);
  const [diffContext, setDiffContext] = useState({ base: 'master', head: 'feature/payment-retry' });

  const [activeFilePath, setActiveFilePath] = useState('src/main/OrderService.java');
  
  const [leftWidth, setLeftWidth] = useState(260);
  const [rightWidth, setRightWidth] = useState(300);
  const [showLeft, setShowLeft] = useState(true);
  const [showRight, setShowRight] = useState(true);
  const [isResizing, setIsResizing] = useState<'left' | 'right' | null>(null);

  const [openRepoModalOpen, setOpenRepoModalOpen] = useState(false);
  const [newTaskModalOpen, setNewTaskModalOpen] = useState(false);
  const [gerritImportOpen, setGerritImportOpen] = useState(false);
  const [gerritServerOpen, setGerritServerOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [submitOpen, setSubmitOpen] = useState(false);
  const [reviewModal, setReviewModal] = useState<{isOpen: boolean, type: ReviewType}>({isOpen: false, type: 'comment'});
  const [tagManagerModalOpen, setTagManagerModalOpen] = useState(false);
  const [syncStatusModalOpen, setSyncStatusModalOpen] = useState(false);
  const [branchCompareModalOpen, setBranchCompareModalOpen] = useState(false);
  const [isInitialSetup, setIsInitialSetup] = useState(false);
  const [tourOpen, setTourOpen] = useState(false);

  // 初始化检查 Gerrit 配置
  useEffect(() => {
    const checkConfig = async () => {
        const config = await getGerritConfig();
        // 如果 Mock 返回了有效的 URL 则视为已配置
        setIsGerritConfigured(!!(config && config.url && config.url.length > 5));
    };
    checkConfig();
  }, []);

  // 全局注入样式块，实时响应 settings 状态
  const dynamicStyles = useMemo(() => `
    :root {
      --editor-font-size: ${fontSize}px;
      --editor-ligatures: ${ligatures ? 'normal' : 'none'};
      --editor-cursor: ${vimMode ? 'block' : 'auto'};
    }
    pre[class*="language-"], 
    code[class*="language-"], 
    .font-mono, 
    .diff-line-content {
      font-size: var(--editor-font-size) !important;
      font-variant-ligatures: var(--editor-ligatures) !important;
    }
    ${vimMode ? '.cursor-text { cursor: var(--editor-cursor) !important; }' : ''}
  `, [fontSize, ligatures, vimMode]);

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return; e.preventDefault();
      if (isResizing === 'left') { const newWidth = e.clientX; if (newWidth >= 150 && newWidth <= 600) setLeftWidth(newWidth); }
      else if (isResizing === 'right') { const newWidth = window.innerWidth - e.clientX; if (newWidth >= 200 && newWidth <= 800) setRightWidth(newWidth); }
    };
    const handleMouseUp = () => { setIsResizing(null); document.body.style.cursor = 'default'; document.body.style.userSelect = 'auto'; };
    if (isResizing) { document.addEventListener('mousemove', handleMouseMove); document.addEventListener('mouseup', handleMouseUp); document.body.style.cursor = 'col-resize'; document.body.style.userSelect = 'none'; }
    return () => { document.removeEventListener('mousemove', handleMouseMove); document.removeEventListener('mouseup', handleMouseUp); };
  }, [isResizing]);

  const showNotification = useCallback((message: string) => {
    setNotification(message);
    const timer = setTimeout(() => setNotification(null), 2500);
    return () => clearTimeout(timer);
  }, []);

  const handleModeChange = (newMode: 'local' | 'remote') => { 
      setMode(newMode); 
      showNotification(`Switched to ${newMode.toUpperCase()} mode`); 
  };
  
  const handleGerritImport = (id: string) => { 
      setGerritImportOpen(false); 
      setActiveTaskId(`gerrit-${id}`); 
      setIsRepoLoaded(true); 
      setMode('remote');
      showNotification(`Gerrit Change #${id} Imported Successfully`); 
  };

  const startGerritFlow = async () => {
      const config = await getGerritConfig();
      if (!config || !config.url || config.url.length < 5) {
          setGerritServerOpen(true);
      } else {
          setGerritImportOpen(true);
      }
  };

  const handleAction = (msg: string) => {
    if (msg === "Global Search Activated") { setSearchOpen(true); return; }
    if (msg === "Gerrit Import Dialog") { setGerritImportOpen(true); return; }
    if (msg === "Gerrit Server Settings") { setGerritServerOpen(true); return; }
    if (msg === "Settings Opened") { setSettingsOpen(true); return; }
    if (msg === "Concern Marked") { setReviewModal({ isOpen: true, type: 'concern' }); return; }
    if (msg === "Rejection Recorded") { setReviewModal({ isOpen: true, type: 'reject' }); return; }
    if (msg === "Question Mode Activated") { setReviewModal({ isOpen: true, type: 'question' }); return; }
    if (msg === "Comment Box Opened") { setReviewModal({ isOpen: true, type: 'comment' }); return; }
    if (msg.includes("Submitting Review")) { setSubmitOpen(true); return; }
    if (msg.includes("File selected: ")) { setActiveFilePath(msg.replace("File selected: ", "")); return; }
    if (msg.includes("Opening Diff: ")) { setActiveFilePath(msg.replace("Opening Diff: ", "")); return; }
    if (msg === "Opening Tag Manager...") { setTagManagerModalOpen(true); return; }
    if (msg === "Syncing with Remote...") { setSyncStatusModalOpen(true); return; }
    if (msg === "Start Tour") { setTourOpen(true); return; }
    if (msg === "Switching Branch...") { setIsInitialSetup(false); setBranchCompareModalOpen(true); return; }
    showNotification(msg);
  };

  return (
    <div className={`flex flex-col h-screen w-screen bg-editor-bg text-editor-fg font-mono overflow-hidden relative ${mode === 'remote' ? 'selection:bg-purple-900/40' : 'selection:bg-editor-selection'}`}>
      <style>{dynamicStyles}</style>

      <TitleBar onAction={handleAction} mode={mode} onModeChange={handleModeChange} />
      
      {isRepoLoaded && (
          mode === 'remote' ? (
              <RemoteToolBar onAction={handleAction} showLeft={showLeft} showRight={showRight} onToggleLeft={() => setShowLeft(!showLeft)} onToggleRight={() => setShowRight(!showRight)} diffContext={diffContext} />
          ) : (
              <LocalToolBar onAction={handleAction} onOpenRepo={() => setOpenRepoModalOpen(true)} onNewTask={() => setNewTaskModalOpen(true)} showLeft={showLeft} showRight={showRight} onToggleLeft={() => setShowLeft(!showLeft)} onToggleRight={() => setShowRight(!showRight)} diffContext={diffContext} />
          )
      )}
      
      {!isRepoLoaded ? (
          <WelcomeView 
            onOpenLocal={() => setOpenRepoModalOpen(true)} 
            onImportRemote={startGerritFlow} 
            isGerritConfigured={isGerritConfigured}
            onSelectHistory={(path, m) => {
                setSelectedRepoPath(path);
                setMode(m);
                setIsRepoLoaded(true);
                showNotification(`Loaded ${path}`);
            }}
          />
      ) : (
          <div className="flex-1 flex overflow-hidden relative pb-[28px]"> 
            {showLeft && (
              <div style={{ width: leftWidth }} className="shrink-0 h-full flex flex-col transition-all duration-300">
                {mode === 'remote' ? (
                    <RemoteTaskTree activeTaskId={activeTaskId} onSelectTask={setActiveTaskId} onAction={handleAction} />
                ) : (
                    <LocalTaskTree activeTaskId={activeTaskId} onSelectTask={setActiveTaskId} onAction={handleAction} />
                )}
              </div>
            )}
            {showLeft && <div className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -ml-[1px]" onMouseDown={() => setIsResizing('left')} />}
            <div className="flex-1 min-w-0 h-full border-r border-editor-line relative pb-[56px] flex flex-col">
              <DiffView isMaximized={!showLeft && !showRight} toggleMaximize={() => { setShowLeft(!showLeft); setShowRight(!showRight); }} onAction={handleAction} diffContext={diffContext} activeFilePath={activeFilePath} mode={mode} />
              <ActionBar onAction={handleAction} />
            </div>
            {showRight && <div className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -mr-[1px]" onMouseDown={() => setIsResizing('right')} />}
            {showRight && (
              <div style={{ width: rightWidth }} className="shrink-0 h-full flex flex-col transition-all duration-300">
                {mode === 'remote' ? (
                    <RemoteRightPanel onAction={handleAction} />
                ) : (
                    <LocalRightPanel onAction={handleAction} />
                )}
              </div>
            )}
          </div>
      )}

      <div className="absolute bottom-0 w-full z-50"><StatusBar mode={mode} /></div>

      {/* Modals remain shared but use mode inside */}
      <Modal isOpen={gerritServerOpen} onClose={() => setGerritServerOpen(false)} title="Gerrit Server Configuration">
        <GerritServerModal 
            onClose={() => setGerritServerOpen(false)} 
            onSuccess={() => { 
                setGerritServerOpen(false); 
                setIsGerritConfigured(true);
                setGerritImportOpen(true); 
                showNotification("Configuration Saved."); 
            }} 
        />
      </Modal>
      <Modal isOpen={gerritImportOpen} onClose={() => setGerritImportOpen(false)} title="Import from Gerrit"><GerritImportModal onClose={() => setGerritImportOpen(false)} onImport={handleGerritImport} onOpenSettings={() => { setGerritImportOpen(false); setGerritServerOpen(true); }} /></Modal>
      <Modal isOpen={openRepoModalOpen} onClose={() => setOpenRepoModalOpen(false)} title={t('modal.open_repo.step1')}><OpenRepoModal onClose={() => setOpenRepoModalOpen(false)} onOpen={(path) => { setSelectedRepoPath(path); setOpenRepoModalOpen(false); setIsInitialSetup(true); setBranchCompareModalOpen(true); }} /></Modal>
      <Modal isOpen={newTaskModalOpen} onClose={() => setNewTaskModalOpen(false)} title={t('modal.new_task.title')}><NewTaskModal onClose={() => setNewTaskModalOpen(false)} onImport={(id) => showNotification(`Task imported: ${id}`)} onCreate={(t) => showNotification(`Task created: ${t.title}`)} /></Modal>
      <Modal isOpen={searchOpen} onClose={() => setSearchOpen(false)} title="Go to File or Command"><CommandPalette onClose={() => setSearchOpen(false)} onNavigate={(target) => { showNotification(`Navigating to ${target}`); setSearchOpen(false); }} /></Modal>
      <Modal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} title={t('modal.settings.title')}><SettingsModal onClose={() => setSettingsOpen(false)} /></Modal>
      <Modal isOpen={reviewModal.isOpen} onClose={() => setReviewModal({ ...reviewModal, isOpen: false })} title="Add Review Comment"><ReviewActionModal type={reviewModal.type} onClose={() => setReviewModal({ ...reviewModal, isOpen: false })} onSubmit={(txt, type) => { showNotification(`${type.toUpperCase()} recorded locally`); setReviewModal({ ...reviewModal, isOpen: false }); }} /></Modal>
      <Modal isOpen={submitOpen} onClose={() => setSubmitOpen(false)} title={mode === 'remote' ? 'Push Review to Gerrit' : t('modal.submit.title')}><SubmitReviewModal onClose={() => setSubmitOpen(false)} mode={mode} onSubmit={(action) => { showNotification(mode === 'remote' ? `Gerrit Review Posted (${action})!` : "Review Submitted Successfully!"); setSubmitOpen(false); }} /></Modal>
      <Modal isOpen={tagManagerModalOpen} onClose={() => setTagManagerModalOpen(false)} title={t('modal.tag_manager.title')}><TagManagerModal onClose={() => setTagManagerModalOpen(false)} /></Modal>
      <Modal isOpen={syncStatusModalOpen} onClose={() => setSyncStatusModalOpen(false)} title={t('modal.sync.title')}><SyncStatusModal onClose={() => setSyncStatusModalOpen(false)} /></Modal>
      <Modal isOpen={branchCompareModalOpen} onClose={() => setBranchCompareModalOpen(false)} title={t('modal.branch_compare.title')}><BranchCompareModal currentBase={diffContext.base} currentHead={diffContext.head} isInitialSetup={isInitialSetup} onClose={() => setBranchCompareModalOpen(false)} onApply={(b, h) => { setDiffContext({base:b, head:h}); setIsRepoLoaded(true); setIsInitialSetup(false); setBranchCompareModalOpen(false); }} /></Modal>
      <TourGuide isOpen={tourOpen} onClose={() => setTourOpen(false)} />
      {notification && (<div className={`absolute top-[100px] left-1/2 -translate-x-1/2 px-4 py-2 rounded shadow-xl z-[150] animate-fade-in-down border text-xs font-bold tracking-wide pointer-events-none ${mode === 'remote' ? 'bg-purple-900 text-white border-purple-400/50' : 'bg-editor-selection text-white border-editor-accent/50'}`}>{notification}</div>)}
    </div>
  );
};

export default App;
