import React, { useState, useCallback, useEffect } from 'react';
import TitleBar from './components/TitleBar';
import ToolBar from './components/ToolBar';
import TaskTree from './components/TaskTree';
import DiffView from './components/DiffView';
import RightPanel from './components/RightPanel';
import ActionBar from './components/ActionBar';
import StatusBar from './components/StatusBar';
import Modal from './components/Modal';
import OpenRepoModal from './components/OpenRepoModal';
import ImportTaskModal from './components/ImportTaskModal';
import CommandPalette from './components/CommandPalette';
import SettingsModal from './components/SettingsModal';
import ReviewActionModal, { ReviewType } from './components/ReviewActionModal';
import SubmitReviewModal from './components/SubmitReviewModal';
import CreateTaskModal from './components/CreateTaskModal';
import TagManagerModal from './components/TagManagerModal';
import SyncStatusModal from './components/SyncStatusModal';
import BranchCompareModal from './components/BranchCompareModal';
import TourGuide from './components/TourGuide';

// New repository components
import RepositorySelector from './components/RepositorySelector';

// Error handling and loading
import ToastContainer from './components/ToastContainer';
import ErrorBoundary from './components/ErrorBoundary';
import { LoadingProvider } from './context/LoadingContext';

// Repository hooks
import { useRepositoryActions } from './hooks/useRepository';
import { useRepositoryStatus } from './hooks/useRepository';

// Toast notifications
import { showSuccess } from './utils/errorHandler';

// Types
import type { Repository } from './api/types';

