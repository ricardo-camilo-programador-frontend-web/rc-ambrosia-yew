# 🍎 rc-ambrosia-yew — Ambrosia: Food of the Gods

> An interactive editorial article about the Ambrosia apple variety, built with **Yew + Rust + WebAssembly**.

![Ambrosia](https://img.shields.io/badge/Ambrosia-Food%20of%20the%20Gods-C0392B?style=for-the-badge)

## About

This is the first project in the **Apple Varieties** series — a collection where each apple variety is built with a different web framework. The hub lives at [click-on-the-malus-domestica-ide.netlify.app/apple-varieties](https://click-on-the-malus-domestica-ide.netlify.app/apple-varieties).

The Ambrosia apple was a chance seedling discovered in the early 1990s in British Columbia, Canada. Named after the mythical food of the Greek gods, it is now one of the most-produced apple varieties in Canada.

## Tech Stack

- **Rust** — Systems programming language
- **Yew 0.21** — Component framework for Rust targeting WebAssembly
- **Trunk** — WASM web application builder
- **web-sys / wasm-bindgen** — Web API bindings
- **rust-i18n** — Internationalization (20 languages)

## Features

- 📖 **11 Chapters** — Scrollytelling editorial: Origin, Timeline, Characteristics, Mythology, Parentage, Nutrition, World Map, Culinary, Ambrosia Gold, Series
- 🌍 **20 Languages** — Full i18n support for the world's most spoken languages
- 📱 **PWA** — Installable, offline-capable
- ♿ **Accessible** — WCAG AA, keyboard navigation, screen reader support, RTL for Arabic
- ⚡ **Performant** — Lighthouse 90+ target, lazy scroll reveals
- 🎨 **"Honeyed Divine" palette** — Hand-crafted design system

## Getting Started

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Development

```bash
trunk serve
```

### Build

```bash
trunk build --release
```

## Project Structure

```
src/
├── main.rs                 # Entry point
├── app.rs                  # Root component
├── components/             # 13 components (11 chapters + navbar + footer)
├── hooks/                  # Scroll reveal + reading progress
├── data/                   # Ambrosia + varieties data
├── i18n/                   # rust-i18n config + 20 locale files
│   └── locales/
└── styles/                 # 6 CSS files (tokens, base, editorial, components, animations, responsive)
```

## Design Inspiration

- [Agriculture & Farm Landing](https://www.figma.com/community/file/1523251924601311280) — Figma Community
- [Greenvest — Sustainable Agriculture](https://dribbble.com/) — Halo Design Studio
- Apple Product Pages — scrollytelling reference

## Requirements Checklist

- [x] counter.dev analytics
- [x] 20 languages (i18n)
- [x] PWA + localStorage/sessionStorage
- [x] GitHub link in footer
- [x] Clean Code (DRY, Object Calisthenics, self-documented)
- [x] No unused imports
- [x] Responsive (320px → 1920px)
- [x] RTL support for Arabic

## Author

**Ricardo Camilo**
- GitHub: [ricardo-camilo-programador-frontend-web](https://github.com/ricardo-camilo-programador-frontend-web)
- LinkedIn: [ricardo-camilo-programador-frontend-web-developer](https://www.linkedin.com/in/ricardo-camilo-programador-frontend-web-developer)

## License

MIT © 2026 Ricardo Camilo
