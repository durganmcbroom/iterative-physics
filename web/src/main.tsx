import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      {/*<div className={"bg-red-800"}>Hello how are you</div>*/}
    <App />
  </StrictMode>,
)
