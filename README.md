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
