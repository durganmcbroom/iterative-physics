import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import PhysicsEngine from './PhysicsEngine.tsx'
import {NotificationProvider} from "./components/Notification.tsx";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      <NotificationProvider>
          <PhysicsEngine />
      </NotificationProvider>
  </StrictMode>
)
