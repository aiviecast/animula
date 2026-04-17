//! aituber-poc native host.
//!
//! Milestone 1: renderer only. As milestones add audio / http / ai, new
//! fields land on `HostState` and their imports get registered through the
//! `extra_imports` closure passed to `obsid_native::run`.

use anyhow::Result;
use obsid_native::{run, HostData, RenderState, WasiP1Ctx};

/// Composite wasmtime store data. Each future subsystem (audio, http, …)
/// adds its own field; corresponding `register_*_imports` helpers go into
/// the `extra_imports` closure.
struct HostState {
    render: RenderState,
    wasi: WasiP1Ctx,
    // audio: AudioState, http: HttpState, ai: AiState, …
}

impl HostData for HostState {
    fn render(&mut self) -> &mut RenderState {
        &mut self.render
    }
    fn wasi(&mut self) -> &mut WasiP1Ctx {
        &mut self.wasi
    }
}

fn main() -> Result<()> {
    let (path, bytes) = obsid_native::load_wasm_from_args("aituber-poc")?;
    let title = format!(
        "aituber-poc — {}",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("wasm")
    );

    run::<HostState, _, _>(
        bytes,
        title,
        |gpu, wasi| HostState {
            render: RenderState::new(gpu),
            wasi,
        },
        |_linker| {
            // Milestone 2 will register audio imports here:
            //   register_audio_imports(linker, |s: &mut HostState| &mut s.audio)?;
            Ok(())
        },
    )
}
