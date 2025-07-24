import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from 'react';
import viewIcon from './assets/view.png';
import hideIcon from './assets/hide.png';

import {
  PieChart, Pie, Cell, Tooltip, ResponsiveContainer
} from 'recharts';

// Fixed color mapping for consistent colors per category
// Fixed color mapping for consistent colors per category
const CATEGORY_COLORS: Record<string, string> = {
  'Work': '#21134D',
  'Communication': '#00C49F',
  'Entertainment': '#B6A6E9',
  'Browsing': '#876FD4',
  'Gaming': '#9b59b6',
  'Music': '#5E40BE',
  'File Management': '#fdcb6e',
  'System': '#2d3436',
  'Development': '#e67e22',
  'Chatting': '#0066CC',
};

const productiveCategories = [
  "Development",
  "Education",
  "Work",
  "Writing",
  "Research",
  "Tools",
  "Design"
];

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

  const [score, setScore] = useState<{ percent: number; rating: string } | null>(null);


  const pieData = Object.entries(categorySummary)
    .filter(([category]) => category !== "Other")
    .sort(([a], [b]) => a.localeCompare(b)) // Sort by category name for consistent order
    .map(([category, seconds]) => ({
      name: category,
      value: seconds,
      color: CATEGORY_COLORS[category] || '#95a5a6' // fallback color
    }));

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
    }, 10000);

    return () => {
      clearInterval(interval);
      clearInterval(summaryInterval);

    };
  }, [])

  // Every 30 sec: fetch productivity score
  useEffect(() => {
    const totalTime = Object.values(categorySummary).reduce((a, b) => a + b, 0);

    const productiveTime = Object.entries(categorySummary)
      .filter(([category]) => productiveCategories.includes(category))
      .reduce((sum, [_, time]) => sum + time, 0);

    const percent = totalTime > 0 ? parseFloat(((productiveTime / totalTime) * 100).toFixed(1)) : 0;

    const rating =
      percent >= 90 ? "🌟 Excellent" :
        percent >= 70 ? "✅ Good" :
          percent >= 50 ? "⚠️ Average" :
            "❌ Needs Improvement";

    setScore({ percent, rating });


    // Store the score to database
    const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
    invoke('store_score', { date: today, score: percent })
      .catch(err => console.error('Failed to store score:', err));
  }, [categorySummary]);  // ✅ Run this logic every time categorySummary updates


  // Custom label function for inside the pie slices
  const renderCustomLabel = ({ cx, cy, midAngle, innerRadius, outerRadius, percent, name }: any) => {
    const RADIAN = Math.PI / 180;
    const radius = innerRadius + (outerRadius - innerRadius) * 0.5;
    const x = cx + radius * Math.cos(-midAngle * RADIAN);
    const y = cy + radius * Math.sin(-midAngle * RADIAN);

    // Only show label if percentage is significant enough (>5%)
    if (percent < 0.05) return null;

    return (
      <text
        x={x}
        y={y}
        fill="black"
        textAnchor={x > cx ? 'start' : 'end'}
        dominantBaseline="central"
        fontSize="10"
        stroke="rgba(0,0,0,0.5)"
        strokeWidth="0.5"
      >
        <tspan x={x} dy="-6">{name}</tspan>
        <tspan x={x} dy="12">{`${(percent * 100).toFixed(1)}%`}</tspan>
      </text>
    );
  };

  // Fixed custom tooltip


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
                <img src={hideIcon} alt="Hide All Windows" className="view-icon" />
                :
                <img src={viewIcon} alt="Show All Windows" className="view-icon" />
              }
            </button>
          </div>
        </div>

        {/* Category Summary */}
        <div className="summary-chart-container">
          <div className="category-summary-box">
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
          <div className="pie-chart-box">
            <ResponsiveContainer width="100%" height={300}>
              <PieChart>
                <Pie
                  data={pieData}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={renderCustomLabel}
                  outerRadius={70}
                  fill="#8884d8"
                  dataKey="value"
                  stroke="#000"
                  strokeWidth={1.5}
                  isAnimationActive={false}
                >
                  {pieData.map((entry) => (
                    <Cell key={entry.name} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip
                  content={({ active, payload }) => {
                    if (active && payload && payload.length) {
                      const { name, value } = payload[0];
                      const totalTime = pieData.reduce((sum, entry) => sum + entry.value, 0);
                      const percent = totalTime > 0 ? ((value / totalTime) * 100).toFixed(1) : "0";
                      return (
                        <div style={{
                          backgroundColor: "#fff",
                          color: "#000",
                          padding: "8px",
                          border: "1px solid #ccc",
                          borderRadius: "4px",
                          boxShadow: "0 2px 8px rgba(0,0,0,0.15)",
                          fontSize: "12px",
                          fontFamily: "Arial, sans-serif"
                        }}>
                          <div style={{ fontWeight: "bold" }}>{name}</div>
                          <div style={{ color: "#000" }}>Percentage: {percent}%</div>
                        </div>
                      );
                    }
                    return null;
                  }}
                  wrapperStyle={{ outline: 'none' }}
                  cursor={{ fill: 'rgba(0,0,0,0.1)' }}
                />
                isAnimationActive={false}
              </PieChart>
            </ResponsiveContainer>
          </div>
        </div>


        {/* 🧠 Today's Productivity Score */}
        {score && (
          <div className="productivity-score-box">
            🎯 Today's productivity: <strong>{score.percent?.toFixed?.(1) || "0.0"}%</strong> — <strong>{score.rating}</strong>
          </div>
        )}


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