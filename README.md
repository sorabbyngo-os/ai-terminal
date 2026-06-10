<div align="center">
<br/>

# ai-terminal

### The AI-native terminal for Sorabbyngo OS — natural language commands, agent assistance, and context-aware completion.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/ai-terminal)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/ai-terminal)

</div>

---

## What It Does

ai-terminal is a next-generation terminal emulator for byngo-os that integrates Byngox AI assistance directly into the shell experience. It understands natural language, explains commands, warns about dangerous operations, and connects to the broader Sorabbyngo OS agent ecosystem.

---

## Features

| Feature | Description |
|---|---|
| **Natural Language Mode** | Type intent in plain English/Swahili — translated to shell commands |
| **Context-aware Completion** | Completions that understand your current project and filesystem |
| **Command Explanation** | Hover any command for an instant plain-language explanation |
| **Danger Detection** | Warns before executing destructive or irreversible operations |
| **Agent Integration** | Pipe output directly to Byngox agents for analysis |
| **Multilingual** | English and Swahili UI — more African languages coming |
| **Voice Input** | Optional Chameha voice input for hands-free terminal use |

---

## Architecture

```
ai-terminal/
├── src/
│   ├── terminal/     # Terminal emulator core (VTE-compatible)
│   ├── shell/        # Shell integration (bash, zsh, fish)
│   ├── nlp/          # Natural language → command translation (Byngox)
│   ├── completion/   # Context-aware completion engine
│   ├── safety/       # Danger detection and confirmation flows
│   ├── voice/        # Chameha voice input integration
│   └── ui/           # Wayland/X11 rendering (wgpu-based)
├── Cargo.toml
└── README.md
```

---

## Natural Language Examples

```
$ create a compressed backup of my documents folder
→ tar -czf ~/backups/documents-$(date +%Y%m%d).tar.gz ~/Documents

$ show me what's using the most disk space
→ du -h --max-depth=2 ~ | sort -rh | head -20

$ find all python files modified in the last week
→ find . -name "*.py" -mtime -7
```

---

## Stack

- **Language:** Rust
- **Rendering:** wgpu (Wayland + X11)
- **AI:** Byngox agent (sorabbyngo-os/byngox)
- **Voice:** Chameha (sorabbyngo-os/chameha)
- **OS target:** byngo-os, server-os

---

*Sorabbyngo OS · Kisii, Kenya · 2026*
