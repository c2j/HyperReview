import { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';
import { useErrorStore } from '../utils/errorHandler';
import { ErrorSeverity } from '../utils/errorHandler';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null
    };
  }

  static getDerivedStateFromError(error: Error): State {
    // Update state so the next render will show the fallback UI
    return {
      hasError: true,
      error,
      errorInfo: null
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // Log the error
    console.error('ErrorBoundary caught an error:', error, errorInfo);

    // Update state with error details
    this.setState({
      error,
      errorInfo
    });

    // Call optional error handler
    this.props.onError?.(error, errorInfo);

    // Add to error store
    const { addError } = useErrorStore.getState();
    addError({
      severity: ErrorSeverity.ERROR,
      title: 'Application Error',
      message: error.message || 'An unexpected error occurred',
      details: error.stack,
      context: { component: 'ErrorBoundary' },
      retryable: true
    });
  }

  handleRetry = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null
    });
  };

  render() {
    if (this.state.hasError) {
      // Custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default error UI
      return (
        <div className="flex flex-col items-center justify-center min-h-[400px] p-8 bg-editor-bg text-editor-fg">
          <div className="bg-red-500/10 border border-red-500/30 rounded-lg p-6 max-w-2xl w-full">
            <div className="flex items-start gap-4">
              <AlertTriangle className="w-8 h-8 text-red-500 flex-shrink-0 mt-1" />

              <div className="flex-1">
                <h2 className="text-xl font-bold text-red-400 mb-2">
                  Something went wrong
                </h2>

                <p className="text-sm text-editor-fg/80 mb-4">
                  An unexpected error occurred in this component. You can try refreshing the component or the entire page.
                </p>

                {this.state.error && (
                  <div className="mb-4 p-3 bg-black/20 rounded border border-red-500/20">
                    <p className="text-xs font-mono text-red-300 break-all">
                      {this.state.error.message}
                    </p>
                  </div>
                )}

                {process.env.NODE_ENV === 'development' && this.state.errorInfo && (
                  <details className="mb-4">
                    <summary className="text-sm cursor-pointer hover:opacity-80">
                      Technical Details (Development Only)
                    </summary>
                    <pre className="mt-2 p-3 bg-black/20 rounded overflow-auto text-xs font-mono text-red-300 max-h-64">
                      {this.state.errorInfo.componentStack}
                    </pre>
                  </details>
                )}

                <div className="flex gap-3">
                  <button
                    onClick={this.handleRetry}
                    className="flex items-center gap-2 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded transition-colors"
                  >
                    <RefreshCw className="w-4 h-4" />
                    Try Again
                  </button>

                  <button
                    onClick={() => window.location.reload()}
                    className="px-4 py-2 bg-editor-accent hover:bg-editor-accent/80 text-white rounded transition-colors"
                  >
                    Reload Page
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
