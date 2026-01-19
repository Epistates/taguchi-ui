# Taguchi UI

> Enterprise-grade UI for Design of Experiments (DOE) using orthogonal arrays.

This application provides a modern, high-performance interface for designing and analyzing experiments, powered by the [Taguchi](https://github.com/nickpaterno/taguchi) core library.

## Tech Stack

**Frontend**
- **Framework**: [SvelteKit](https://kit.svelte.dev/)
- **UI Architecture**: Svelte 5 (Runes)
- **Language**: TypeScript
- **Visualization**: [ECharts](https://echarts.apache.org/)

**Backend**
- **Framework**: [Tauri 2](https://tauri.app/)
- **Language**: Rust

## Architecture

**taguchi-ui** adheres to a strict separation of concerns:

- **Core Logic**: Statistical calculations (OA construction, ANOVA, S/N ratios) are handled by the `taguchi` Rust crate.
- **UI Logic**: The frontend manages user interaction, state, and persistence.
- **Tauri Bridge**: Acts as a thin wrapper, converting UI types to library types and invoking core functions.

## Prerequisites

- [Node.js](https://nodejs.org/) (Latest LTS)
- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) (OS-specific build tools)

## Getting Started

1. **Install dependencies:**
   ```bash
   bun install
   ```

2. **Run the development server:**
   ```bash
   bun tauri dev
   ```
   This will start the SvelteKit frontend and launch the Tauri application window.

## Building for Production

To create a release build for your operating system:

```bash
bun tauri build
```

The output binaries will be located in `src-tauri/target/release/bundle/`.

## License

MIT
