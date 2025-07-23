
# 🧠 DeskFlow – Your Personal Desktop Productivity Tracker

> A lightweight, always-on-screen Heads-Up Display (HUD) for Windows that passively tracks your desktop activity, categorizes your time (e.g., YouTube, Coding, Gaming), and gives you a daily productivity score and calendar streak.

---

![License](https://img.shields.io/badge/license-MIT-green.svg)
![Made with Rust](https://img.shields.io/badge/Rust-Tauri-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)

---

## ✨ Why DeskFlow?

Have you ever reached the end of your day and asked yourself:  
**“Where did all my time go?”**

**DeskFlow** passively answers that for you.  
No buttons. No distractions. Just pure awareness.  
It's designed to be a **minimal, low-resource desktop app** that blends into your workflow — like Rainmeter, but smarter.

---

## ⚙️ Tech Stack

- **Framework:** [Tauri](https://tauri.app) + [Rust](https://www.rust-lang.org/)
- **Frontend:** HTML/CSS/JS (optional integration with Vite/React)
- **Local DB:** SQLite (`rusqlite`)
- **Windows Integration:** `winapi`, `sysinfo`, `windows` crate

---

## 🧩 Core Features

### 1. 🕵️ Passive Activity Tracker
- Automatically detects active apps/websites:
  - VS Code, YouTube, Steam, Chrome, etc.
- Categorizes into:
  - `Reading`, `Music`, `Gaming`, `YouTube`, `Productive`

### 2. 🧊 Minimal Overlay Widget (Rainmeter-style)
- Transparent, frameless widget that blends with wallpaper
- Sticks to a screen corner or floats freely
- Displays:
  - Active app name
  - Optional live timer
  - Pie chart for today’s categories
  - Calendar icon for detailed view

### 3. 📊 Productivity Scoring System
- At the end of the day:
  - Calculates percentage of `Productive` time
  - Assigns a rating:
    - 😎 Productive
    - 😐 Neutral
    - 😞 Unproductive

### 4. 📆 Calendar Streak View - In developement 
- Visual calendar with colored streaks:
  - ✅ Green = Productive
  - ⚠️ Yellow = Neutral
  - ❌ Red = Unproductive
- Click on any date to see:
  - Time spent per category
  - Detailed session logs

---

## 🚀 How It Works

### 🧠 Workflow

1. **App launches silently on boot**
2. **Every few seconds**, it fetches:
   - Active window name
   - Associated app/process
3. **Categorization engine** tags the activity
4. Data is **saved locally** and reflected on the UI
5. End-of-day reports & stats are shown on your calendar

---

## 📦 Installation

### 🛠 Prerequisites (One-Time Setup)

- ✅ Install [Rust](https://www.rust-lang.org/tools/install)
- ✅ Install [Node.js (LTS)](https://nodejs.org/en/)
- ✅ Install Tauri CLI:
  ```bash
  cargo install create-tauri-app
  ```
- ✅ (Optional) Use `pnpm` or `yarn` for faster builds

---

## 🧪 Local Development

```bash
# Clone the repo
git clone https://github.com/your-username/deskflow
cd deskflow

# Install frontend dependencies
npm install

# Run the app in dev mode
npm run tauri dev
```

---

## 🛠 Key Implementation Details

- **Transparent always-on HUD:**  
  `tauri.conf.json` → `decorations: false`, `transparent: true`
- **Activity Detection:**  
  Uses `windows` crate to get active app titles & processes
- **Local Storage:**  
  Sessions stored with:
  - Start time / End time
  - App name
  - Category
- **Optimized:**  
  Polling interval = 5s  
  Minimal redraws to keep resource usage low

---

## 📈 Future Enhancements

- ⏰ Daily productivity notification
- 📤 Export data as CSV
- 📅 Google Calendar sync
- 🤖 AI insights: *“You’ve watched too much YouTube today 😅”*
- 🌙 Dark/Light mode toggle

---

## 📸 Screenshots


---

## 📝 License

This project is licensed under the **MIT License**.

---

## 🙌 Made with ❤️ by [Ashif](https://github.com/your-username)
