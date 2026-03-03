# Claunux

> A native Linux desktop app for Claude AI — with MCP support, local agent integration, and Anthropic cowork potential.

**Status:** MVP running — 2026-03-03
**Built by:** Base76 Research Lab

<p align="center">
  <img src="src/claunux.png" alt="Claunux" width="180" />
</p>

---

## Why this exists

Anthropic ships desktop apps for Mac and Windows. Linux users get nothing.

Claunux is what the Linux community deserves: a real native app — not a browser wrapper — built with the same care as the official clients.

---

## What makes it different from a browser tab

1. **Native MCP integration** — configure and run MCP servers directly in the app
2. **Agent sidebar** — real-time status from local AI agents alongside your chat
3. **System tray** — Claude always available, one keyboard shortcut away
4. **Filesystem context** — drag files/folders into context, no copy-paste
5. **Offline fallback** — routes to local model (Ollama) when disconnected
6. **Tight Linux integration** — D-Bus, notifications, Wayland/X11

---

## Login — dual mode

**Who can use it:**
- Anyone with a Claude account (Free, Pro, Team) — login via embedded claude.ai WebView
- Claude Code users — same login
- Developers — API key directly

```
Choose at startup:
  [A] API key          ← developers, power users
  [B] Log in with claude.ai  ← anyone with a Claude account
```

Mode B requires zero technical setup. Anyone paying for Claude today who runs Linux can use the app immediately. That's what makes it a product, not a dev tool.

---

## Tech

**Tauri 2** (Rust + WebView + Svelte)
- ~10MB binary vs Electron's ~150MB
- Linux-native: packages as `.deb`, `.AppImage`, `.flatpak`
- Rust backend = fast, secure, memory-safe

---

## Running locally

```bash
# Prerequisites
sudo apt-get install -y libssl-dev pkg-config libwebkit2gtk-4.1-dev

# Install
git clone https://github.com/base76-research-lab/claunux.git
cd claunux
npm install
cargo install tauri-cli --version "^2"

# Run (requires ANTHROPIC_API_KEY)
export ANTHROPIC_API_KEY=your_key_here
cargo tauri dev
```

---

## Roadmap

- [x] Chat view + Anthropic API
- [x] Claunux logo + sidebar navigation
- [x] Dark mode, custom titlebar
- [x] AppImage + .deb + .rpm packaging — [v0.1.0 release](https://github.com/base76-research-lab/claunux/releases/tag/v0.1.0)
- [ ] Flathub submission *(PR open)*
- [x] MCP server configuration panel
- [ ] File/folder drag-and-drop into context
- [ ] System tray + keyboard shortcut
- [x] Login via claude.ai WebView (Mode B)
- [ ] Offline fallback to Ollama

---

## About

Built by [Base76 Research Lab](https://github.com/base76-research-lab) — building what Anthropic hasn't gotten to yet.

Interested in collaborating? Open an issue or reach out.
