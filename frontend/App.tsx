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

const App: React.FC = () => {
  const [activeTaskId, setActiveTaskId] = useState('1');
  const [notification, setNotification] = useState<string | null>(null);
  
  // Repository & Diff Context State
  const [isRepoLoaded, setIsRepoLoaded] = useState(false);
  const [selectedRepoPath, setSelectedRepoPath] = useState<string | null>(null);
  const [diffContext, setDiffContext] = useState({
      base: 'master',
      head: 'feature/payment-retry'
  });

  // Layout State
  const [leftWidth, setLeftWidth] = useState(260);
  const [rightWidth, setRightWidth] = useState(300);
  const [showLeft, setShowLeft] = useState(true);
  const [showRight, setShowRight] = useState(true);
  const [isResizing, setIsResizing] = useState<'left' | 'right' | null>(null);

  // Modal States
  const [openRepoModalOpen, setOpenRepoModalOpen] = useState(false);
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
    // If no repo is loaded on startup, prompt user to open one
    if (!isRepoLoaded) {
      // Small timeout to ensure UI is ready
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

  const showNotification = useCallback((message: string) => {
    setNotification(message);
    const timer = setTimeout(() => setNotification(null), 2000);
    return () => clearTimeout(timer);
  }, []);

  // --- Handlers for Open Repo Wizard ---
  
  // Step 1: Select Repo -> Proceed to Step 2
  const handleOpenRepo = (path: string) => {
    setSelectedRepoPath(path);
    setOpenRepoModalOpen(false);
    
    // Trigger Step 2 immediately
    setIsInitialSetup(true);
    setBranchCompareModalOpen(true);
  };

  // Back from Step 2 -> Step 1
  const handleBackToRepoSelection = () => {
      setBranchCompareModalOpen(false);
      // Small delay for smooth visual transition
      setTimeout(() => setOpenRepoModalOpen(true), 50);
  };

  // Step 2: Select Branches -> Load UI
  const handleApplyBranchCompare = (base: string, head: string) => {
      setDiffContext({ base, head });
      
      // If this was part of the initial setup, finalize the load
      if (isInitialSetup) {
          setIsRepoLoaded(true);
          setIsInitialSetup(false);
          showNotification(`Repository Loaded: ${selectedRepoPath} (${base} ← ${head})`);
      } else {
          showNotification(`Comparing ${base} ← ${head}`);
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
    if (msg === "Switching Branch...") { setIsInitialSetup(false); setBranchCompareModalOpen(true); return; }
    
    showNotification(msg);
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-editor-bg text-editor-fg font-mono overflow-hidden relative">
      <TitleBar onAction={handleAction} />
      <ToolBar 
        onAction={handleAction} 
        onOpenRepo={() => setOpenRepoModalOpen(true)}
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

      {/* Toast Notification */}
      {notification && (
        <div className="absolute top-[100px] left-1/2 -translate-x-1/2 bg-editor-selection text-white px-4 py-2 rounded shadow-xl z-[150] animate-fade-in-down border border-editor-accent/50 text-xs font-bold tracking-wide pointer-events-none">
          {notification}
        </div>
      )}
    </div>
  );
};

export default App;