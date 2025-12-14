import React, { useEffect, useState } from 'react';
import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from 'lucide-react';
import { useErrorStore, ErrorSeverity, ToastNotification } from '../utils/errorHandler';

const ToastIcon: React.FC<{ severity: ErrorSeverity }> = ({ severity }) => {
  const iconClass = "w-5 h-5 flex-shrink-0";

  switch (severity) {
    case ErrorSeverity.SUCCESS:
      return <CheckCircle className={`${iconClass} text-green-500`} />;
    case ErrorSeverity.WARNING:
      return <AlertTriangle className={`${iconClass} text-yellow-500`} />;
    case ErrorSeverity.ERROR:
      return <AlertCircle className={`${iconClass} text-red-500`} />;
    case ErrorSeverity.INFO:
    default:
      return <Info className={`${iconClass} text-blue-500`} />;
  }
};

const ToastItem: React.FC<{
  toast: ToastNotification;
  onDismiss: (id: string) => void;
}> = ({ toast, onDismiss }) => {
  const [isVisible, setIsVisible] = useState(false);
  const [isLeaving, setIsLeaving] = useState(false);

  useEffect(() => {
    // Animate in
    const timer = setTimeout(() => setIsVisible(true), 10);
    return () => clearTimeout(timer);
  }, []);

  const handleDismiss = () => {
    setIsLeaving(true);
    setTimeout(() => {
      onDismiss(toast.id);
    }, 300); // Match animation duration
  };

  const getSeverityStyles = () => {
    switch (toast.severity) {
      case ErrorSeverity.SUCCESS:
        return 'bg-green-500/10 border-green-500/30 text-green-400';
      case ErrorSeverity.WARNING:
        return 'bg-yellow-500/10 border-yellow-500/30 text-yellow-400';
      case ErrorSeverity.ERROR:
        return 'bg-red-500/10 border-red-500/30 text-red-400';
      case ErrorSeverity.INFO:
      default:
        return 'bg-blue-500/10 border-blue-500/30 text-blue-400';
    }
  };

  return (
    <div
      className={`
        ${getSeverityStyles()}
        border rounded-lg p-4 shadow-xl backdrop-blur-sm
        transition-all duration-300 ease-in-out
        ${isVisible && !isLeaving ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-full'}
        max-w-md w-full
      `}
      style={{
        animation: isLeaving ? 'slideOut 0.3s ease-in-out' : undefined
      }}
    >
      <div className="flex items-start gap-3">
        <ToastIcon severity={toast.severity} />

        <div className="flex-1 min-w-0">
          <div className="font-semibold text-sm mb-1">
            {toast.title}
          </div>
          <div className="text-sm opacity-90 break-words">
            {toast.message}
          </div>

          {toast.actions && toast.actions.length > 0 && (
            <div className="flex gap-2 mt-3">
              {toast.actions.map((action, index) => (
                <button
                  key={index}
                  onClick={action.action}
                  className={`
                    px-3 py-1 rounded text-xs font-medium
                    transition-colors duration-200
                    hover:opacity-80
                    ${toast.severity === ErrorSeverity.SUCCESS && 'bg-green-500/20 text-green-400'}
                    ${toast.severity === ErrorSeverity.WARNING && 'bg-yellow-500/20 text-yellow-400'}
                    ${toast.severity === ErrorSeverity.ERROR && 'bg-red-500/20 text-red-400'}
                    ${toast.severity === ErrorSeverity.INFO && 'bg-blue-500/20 text-blue-400'}
                  `}
                >
                  {action.label}
                </button>
              ))}
            </div>
          )}
        </div>

        <button
          onClick={handleDismiss}
          className="opacity-60 hover:opacity-100 transition-opacity p-1 rounded hover:bg-white/5"
          aria-label="Dismiss notification"
        >
          <X className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};

const ToastContainer: React.FC = () => {
  const { toasts, removeToast } = useErrorStore();

  if (toasts.length === 0) {
    return null;
  }

  return (
    <div
      className="fixed top-20 right-4 z-[200] flex flex-col gap-2 pointer-events-none"
      style={{
        maxHeight: 'calc(100vh - 120px)',
        overflowY: 'auto'
      }}
    >
      {toasts.map((toast) => (
        <div key={toast.id} className="pointer-events-auto">
          <ToastItem toast={toast} onDismiss={removeToast} />
        </div>
      ))}
    </div>
  );
};

export default ToastContainer;

// Add CSS animation for slideOut (should be added to global CSS)
export const toastStyles = `
  @keyframes slideOut {
    from {
      opacity: 1;
      transform: translateX(0);
    }
    to {
      opacity: 0;
      transform: translateX(100%);
    }
  }
`;
