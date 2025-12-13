import { DiffLine, FileNode, ReviewSeverity, Task } from './types';

export const MOCK_DIFF_LINES: DiffLine[] = [
  // Imports Section
  { oldLineNumber: 1, newLineNumber: 1, content: 'import java.util.List;', type: 'context' },
  { oldLineNumber: 2, newLineNumber: 2, content: 'import java.util.Map;', type: 'context' },
  { oldLineNumber: 3, newLineNumber: 3, content: 'import lombok.extern.slf4j.Slf4j;', type: 'context' },
  { oldLineNumber: 4, newLineNumber: 4, content: 'import org.springframework.stereotype.Service;', type: 'context' },
  { oldLineNumber: 5, newLineNumber: 5, content: 'import com.alipay.payment.mapper.OrderMapper;', type: 'context' },
  { oldLineNumber: 6, newLineNumber: 6, content: '', type: 'context' },
  
  // Lombok Section
  { oldLineNumber: 7, newLineNumber: 7, content: '@Slf4j', type: 'context' },
  { oldLineNumber: 8, newLineNumber: 8, content: '@Service', type: 'context' },
  { oldLineNumber: 9, newLineNumber: 9, content: '@RequiredArgsConstructor', type: 'context' },
  { oldLineNumber: 10, newLineNumber: 10, content: 'public class RetryServiceImpl implements RetryService {', type: 'context' },
  { oldLineNumber: 11, newLineNumber: 11, content: '', type: 'context' },

  // Existing Business Logic
  { oldLineNumber: 101, content: '    public void retry(String orderId) {', type: 'context' },
  { oldLineNumber: 102, content: '        // Old logic was here', type: 'removed' },
  { newLineNumber: 125, content: '        log.info("Starting retry process for order: {}", orderId);', type: 'added' },
  { newLineNumber: 126, content: '        Order order = orderMapper.selectById(orderId);', type: 'added' },
  { 
    newLineNumber: 127, 
    content: '        orderMapper.updateStatus(orderId, OrderStatus.RETRYING);', 
    type: 'added',
    severity: ReviewSeverity.WARNING,
    message: 'Potential N+1 Query Risk'
  },
  { 
    newLineNumber: 128, 
    content: '        // TODO: Add distributed lock', 
    type: 'added',
    severity: ReviewSeverity.ERROR,
    message: 'Missing @Transactional annotation'
  },
  { newLineNumber: 129, content: '    }', type: 'context' },
  { oldLineNumber: 105, newLineNumber: 130, content: '', type: 'context' },
  { oldLineNumber: 106, newLineNumber: 131, content: '    private void validate(Order order) {', type: 'context' },
  { oldLineNumber: 107, newLineNumber: 132, content: '        if (order == null) throw new IllegalArgumentException();', type: 'context' },
];

export const MOCK_TASKS: Task[] = [
  { id: '1', title: 'PR#2877 Pay Retry Refactor', status: 'active' },
  { id: '2', title: 'PR#2871 Auth Center Update', status: 'pending', unreadCount: 1 },
  { id: '3', title: 'PR#2869 Oracle Proc Opt', status: 'pending' },
  { id: '4', title: 'Local#11 SQL Review', status: 'pending' },
];

export const MOCK_WATCHED: Task[] = [
  { id: '5', title: 'PR#2847 Async Task Refactor', status: 'pending', unreadCount: 3 },
];

export const MOCK_FILES: FileNode[] = [
  {
    id: 'root',
    name: 'src',
    path: '/src',
    type: 'folder',
    status: 'modified',
    children: [
      {
        id: 'main',
        name: 'main',
        path: '/src/main',
        type: 'folder',
        status: 'modified',
        children: [
          {
            id: 'java',
            name: 'java',
            path: '/src/main/java',
            type: 'folder',
            status: 'modified',
            children: [
              {
                id: 'service',
                name: 'RetryServiceImpl.java',
                path: '/src/main/java/.../RetryServiceImpl.java',
                type: 'file',
                status: 'modified',
                stats: { added: 342, removed: 108 }
              }
            ]
          }
        ]
      }
    ]
  }
];