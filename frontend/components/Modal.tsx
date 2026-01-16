import React, { useEffect } from 'react';
import { X } from 'lucide-react';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, title, children }) => {
  // Handle ESC key to close - must be before any conditional returns
  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handleEsc);
    return () => window.removeEventListener('keydown', handleEsc);
  }, [onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-[2px] animate-fade-in">
      <div className="bg-editor-bg border border-editor-line shadow-2xl rounded-lg w-[500px] max-w-[90vw] overflow-hidden flex flex-col max-h-[80vh] animate-scale-in">
        <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-sidebar">
          <span className="font-bold text-sm text-editor-fg tracking-wide">{title}</span>
          <button onClick={onClose} className="text-gray-400 hover:text-white transition-colors">
            <X size={16} />
          </button>
        </div>
        <div className="p-4 overflow-y-auto">
          {children}
        </div>
      </div>
    </div>
  );
};

export default Modal;