import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App'
import {NotificationProvider} from "./components/Notification.tsx";

type Notifications = {
    warn(msg: string): void
    error(msg: string): void
    info(msg: string): void
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      <NotificationProvider>
          <App />
      </NotificationProvider>
  </StrictMode>
)
