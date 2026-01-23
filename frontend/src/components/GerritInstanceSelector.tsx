import React, { useState, useEffect } from 'react';
import { Globe, Check, Plus, Loader2 } from 'lucide-react';
import { simpleGerritService, SimpleGerritInstance } from '../services/gerrit-simple-service';

interface GerritInstanceSelectorProps {
  onClose: () => void;
  onSelectInstance: () => void;
  onConfigureNew: () => void;
}

const GerritInstanceSelector: React.FC<GerritInstanceSelectorProps> = ({
  onClose,
  onSelectInstance,
  onConfigureNew
}) => {
  const [instances, setInstances] = useState<SimpleGerritInstance[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedInstanceId, setSelectedInstanceId] = useState<string | null>(null);

  useEffect(() => {
    const loadInstances = async () => {
      try {
        const data = await simpleGerritService.getInstances();
        setInstances(data);
        const activeInstance = data.find(inst => inst.is_active);
        if (activeInstance) {
          setSelectedInstanceId(activeInstance.id);
        }
      } catch (error) {
        console.error('Failed to load Gerrit instances:', error);
      } finally {
        setLoading(false);
      }
    };
    loadInstances();
  }, []);

  const handleSelect = () => {
    if (selectedInstanceId) {
      onSelectInstance();
      onClose();
    }
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <Loader2 size={32} className="animate-spin text-purple-500" />
        <span className="text-xs text-gray-500 mt-3">Loading instances...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <div>
        <p className="text-xs text-gray-400 mb-3">
          Select a configured Gerrit instance or configure a new one
        </p>
      </div>

      {instances.length === 0 ? (
        <div className="text-center py-8 bg-editor-line/20 rounded-lg border border-editor-line/50">
          <Globe size={32} className="mx-auto text-gray-600 mb-3" />
          <div className="text-sm text-gray-400">No Gerrit instances configured</div>
          <div className="text-xs text-gray-500 mt-1">
            Configure your first Gerrit server to start importing changes
          </div>
        </div>
      ) : (
        <div className="space-y-2 max-h-[300px] overflow-y-auto">
          {instances.map((instance) => (
            <div
              key={instance.id}
              onClick={() => setSelectedInstanceId(instance.id)}
              className={`p-3 rounded-lg border cursor-pointer transition-all flex items-start gap-3 ${
                selectedInstanceId === instance.id
                  ? 'border-purple-500 bg-purple-900/20'
                  : 'border-editor-line hover:border-editor-line/80 bg-editor-line/30'
              }`}
            >
              <div className="mt-0.5">
                <Globe size={16} className={selectedInstanceId === instance.id ? 'text-purple-400' : 'text-gray-500'} />
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1">
                  <div className="text-sm font-medium text-editor-fg truncate">
                    {instance.name}
                  </div>
                  {instance.is_active && (
                    <span className="text-[9px] px-1.5 py-0.5 rounded bg-green-500/20 text-green-400 font-medium">
                      Active
                    </span>
                  )}
                </div>
                <div className="text-xs text-gray-500 font-mono truncate">
                  {instance.url}
                </div>
                <div className="flex items-center gap-2 mt-1 text-[10px] text-gray-600">
                  <span>User: {instance.username}</span>
                  <span>•</span>
                  <span>{instance.status}</span>
                </div>
              </div>
              {selectedInstanceId === instance.id && (
                <Check size={16} className="text-purple-400 shrink-0 mt-0.5" />
              )}
            </div>
          ))}
        </div>
      )}

      <div className="flex gap-2 pt-3 border-t border-editor-line mt-1">
        <button
          onClick={onClose}
          className="flex-1 px-4 py-2 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={onConfigureNew}
          className="flex-1 px-4 py-2 rounded text-xs bg-editor-line hover:bg-editor-line/80 text-white transition-colors flex items-center justify-center gap-2"
        >
          <Plus size={14} />
          Configure New
        </button>
        {selectedInstanceId && instances.length > 0 && (
          <button
            onClick={handleSelect}
            className="flex-1 px-4 py-2 rounded text-xs bg-purple-600 hover:bg-purple-500 text-white transition-colors font-medium shadow-lg shadow-purple-900/20"
          >
            Select Instance
          </button>
        )}
      </div>
    </div>
  );
};

export default GerritInstanceSelector;
