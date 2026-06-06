# Zelfi

> *Your local AI agent. Always on, always private.*

Zelfi is a local-first autonomous AI agent that lives in your macOS system tray. It's powered by LM Studio running Gemma 4 E4B-it MLX entirely on your machine — no cloud calls, no telemetry, no data leaving your device. Zelfi watches your clipboard, listens for your voice, browses on your behalf, and takes action — all without phoning home.

---

## Status

🚧 In development — spec complete, PoC in progress

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | Tauri 2.0 |
| UI | SvelteKit + DaisyUI |
| Systems layer | Rust |
| Language model | LM Studio (Gemma 4 E4B-it MLX) |

---

## Requirements

- macOS (Apple Silicon)
- [LM Studio](https://lmstudio.ai/) installed and running locally with Gemma 4 E4B-it MLX loaded

---

## Licence

MIT

## Running the PoC locally (macOS only)

### Prerequisites
- macOS with Apple Silicon (M1 or later)
- [Rust](https://rustup.rs) installed
- [Node.js](https://nodejs.org) 18+
- [LM Studio](https://lmstudio.ai) running with Gemma 4 E4B-it MLX loaded and the local server started

### Steps
```bash
git clone https://github.com/rvh-zelf/Zelfi.git
cd Zelfi
npm install
npm run tauri dev
```

Zelfi will appear in your menu bar. Click the tray icon to open the window.
Enter a goal and press Send — responses stream directly from your local model.
