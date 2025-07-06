import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from 'react';

function App() {
  const [activeApp, setActiveApp] = useState({ title: "", process: "" });
  const [windows, setWindows] = useState<[string, string][]>([]);


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
          <strong>🪟 Active Window:</strong> {activeApp.title}<br />
          <strong>📦 Process:</strong> {activeApp.process}
        </div>

        <div className="visible-windows">
          <h3>Visible Windows:</h3>
          <ul>
            {windows.map(([title, exe], i) => (
              <li key={i}><b>{title}</b> {exe}</li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default App;
