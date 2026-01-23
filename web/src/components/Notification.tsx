import React, { createContext, useContext, useState, useCallback, useRef } from 'react';
import {AlertTriangle, Info, X, XCircle} from 'lucide-react';

// --- Types ---

type NotificationType = 'info' | 'warn' | 'error';

interface NotificationItem {
    id: string;
    type: NotificationType;
    msg: string;
    isExiting?: boolean;
}

// The exact exposed context type requested
type Notifications = {
    warn(msg: string): void;
    error(msg: string): void;
    info(msg: string): void;
};

// --- Context ---

const NotificationContext = createContext<Notifications | null>(null);

export const useNotifications = () => {
    const context = useContext(NotificationContext);
    if (!context) {
        throw new Error('useNotifications must be used within a NotificationProvider');
    }
    return context;
};

// --- Styles (Injected to avoid external CSS files) ---

const styles = `
  @keyframes slideInRight {
    from { opacity: 0; transform: translateX(100%); }
    to { opacity: 1; transform: translateX(0); }
  }
  
  @keyframes fadeOutRight {
    from { opacity: 1; transform: translateX(0); }
    to { opacity: 0; transform: translateX(20px); }
  }

  .notification-enter {
    animation: slideInRight 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }

  .notification-exit {
    animation: fadeOutRight 0.3s ease-in forwards;
  }
`;

// --- Component ---

export const NotificationProvider = ({ children }: { children: React.ReactNode }) => {
    const [notifications, setNotifications] = useState<NotificationItem[]>([]);

    // Use a ref to track current notifications for the timeout closures
    const notificationsRef = useRef(notifications);
    notificationsRef.current = notifications;

    const removeNotification = useCallback((id: string) => {
        setNotifications((prev) =>
            prev.map(n => n.id === id ? { ...n, isExiting: true } : n)
        );

        // Wait for animation to finish before actual removal from DOM
        setTimeout(() => {
            setNotifications((prev) => prev.filter((n) => n.id !== id));
        }, 300); // Matches CSS animation duration
    }, []);

    const addNotification = useCallback((type: NotificationType, msg: string) => {
        const id = Math.random().toString(36).substring(7);

        setNotifications((prev) => {
            // Create new item
            const newItem = { id, type, msg };

            // Enforce limit of 3. If we have 3, remove the oldest (first index)
            // We slice the last 2 to make room for the 3rd
            const currentList = prev.length >= 3 ? prev.slice(prev.length - 2) : prev;

            return [...currentList, newItem];
        });

        // Auto-dismiss after 4 seconds
        setTimeout(() => {
            // Check if it still exists (hasn't been manually closed or pushed out)
            if (notificationsRef.current.find(n => n.id === id)) {
                removeNotification(id);
            }
        }, 4000);
    }, [removeNotification]);

    const api: Notifications = {
        warn: (msg) => addNotification('warn', msg),
        error: (msg) => addNotification('error', msg),
        info: (msg) => addNotification('info', msg),
    };

    return (
        <NotificationContext.Provider value={api}>
            <style>{styles}</style>
            {children}

            <div className="fixed bottom-6 right-6 z-50 flex flex-col gap-3 max-w-[360px] w-full pointer-events-none">
                {notifications.map((note) => (
                    <NotificationToast
                        key={note.id}
                        item={note}
                        onClose={() => removeNotification(note.id)}
                    />
                ))}
            </div>
        </NotificationContext.Provider>
    );
};

const NotificationToast = ({ item, onClose }: { item: NotificationItem, onClose: () => void }) => {
    // Visual config based on type
    const config = {
        info: {
            icon: <Info size={18} />,
            bg: 'bg-white',
            border: 'border-l-4 border-l-blue-500 border-y border-r border-slate-200',
            text: 'text-slate-700',
            iconColor: 'text-blue-500'
        },
        warn: {
            icon: <AlertTriangle size={18} />,
            bg: 'bg-white',
            border: 'border-l-4 border-l-amber-500 border-y border-r border-slate-200',
            text: 'text-slate-700',
            iconColor: 'text-amber-500'
        },
        error: {
            icon: <XCircle size={18} />,
            bg: 'bg-white',
            border: 'border-l-4 border-l-red-500 border-y border-r border-slate-200',
            text: 'text-slate-700',
            iconColor: 'text-red-500'
        }
    }[item.type];

    return (
        <div
            className={`
        pointer-events-auto shadow-md rounded-sm p-3 flex items-start gap-3 
        font-mono text-sm transition-all
        ${config.bg} ${config.border} ${config.text}
        ${item.isExiting ? 'notification-exit' : 'notification-enter'}
      `}
            role="alert"
        >
            <div className={`mt-0.5 shrink-0 ${config.iconColor}`}>
                {config.icon}
            </div>

            <div className="flex-1 leading-tight pt-0.5">
                {item.msg}
            </div>

            <button
                onClick={onClose}
                className="shrink-0 text-slate-400 hover:text-slate-600 transition-colors"
                aria-label="Close notification"
            >
                <X size={14} />
            </button>
        </div>
    );
};