import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from 'react';
import viewIcon from './assets/view.png';
import hideIcon from './assets/hide.png';

function formatDuration(seconds: number) {
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return `${hrs}h ${mins}m`;
}


function App() {
  const [activeApp, setActiveApp] = useState({ title: "", process: "" });
  const [windows, setWindows] = useState<[string, string][]>([]);

  const [categorySummary, setCategorySummary] = useState<Record<string, number>>({});
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

    // Every 10 sec: fetch category summary
    const summaryInterval = setInterval(async () => {
      try {
        const result = await invoke<Record<string, number>>("get_category_summary");
        setCategorySummary(result);
      } catch (err) {
        console.error("Failed to fetch summary", err);
      }
    }, 5000);

    return () => {
      clearInterval(interval);
      clearInterval(summaryInterval);
    };
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

        {/* Category Summary */}
        <div className="category-summary">
          <h3>📊 Time Spent</h3>
          <ul>
            {Object.entries(categorySummary)
              .filter(([category]) => category !== "Other") // 🔥 Hide "Other"
              .map(([category, seconds]) => (
                <li key={category}>
                  <strong>{category}</strong>: {formatDuration(seconds)}
                </li>
              ))}
          </ul>
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
