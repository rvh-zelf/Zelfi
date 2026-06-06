# Zelfi — Project Specification

> **Version:** 2026.06.06.04  
> **Status:** Pre-code — authoritative PoC specification  
> **Owner:** Ruan van Heerden  
> **Division:** Zelf (personal)  

---

## Table of Contents

1. [Project Identity](#1-project-identity)
2. [Vision](#2-vision)
3. [Architecture Overview](#3-architecture-overview)
4. [Agent Loop (Level C — Fully Autonomous)](#4-agent-loop-level-c--fully-autonomous)
5. [Tool Registry](#5-tool-registry)
6. [Memory Architecture](#6-memory-architecture)
7. [Input / Output Pipeline Architecture (Voice-Ready)](#7-input--output-pipeline-architecture-voice-ready)
8. [Browser Companion Extension](#8-browser-companion-extension)
9. [Onboarding UX](#9-onboarding-ux)
10. [Advanced Settings — "About Me" Refinement UI](#10-advanced-settings--about-me-refinement-ui)
11. [Zelf Pro — Architecture Hooks](#11-zelf-pro--architecture-hooks)
12. [UI Structure](#12-ui-structure)
13. [Tech Stack](#13-tech-stack)
14. [Non-Goals](#14-non-goals)
15. [MVP Scope vs Later](#15-mvp-scope-vs-later)
16. [Project Structure](#16-project-structure)
17. [Inbox & Notification System](#17-inbox--notification-system)
18. [Prompt Efficiency & Context Management](#18-prompt-efficiency--context-management)

---

## 1. Project Identity

| Field        | Value                                              |
|--------------|----------------------------------------------------|
| **Name**     | Zelfi                                              |
| **Tagline**  | Your local AI agent — thinks, acts, remembers.    |
| **Division** | Zelf (personal project)                            |
| **Owner**    | Ruan van Heerden                                   |
| **Platform** | macOS — Apple Silicon only (M1 Pro first)          |
| **Version**  | CalVer — `YYYY.MM.DD.N` (e.g. `2026.06.06.01`)   |
| **Licence**  | Proprietary (Zelf) — free tier + Zelf Pro tier    |

---

## 2. Vision

### What Zelfi Is

Zelfi is a fully autonomous, local-first AI agent that lives in your macOS system tray. You give it a goal in plain language — "summarise all the PDFs in my Downloads folder and email me the highlights", "find the cheapest flight to Cape Town this weekend and open the booking page", "draft a reply to that client email and save it as a draft" — and Zelfi figures out the steps, picks the right tools, executes them one by one, and reports back when it is done.

It does not wait to be micromanaged. It does not phone home. It runs entirely on your machine using a local LLM.

### What Makes It Distinct

- **Truly local.** The LLM runs on your hardware via LM Studio. No cloud API calls. No subscriptions to OpenAI or Anthropic. No data leaving your machine.
- **Genuinely autonomous.** Zelfi runs a full ReAct loop (Reason → Act → Observe) until the goal is complete, not just until it runs out of ideas.
- **Browser-integrated.** The Chrome companion extension lets Zelfi read, navigate, click, and fill forms in your actual browser — not a headless scraper.
- **Memory that grows with you.** Zelfi maintains a git-backed markdown memory store — human-readable, rollback-capable, and directly editable. It remembers who you are, what you are working on, and what you care about.
- **Honest about what it cannot do.** If Zelfi is stuck, it says so clearly and gives you the option to intervene or replan — it does not hallucinate progress.

### Philosophy

**Local-first, private, no cloud, no telemetry.**

Your data, your machine, your rules. Zelfi will never:
- Send your goals, memory, or results to any external server
- Require a cloud account to function
- Collect usage analytics
- Upsell you on a more powerful cloud version

The free tier is genuinely useful. The Pro tier exists to sustain development, not to cripple the free experience.

### Pricing Philosophy

"Empower people profitably, not drain their wallets."

- **Free tier:** All core tools, full agent loop, full memory system, unlimited local usage.
- **Zelf Pro:** ~R120/year via Zelf account. Unlocks tools that require Zelf server proxying (e.g. IP geolocation, future integrations). Priced to be affordable for individuals, sustainable for the project.

---

## 3. Architecture Overview

```
╔══════════════════════════════════════════════════════════════════════╗
║                        ZELFI — SYSTEM OVERVIEW                      ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                      ║
║  ┌─────────────────────────────────────────────────────────────┐    ║
║  │                   TAURI 2.0 SHELL (macOS)                   │    ║
║  │                                                             │    ║
║  │  ┌──────────────────────────────┐  ┌─────────────────────┐ │    ║
║  │  │  SVELTEKIT FRONTEND (static) │  │   RUST BACKEND      │ │    ║
║  │  │                              │  │                     │ │    ║
║  │  │  • Goals view                │  │  ┌───────────────┐  │ │    ║
║  │  │  • History view              │◄─┼─►│  Agent Loop   │  │ │    ║
║  │  │  • Memory view               │  │  │  (ReAct)      │  │ │    ║
║  │  │  • Settings view             │  │  └───────┬───────┘  │ │    ║
║  │  │                              │  │          │           │ │    ║
║  │  │  DaisyUI components          │  │  ┌───────▼───────┐  │ │    ║
║  │  │  Tauri JS API                │  │  │ Tool Executor │  │ │    ║
║  │  │  WebSocket client            │  │  └───────┬───────┘  │ │    ║
║  │  └──────────────────────────────┘  │          │           │ │    ║
║  │                                    │  ┌───────▼───────┐  │ │    ║
║  │                                    │  │ Memory System │  │ │    ║
║  │                                    │  └───────┬───────┘  │ │    ║
║  │                                    │          │           │ │    ║
║  │                                    │  ┌───────▼───────┐  │ │    ║
║  │                                    │  │  Auth Module  │  │ │    ║
║  │                                    │  │  (Zelf Pro)   │  │ │    ║
║  │                                    │  └───────────────┘  │ │    ║
║  │                                    │                     │ │    ║
║  │                                    │  ┌───────────────┐  │ │    ║
║  │                                    │  │  WebSocket    │  │ │    ║
║  │                                    │  │  Server       │  │ │    ║
║  │                                    │  └───────┬───────┘  │ │    ║
║  │                                    └──────────┼──────────┘ │    ║
║  └───────────────────────────────────────────────┼────────────┘    ║
║                                                  │                  ║
║  ┌───────────────────────────────┐  ┌────────────▼────────────┐    ║
║  │  LM STUDIO (localhost:1234)   │  │  CHROME EXTENSION       │    ║
║  │  OpenAI-compatible /v1 API    │  │  (Manifest V3)          │    ║
║  │  Gemma 4 E4B-it MLX 4-bit    │  │  Vanilla JS             │    ║
║  └───────────────────────────────┘  │  • browser.read         │    ║
║                                     │  • browser.navigate     │    ║
║  ┌───────────────────────────────┐  │  • browser.search       │    ║
║  │  FILESYSTEM / SHELL / HTTP    │  │  • browser.click        │    ║
║  │  (Rust native tool impls)     │  │  • browser.fill         │    ║
║  │  • fs.read / fs.write / list  │  │  • browser.screenshot   │    ║
║  │  • shell.run                  │  └─────────────────────────┘    ║
║  │  • http.get / http.post       │                                  ║
║  └───────────────────────────────┘                                  ║
║                                                                      ║
║  ┌───────────────────────────────┐  ┌─────────────────────────┐    ║
║  │  SQLITE                       │  │  GIT-BACKED MARKDOWN    │    ║
║  │  Working memory               │  │  ~/.zelfi/memory/       │    ║
║  │  Task state                   │  │  episodes/ (git repo)   │    ║
║  │  Session history              │  │  facts/ (git repo)      │    ║
║  └───────────────────────────────┘  └─────────────────────────┘    ║
║                                                                      ║
║  ┌───────────────────────────────────────────────────────────────┐  ║
║  │  ZELF PRO SERVER (future / external)                          │  ║
║  │  Auth endpoint + API proxy for Pro tools                      │  ║
║  └───────────────────────────────────────────────────────────────┘  ║
╚══════════════════════════════════════════════════════════════════════╝
```

### Component Responsibilities

| Component | Language/Tech | Responsibility |
|---|---|---|
| Tauri 2.0 shell | Rust + webview | App lifecycle, system tray, native APIs, IPC bridge |
| SvelteKit frontend | TypeScript + DaisyUI | All UI views, real-time log streaming, user interaction |
| Rust backend | Rust | Agent loop, tool executor, memory system, WebSocket server, auth module |
| LM Studio | External process | LLM inference (localhost:1234/v1 OpenAI-compatible) |
| Chrome extension | Vanilla JS | Browser tool execution on behalf of the agent |
| SQLite | DB file | Working memory, task state, session history |
| Git markdown store | `~/.zelfi/memory/` | Episodic and semantic long-term memory |
| Zelf Pro server | External (future) | JWT auth, API proxying for Pro tools |

---

## 4. Agent Loop (Level C — Fully Autonomous)

Zelfi operates at **Level C autonomy**: given a goal, it plans, executes, observes, and iterates without requiring confirmation for each step. The user can pause or stop at any time, but Zelfi does not stop to ask permission between steps.

### The ReAct Loop

```
User submits goal
        │
        ▼
┌──────────────────────────────────────────────┐
│  STEP 1: PLAN                                │
│  LLM receives: goal + memory context +       │
│  available tools + prior steps               │
│  LLM outputs: ordered list of steps          │
│  with tool assignments                       │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│  STEP 2: SELECT                              │
│  Agent picks the next incomplete step        │
│  Validates that the required tool is         │
│  available and (if Pro) authorised           │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│  STEP 3: ACT                                 │
│  Tool is invoked with extracted parameters   │
│  For browser tools: instruction dispatched   │
│  via WebSocket to Chrome extension           │
│  For fs/shell/http: executed natively        │
│  in Rust backend                             │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│  STEP 4: OBSERVE                             │
│  Tool result is returned to the agent        │
│  Result is appended to the working context   │
│  Result is streamed to the UI log            │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│  STEP 5: EVALUATE                            │
│  LLM receives: original goal + all steps     │
│  completed so far + latest observation       │
│  LLM decides:                                │
│    (a) Goal is complete → finish             │
│    (b) Next step is clear → continue         │
│    (c) Error or unexpected result → replan   │
│    (d) Impossible → surface to user          │
└──────────────────────┬───────────────────────┘
                       │
              ┌────────┴─────────┐
              │                  │
        Complete?           Continue?
              │                  │
              ▼                  ▼
       End session         Back to STEP 2
       Summarise           (SELECT next step)
       to episodic
       memory
```

### Streaming to the UI

Every event in the loop emits a structured message over a Tauri IPC channel (or internal broadcast) that the SvelteKit frontend listens to:

```
AgentEvent {
  type: "thought" | "tool_call" | "tool_result" | "replan" | "done" | "error"
  step_index: usize
  tool_name: Option<String>
  content: String
  timestamp: u64
}
```

Each event is appended to the real-time task log in the Goals view as it arrives. The user sees the agent's thinking, the tool being called, and the result — in sequence, as they happen.

### Pause and Stop

The agent loop checks for a cancellation signal between every step (before SELECT). Two controls are always visible during execution:

- **Pause** — suspends the loop after the current tool call completes. The user can review the log and resume. State is preserved in SQLite.
- **Stop** — terminates the loop cleanly. The session is summarised and committed to episodic memory. Any partial work is noted in the summary.

Neither pause nor stop can interrupt a tool call mid-execution (that would leave browser or filesystem state inconsistent). They take effect at step boundaries.

### Replanning on Error

When EVALUATE determines that an unexpected result has occurred (e.g. a web page did not load, a file was not found, a shell command failed), the agent enters replan mode:

1. The current plan is marked as invalid.
2. The LLM is prompted with: original goal + completed steps + the failed step + the error observation.
3. The LLM outputs a revised plan from the current position.
4. The revised plan replaces the remaining steps in SQLite working memory.
5. Execution continues from the first step of the revised plan.

If replanning fails twice on the same goal (the LLM cannot find a workable path), the agent surfaces the problem to the user with a plain-language explanation and a summary of what was tried.

---

## 5. Tool Registry

All tools are registered at startup. Pro tools are only registered if a valid Zelf Pro token is present in the macOS Keychain.

### Free Tools

---

#### `browser.read`

| Field | Value |
|---|---|
| **Description** | Extract the full readable text content from the currently active browser tab |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{}
```
*(No parameters — operates on the active tab)*

**Outputs:**
```json
{
  "url": "string",
  "title": "string",
  "content": "string (extracted readable text, cleaned of nav/footer boilerplate)"
}
```

---

#### `browser.navigate`

| Field | Value |
|---|---|
| **Description** | Navigate the active browser tab to a specified URL |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{
  "url": "string (required) — fully qualified URL to navigate to"
}
```

**Outputs:**
```json
{
  "success": "boolean",
  "final_url": "string (after any redirects)",
  "title": "string"
}
```

---

#### `browser.search`

| Field | Value |
|---|---|
| **Description** | Open a Google search for the given query and return the top results |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{
  "query": "string (required) — the search query"
}
```

**Outputs:**
```json
{
  "results": [
    {
      "title": "string",
      "url": "string",
      "snippet": "string"
    }
  ]
}
```
*(Returns up to 10 organic results from the SERP)*

---

#### `browser.click`

| Field | Value |
|---|---|
| **Description** | Click a DOM element in the active tab identified by a CSS selector |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{
  "selector": "string (required) — CSS selector for the target element",
  "wait_after_ms": "integer (optional, default 500) — ms to wait after clicking"
}
```

**Outputs:**
```json
{
  "success": "boolean",
  "error": "string (if success is false)"
}
```

---

#### `browser.fill`

| Field | Value |
|---|---|
| **Description** | Fill a form field in the active tab with a given value |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{
  "selector": "string (required) — CSS selector for the input element",
  "value": "string (required) — value to fill into the field",
  "submit": "boolean (optional, default false) — press Enter after filling"
}
```

**Outputs:**
```json
{
  "success": "boolean",
  "error": "string (if success is false)"
}
```

---

#### `browser.screenshot`

| Field | Value |
|---|---|
| **Description** | Capture the visible area of the active browser tab as a PNG image |
| **Backend** | Chrome Extension |
| **Tier** | Free |

**Inputs:**
```json
{
  "save_path": "string (optional) — if provided, saves the image to this local path via Tauri backend"
}
```

**Outputs:**
```json
{
  "data_url": "string (base64 PNG data URL)",
  "saved_path": "string (if save_path was provided and succeeded)"
}
```

---

#### `fs.read`

| Field | Value |
|---|---|
| **Description** | Read the contents of a file from the local filesystem |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "path": "string (required) — absolute or ~ expanded file path"
}
```

**Outputs:**
```json
{
  "content": "string",
  "size_bytes": "integer",
  "encoding": "string (utf-8 or base64 for binary)"
}
```

---

#### `fs.write`

| Field | Value |
|---|---|
| **Description** | Write content to a file on the local filesystem, creating it if it does not exist |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "path": "string (required) — absolute or ~ expanded file path",
  "content": "string (required)",
  "append": "boolean (optional, default false) — append instead of overwrite"
}
```

**Outputs:**
```json
{
  "success": "boolean",
  "bytes_written": "integer"
}
```

---

#### `fs.list`

| Field | Value |
|---|---|
| **Description** | List the contents of a directory |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "path": "string (required) — directory path",
  "recursive": "boolean (optional, default false)",
  "filter": "string (optional) — glob pattern, e.g. '*.pdf'"
}
```

**Outputs:**
```json
{
  "entries": [
    {
      "name": "string",
      "path": "string",
      "kind": "file | directory",
      "size_bytes": "integer (files only)",
      "modified_at": "ISO8601 string"
    }
  ]
}
```

---

#### `shell.run`

| Field | Value |
|---|---|
| **Description** | Execute a bash command on the local machine and return its output |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "command": "string (required) — the shell command to execute",
  "cwd": "string (optional) — working directory",
  "timeout_ms": "integer (optional, default 30000)"
}
```

**Outputs:**
```json
{
  "stdout": "string",
  "stderr": "string",
  "exit_code": "integer",
  "timed_out": "boolean"
}
```

> **Security note:** shell.run executes with the permissions of the user running Zelfi. No sandboxing is applied — the user is in control of their own machine. This is intentional and consistent with the local-first philosophy.

---

#### `http.get`

| Field | Value |
|---|---|
| **Description** | Make an HTTP GET request and return the response |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "url": "string (required)",
  "headers": "object (optional) — key/value header map",
  "timeout_ms": "integer (optional, default 15000)"
}
```

**Outputs:**
```json
{
  "status": "integer",
  "body": "string",
  "headers": "object"
}
```

---

#### `http.post`

| Field | Value |
|---|---|
| **Description** | Make an HTTP POST request with a JSON or form body |
| **Backend** | Rust (Tauri native) |
| **Tier** | Free |

**Inputs:**
```json
{
  "url": "string (required)",
  "body": "object | string (required)",
  "content_type": "string (optional, default 'application/json')",
  "headers": "object (optional)",
  "timeout_ms": "integer (optional, default 15000)"
}
```

**Outputs:**
```json
{
  "status": "integer",
  "body": "string",
  "headers": "object"
}
```

---

#### `memory.recall`

| Field | Value |
|---|---|
| **Description** | Search Zelfi's semantic memory for relevant facts, preferences, or past context using full-text search across the markdown memory files |
| **Backend** | Rust (reads `~/.zelfi/memory/`) |
| **Tier** | Free |

**Inputs:**
```json
{
  "query": "string (required) — the subject to recall",
  "limit": "integer (optional, default 5) — max results to return"
}
```

**Outputs:**
```json
{
  "results": [
    {
      "file": "string (relative path within memory store)",
      "excerpt": "string (matched passage)",
      "score": "float (FTS relevance score)"
    }
  ]
}
```

---

#### `memory.store`

| Field | Value |
|---|---|
| **Description** | Store a fact, preference, or note in Zelfi's semantic memory. Appends to the appropriate domain file and commits the change to the git memory repo. |
| **Backend** | Rust (writes `~/.zelfi/memory/facts/` via git2) |
| **Tier** | Free |

**Inputs:**
```json
{
  "domain": "string (required) — one of: preferences, projects, contacts, tools, general",
  "content": "string (required) — the fact or note to store",
  "source": "string (optional) — where this came from (e.g. 'user stated', 'inferred from goal')"
}
```

**Outputs:**
```json
{
  "success": "boolean",
  "commit_hash": "string (short git hash of the commit)",
  "file_path": "string"
}
```

---

#### `inbox.notify`

| Field | Value |
|---|---|
| **Description** | Creates a persistent flagged item in the user's Inbox; use when the agent needs human input, wants to surface a completed result for review, or notices something worth the user's attention |
| **Backend** | Rust |
| **Tier** | Free |

**Inputs:**
```json
{
  "type": "string (required) — NEEDS_INPUT | REVIEW | FYI",
  "title": "string (required, max 80 chars) — short summary of the item",
  "body": "string (required) — full context, markdown-formatted"
}
```

**Outputs:**
```json
{
  "item_id": "string (UUID of the created Inbox item)"
}
```

---

### Pro Tools (gated by Zelf account authentication)

---

#### `ip.lookup`

| Field | Value |
|---|---|
| **Description** | Look up geolocation and network metadata for a given IP address, proxied via the Zelf server so API keys never reside on the user's machine |
| **Backend** | Rust → Zelf Pro server (proxied) |
| **Tier** | Pro |

**Inputs:**
```json
{
  "ip": "string (required) — IPv4 or IPv6 address"
}
```

**Outputs:**
```json
{
  "ip": "string",
  "country": "string",
  "region": "string",
  "city": "string",
  "latitude": "float",
  "longitude": "float",
  "isp": "string",
  "org": "string"
}
```

---

#### `[pro.placeholder]`

| Field | Value |
|---|---|
| **Description** | Placeholder for additional Zelf Pro server-backed tools to be defined as the Pro tier develops |
| **Backend** | Zelf Pro server (proxied) |
| **Tier** | Pro |

*Additional Pro tools will be registered here as the Zelf ecosystem grows. Examples include: scheduled task execution, cloud storage bridges, third-party API integrations.*

---

## 6. Memory Architecture

Zelfi uses a three-layer hybrid memory system. Each layer has a distinct scope, storage backend, and durability guarantee.

```
┌─────────────────────────────────────────────────────────────────┐
│                    ZELFI MEMORY LAYERS                          │
├─────────────────────────┬────────────────┬──────────────────────┤
│ Layer                   │ Backend        │ Scope                │
├─────────────────────────┼────────────────┼──────────────────────┤
│ Working Memory          │ SQLite         │ Current session      │
│ Episodic Memory         │ Git markdown   │ Past sessions        │
│ Semantic Memory         │ Git markdown   │ Persistent facts     │
└─────────────────────────┴────────────────┴──────────────────────┘
```

---

### Layer 1 — Working Memory (SQLite)

**Location:** `~/.zelfi/zelfi.db` (SQLite file)  
**Scope:** Current session only  
**Wiped on:** Clean exit (or configurable retention)

Working memory is the agent's short-term context. It holds everything the agent needs for the current goal:

**Tables:**

```sql
-- Current active goal and its decomposed steps
CREATE TABLE goal_state (
    id TEXT PRIMARY KEY,
    goal_text TEXT NOT NULL,
    status TEXT CHECK(status IN ('active','paused','complete','failed')),
    plan_json TEXT,  -- JSON array of steps
    created_at INTEGER,
    updated_at INTEGER
);

-- Individual steps within a goal
CREATE TABLE task_steps (
    id TEXT PRIMARY KEY,
    goal_id TEXT REFERENCES goal_state(id),
    step_index INTEGER,
    tool_name TEXT,
    tool_inputs TEXT,  -- JSON
    status TEXT CHECK(status IN ('pending','running','complete','failed','skipped')),
    result TEXT,       -- JSON tool output
    started_at INTEGER,
    completed_at INTEGER
);

-- Session conversation history (goal + agent turns)
CREATE TABLE session_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_id TEXT,
    role TEXT CHECK(role IN ('user','agent','tool')),
    content TEXT,
    event_type TEXT,  -- thought, tool_call, tool_result, replan, etc.
    created_at INTEGER
);
```

Working memory is fast and ephemeral. It is not intended to survive between sessions, though the data remains in SQLite until it is explicitly pruned.

---

### Layer 2 — Episodic Memory (Git-backed Markdown)

**Location:** `~/.zelfi/memory/episodes/`  
**Format:** One markdown file per session  
**Versioned by:** Git (libgit2 / git2 crate)

At the end of every session (clean exit, or when the user stops the agent), Zelfi:

1. Summarises the session using the LLM (goal, steps taken, outcome, anything notable).
2. Writes the summary to a new file: `~/.zelfi/memory/episodes/YYYY-MM-DD-HH-MM-<slug>.md`
3. Commits it to the git repo with message: `session: <goal slug> YYYY-MM-DD HH:MM`

**Example episode file:**

```markdown
# Session: Summarise PDFs in Downloads
**Date:** 2026-06-06 14:32
**Goal:** Summarise all PDFs in ~/Downloads and save highlights to Desktop
**Outcome:** Complete

## Steps taken
1. Listed ~/Downloads — found 7 PDF files
2. Read each file using fs.read
3. Summarised each using the LLM
4. Wrote consolidated highlights to ~/Desktop/pdf-highlights.md

## Notable observations
- One PDF (invoice-march.pdf) was password-protected and could not be read
- User may want to revisit this file manually
```

Episodic memory is:
- Human-readable at any time
- Rollback-capable (any episode can be reverted via git)
- Searchable by `memory.recall` (FTS across all markdown files)

---

### Layer 3 — Semantic Memory (Git-backed Markdown)

**Location:** `~/.zelfi/memory/facts/`  
**Format:** Domain-partitioned markdown files  
**Versioned by:** Git (every update is a commit)

Semantic memory holds persistent, structured knowledge about the user. It is updated by `memory.store()` calls, and directly human-editable — Zelfi picks up changes on next load.

**Domain files:**

| File | Contents |
|---|---|
| `preferences.md` | Likes, dislikes, communication style, working hours, preferences |
| `projects.md` | Active and recent projects — name, status, key details |
| `contacts.md` | People Zelfi knows about — name, role, relationship, contact hints |
| `tools.md` | Software and tools the user uses — names, versions, habits |
| `general.md` | Catch-all for facts that don't fit the above categories |

**Example `contacts.md`:**

```markdown
# Contacts

## Leoni
- Role: Colleague at Media24 B2B
- Context: Collaborates on the Medical Academic website build
- Notes: Standing Monday morning meeting
- First mentioned: 2026-06-01

## Ruan van Heerden
- Role: Owner / user
- Full name: Ruan Xavier van Heerden Schultze Smith
- Email: ruan.vanheerden@media24.com
```

Every call to `memory.store()` appends to the relevant domain file and commits with message: `memory: update <domain> — <brief description>`.

---

### Passive Learning

During any session, if the agent encounters a name, project, or preference that is not in semantic memory, it surfaces a non-blocking toast notification:

> **"I noticed you mentioned Leoni — want me to remember that?"**  
> [Remember] [Skip]

One click on **Remember** triggers `memory.store()` with the extracted fact. The toast disappears. No interruption to the current task.

---

### Future Upgrade Path

- **Current:** SQLite FTS5 for full-text search across markdown files
- **Planned (not v1):** Local embedding model (e.g. `nomic-embed-text` via LM Studio) for vector similarity search. This improves recall for conceptually related queries that do not share keywords. The upgrade path is additive — the FTS layer remains as a fallback.

---

## 7. Input / Output Pipeline Architecture (Voice-Ready)

The input/output pipeline is designed as an abstraction from day one. Only text is implemented in v1, but the architecture accommodates voice without structural rework.

### Rust Traits

```rust
/// Abstracts any source of user input — keyboard, voice, future modalities
trait InputSource: Send {
    async fn next_input(&mut self) -> String;
}

/// Abstracts any output channel — UI display, voice synthesis, future modalities
trait OutputSink: Send + Sync {
    async fn emit(&self, text: &str);
}
```

All agent loop code interacts with `InputSource` and `OutputSink` — never with a concrete implementation directly. This ensures that swapping in a voice implementation requires no changes to agent logic.

---

### Day One Implementations

#### `TextInput`

```rust
struct TextInput {
    receiver: tokio::sync::mpsc::Receiver<String>,
}

impl InputSource for TextInput {
    async fn next_input(&mut self) -> String {
        self.receiver.recv().await.unwrap_or_default()
    }
}
```

The `TextInput` receiver is fed from the SvelteKit frontend via a Tauri IPC command when the user submits a goal.

#### `TextOutput`

```rust
struct TextOutput {
    sender: tauri::AppHandle,
}

impl OutputSink for TextOutput {
    async fn emit(&self, text: &str) {
        self.sender.emit_all("agent_event", text).ok();
    }
}
```

`TextOutput` broadcasts agent events to all frontend windows via Tauri's event system. The SvelteKit frontend listens and appends to the task log.

---

### Planned Future Implementations (do not implement in v1)

#### `VoiceInput`

- **Approach A (preferred if available):** Route audio input directly to Gemma 4 E4B-it via LM Studio's mlx-engine, if it exposes audio input in its OpenAI-compatible API. This eliminates a separate STT step entirely, since Gemma 4 is natively multimodal.
- **Approach B (fallback):** Capture microphone audio via CoreAudio/AVFoundation, run through `whisper-rs` (Whisper.cpp Rust bindings) or Apple's Speech Framework for STT, then pipe the transcript to the existing text pipeline.

#### `VoiceOutput`

- **Approach A:** `AVSpeechSynthesizer` (macOS native, zero dependencies, good quality).
- **Approach B:** `Kokoro` or `Piper` TTS model, run locally, output via CoreAudio. Higher quality, more dependencies.

---

### UI — Voice Readiness

The goal input area in the Goals view includes a **push-to-talk button** rendered from day one. In v1 it is:

- Visible (microphone icon, next to the text input)
- Disabled (greyed out)
- Accompanied by a tooltip on hover: **"Voice input — coming soon"**

This ensures the layout is voice-ready when voice is implemented, with no UX rework required.

---

## 8. Browser Companion Extension

### Overview

| Field | Value |
|---|---|
| **Type** | Chrome Extension |
| **Manifest** | V3 |
| **Framework** | Vanilla JS only — no React, no Svelte, no bundler |
| **Communication** | WebSocket to Tauri backend (`ws://localhost:[dynamic port]`) |
| **Role** | Dumb executor — receives instructions, reports results. All intelligence stays in Tauri. |

### Permissions

```json
{
  "permissions": ["tabs", "scripting", "activeTab", "storage"],
  "host_permissions": ["<all_urls>"]
}
```

`<all_urls>` is required for `scripting.executeScript` to operate on arbitrary pages (needed for browser.read, browser.click, browser.fill, browser.screenshot).

### Communication Protocol

The Tauri backend starts a WebSocket server on a randomly assigned local port at startup. The port is written to `~/.zelfi/extension-port` and read by the extension via a local HTTP call on install/connect.

Message format (JSON):

```json
// Instruction (Tauri → Extension)
{
  "id": "uuid",
  "tool": "browser.click",
  "params": { "selector": "#submit-btn" }
}

// Result (Extension → Tauri)
{
  "id": "uuid",
  "success": true,
  "data": { ... },
  "error": null
}
```

### Extension UI (popup.html)

Minimal popup — approximately 280×120px:

```
┌──────────────────────────────────┐
│  🟢  Connected to Zelfi          │
│  Task: Summarise PDFs            │
│                                  │
│  [Disconnect]                    │
└──────────────────────────────────┘
```

States:
- **🟢 Connected** — WebSocket open, Zelfi active
- **🟡 Connecting** — attempting to connect (retry with backoff)
- **🔴 Disconnected** — Tauri app not running or port changed

### Capability Matrix

| Capability | Supported | Notes |
|---|---|---|
| Read page text content | ✅ | Via `document.body.innerText` extraction with boilerplate removal |
| Navigate to URL | ✅ | Via `chrome.tabs.update` |
| Open Google search | ✅ | Navigate + extract SERP results |
| Click element | ✅ | Via `scripting.executeScript` |
| Fill form field | ✅ | Via `scripting.executeScript` |
| Capture screenshot | ✅ | Via `chrome.tabs.captureVisibleTab` |
| Access cross-origin iframes | ❌ | Browser security boundary — not possible |
| Download files | ❌ | Not in v1 scope |
| Intercept network requests | ❌ | Not needed; would require `declarativeNetRequest` |
| Operate on multiple tabs simultaneously | ❌ | One active tab at a time in v1 |
| Work in incognito mode | ⚠️ | Requires explicit user permission in extension settings |

---

## 9. Onboarding UX

Onboarding is a 5-step progressive setup flow that runs the first time Zelfi is launched. It is skipped on subsequent launches (tracked via a `onboarding_complete` flag in SQLite or a local config file).

The flow is full-screen (covers the system tray window). It is warm, conversational, and free of jargon.

---

### Step 1 — Welcome

**Layout:** Full-screen, centred.

**Content:**
- Zelfi logo / icon, large, centred
- Single warm sentence below: *"Zelfi is your personal AI agent — it lives here, it thinks locally, and it remembers what matters to you."*
- One CTA button: **"Let's get started"**

**No settings, no configuration, no explanation of how it works.** This step exists solely to give the user a moment to orient and feel welcome.

---

### Step 2 — LM Studio Check

**Layout:** Full-screen with a status card in the centre.

**Behaviour:**
- On entering this step, Zelfi immediately pings `http://localhost:1234/v1/models`
- **If found:**
  - Show a green checkmark
  - Display: *"LM Studio is running and ready."*
  - Show the detected model name (from the API response)
  - CTA: **"Looks good — continue"**
- **If not found:**
  - Show a clear, human instruction (no error codes):  
    *"Zelfi uses LM Studio to think locally. Please open LM Studio, load a model, and press 'Check again'."*
  - Show a visual hint (an image or icon of LM Studio in the dock)
  - Button: **"Check again"** (re-pings the API)
  - Small secondary link: **"What is LM Studio?"** (opens the LM Studio website in the browser)

**No "skip" option** — LM Studio is required for Zelfi to function.

---

### Step 3 — About You

**Layout:** Full-screen, conversational. Feels like a chat, not a form.

**Behaviour:**
Zelfi presents gentle prompts one at a time. Each prompt appears with a small animation (fade in) once the previous one is answered. Answers are free-text — no dropdowns, no required formats.

Prompts (in order):
1. **"What's your name?"**
2. **"What do you do — in a sentence or two?"**
3. **"What are you currently working on?"**
4. **"Anything you'd like me to know upfront? (Optional — you can skip this one)"**

After all four are answered (or the last one is skipped):
- Zelfi processes the free-text answers through the LLM to extract structured facts.
- Those facts are written into `~/.zelfi/memory/facts/` (the first-ever git commit to the memory repo).
- Display: *"Got it. I've made a note of that — and you can always update it later in Settings → About Me."*
- CTA: **"Next"**

---

### Step 4 — Browser Extension

**Layout:** Full-screen. Split: left = explanation, right = visual.

**Content (left):**
- Heading: *"Want Zelfi to work in your browser?"*
- Short explanation: *"The Zelfi browser extension lets me read pages, click buttons, and fill forms on your behalf — directly in Chrome."*
- Button: **"Install Chrome Extension"** (opens Chrome Web Store / sideload page)
- Secondary link: **"Skip for now"** (smaller, below the button)

**Visual (right):**
- An illustration or screenshot of the Zelfi extension icon in the Chrome toolbar, with the popup visible.

**Behaviour:**
- After clicking Install, Zelfi opens the installation URL and waits.
- It polls the WebSocket port periodically — if the extension connects, the step auto-advances to a "Connected!" confirmation and then proceeds to Step 5.
- If the user clicks Skip, the step advances immediately. The extension can be installed later from Settings → Extension.

---

### Step 5 — Ready

**Layout:** Full-screen, celebratory but minimal.

**Content:**
- Zelfi's first message to the user, personalised using the name from Step 3:  
  *"You're all set, [name]. I'm ready when you are."*
- 2–3 example goal prompts displayed below the main goal input box as tappable chips or cards, to inspire first use. Examples:
  - *"Summarise all the PDFs in my Downloads folder"*
  - *"Search for the latest news on AI agents and give me a summary"*
  - *"List all files modified in the last 24 hours on my Desktop"*
- The goal input is active and focused — the user can type their first goal immediately.

**No back button.** Onboarding is complete. The system tray window is now in its normal Goals view state.

---

## 10. Advanced Settings — "About Me" Refinement UI

Accessible from **Settings → About Me** after onboarding is complete.

This view maps directly to the semantic memory fact files in `~/.zelfi/memory/facts/`. Each section corresponds to a domain or sub-domain. Saving any field commits the change to the git memory repo via `memory.store()`.

### Layout

The view is a single scrollable page within the Settings view. Each section has:
- A clear heading
- A multi-line free-text area (no character limits)
- A **Save** button (per section, not a global save — to avoid accidental overwrites)
- A small note: *"Changes are saved to your local memory and can be edited directly at `~/.zelfi/memory/facts/`"*

### Sections

| Section heading | Maps to in memory |
|---|---|
| Things I like... | `preferences.md` → likes |
| Things I don't like... | `preferences.md` → dislikes |
| Topics I care about... | `preferences.md` → topics |
| My communication style... | `preferences.md` → communication_style |
| Projects I'm working on... | `projects.md` |
| People I work with... | `contacts.md` |
| Tools I use... | `tools.md` |

### Behaviour

- On load: each field is pre-populated by reading and parsing the relevant section from the markdown files.
- On save: the updated content is written back to the markdown file and a git commit is made: `memory: user updated <section> via Settings`
- The About Me view does **not** use the LLM — it is direct editing of the memory store, with the agent's git hygiene applied on top.

---

## 11. Zelf Pro — Architecture Hooks

**Do not implement in v1.** This section documents the planned approach so the architecture is Pro-ready from day one.

### Auth Module (Rust backend)

```rust
// Conceptual — not v1 implementation
struct ZelfProAuth {
    keychain_service: String,   // "zelfi-pro"
    keychain_account: String,   // user's Zelf account email
}

impl ZelfProAuth {
    async fn login(&self, email: &str, password: &str) -> Result<Token>;
    async fn get_token(&self) -> Option<Token>;  // reads from macOS Keychain
    async fn validate_token(&self) -> bool;      // calls Zelf server
    async fn logout(&self);                      // removes from Keychain
}
```

### Token Storage

- JWT / opaque token stored in the **macOS Keychain** (via the `keychain` or `security-framework` crate).
- Never stored in a flat file, SQLite, or the memory repo.
- Retrieved at startup; if absent, Pro tools are not registered.

### Pro Tool Registration

At startup, the tool executor:

1. Calls `ZelfProAuth::validate_token()`.
2. If valid: registers all Pro tools in the tool registry.
3. If invalid / absent: Pro tools are absent from the registry. If the LLM tries to call one, the tool executor returns a structured error: *"This tool requires Zelf Pro. Visit [link] to upgrade."*

### Pro API Proxy

All Pro tool API calls are routed through the Zelf server:

```
Zelfi (Rust) → Zelf Pro server (HTTPS) → Third-party API
```

API keys for third-party services (e.g. IP geolocation providers) reside only on the Zelf server — never on the user's machine. The Zelf server validates the JWT before proxying the request.

### Pricing

- **~R120/year** — Zelf account login
- Access to all Zelf ecosystem extras (Pro tools + future Zelf services)
- Priced to be affordable for individuals, sustainable for the project

---

## 12. UI Structure

### Five Main Views

| View | Purpose |
|---|---|
| **Goals** | Primary input, real-time agent log, pause/stop controls |
| **Inbox** | Flagged items from the agent — requires acknowledgement; default landing when badge is active |
| **History** | Past goals, steps taken, outcomes — searchable |
| **Memory** | View/edit semantic memory files; review episodic summaries |
| **Settings** | LM Studio config, extension pairing, About Me, voice (future), Pro |

### UX Principles

- **Dark mode by default.** System theme is respected — if the user switches macOS to light mode, Zelfi follows. Dark is the default when no preference is detected.
- **Minimal chrome.** The goal input and task log are the hero elements. Navigation is secondary.
- **Real-time streaming.** Every agent step appears in the task log as it happens — no polling, no "thinking..." spinner that hides progress. The user sees: thought → tool call → result → next thought.
- **Pause and Stop are always reachable.** During task execution, Pause and Stop are prominent in the Goals view — never hidden behind a menu or modal. They are never disabled.
- **Passive memory prompts are non-blocking.** Toast notifications for memory suggestions appear at the bottom of the screen. They do not interrupt the task log, do not require immediate action, and auto-dismiss after 10 seconds if ignored.
- **No confirmation dialogs for tool calls.** Zelfi is Level C autonomous — it does not ask "are you sure?" before each tool call. If the user wants oversight, they use Pause.
- **When the tray icon badge is active, the window opens directly to the Inbox view.** The badge signals items that need attention — the app respects that signal by landing the user where they need to be.
- **Inbox is the bridge between overnight autonomous runs and the user's morning review.** Anything the agent could not resolve without human input, or completed while the user was away, lives here until explicitly acknowledged.

### Tray Icon Badge

The macOS system tray icon displays a numeric badge when unread items requiring attention exist in the Inbox:

- 🔴 **NEEDS INPUT** items contribute to the badge count — the agent is blocked and cannot continue without the user.
- 🟡 **REVIEW** items contribute to the badge count — a task completed with caveats or warnings that the user should look at.
- 🔵 **FYI** items do **not** contribute to the badge count — they appear in Inbox but do not inflate urgent signals.
- The badge clears automatically when all 🔴 and 🟡 items are acknowledged or dismissed.
- Tauri 2.0's `tray-icon` feature supports numeric badge display natively via `TrayIconBuilder`.

### Goals View — Detailed Layout

```
┌──────────────────────────────────────────────────────────────┐
│  Zelfi                                    [Pause] [Stop]     │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Task log (scrollable, newest at bottom)              │   │
│  │                                                      │   │
│  │  💭 Planning how to approach this...                 │   │
│  │  🔧 browser.navigate → https://example.com           │   │
│  │  ✅ Page loaded: "Example Domain"                    │   │
│  │  💭 Content retrieved. Extracting key points...      │   │
│  │  🔧 fs.write → ~/Desktop/summary.md                  │   │
│  │  ✅ File written (842 bytes)                          │   │
│  │  🏁 Done. Summary saved to ~/Desktop/summary.md      │   │
│  │                                                      │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌──────────────────────────────────────┐  [🎤]  [Send]    │
│  │  Enter a goal...                     │                   │
│  └──────────────────────────────────────┘                   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

- `[🎤]` is the push-to-talk button — visible, disabled in v1, with "Voice input — coming soon" tooltip.
- `[Pause]` and `[Stop]` are only active during task execution; greyed out when idle.

---

## 13. Tech Stack

| Layer | Technology | Version / Notes |
|---|---|---|
| App shell | Tauri | 2.0 (latest stable) |
| Frontend framework | SvelteKit | 2.x, static adapter (`@sveltejs/adapter-static`) |
| UI component library | DaisyUI | 4.x (Tailwind CSS 3.x) |
| Frontend language | TypeScript | 5.x |
| Backend language | Rust | Stable toolchain, 2021 edition |
| Async runtime | Tokio | 1.x |
| HTTP client (Rust) | reqwest | 0.12.x |
| WebSocket server | tokio-tungstenite | Latest compatible |
| Database | SQLite | 3.x via `rusqlite` crate |
| Git integration | libgit2 via git2 crate | Latest compatible |
| LLM inference | LM Studio | localhost:1234/v1 (OpenAI-compatible REST API) |
| LLM model | Gemma 4 E4B-it MLX 4-bit | Loaded in LM Studio; multimodal (text + image + audio) |
| Extension framework | Vanilla JS | No bundler, no framework — Manifest V3 |
| Extension communication | WebSocket | Native browser WebSocket API |
| macOS Keychain | security-framework crate | For Pro token storage |
| Platform | macOS Apple Silicon | M1 Pro minimum; Intel not supported |
| Versioning | CalVer | `YYYY.MM.DD.N` |
| Memory FTS | SQLite FTS5 | For markdown file full-text search (v1) |
| Future memory search | Local embedding model | Vector similarity — planned, not v1 |

---

## 14. Non-Goals

The following are explicitly out of scope for Zelfi v1. They may be planned for later but will not influence v1 architecture decisions.

| Non-goal | Reason / Note |
|---|---|
| Cloud sync of memory or goals | Zelfi is local-first; cloud sync contradicts the core philosophy |
| Windows or Linux support | macOS Apple Silicon only; cross-platform adds significant complexity |
| Voice input / output | Architecture is voice-ready; implementation is post-v1 |
| Multi-user support | Single-user, single-machine app |
| Plugin marketplace | No third-party extension system in v1 |
| Custom model training or fine-tuning | Zelfi uses pre-trained models via LM Studio; training is out of scope |
| Replacing LM Studio | LM Studio is a dependency, not a target for replacement |
| Headless / server mode | Zelfi is a desktop app; server mode is not a use case |
| Mobile apps (iOS, Android) | Desktop only |
| Browser extension for Firefox or Safari | Chrome/Chromium only in v1 |
| Automated scheduling of goals | Goals are user-initiated in v1 |
| Collaborative goals (shared between users) | Single-user only |

---

## 15. MVP Scope vs Later

| Feature | PoC / MVP (v1) | Later |
|---|---|---|
| System tray app (macOS) | ✅ | — |
| Goal input (text) | ✅ | Voice input |
| ReAct agent loop | ✅ | — |
| LM Studio integration | ✅ | Direct model loading (no LM Studio dependency) |
| All Free tools | ✅ | — |
| Chrome companion extension | ✅ | Firefox / Safari extensions |
| SQLite working memory | ✅ | — |
| Git-backed episodic memory | ✅ | — |
| Git-backed semantic memory | ✅ | Vector embedding layer |
| Memory FTS (SQLite FTS5) | ✅ | Vector similarity search |
| Passive memory prompts (toasts) | ✅ | — |
| 5-step onboarding flow | ✅ | — |
| About Me refinement UI | ✅ | — |
| Goals view (real-time log) | ✅ | — |
| History view | ✅ | — |
| Memory view | ✅ | — |
| Settings view | ✅ | — |
| Dark mode / system theme | ✅ | — |
| Push-to-talk button (disabled) | ✅ (placeholder) | Working voice input |
| Zelf Pro auth module | ⬜ hooks only | Full implementation |
| `ip.lookup` Pro tool | ⬜ defined, not active | Active with Pro auth |
| Additional Pro tools | ⬜ placeholder | Defined as Zelf ecosystem grows |
| Voice input (VoiceInput trait) | ⬜ trait defined | Implementation |
| Voice output (VoiceOutput trait) | ⬜ trait defined | Implementation |
| Vector memory embeddings | ⬜ documented | Implementation |
| Automated goal scheduling | ❌ non-goal v1 | Possible later |
| Multi-user | ❌ non-goal | Not planned |
| Windows / Linux | ❌ non-goal | Not planned |

Legend: ✅ In scope | ⬜ Architecture hooks only | ❌ Non-goal

---

## 16. Project Structure

The Zelfi project lives at `data/zelf/zelfi/` in the workspace. It contains the Tauri application and the Chrome companion extension as sibling directories.

```
data/zelf/zelfi/
├── VERSION                          # CalVer version file — e.g. 2026.06.06.01
├── SPEC.md                          # This document — single source of truth
├── README.md                        # Short project overview and dev setup
│
├── app/                             # Tauri 2.0 application
│   ├── src-tauri/                   # Rust backend
│   │   ├── Cargo.toml
│   │   ├── Cargo.lock
│   │   ├── tauri.conf.json          # Tauri configuration
│   │   ├── build.rs
│   │   └── src/
│   │       ├── main.rs              # Entry point — sets up Tauri, tray, windows
│   │       ├── agent/
│   │       │   ├── mod.rs
│   │       │   ├── loop.rs          # ReAct agent loop (PLAN→SELECT→ACT→OBSERVE→EVALUATE)
│   │       │   ├── planner.rs       # LLM-based planning and replanning
│   │       │   └── events.rs        # AgentEvent struct and IPC emission
│   │       ├── tools/
│   │       │   ├── mod.rs           # Tool registry — registers tools at startup
│   │       │   ├── registry.rs      # Tool trait and registration logic
│   │       │   ├── browser.rs       # browser.* tools (dispatched via WebSocket)
│   │       │   ├── fs.rs            # fs.read, fs.write, fs.list
│   │       │   ├── shell.rs         # shell.run
│   │       │   ├── http.rs          # http.get, http.post
│   │       │   ├── memory.rs        # memory.recall, memory.store
│   │       │   └── pro/
│   │       │       ├── mod.rs       # Pro tool conditional registration
│   │       │       └── ip.rs        # ip.lookup (proxied via Zelf server)
│   │       ├── memory/
│   │       │   ├── mod.rs
│   │       │   ├── working.rs       # SQLite working memory (rusqlite)
│   │       │   ├── episodic.rs      # Git-backed episode files (git2)
│   │       │   ├── semantic.rs      # Git-backed fact files (git2)
│   │       │   └── search.rs        # FTS5 search across markdown files
│   │       ├── io/
│   │       │   ├── mod.rs
│   │       │   ├── traits.rs        # InputSource + OutputSink trait definitions
│   │       │   ├── text_input.rs    # TextInput implementation
│   │       │   └── text_output.rs   # TextOutput implementation
│   │       ├── websocket/
│   │       │   ├── mod.rs
│   │       │   └── server.rs        # WebSocket server for extension communication
│   │       ├── auth/
│   │       │   ├── mod.rs
│   │       │   └── zelf_pro.rs      # ZelfProAuth hooks (not active in v1)
│   │       ├── llm/
│   │       │   ├── mod.rs
│   │       │   └── client.rs        # LM Studio API client (OpenAI-compatible)
│   │       └── db/
│   │           ├── mod.rs
│   │           └── schema.rs        # SQLite schema migrations
│   │
│   ├── src/                         # SvelteKit frontend
│   │   ├── app.html
│   │   ├── app.css                  # Tailwind + DaisyUI base styles
│   │   ├── lib/
│   │   │   ├── stores/
│   │   │   │   ├── agent.ts         # Agent state store (goal, steps, status)
│   │   │   │   └── memory.ts        # Memory view state store
│   │   │   ├── components/
│   │   │   │   ├── TaskLog.svelte   # Real-time agent event log
│   │   │   │   ├── GoalInput.svelte # Goal text input + push-to-talk button
│   │   │   │   ├── MemoryToast.svelte # Passive memory prompt toast
│   │   │   │   └── StatusBadge.svelte # LM Studio / extension connection status
│   │   │   └── tauri.ts             # Tauri IPC helpers and event listeners
│   │   └── routes/
│   │       ├── +layout.svelte       # App shell — nav, system tray window frame
│   │       ├── goals/
│   │       │   └── +page.svelte     # Goals view (primary)
│   │       ├── history/
│   │       │   └── +page.svelte     # History view
│   │       ├── memory/
│   │       │   └── +page.svelte     # Memory view (episodic + semantic tabs)
│   │       ├── settings/
│   │       │   ├── +page.svelte     # Settings root
│   │       │   └── about-me/
│   │       │       └── +page.svelte # About Me refinement UI
│   │       └── onboarding/
│   │           ├── +page.svelte     # Onboarding shell (step controller)
│   │           ├── step1-welcome/
│   │           │   └── +page.svelte
│   │           ├── step2-lmstudio/
│   │           │   └── +page.svelte
│   │           ├── step3-about-you/
│   │           │   └── +page.svelte
│   │           ├── step4-extension/
│   │           │   └── +page.svelte
│   │           └── step5-ready/
│   │               └── +page.svelte
│   │
│   ├── static/                      # Static assets
│   │   ├── zelfi-icon.png
│   │   └── zelfi-icon.icns          # macOS app icon
│   ├── package.json
│   ├── svelte.config.js             # static adapter config
│   ├── tailwind.config.js
│   └── vite.config.ts
│
└── extension/                       # Chrome companion extension (Manifest V3)
    ├── manifest.json                # Extension manifest
    ├── background.js                # Service worker — WebSocket client, message routing
    ├── content.js                   # Content script — DOM interaction (injected by scripting API)
    ├── popup/
    │   ├── popup.html               # Extension popup UI
    │   ├── popup.css
    │   └── popup.js
    └── icons/
        ├── icon-16.png
        ├── icon-32.png
        ├── icon-48.png
        └── icon-128.png
```

### User Data Directory

Zelfi stores all user data at `~/.zelfi/` on the user's machine. This directory is created on first launch.

```
~/.zelfi/
├── zelfi.db                         # SQLite database (working memory + task state)
├── extension-port                   # Current WebSocket port for extension communication
├── config.json                      # App config (LM Studio URL, theme preference, etc.)
└── memory/                          # Git repository (initialised on first launch)
    ├── .git/
    ├── episodes/                    # One markdown file per session
    │   ├── 2026-06-06-14-32-summarise-pdfs.md
    │   └── ...
    └── facts/                       # Persistent semantic memory
        ├── preferences.md
        ├── projects.md
        ├── contacts.md
        ├── tools.md
        └── general.md
```

---


## 17. Inbox & Notification System

### Overview

Zelfi operates two distinct notification layers:

| Layer | Type | Lifetime | Example |
|---|---|---|---|
| Toast | Ephemeral, non-blocking | ~5 seconds, auto-dismisses | "Remember Leoni?" |
| Inbox item | Persistent, stored in SQLite | Until explicitly acknowledged | "Hit a paywall overnight — need your call" |

Toasts (already documented in Section 12) handle right-now suggestions. Inbox items handle anything that must survive a reboot and requires the user's eventual attention.

---

### Inbox item types

| Badge | Type | Meaning | Contributes to tray badge? |
|---|---|---|---|
| 🔴 NEEDS INPUT | Agent is blocked and cannot continue | User must respond before the task resumes | Yes |
| 🟡 REVIEW | Task completed with caveats or warnings | Agent finished but flags it for user review | Yes |
| 🔵 FYI | Informational, no action required | Agent noticed something worth surfacing | No |
| ✅ Acknowledged | Dismissed or acted upon | Retained for reference only | No |

---

### When Zelfi creates an Inbox item

The agent loop creates an Inbox item in these situations:

- **NEEDS INPUT:** a tool call fails due to a blocker requiring human judgement (paywall, missing credential, ambiguous instruction, rate limit requiring a decision)
- **NEEDS INPUT:** an overnight task is paused mid-loop waiting for a human decision
- **REVIEW:** a goal completes but the agent detected anomalies, low-confidence results, or partial failures
- **REVIEW:** a long-running task completes while the user is not active (determined by checking last UI interaction timestamp)
- **FYI:** the agent passively notices something worth surfacing that is not tied to the current goal (e.g. a recurring topic across tasks that might warrant a memory entry)

---

### Inbox item data structure

Each Inbox item stored in SQLite contains:

| Field | Type | Description |
|---|---|---|
| `id` | UUID | Unique identifier for the item |
| `type` | TEXT | `NEEDS_INPUT` \| `REVIEW` \| `FYI` \| `ACKNOWLEDGED` |
| `title` | TEXT | Short summary (max 80 chars) |
| `body` | TEXT | Full context, markdown-formatted |
| `task_id` | TEXT (nullable) | Reference to the related goal/task in the history table |
| `created_at` | INTEGER | Unix timestamp |
| `acknowledged_at` | INTEGER (nullable) | Unix timestamp when the user acknowledged the item |
| `action_taken` | TEXT (nullable) | String describing what the user did |

---

### Inbox view layout

The Inbox view is designed for a clear morning-review workflow. A UI developer should implement it as follows:

- **Header:** "Inbox" label + unread count badge (e.g. "Inbox · 3 unread"); "Mark all as reviewed" button for bulk acknowledgement of REVIEW + FYI items (not NEEDS_INPUT)
- **Sort order:** NEEDS_INPUT first → REVIEW → FYI → ACKNOWLEDGED (ACKNOWLEDGED items collapsed by default)
- **Item card:** each card shows the type badge (colour-coded per type), title, relative timestamp (e.g. "2h ago"), and the body text (collapsible if longer than ~3 lines)
- **Action buttons per type:**
  - 🔴 NEEDS INPUT: `[Resume task]` `[Dismiss]`
  - 🟡 REVIEW: `[View full log]` `[Dismiss]`
  - 🔵 FYI: `[Remember it]` (triggers `memory.store`) `[Dismiss]`
  - ✅ ACKNOWLEDGED: `[View]` only
- **Dismissal behaviour:** dismissing any item transitions it to ACKNOWLEDGED state — items are never deleted from the database
- **Empty state:** a clean, uncluttered message — "Nothing needs your attention right now."

---

### Storage

- SQLite table: `notifications` — separate from working memory tables; persists across sessions and reboots
- Does **not** use the git-backed memory store — Inbox items are operational state, not knowledge
- Retention: all items kept indefinitely; the user can manually clear ACKNOWLEDGED items from Settings

---

### Integration with the agent loop

- The Rust backend exposes a `notify(type, title, body, task_id)` function callable from the agent loop at any point in execution
- The agent loop calls `notify()` automatically when it detects a blocker condition (NEEDS_INPUT) or completes a long-running task while the user is inactive (REVIEW)
- The LLM may also explicitly call `notify()` as a tool via `inbox.notify` (registered in Section 5)
- On any `notify()` call:
  1. Item is written to the `notifications` table in SQLite
  2. Tray badge count updates immediately via a Tauri event emitted from the Rust backend
  3. The UI Inbox view refreshes automatically if it is currently open

---


---

## 18. Prompt Efficiency & Context Management

### Overview

Zelfi's agent loop accumulates context across every ReAct cycle. With Gemma 4 E4B-it's 128k context window, the risk is not hitting a length limit — it's speed and RAM pressure. Every extra token in the KV cache slows generation and increases memory footprint on the M1 Pro. This section defines how Zelfi keeps its context lean without sacrificing quality.

---

### 18.1 — Tool Result Compression Pipeline

Every tool result passes through a compression step before being appended to context. Raw tool output is never injected directly.

**Pipeline steps:**
1. **Truncate** — if result exceeds the raw character limit (default: 8,000 chars, configurable), truncate to that limit
2. **Summarise** — if the truncated result is still above a secondary threshold (default: 2,000 chars), trigger a fast summarisation pass using a second call to Gemma 4 with a minimal prompt: `"Summarise the following content, retaining all facts relevant to: [current goal]. Be concise."`
3. **Tag** — always prepend a compression tag so the agent knows what it's working with: `[SOURCE: browser.search | original: 42KB | compressed: 380 chars]`

**Per-tool compression rules (document each):**
- `browser.read` → extract: page title, headings (h1–h3), key paragraphs, visible links. Drop: nav, footer, ads, scripts, styles
- `browser.search` → top 5 results only, format: `title | url | snippet`
- `browser.screenshot` → not compressed (image); described in text by the agent after viewing
- `shell.run` → keep last 100 lines of stdout/stderr if output exceeds limit; prepend line count
- `fs.read` → chunk large files; read and compress in sections relevant to the current goal
- `http.get` / `http.post` → compress response body using same truncate → summarise pipeline
- `memory.recall` → return top 5 matches only, not full files

**Configuration (all in Settings → Advanced):**
- Raw result character limit: default 8,000
- Summarisation threshold: default 2,000
- Tool-specific overrides: configurable per tool

---

### 18.2 — Internal Reasoning Format

The agent's system prompt and internal ReAct steps use a structured, abbreviated format. No prose, no filler. The compression is not cosmetic — it is the format.

**ReAct step format:**
```
PLAN: [what the agent intends to do next, one line]
TOOL: [tool.name(param="value")]
RESULT: [compressed tool output with SOURCE tag]
EVAL: [done? continue? replan? one line decision]
```

**System prompt principles:**
- Tool definitions: name, inputs, outputs only — no verbose descriptions inside the prompt
- Memory block: compact context brief (see 18.4), not raw file contents
- Goal framing: one sentence, present tense
- No pleasantries, hedging, or filler anywhere in the system prompt

**Example internal cycle:**
```
PLAN: find current pricing for Xneelo managed hosting
TOOL: browser.search("Xneelo managed hosting pricing 2026")
RESULT: [SOURCE: browser.search | 5 results]
  1. Xneelo.co.za/hosting | "Managed hosting from R89/mo..."
  2. mybroadband.co.za | "Xneelo raises prices..."
  ...
EVAL: result 1 is authoritative. Read detail page.
PLAN: navigate to pricing page
```

---

### 18.3 — Rolling Context Compression

When the accumulated context grows beyond a configurable threshold, Zelfi compresses the oldest reasoning steps into a single history block and replaces them in the context.

**Trigger:** context size > 40% of model context window (default; configurable)

**Compression:**
- Steps older than the last N active cycles (default: 5) are summarised into a `[HISTORY]` block
- Format: `[HISTORY: goal fragment "X" — tools used: browser.search, fs.write — outcome: draft saved to ~/Desktop/draft.md]`
- Active steps (last N cycles) remain uncompressed
- The `[HISTORY]` block replaces the compressed steps in the context — it is not appended

**Implementation note:** the rolling compression is triggered by the Rust backend after each EVAL step; it checks token count estimate before sending the next prompt to LM Studio.

---

### 18.4 — Memory Injection Efficiency

Zelfi never injects raw memory files into the system prompt. Memory enters context as a compact brief.

**Session start:**
- Build a "context brief" from semantic memory facts: `~500 tokens max`
- Format: `[MEMORY: name=Ruan, role=Digital Solutions Manager, current_projects=[Medical Academic, Buyer's Guide], preferences=[brief responses, SA English, dark mode], contacts=[Leoni=colleague]]`
- Episodic memory: inject only the summary of the most recent session (`~200 tokens`)

**During a task:**
- `memory.recall` results are compressed per tool result rules (top 5 matches, tagged)
- Full memory files are never read into context directly; always recalled via search

**Memory brief refresh:** re-generated at session start and whenever `memory.store` commits a new fact during the session.

---

### 18.5 — User-Facing vs Internal Language

The agent maintains two distinct language registers:

| Context | Register | Example |
|---|---|---|
| Internal ReAct steps | Structured, abbreviated, machine-first | `EVAL: partial result. retry with refined query.` |
| Goals log (UI) | Clear, warm, natural language | "I found 5 pricing options — here's a summary..." |
| Inbox items | Natural language, markdown-formatted | Full sentences, human-readable body |
| Memory facts | Plain prose notes | Written to be read by a human directly |

The UI rendering layer is responsible for presenting agent output in natural language. Internal steps are never displayed raw — they are formatted by the SvelteKit frontend before display.

---

### 18.6 — Configuration Reference

All efficiency settings live under Settings → Advanced:

| Setting | Default | Description |
|---|---|---|
| Tool result raw limit | 8,000 chars | Truncate threshold before summarisation |
| Tool result summarise threshold | 2,000 chars | Trigger secondary summarisation pass |
| Rolling compression trigger | 40% | % of context window before rolling compression fires |
| Active step window | 5 cycles | How many recent ReAct cycles stay uncompressed |
| Memory brief max tokens | 500 | Cap on system prompt memory block |
| Recent episode tokens | 200 | Cap on episodic memory injection |

---

*End of SPEC.md — Zelfi v2026.06.06.04*
