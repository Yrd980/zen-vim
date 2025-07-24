````markdown
# 🧘 Zen-Vim

> Minimalist Neovim wrapper inspired by [snack.nvim](https://github.com/folke/snack.nvim), crafted for geeks who love clarity, speed, and simplicity.

Zen-Vim is a distraction-free editor environment that adheres to the "less is more" philosophy. Designed with intuitive leader-key navigation, fast picker UIs, and zero visual clutter — it's your stable, scriptable, hackable Vim core.

---

## ✨ Features

- 🚀 Ultra-fast startup with clean dashboard
- 🔎 Powerful pickers: files, text grep, buffers
- 💾 Resume last session or file with a keypress
- ✍️ Rename current file in-place
- 🎯 Minimal UI — no gutter, no LSP, no trees
- 🔧 snack.nvim-compatible pickers
- 🧩 Plugin-ready: extend without breaking core

---

## 📁 File Structure

```txt
~/.config/zenvim/
├── init.lua               # Entry point
├── plugins.lua            # Plugin loader
├── keymaps.lua            # Leader key mappings
├── core/
│   ├── rename.lua         # Rename logic
│   └── resume.lua         # Session restore
└── ui/
    ├── dashboard.lua      # ASCII dashboard
    └── pick.lua           # Picker UI proxy (snack wrapper)
````

---

## 🧪 Usage

### Launch Neovim:

```bash
NVIM_APPNAME=zenvim nvim
```

### Keybindings (Default `<leader>` is `<space>`):

| Action         | Keybinding   |
| -------------- | ------------ |
| 🗂 Find File   | `<leader>pf` |
| 🔍 Grep Text   | `<leader>pt` |
| 📚 Buffers     | `<leader>pb` |
| 🔁 Resume File | `<leader>pr` |
| ✏️ Rename File | `<leader>rn` |

---

## 📌 Requirements

* Neovim >= 0.9
* [lazy.nvim](https://github.com/folke/lazy.nvim)
* (Optional) [snack.nvim](https://github.com/folke/snack.nvim)

---

## 🧱 Installation

Clone this into your config directory:

```bash
git clone https://github.com/YOUR_USERNAME/zenvim ~/.config/zenvim
```

Then launch it with:

```bash
NVIM_APPNAME=zenvim nvim
```

---

## 🧠 Philosophy

Zen-Vim obeys **Open–Closed Principle**:

> “Open for extension, closed for modification.”

Instead of bloating the core, you extend it with optional pickers, tips, or dash modules.