import React, { useState, useEffect, useLayoutEffect } from 'react';
import { useTranslation } from '../i18n';
import { X, ChevronRight, ChevronLeft } from 'lucide-react';

interface TourGuideProps {
  isOpen: boolean;
  onClose: () => void;
}

const STEPS = [
  { target: null, titleKey: 'tour.welcome.title', contentKey: 'tour.welcome.content', position: 'center' },
  { target: '#tour-task-tree', titleKey: 'tour.tasks.title', contentKey: 'tour.tasks.content', position: 'right' },
  { target: '#tour-toolbar', titleKey: 'tour.toolbar.title', contentKey: 'tour.toolbar.content', position: 'bottom' },
  { target: '#tour-diff-view', titleKey: 'tour.diff.title', contentKey: 'tour.diff.content', position: 'bottom' },
  { target: '#tour-right-panel', titleKey: 'tour.right.title', contentKey: 'tour.right.content', position: 'left' },
  { target: '#tour-action-bar', titleKey: 'tour.action.title', contentKey: 'tour.action.content', position: 'top' },
  { target: null, titleKey: 'tour.finish.title', contentKey: 'tour.finish.content', position: 'center' },
];

const TourGuide: React.FC<TourGuideProps> = ({ isOpen, onClose }) => {
  const { t } = useTranslation();
  const [currentStep, setCurrentStep] = useState(0);
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    if (isOpen) {
      setCurrentStep(0);
    }
  }, [isOpen]);

  useLayoutEffect(() => {
    if (!isOpen) return;

    const step = STEPS[currentStep];
    if (step.target) {
      const el = document.querySelector(step.target);
      if (el) {
        setTargetRect(el.getBoundingClientRect());
      } else {
        // Fallback if element not found, just center or skip
        setTargetRect(null);
      }
    } else {
      setTargetRect(null); // Center modal
    }
  }, [currentStep, isOpen]);

  if (!isOpen) return null;

  const step = STEPS[currentStep];
  const isLast = currentStep === STEPS.length - 1;

  const handleNext = () => {
    if (isLast) onClose();
    else setCurrentStep(prev => prev + 1);
  };

  const handlePrev = () => {
    if (currentStep > 0) setCurrentStep(prev => prev - 1);
  };

  // Spotlight Style
  // We use a massive box-shadow on a div positioned exactly over the target to create the "cutout" effect
  const spotlightStyle: React.CSSProperties = targetRect
    ? {
        position: 'fixed',
        top: targetRect.top,
        left: targetRect.left,
        width: targetRect.width,
        height: targetRect.height,
        boxShadow: '0 0 0 9999px rgba(0, 0, 0, 0.75)',
        borderRadius: '4px',
        pointerEvents: 'none', // Allow clicking through if needed, but usually we block interaction during tour
        zIndex: 1000,
        transition: 'all 0.4s cubic-bezier(0.25, 1, 0.5, 1)',
      }
    : {
        // Full screen cover if no target
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.75)',
        zIndex: 1000,
        transition: 'all 0.4s ease',
    };

  // Popover Position Logic
  let popoverStyle: React.CSSProperties = {
    position: 'fixed',
    zIndex: 1001,
    transition: 'all 0.4s cubic-bezier(0.25, 1, 0.5, 1)',
  };

  if (!targetRect) {
    // Center
    popoverStyle = {
      ...popoverStyle,
      top: '50%',
      left: '50%',
      transform: 'translate(-50%, -50%)',
    };
  } else {
    // Offset from target
    const gap = 12;
    switch (step.position) {
      case 'right':
        popoverStyle.top = targetRect.top + 20;
        popoverStyle.left = targetRect.right + gap;
        break;
      case 'left':
        popoverStyle.top = targetRect.top + 20;
        popoverStyle.left = targetRect.left - 320 - gap; // Assumes width 320
        break;
      case 'bottom':
        popoverStyle.top = targetRect.bottom + gap;
        popoverStyle.left = targetRect.left + (targetRect.width / 2) - 160;
        break;
      case 'top':
        popoverStyle.top = targetRect.top - 180 - gap; // Assumes height ~180
        popoverStyle.left = targetRect.left + (targetRect.width / 2) - 160;
        break;
    }
  }

  // Boundary check to keep inside viewport (simple version)
  // In a real lib, this would be more robust (popper.js)
  
  return (
    <>
      {/* Spotlight / Backdrop */}
      <div style={spotlightStyle}></div>

      {/* Tooltip Card */}
      <div 
        style={popoverStyle} 
        className="w-[320px] bg-editor-bg border border-editor-accent shadow-[0_8px_30px_rgba(0,0,0,0.5)] rounded-lg p-5 flex flex-col gap-3 animate-fade-in"
      >
        <div className="flex justify-between items-start">
            <h3 className="text-white font-bold text-sm">{t(step.titleKey)}</h3>
            <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors">
                <X size={14} />
            </button>
        </div>
        
        <p className="text-xs text-gray-300 leading-relaxed min-h-[40px]">
            {t(step.contentKey)}
        </p>

        <div className="flex items-center justify-between mt-2 pt-2 border-t border-editor-line">
            <div className="flex gap-1">
                {STEPS.map((_, i) => (
                    <div key={i} className={`w-1.5 h-1.5 rounded-full transition-colors ${i === currentStep ? 'bg-editor-accent' : 'bg-editor-line'}`}></div>
                ))}
            </div>
            
            <div className="flex gap-2">
                {currentStep > 0 && (
                     <button onClick={handlePrev} className="px-3 py-1 text-xs text-gray-400 hover:text-white transition-colors border border-transparent hover:border-editor-line rounded">
                        {t('tour.prev')}
                    </button>
                )}
                <button onClick={handleNext} className="px-3 py-1 text-xs bg-editor-accent hover:bg-blue-600 text-white rounded transition-colors flex items-center gap-1">
                    {isLast ? t('tour.finish') : t('tour.next')}
                    {!isLast && <ChevronRight size={12} />}
                </button>
            </div>
        </div>
      </div>
    </>
  );
};

export default TourGuide;