import React, { useState, useCallback, useEffect, useMemo } from 'react';
import TitleBar from './components/TitleBar';
import ToolBar from './components/ToolBar';
import TaskTree from './components/TaskTree';
import DiffView from './components/DiffView';
import RightPanel from './components/RightPanel';
import ActionBar from './components/ActionBar';
import StatusBar from './components/StatusBar';
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
import TourGuide from './components/TourGuide';
import { useTranslation } from './i18n';
import { useApiClient } from './api/client';

const App: React.FC = () => {
  const { t } = useTranslation();
  const apiClient = useApiClient();
  const [activeTaskId, setActiveTaskId] = useState('1');
  const [notification, setNotification] = useState<string | null>(null);

  // Repository & Diff Context State
  const [isRepoLoaded, setIsRepoLoaded] = useState(false);
  const [selectedRepoPath, setSelectedRepoPath] = useState<string | null>(null);
  const [repoRefreshKey, setRepoRefreshKey] = useState(0); // 用于触发数据刷新
  const [diffContext, setDiffContext] = useState({
    base: 'master',
    head: 'feature/payment-retry',
  });

  // Current File Context
  const [activeFilePath, setActiveFilePath] = useState('src/main/OrderService.java');
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const activeFileExtension = useMemo(() => {
    const parts = activeFilePath.split('.');
    return parts.length > 1 ? `.${parts.pop()}` : '';
  }, [activeFilePath]);

  // Debug: Log state changes
  useEffect(() => {
    console.log('[App] selectedFile changed:', selectedFile);
  }, [selectedFile]);

  useEffect(() => {
    console.log('[App] activeFilePath changed:', activeFilePath);
  }, [activeFilePath]);

  // Layout State
  const [leftWidth, setLeftWidth] = useState(260);
  const [rightWidth, setRightWidth] = useState(300);
  const [showLeft, setShowLeft] = useState(true);
  const [showRight, setShowRight] = useState(true);
  const [isResizing, setIsResizing] = useState<'left' | 'right' | null>(null);

  // Modal States
  const [openRepoModalOpen, setOpenRepoModalOpen] = useState(false);
  const [newTaskModalOpen, setNewTaskModalOpen] = useState(false);
  const [newTaskInitialTab, setNewTaskInitialTab] = useState<'import' | 'create'>('import');

  const [searchOpen, setSearchOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [submitOpen, setSubmitOpen] = useState(false);
  const [reviewModal, setReviewModal] = useState<{
    isOpen: boolean;
    type: ReviewType;
    taskId?: string;
    fileId?: string;
    filePath?: string;
    lineNumber?: number;
  }>({ isOpen: false, type: 'comment' });
  const [tagManagerModalOpen, setTagManagerModalOpen] = useState(false);
  const [syncStatusModalOpen, setSyncStatusModalOpen] = useState(false);
  const [branchCompareModalOpen, setBranchCompareModalOpen] = useState(false);
  const [isInitialSetup, setIsInitialSetup] = useState(false);

  // Tour State
  const [tourOpen, setTourOpen] = useState(false);

  // Initial Load Effect
  useEffect(() => {
    if (!isRepoLoaded) {
      const timer = setTimeout(() => setOpenRepoModalOpen(true), 100);
      return () => clearTimeout(timer);
    }
  }, [isRepoLoaded]);

  // Resize Logic
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;
      e.preventDefault();
      if (isResizing === 'left') {
        const newWidth = e.clientX;
        if (newWidth >= 150 && newWidth <= 600) setLeftWidth(newWidth);
      } else if (isResizing === 'right') {
        const newWidth = window.innerWidth - e.clientX;
        if (newWidth >= 200 && newWidth <= 800) setRightWidth(newWidth);
      }
    };
    const handleMouseUp = () => {
      setIsResizing(null);
      document.body.style.cursor = 'default';
      document.body.style.userSelect = 'auto';
    };
    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = 'col-resize';
      document.body.style.userSelect = 'none';
    }
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isResizing]);

  const toggleLeft = useCallback(() => setShowLeft((prev) => !prev), []);
  const toggleRight = useCallback(() => setShowRight((prev) => !prev), []);
  const isMaximized = !showLeft && !showRight;

  const toggleMaximize = useCallback(() => {
    if (isMaximized) {
      setShowLeft(true);
      setShowRight(true);
    } else {
      setShowLeft(false);
      setShowRight(false);
    }
  }, [isMaximized]);

  const showNotification = useCallback((message: string) => {
    setNotification(message);
    const timer = setTimeout(() => setNotification(null), 2000);
    return () => clearTimeout(timer);
  }, []);

  const handleOpenRepo = async (path: string) => {
    setSelectedRepoPath(path);
    setOpenRepoModalOpen(false);
    setIsInitialSetup(true);
    setBranchCompareModalOpen(true);
    // 仓库路径已设置，BranchCompareModal 会自动加载仓库并获取分支
  };

  const handleBackToRepoSelection = () => {
    setBranchCompareModalOpen(false);
    setTimeout(() => setOpenRepoModalOpen(true), 50);
  };

  const handleApplyBranchCompare = (base: string, head: string) => {
    setDiffContext({ base, head });
    if (isInitialSetup) {
      setIsRepoLoaded(true);
      setIsInitialSetup(false);
      showNotification(`Repository Loaded: ${selectedRepoPath} (${base} ← ${head})`);
    } else {
      showNotification(`Comparing ${base} ← ${head}`);
    }
    // 重置当前文件为默认值，避免显示不存在的文件
    setActiveFilePath('src/main/OrderService.java');
    // 刷新所有相关数据
    setRepoRefreshKey((prev) => prev + 1);
    setBranchCompareModalOpen(false);
  };

  const handleNewTask = (tab: 'import' | 'create' = 'import') => {
    setNewTaskInitialTab(tab);
    setNewTaskModalOpen(true);
  };

  const handleImportTask = (id: string) => {
    showNotification(`Task imported: ${id}`);
    setNewTaskModalOpen(false);
  };

  const handleCreateTask = async (task: { title: string; type: string; files: string[] }) => {
    try {
      // 调用后端 API 创建本地任务
      await apiClient.createLocalTask(task.title, task.type, task.files);
      showNotification(
        `Task created: ${task.title} (${task.type}) with ${task.files.length} files`,
      );
      setNewTaskModalOpen(false);

      // 刷新本地任务列表
      setRepoRefreshKey((prev) => prev + 1);
    } catch (error) {
      console.error('Failed to create local task:', error);
      showNotification(`Failed to create task: ${error}`);
    }
  };

  const handleNavigate = (target: string, type?: 'file' | 'command') => {
    if (target === 'close') {
      setSearchOpen(false);
      return;
    }

    if (type === 'file') {
      // Handle file navigation - update both state variables
      setSelectedFile(target);
      setActiveFilePath(target);
      showNotification(`Opened file: ${target}`);
      setSearchOpen(false);
    } else if (type === 'command') {
      // Handle command execution
      showNotification(`Executing command: ${target}`);
      setSearchOpen(false);
    } else {
      // Default behavior
      showNotification(`Navigated to: ${target}`);
      setSearchOpen(false);
    }
  };

  const handleReviewSubmit = async (
    text: string,
    type: ReviewType,
    taskId?: string,
    fileId?: string,
  ) => {
    // If taskId and fileId are provided, this is a file review action from TaskTree
    if (taskId && fileId) {
      try {
        // Map 'approved' to 'approved', 'reject' to 'must_change'
        const reviewStatus =
          type === 'reject' ? 'must_change' : (type as 'approved' | 'concern' | 'question');
        await apiClient.updateFileReviewStatus(
          taskId,
          fileId,
          reviewStatus,
          text || undefined,
          'Reviewer',
        );
        showNotification(
          `文件审核状态已更新: ${type === 'approved' ? '通过' : type === 'concern' ? '关注' : type === 'reject' ? '必须修改' : '提问'}`,
        );
        // Trigger a refresh of local tasks
        setRepoRefreshKey((prev) => prev + 1);
      } catch (error) {
        console.error('[handleReviewSubmit] Failed to update file review status:', error);
        showNotification('更新失败，请重试');
      }
    } else {
      // Regular review comment (for ActionBar)
      showNotification(`${type.toUpperCase()} posted: ${text.substring(0, 20)}...`);
    }
    setReviewModal({ ...reviewModal, isOpen: false });
  };

  const handleFinalSubmit = () => {
    showNotification('Review Submitted Successfully!');
    setSubmitOpen(false);
  };

  const handleAction = (msg: string) => {
    console.log('[handleAction] Received action:', msg);
    if (msg === 'Global Search Activated') {
      setSearchOpen(true);
      return;
    }
    if (msg === 'Settings Opened') {
      setSettingsOpen(true);
      return;
    }
    if (msg === 'Concern Marked') {
      setReviewModal({ isOpen: true, type: 'concern' });
      return;
    }
    if (msg === 'Rejection Recorded') {
      setReviewModal({ isOpen: true, type: 'reject' });
      return;
    }
    if (msg === 'Question Mode Activated') {
      setReviewModal({ isOpen: true, type: 'question' });
      return;
    }
    if (msg === 'Comment Box Opened') {
      setReviewModal({ isOpen: true, type: 'comment' });
      return;
    }
    if (msg.includes('Submitting Review')) {
      setSubmitOpen(true);
      return;
    }

    // File review actions from TaskTree
    if (msg.includes('FileReviewAction:')) {
      const parts = msg.replace('FileReviewAction:', '').split(':');
      if (parts.length >= 4) {
        const [type, taskId, fileId, filePath] = parts;
        setReviewModal({
          isOpen: true,
          type: type as ReviewType,
          taskId,
          fileId,
          filePath,
        });
        return;
      }
    }

    if (msg.includes('File selected: ')) {
      const filePath = msg.replace('File selected: ', '');
      console.log('[App] File selected, setting paths:', filePath);
      setActiveFilePath(filePath);
      setSelectedFile(filePath);
      return;
    }
    if (msg.includes('Opening Diff: ')) {
      const filePath = msg.replace('Opening Diff: ', '');
      setActiveFilePath(filePath);
      setSelectedFile(filePath);
      return;
    }
    if (msg === 'Creating Local Task...') {
      handleNewTask('create');
      return;
    }
    if (msg === 'Opening New Task Modal...') {
      handleNewTask('import');
      return;
    }
    if (msg === 'Importing Task...') {
      handleNewTask('import');
      return;
    }
    if (msg === 'Opening Tag Manager...') {
      setTagManagerModalOpen(true);
      return;
    }
    if (msg === 'Syncing with Remote...') {
      setSyncStatusModalOpen(true);
      return;
    }
    if (msg === 'Start Tour') {
      setTourOpen(true);
      return;
    }
    if (msg === 'Switching Branch...') {
      console.log('[handleAction] Opening branch compare modal, isRepoLoaded:', isRepoLoaded);
      setIsInitialSetup(false);
      setBranchCompareModalOpen(true);
      console.log('[handleAction] branchCompareModalOpen set to true');
      return;
    }
    showNotification(msg);
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-editor-bg text-editor-fg font-mono overflow-hidden relative">
      <TitleBar onAction={handleAction} />
      <ToolBar
        onAction={handleAction}
        onOpenRepo={() => setOpenRepoModalOpen(true)}
        onNewTask={() => handleNewTask('import')}
        showLeft={showLeft}
        showRight={showRight}
        onToggleLeft={toggleLeft}
        onToggleRight={toggleRight}
        diffContext={diffContext}
      />

      <div className="flex-1 flex overflow-hidden relative pb-[28px]">
        {showLeft && (
          <div
            style={{ width: leftWidth }}
            className={`shrink-0 h-full flex flex-col ${isResizing ? '' : 'transition-all duration-300 ease-in-out'}`}
          >
            <TaskTree
              activeTaskId={activeTaskId}
              onSelectTask={setActiveTaskId}
              onAction={handleAction}
              repoRefreshKey={repoRefreshKey}
              onSelectFile={setSelectedFile}
              selectedFile={selectedFile}
            />
          </div>
        )}

        {showLeft && (
          <div
            className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -ml-[1px] hover:-ml-[2px] transition-all duration-100 delay-100 flex items-center justify-center group"
            onMouseDown={() => setIsResizing('left')}
          />
        )}

        <div className="flex-1 min-w-0 h-full border-r border-editor-line relative pb-[56px] flex flex-col">
          <DiffView
            isMaximized={isMaximized}
            toggleMaximize={toggleMaximize}
            onAction={handleAction}
            diffContext={diffContext}
            selectedFile={selectedFile}
            activeFilePath={activeFilePath}
          />
          <ActionBar onAction={handleAction} />
        </div>

        {showRight && (
          <div
            className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -mr-[1px] hover:-mr-[2px] transition-all duration-100 delay-100 flex items-center justify-center group"
            onMouseDown={() => setIsResizing('right')}
          />
        )}

        {showRight && (
          <div
            style={{ width: rightWidth }}
            className={`shrink-0 h-full flex flex-col ${isResizing ? '' : 'transition-all duration-300 ease-in-out'}`}
          >
            <RightPanel
              onAction={handleAction}
              activeFileExtension={activeFileExtension}
              repoRefreshKey={repoRefreshKey}
              onSelectFile={setSelectedFile}
              selectedFile={selectedFile}
              diffContext={diffContext}
            />
          </div>
        )}
      </div>

      <div className="absolute bottom-0 w-full z-50">
        <StatusBar />
      </div>

      <Modal
        isOpen={openRepoModalOpen}
        onClose={() => {
          if (isRepoLoaded) setOpenRepoModalOpen(false);
        }}
        title={t('modal.open_repo.step1')}
      >
        <OpenRepoModal
          onClose={() => {
            if (isRepoLoaded) setOpenRepoModalOpen(false);
          }}
          onOpen={handleOpenRepo}
        />
      </Modal>

      <Modal
        isOpen={newTaskModalOpen}
        onClose={() => setNewTaskModalOpen(false)}
        title={t('modal.new_task.title')}
      >
        <NewTaskModal
          onClose={() => setNewTaskModalOpen(false)}
          onImport={handleImportTask}
          onCreate={handleCreateTask}
          initialTab={newTaskInitialTab}
        />
      </Modal>

      <Modal isOpen={searchOpen} onClose={() => setSearchOpen(false)} title="Go to File or Command">
        <CommandPalette onClose={() => setSearchOpen(false)} onNavigate={handleNavigate} />
      </Modal>

      <Modal
        isOpen={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        title={t('modal.settings.title')}
      >
        <SettingsModal onClose={() => setSettingsOpen(false)} />
      </Modal>

      <Modal
        isOpen={reviewModal.isOpen}
        onClose={() => setReviewModal({ ...reviewModal, isOpen: false })}
        title="Add Review Comment"
      >
        <ReviewActionModal
          type={reviewModal.type}
          taskId={reviewModal.taskId}
          fileId={reviewModal.fileId}
          filePath={reviewModal.filePath || activeFilePath}
          onClose={() => setReviewModal({ ...reviewModal, isOpen: false })}
          onSubmit={handleReviewSubmit}
        />
      </Modal>

      <Modal
        isOpen={submitOpen}
        onClose={() => setSubmitOpen(false)}
        title={t('modal.submit.title')}
      >
        <SubmitReviewModal onClose={() => setSubmitOpen(false)} onSubmit={handleFinalSubmit} />
      </Modal>

      <Modal
        isOpen={tagManagerModalOpen}
        onClose={() => setTagManagerModalOpen(false)}
        title={t('modal.tag_manager.title')}
      >
        <TagManagerModal onClose={() => setTagManagerModalOpen(false)} />
      </Modal>

      <Modal
        isOpen={syncStatusModalOpen}
        onClose={() => setSyncStatusModalOpen(false)}
        title={t('modal.sync.title')}
      >
        <SyncStatusModal onClose={() => setSyncStatusModalOpen(false)} />
      </Modal>

      <Modal
        isOpen={branchCompareModalOpen}
        onClose={() => {
          if (isRepoLoaded) setBranchCompareModalOpen(false);
        }}
        title={t('modal.branch_compare.title')}
      >
        <BranchCompareModal
          currentBase={diffContext.base}
          currentHead={diffContext.head}
          isInitialSetup={isInitialSetup}
          onClose={() => {
            if (isRepoLoaded) setBranchCompareModalOpen(false);
          }}
          onApply={handleApplyBranchCompare}
          onBack={handleBackToRepoSelection}
          selectedRepoPath={selectedRepoPath || undefined}
        />
      </Modal>

      <TourGuide isOpen={tourOpen} onClose={() => setTourOpen(false)} />

      {notification && (
        <div className="absolute top-[100px] left-1/2 -translate-x-1/2 bg-editor-selection text-white px-4 py-2 rounded shadow-xl z-[150] animate-fade-in-down border border-editor-accent/50 text-xs font-bold tracking-wide pointer-events-none">
          {notification}
        </div>
      )}
    </div>
  );
};

export default App;
