import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from 'react';
import viewIcon from './assets/view.png';
import hideIcon from './assets/hide.png';

function App() {
  const [activeApp, setActiveApp] = useState({ title: "", process: "" });
  const [windows, setWindows] = useState<[string, string][]>([]);

  const [showModal, setShowModal] = useState(false);


  useEffect(() => {
    invoke('init_position')

    // Start checking the active window every 5 seconds
    const interval = setInterval(async () => {
      try {
        const result = await invoke<[string, string]>("get_active_app");
        if (result) {
          const [title, process] = result;
          setActiveApp({ title, process });
          console.log("🪟 Title:", title);
          console.log("📦 Process:", process);
        }

        invoke<[string, string][]>('get_all_visible_windows').then(setWindows);

      } catch (error) {
        console.error("Failed to get active app:", error);
      }
    }, 5000);


    return () => clearInterval(interval);
  }, [])


  return (
    <div className="app-container">
      <header className="App-header">
        <h1 className="App-title">DeskFlow 🏎️</h1>
      </header>
      <div className="App-content">
        {/* ✅ Add this part to show active window info */}
        <div className="active-window-info">
          <div>

            <strong>🪟 Active Window:</strong> {activeApp.title}<br />
            <strong>📦 Process:</strong> {activeApp.process}
          </div>

          <div>

            <button
              onClick={() => setShowModal(!showModal)}
              className="show-all-btn"
            >
              {showModal ? 
                <img src={hideIcon} alt="Show All Windows" className="view-icon" />
                :
                <img src={viewIcon} alt="Show All Windows" className="view-icon" />
              }
            </button>
          </div>
        </div>

        {/* 🚪 Popup Modal */}
        {showModal && (
          <div className="modal-overlay">
            <div className="visible-windows ">
              <h3>🪟 All Active Windows</h3>
              <ul>
                {windows.map(([title, exe], i) => (
                  <li key={i}>
                    <b>{title}</b> — <span>{exe}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
