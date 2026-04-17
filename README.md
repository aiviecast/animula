# animula

> *Animula, vagula, blandula, / Hospes comesque corporis…*
> — Hadrian (117–138 CE)

**animula** is a framework for AI VTubers written in
[Almide](https://github.com/almide/almide). The name is Latin for "little
soul" — the soul that inhabits the character, drawn from Hadrian's famous
deathbed poem addressing his own departing spirit.

## Vision

End-to-end AI VTuber operation — rendering, audio, network, LLM, TTS —
with the character logic written in Almide and a single native Rust host.
The bet: a tiny binary, LLM-authorable codebase is the right shape for the
next generation of AI-driven virtual presence.

## Status

**Milestone 1** — programmatic head (face + eyes + mouth) rendered via
obsid's renderer on a wasmtime + wgpu + winit native host. Mesh parts are
split so later milestones can animate them independently.

Audio, network, and AI integrations land in subsequent milestones.

## Layout

```
src/              Almide application (main.almd)
host/             Rust native host — composes obsid + (future) audio / http / ai
```

## Build + run

```bash
# 1. Build the Almide program to wasm.
mkdir -p build
almide build src/main.almd --target wasm -o build/animula.wasm

# 2. Run via the native host.
cd host && cargo run --release -- ../build/animula.wasm
```

## Roadmap

1. **Programmatic head** (this milestone) — face + eyes + mouth as separate
   meshes, orbit camera, ambient idle motion.
2. **Audio host extension** — `audio.play_pcm`, `audio.get_amplitude`. Wav
   playback end-to-end.
3. **HTTP host extension** — synchronous wasm → async host with a real
   `reqwest` client on the Rust side. Tested against a live LLM endpoint.
4. **TTS glue** — VOICEVOX (or equivalent). Text in, PCM out, played via 2.
5. **Lip sync** — 2's amplitude drives mouth Y scale, then morph weights
   once obsid gains morph targets.
6. **LLM loop** — user prompt → chat → TTS → animated speech.

Each milestone adds fields to `host/`'s composite `HostState` and registers
their imports against the wasmtime linker alongside obsid's renderer.

## Related

- [almide](https://github.com/almide/almide) — the language
- [obsid](https://github.com/almide/obsid) — 3D rendering runtime (this
  project's `host/native/` library is reused here)
- [lumen](https://github.com/almide/lumen) — math primitives
