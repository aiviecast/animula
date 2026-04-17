# aituber-poc

AI VTuber proof of concept. The goal is end-to-end operation — rendering,
audio, network, LLM, TTS — with the application logic written in
[Almide](https://github.com/almide/almide) and a single native Rust host.

## Status

Milestone 1 — programmatic head rendering via obsid's renderer. Audio,
network, and AI integrations land in subsequent milestones.

## Layout

```
src/              Almide application (main.almd)
host/             Rust native host — composes obsid + (future) audio / http / ai
```

## Build + run

```bash
# 1. Build the Almide program to wasm.
mkdir -p build
almide build src/main.almd --target wasm -o build/aituber.wasm

# 2. Run via the native host.
cd host && cargo run --release -- ../build/aituber.wasm
```

## Roadmap

1. **Programmatic head** (this milestone) — head + eyes + mouth rendered,
   orbit camera, ambient animation.
2. **Audio host extension** — `audio.play_pcm`, `audio.get_amplitude`. Wav
   playback end-to-end.
3. **HTTP host extension** — synchronous wasm → async host with a real
   `reqwest` client on the Rust side. Tested against a live LLM endpoint.
4. **TTS glue** — VOICEVOX (or equivalent). Text in, PCM out, played via 2.
5. **Lip sync** — 2's amplitude drives mouth morph (once meshes support
   morph targets).
6. **LLM loop** — user prompt → chat → TTS → animated speech.

Each milestone adds imports to `host/`'s `HostState` and registers them
against the wasmtime linker alongside obsid's existing renderer imports.
