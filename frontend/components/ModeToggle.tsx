import { useAppStore } from '../store/useAppStore';

export const ModeToggle = () => {
  const { mode, setMode } = useAppStore();

  return (
    <div className="flex items-center gap-2 bg-gray-800 rounded-lg p-2">
      <button
        onClick={() => setMode('local')}
        className={`px-4 py-2 rounded-lg font-medium transition-all ${
          mode === 'local'
            ? 'bg-blue-600 text-white hover:bg-blue-700'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
        }`}
      >
        Local
      </button>
      <button
        onClick={() => setMode('remote')}
        className={`px-4 py-2 rounded-lg font-medium transition-all ${
          mode === 'remote'
            ? 'bg-purple-600 text-white hover:bg-purple-700'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
        }`}
      >
        Remote
      </button>
    </div>
  );
};
