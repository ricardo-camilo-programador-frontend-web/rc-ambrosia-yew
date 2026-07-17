# 🍎 Ambrosia — Landing Page

> **Food of the Gods** — A thematic landing page about the Ambrosia apple variety, built with Yew + Rust + WebAssembly.

## About

The Ambrosia apple was discovered as a chance seedling in British Columbia, Canada. Its name comes from Greek mythology — _ambrosia_ being the food of the gods, granting immortality to those who consumed it.

This project is part of a series exploring apple varieties through modern web technology.

## Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Framework | [Yew](https://yew.rs/) |
| Build | [Trunk](https://trunkrs.dev/) |
| Target | WebAssembly (wasm32-unknown-unknown) |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/): `cargo install trunk wasm-bindgen-cli`
- wasm target: `rustup target add wasm32-unknown-unknown`

### Run

```bash
trunk serve
# Open http://localhost:8080
```

### Build for Production

```bash
trunk build --release
```

Output goes to `dist/`.

## Project Structure

```
rc-ambrosia-yew/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Root App component
│   └── components/
│       ├── hero.rs       # Hero section
│       ├── history.rs    # Origin & discovery
│       ├── traits.rs     # Visual & taste characteristics
│       └── footer.rs     # Footer
└── README.md
```

## Roadmap

- [x] Project scaffold
- [ ] Hero section with Ambrosia imagery
- [ ] Origin story (British Columbia, chance seedling)
- [ ] Characteristics (color, taste, texture, season)
- [ ] Cultivation & growing regions
- [ ] Nutritional information
- [ ] Multi-language support (PT-BR, EN, ES)
- [ ] PWA capabilities
- [ ] SEO optimization

## License

MIT

---

_Part of the **rc-ambrosia-yew** series — one project per apple variety._
