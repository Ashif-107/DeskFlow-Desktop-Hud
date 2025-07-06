import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect } from 'react';

function App() {

  useEffect(() => {
    invoke('init_position')
  }, [])


  return (
    <div className="app-container">
      <header className="App-header">
        <h1 className="App-title">DeskFlow 🏎️</h1>
        <p>This is a simple React application.</p>
      </header>
    </div>
  );
}

export default App;
