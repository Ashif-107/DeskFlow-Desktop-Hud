import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect } from 'react';

function App() {

  useEffect(() => {
    invoke('init_position')
  }, [])


  return (
    <div className="App">
      <header className="App-header">
        <h1 className="App-title">Welcome to My React App</h1>
        <p>This is a simple React application.</p>
      </header>
    </div>
  );
}

export default App;
