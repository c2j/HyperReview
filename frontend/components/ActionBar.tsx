import React from 'react';
import { Check, AlertTriangle, XCircle, HelpCircle } from 'lucide-react';
import { useTranslation } from '../i18n';

interface ActionBarProps {
  onAction: (msg: string) => void;
}

const ActionBar: React.FC<ActionBarProps> = ({ onAction }) => {
  const { t } = useTranslation();
  return (
    <div id="tour-action-bar" className="flex flex-col gap-1 py-1.5 bg-editor-bg/95 backdrop-blur border-t border-editor-line shrink-0 absolute bottom-0 left-0 right-0 w-full z-50 shadow-[0_-4px_12px_rgba(0,0,0,0.4)]">
      
      {/* Row 1: Decisions & Tools - Scrollable Container */}
      <div className="w-full overflow-x-auto [&::-webkit-scrollbar]:hidden">
        <div className="flex items-center justify-start md:justify-center gap-4 px-3 min-w-max">
           {/* Decisions Group */}
           <div className="flex items-center gap-2">
               <button 
                  onClick={() => onAction("File Approved")} 
                  className="flex items-center gap-1.5 px-3 h-6 bg-editor-success/10 text-editor-success hover:bg-editor-success/20 rounded text-xs font-medium border border-editor-success/20 transition-all active:scale-95 whitespace-nowrap">
                  <Check size={13} /> {t('actionbar.approve')}
               </button>
               <button 
                  onClick={() => onAction("Concern Marked")} 
                  className="flex items-center gap-1.5 px-3 h-6 bg-editor-warning/10 text-editor-warning hover:bg-editor-warning/20 rounded text-xs font-medium border border-editor-warning/20 transition-all active:scale-95 whitespace-nowrap">
                  <AlertTriangle size={13} /> {t('actionbar.concern')}
               </button>
               <button 
                  onClick={() => onAction("Rejection Recorded")} 
                  className="flex items-center gap-1.5 px-3 h-6 bg-editor-error/10 text-editor-error hover:bg-editor-error/20 rounded text-xs font-medium border border-editor-error/20 transition-all active:scale-95 whitespace-nowrap">
                  <XCircle size={13} /> {t('actionbar.reject')}
               </button>
               <button 
                  onClick={() => onAction("Question Mode Activated")} 
                  className="flex items-center gap-1.5 px-3 h-6 bg-editor-info/10 text-editor-info hover:bg-editor-info/20 rounded text-xs font-medium border border-editor-info/20 transition-all active:scale-95 whitespace-nowrap">
                  <HelpCircle size={13} /> {t('actionbar.question')}
               </button>
           </div>
           
           <div className="w-[1px] h-4 bg-editor-line/50"></div>

           {/* Tools Group */}
           <div className="flex items-center gap-2">
              <button 
                  onClick={() => onAction("Comment Box Opened")} 
                  className="flex items-center gap-1.5 px-2 h-6 bg-editor-line/40 hover:bg-editor-line hover:text-white rounded text-[11px] text-gray-400 font-mono transition-colors border border-transparent hover:border-gray-500 whitespace-nowrap" 
                  title={t('actionbar.comment')}>
                  <span className="font-bold text-editor-accent">c</span> {t('actionbar.comment')}
              </button>
              <button 
                  onClick={() => onAction("Goto Definition")} 
                  className="flex items-center gap-1.5 px-2 h-6 bg-editor-line/40 hover:bg-editor-line hover:text-white rounded text-[11px] text-gray-400 font-mono transition-colors border border-transparent hover:border-gray-500 whitespace-nowrap">
                  <span className="font-bold text-editor-accent">gd</span> {t('actionbar.go_def')}
              </button>
               <button 
                  onClick={() => onAction("Toggle Blame")} 
                  className="flex items-center gap-1.5 px-2 h-6 bg-editor-line/40 hover:bg-editor-line hover:text-white rounded text-[11px] text-gray-400 font-mono transition-colors border border-transparent hover:border-gray-500 whitespace-nowrap">
                  <span className="font-bold text-editor-accent">gb</span> {t('actionbar.blame')}
              </button>
               <button 
                  onClick={() => onAction("Toggle Ref Line")} 
                  className="flex items-center gap-1.5 px-2 h-6 bg-editor-line/40 hover:bg-editor-line hover:text-white rounded text-[11px] text-gray-400 font-mono transition-colors border border-transparent hover:border-gray-500 whitespace-nowrap">
                  <span className="font-bold text-editor-accent">gr</span> {t('actionbar.ref_line')}
              </button>
           </div>
        </div>
      </div>

      {/* Row 2: Shortcuts - Scrollable Container */}
      <div className="w-full overflow-x-auto [&::-webkit-scrollbar]:hidden border-t border-editor-line/30 mt-0.5 pt-1">
        <div className="flex items-center justify-start md:justify-center gap-6 px-3 min-w-max text-[10px] text-gray-500 font-mono select-none">
            <span className="flex items-center gap-1.5 cursor-pointer hover:text-gray-300 transition-colors" onClick={() => onAction("Next File")}>
                <span className="bg-editor-line/50 px-1 rounded text-gray-400 border border-editor-line/30">Ctrl+Enter</span> {t('actionbar.next_file')}
            </span>
            <span className="flex items-center gap-1.5 cursor-pointer hover:text-gray-300 transition-colors" onClick={() => onAction("Submitting Review to CodeArts...")}>
                <span className="bg-editor-line/50 px-1 rounded text-gray-400 border border-editor-line/30">Shift+Enter</span> {t('actionbar.submit_batch')}
            </span>
             <span className="flex items-center gap-1.5 opacity-70 cursor-pointer hover:text-gray-300 transition-colors" onClick={() => onAction("Switch Panel")}>
                <span className="bg-editor-line/30 px-1 rounded">Alt+1~4</span> {t('actionbar.switch_panel')}
            </span>
            <span className="flex items-center gap-1.5 cursor-pointer hover:text-gray-300 transition-colors" onClick={() => onAction("Exit Review")}>
                <span className="bg-editor-line/50 px-1 rounded text-gray-400 border border-editor-line/30">Esc</span> {t('actionbar.exit')}
            </span>
        </div>
      </div>
    </div>
  );
};

export default ActionBar;