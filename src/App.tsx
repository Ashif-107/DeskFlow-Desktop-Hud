import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from 'react';

function App() {
  const [activeApp, setActiveApp] = useState({ title: "", process: "" });


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
      </div>
    </div>
  );
}

export default App;