const App: React.FC = () => {
  const [activeTaskId, setActiveTaskId] = useState('1');

  // Repository state using hooks
  const {
    currentBranch,
    openRepository
  } = useRepositoryActions();

  const { isRepoLoaded } = useRepositoryStatus();

  // Diff context state
  const [diffContext, setDiffContext] = useState({
    base: currentBranch?.name || 'main',
    head: currentBranch?.name || 'main'
  });

  // Selected file state
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  // Update diff context when current branch changes
  useEffect(() => {
    if (currentBranch) {
      setDiffContext(prev => ({
        ...prev,
        base: prev.base || currentBranch.name,
        head: currentBranch.name
      }));
    }
  }, [currentBranch]);

  // Layout State
  const [leftWidth, setLeftWidth] = useState(260);
  const [rightWidth, setRightWidth] = useState(300);
  const [showLeft, setShowLeft] = useState(true);
  const [showRight, setShowRight] = useState(true);
  const [isResizing, setIsResizing] = useState<'left' | 'right' | null>(null);

  // Modal States
  const [openRepoModalOpen, setOpenRepoModalOpen] = useState(false);
  const [repositorySelectorOpen, setRepositorySelectorOpen] = useState(false);
  const [importTaskModalOpen, setImportTaskModalOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [submitOpen, setSubmitOpen] = useState(false);
  const [reviewModal, setReviewModal] = useState<{isOpen: boolean, type: ReviewType}>({isOpen: false, type: 'comment'});
  const [createTaskModalOpen, setCreateTaskModalOpen] = useState(false);
  const [tagManagerModalOpen, setTagManagerModalOpen] = useState(false);
  const [syncStatusModalOpen, setSyncStatusModalOpen] = useState(false);
  const [branchCompareModalOpen, setBranchCompareModalOpen] = useState(false);
  const [isInitialSetup, setIsInitialSetup] = useState(false);

  // Tour State
  const [tourOpen, setTourOpen] = useState(false);

  // Initial Load Effect
  useEffect(() => {
    // If no repo is loaded on startup, show repository selector
    if (!isRepoLoaded) {
      const timer = setTimeout(() => setRepositorySelectorOpen(true), 100);
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
        if (newWidth >= 150 && newWidth <= 600) {
          setLeftWidth(newWidth);
        }
      } else if (isResizing === 'right') {
        const newWidth = window.innerWidth - e.clientX;
        if (newWidth >= 200 && newWidth <= 800) {
          setRightWidth(newWidth);
        }
      }
    };

    const handleMouseUp = () => {
      setIsResizing(null);
      document.body.style.cursor = 'default';
      // Re-enable selection
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
      document.body.style.cursor = 'default';
      document.body.style.userSelect = 'auto';
    };
  }, [isResizing]);

  const toggleLeft = useCallback(() => setShowLeft(prev => !prev), []);
  const toggleRight = useCallback(() => setShowRight(prev => !prev), []);

  // Derived maximize state: if both panels are hidden, we are maximized
  const isMaximized = !showLeft && !showRight;

  const toggleMaximize = useCallback(() => {
    if (isMaximized) {
      // Restore both (or just defaults)
      setShowLeft(true);
      setShowRight(true);
    } else {
      // Maximize
      setShowLeft(false);
      setShowRight(false);
    }
  }, [isMaximized]);

  // Notification handler (legacy - using toast system now)
  const showNotification = useCallback((message: string) => {
    // Use toast notification instead
    console.log('Notification:', message);
  }, []);

  // --- Repository Event Handlers ---

  // Handle repository selection from RepositorySelector
  const handleRepositorySelected = useCallback((repository: Repository) => {
    const repoName = repository.path.split('/').pop() || repository.path;
    showSuccess(`Repository loaded: ${repoName}`);
    setRepositorySelectorOpen(false);
    setIsInitialSetup(false);
  }, []);

  // Handle opening new repository
  const handleOpenRepo = useCallback(async () => {
    const repository = await openRepository();
    if (repository) {
      handleRepositorySelected(repository);
    }
  }, [openRepository, handleRepositorySelected]);

  // --- Handlers for Open Repo Wizard (Legacy) ---

  // Back from Step 2 -> Step 1
  const handleBackToRepoSelection = () => {
    setBranchCompareModalOpen(false);
    setTimeout(() => setRepositorySelectorOpen(true), 50);
  };

  // Step 2: Select Branches -> Load UI
  const handleApplyBranchCompare = (base: string, head: string) => {
    setDiffContext({ base, head });

    if (isInitialSetup) {
      setIsInitialSetup(false);
      showSuccess(`Repository configured: ${base} ← ${head}`);
    } else {
      showSuccess(`Comparing ${base} ← ${head}`);
    }

    setBranchCompareModalOpen(false);
  };

  // --- Other Handlers ---

  const handleImportTask = (id: string) => {
    showNotification(`Task imported: ${id}`);
    setImportTaskModalOpen(false);
  };

  const handleNavigate = (target: string) => {
    showNotification(`Navigated to: ${target}`);
    setSearchOpen(false);
  };

  const handleReviewSubmit = (text: string, type: ReviewType) => {
    showNotification(`${type.toUpperCase()} posted: ${text.substring(0, 20)}...`);
    setReviewModal({ ...reviewModal, isOpen: false });
  };

  const handleFinalSubmit = () => {
    showNotification("Review Submitted Successfully!");
    setSubmitOpen(false);
  };

  const handleCreateTask = (task: { title: string; type: string }) => {
    showNotification(`Task created: ${task.title} (${task.type})`);
    setCreateTaskModalOpen(false);
  };

  // Generalized Action Handler
  const handleAction = (msg: string) => {
    console.log('handleAction received:', msg);

    // Handle file selection from heatmap
    if (msg.startsWith("FILE_SELECTED:")) {
      const filePath = msg.substring(14); // Remove "FILE_SELECTED:" prefix
      console.log('Setting selected file:', filePath);
      setSelectedFile(filePath);
      return;
    }

    if (msg === "Global Search Activated") { setSearchOpen(true); return; }
    if (msg === "Settings Opened") { setSettingsOpen(true); return; }
    if (msg === "Concern Marked") { setReviewModal({ isOpen: true, type: 'concern' }); return; }
    if (msg === "Rejection Recorded") { setReviewModal({ isOpen: true, type: 'reject' }); return; }
    if (msg === "Question Mode Activated") { setReviewModal({ isOpen: true, type: 'question' }); return; }
    if (msg === "Comment Box Opened") { setReviewModal({ isOpen: true, type: 'comment' }); return; }
    if (msg.includes("Submitting Review")) { setSubmitOpen(true); return; }
    if (msg === "Creating Local Task...") { setCreateTaskModalOpen(true); return; }
    if (msg === "Opening Tag Manager...") { setTagManagerModalOpen(true); return; }
    if (msg === "Syncing with Remote...") { setSyncStatusModalOpen(true); return; }
    if (msg === "Start Tour") { setTourOpen(true); return; }
    if (msg === "Opening Repository...") { setRepositorySelectorOpen(true); return; }
    if (msg === "Switching Branch...") { setIsInitialSetup(false); setBranchCompareModalOpen(true); return; }

    showNotification(msg);
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-editor-bg text-editor-fg font-mono overflow-hidden relative">
      <TitleBar onAction={handleAction} />
      <ToolBar
        onAction={handleAction}
        onOpenRepo={() => setRepositorySelectorOpen(true)}
        onImportTask={() => setImportTaskModalOpen(true)}
        showLeft={showLeft}
        showRight={showRight}
        onToggleLeft={toggleLeft}
        onToggleRight={toggleRight}
        diffContext={diffContext}
      />
      
      {/* Resizable Split Pane Area */}
      <div className="flex-1 flex overflow-hidden relative pb-[28px]"> 
        
        {/* Left Pane (Task Tree) */}
        {showLeft && (
          <div 
            style={{ width: leftWidth }} 
            className={`shrink-0 h-full flex flex-col ${isResizing ? '' : 'transition-all duration-300 ease-in-out'}`}
          >
            <TaskTree 
              activeTaskId={activeTaskId} 
              onSelectTask={setActiveTaskId} 
              onAction={handleAction}
            />
          </div>
        )}

        {/* Left Resizer */}
        {showLeft && (
          <div 
            className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -ml-[1px] hover:-ml-[2px] transition-all duration-100 delay-100 flex items-center justify-center group"
            onMouseDown={() => setIsResizing('left')}
          >
          </div>
        )}

        {/* Center Pane (Diff View) */}
        <div className="flex-1 min-w-0 h-full border-r border-editor-line relative pb-[56px] flex flex-col">
          <DiffView
            isMaximized={isMaximized}
            toggleMaximize={toggleMaximize}
            onAction={handleAction}
            diffContext={diffContext}
            selectedFile={selectedFile}
          />
          <ActionBar onAction={handleAction} />
        </div>

        {/* Right Resizer */}
        {showRight && (
          <div 
             className="w-[1px] hover:w-[4px] bg-editor-line hover:bg-editor-accent cursor-col-resize z-20 relative -mr-[1px] hover:-mr-[2px] transition-all duration-100 delay-100 flex items-center justify-center group"
             onMouseDown={() => setIsResizing('right')}
          >
          </div>
        )}

        {/* Right Pane (Tabs) */}
        {showRight && (
          <div 
            style={{ width: rightWidth }} 
            className={`shrink-0 h-full flex flex-col ${isResizing ? '' : 'transition-all duration-300 ease-in-out'}`}
          >
            <RightPanel onAction={handleAction} />
          </div>
        )}
      </div>

      {/* Status Bar */}
      <div className="absolute bottom-0 w-full z-50">
         <StatusBar />
      </div>

      {/* --- MODALS --- */}
      <Modal isOpen={repositorySelectorOpen} onClose={() => setRepositorySelectorOpen(false)} title="Select Repository">
        <RepositorySelector onRepositorySelected={handleRepositorySelected} />
      </Modal>

      <Modal isOpen={openRepoModalOpen} onClose={() => { if(isRepoLoaded) setOpenRepoModalOpen(false); }} title="Open Repository">
        <OpenRepoModal onClose={() => { if(isRepoLoaded) setOpenRepoModalOpen(false); }} onOpen={handleOpenRepo} />
      </Modal>

      <Modal isOpen={importTaskModalOpen} onClose={() => setImportTaskModalOpen(false)} title="Import Task">
        <ImportTaskModal onClose={() => setImportTaskModalOpen(false)} onImport={handleImportTask} />
      </Modal>

      <Modal isOpen={searchOpen} onClose={() => setSearchOpen(false)} title="Go to File or Command">
        <CommandPalette onClose={() => setSearchOpen(false)} onNavigate={handleNavigate} />
      </Modal>

      <Modal isOpen={settingsOpen} onClose={() => setSettingsOpen(false)} title="Editor Preferences">
        <SettingsModal onClose={() => setSettingsOpen(false)} />
      </Modal>

      <Modal isOpen={reviewModal.isOpen} onClose={() => setReviewModal({ ...reviewModal, isOpen: false })} title="Add Review Comment">
        <ReviewActionModal type={reviewModal.type} onClose={() => setReviewModal({ ...reviewModal, isOpen: false })} onSubmit={handleReviewSubmit} />
      </Modal>

      <Modal isOpen={submitOpen} onClose={() => setSubmitOpen(false)} title="Submit Review">
        <SubmitReviewModal onClose={() => setSubmitOpen(false)} onSubmit={handleFinalSubmit} />
      </Modal>

      <Modal isOpen={createTaskModalOpen} onClose={() => setCreateTaskModalOpen(false)} title="Create Local Task">
        <CreateTaskModal onClose={() => setCreateTaskModalOpen(false)} onCreate={handleCreateTask} />
      </Modal>

      <Modal isOpen={tagManagerModalOpen} onClose={() => setTagManagerModalOpen(false)} title="Manage Quick Tags">
        <TagManagerModal onClose={() => setTagManagerModalOpen(false)} />
      </Modal>

      <Modal isOpen={syncStatusModalOpen} onClose={() => setSyncStatusModalOpen(false)} title="Remote Sync Status">
        <SyncStatusModal onClose={() => setSyncStatusModalOpen(false)} />
      </Modal>

      <Modal isOpen={branchCompareModalOpen} onClose={() => { if(isRepoLoaded) setBranchCompareModalOpen(false); }} title="Branch Comparison">
        <BranchCompareModal
          currentBase={diffContext.base}
          currentHead={diffContext.head}
          isInitialSetup={isInitialSetup}
          onClose={() => { if(isRepoLoaded) setBranchCompareModalOpen(false); }}
          onApply={handleApplyBranchCompare}
          onBack={handleBackToRepoSelection}
        />
      </Modal>

      {/* Tour Guide */}
      <TourGuide isOpen={tourOpen} onClose={() => setTourOpen(false)} />

      {/* Toast Notifications */}
      <ToastContainer />
    </div>
  );
};

// Wrap App with LoadingProvider and ErrorBoundary
const AppWithProviders: React.FC = () => {
  return (
    <ErrorBoundary>
      <LoadingProvider>
        <App />
      </LoadingProvider>
    </ErrorBoundary>
  );
};

export default AppWithProviders;