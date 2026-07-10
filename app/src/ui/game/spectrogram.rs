use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    AnalyserNode, AudioContext, CanvasRenderingContext2d, HtmlAudioElement, HtmlCanvasElement,
};

use crate::bird::Bird;

const SPECTROGRAM_WIDTH: i32 = 300;
const SPECTROGRAM_HEIGHT: i32 = 56;
const FFT_SIZE: u32 = 1024;
// TODO: write comment here
const MAX_DISPLAY_FREQUENCY_HZ: f64 = 15_000.0;

const COLOR_BLUE: (u8, u8, u8) = (57, 84, 227);
const COLOR_BLUE_LIGHT: (u8, u8, u8) = (164, 211, 229);
const COLOR_PINK_DARK: (u8, u8, u8) = (250, 143, 107);
const COLOR_RED_DARK: (u8, u8, u8) = (244, 59, 47);

/// Renders a spectrogram of `audio_element`'s playback, built up one column per frame via the
/// Web Audio API, with a playhead line tracking the current position. Starts (or resumes) the
/// render loop whenever `playing` becomes true, and clears itself whenever `bird` changes.
#[component]
pub fn Spectrogram(
    bird: Memo<Bird>,
    audio_element: Signal<Option<HtmlAudioElement>>,
    playing: Signal<bool>,
) -> Element {
    let mut spectrogram_canvas: Signal<Option<HtmlCanvasElement>> = use_signal(|| None);
    let mut playhead_canvas: Signal<Option<HtmlCanvasElement>> = use_signal(|| None);
    let analysis: Signal<Option<(AudioContext, AnalyserNode)>> = use_signal(|| None);
    let loop_active = use_hook(|| Rc::new(Cell::new(false)));

    use_effect(move || {
        let _ = bird.read();
        clear_canvas(spectrogram_canvas.read().as_ref());
        clear_canvas(playhead_canvas.read().as_ref());
    });

    use_effect(move || {
        if playing() {
            start_visualizer(
                audio_element,
                spectrogram_canvas,
                playhead_canvas,
                analysis,
                loop_active.clone(),
            );
        }
    });

    rsx! {
        div {
            class: "relative flex-1 rounded-lg overflow-hidden",
            canvas {
                class: "block w-full bg-blue-light",
                width: SPECTROGRAM_WIDTH,
                height: SPECTROGRAM_HEIGHT,
                onmounted: move |mnt| {
                    spectrogram_canvas
                        .set(mnt.downcast::<web_sys::Element>().cloned().map(|el| el.unchecked_into()))
                },
            }
            canvas {
                class: "absolute inset-0 block w-full pointer-events-none",
                width: SPECTROGRAM_WIDTH,
                height: SPECTROGRAM_HEIGHT,
                onmounted: move |mnt| {
                    playhead_canvas
                        .set(mnt.downcast::<web_sys::Element>().cloned().map(|el| el.unchecked_into()))
                },
            }
        }
    }
}

/// Kicks off the spectrogram render loop if it isn't already running.
fn start_visualizer(
    audio_element: Signal<Option<HtmlAudioElement>>,
    spectrogram_canvas: Signal<Option<HtmlCanvasElement>>,
    playhead_canvas: Signal<Option<HtmlCanvasElement>>,
    mut analysis: Signal<Option<(AudioContext, AnalyserNode)>>,
    loop_active: Rc<Cell<bool>>,
) {
    if loop_active.get() {
        return;
    }
    let (Some(audio), Some(spec_canvas), Some(head_canvas)) = (
        audio_element.read().clone(),
        spectrogram_canvas.read().clone(),
        playhead_canvas.read().clone(),
    ) else {
        return;
    };
    let Some((ctx, analyser)) = ensure_analysis(&mut analysis, &audio, &spec_canvas) else {
        return;
    };
    let sample_rate = f64::from(ctx.sample_rate());
    loop_active.set(true);
    start_render_loop(
        audio,
        analyser,
        sample_rate,
        spec_canvas,
        head_canvas,
        loop_active,
    );
}

/// Lazily builds the Web Audio graph (`AudioContext` -> `MediaElementAudioSourceNode` ->
/// `AnalyserNode` -> destination) for the given audio element, caching it in `analysis` since
/// an `HtmlMediaElement` can only ever be connected to one `MediaElementAudioSourceNode`.
fn ensure_analysis(
    analysis: &mut Signal<Option<(AudioContext, AnalyserNode)>>,
    audio: &HtmlAudioElement,
    spec_canvas: &HtmlCanvasElement,
) -> Option<(AudioContext, AnalyserNode)> {
    if let Some(existing) = analysis.read().as_ref() {
        existing.0.resume().ok();
        return Some(existing.clone());
    }

    let ctx = AudioContext::new()
        .inspect_err(|e| tracing::error!("AudioContext::new failed: {e:?}"))
        .ok()?;
    let source = ctx
        .create_media_element_source(audio)
        .inspect_err(|e| tracing::error!("create_media_element_source failed: {e:?}"))
        .ok()?;
    let analyser = ctx
        .create_analyser()
        .inspect_err(|e| tracing::error!("create_analyser failed: {e:?}"))
        .ok()?;
    analyser.set_fft_size(FFT_SIZE);
    source
        .connect_with_audio_node(&analyser)
        .inspect_err(|e| tracing::error!("source.connect failed: {e:?}"))
        .ok()?;
    analyser
        .connect_with_audio_node(&ctx.destination())
        .inspect_err(|e| tracing::error!("analyser.connect failed: {e:?}"))
        .ok()?;
    ctx.resume().ok();
    tracing::debug!(
        "spectrogram analysis graph ready, context state: {:?}",
        ctx.state()
    );

    clear_canvas(Some(spec_canvas));
    let built = (ctx, analyser);
    analysis.set(Some(built.clone()));
    Some(built)
}

/// Schedules a self-rescheduling `requestAnimationFrame` loop that draws one new spectrogram
/// column per frame at the audio's current playback position, plus a playhead line tracking it.
/// Stops rescheduling (and flips `active` back to false) as soon as the audio is paused.
fn start_render_loop(
    audio: HtmlAudioElement,
    analyser: AnalyserNode,
    sample_rate: f64,
    spec_canvas: HtmlCanvasElement,
    head_canvas: HtmlCanvasElement,
    active: Rc<Cell<bool>>,
) {
    let Some(spec_ctx) = get_2d_context(&spec_canvas) else {
        active.set(false);
        return;
    };
    let Some(head_ctx) = get_2d_context(&head_canvas) else {
        active.set(false);
        return;
    };

    let bin_count = analyser.frequency_bin_count() as usize;
    let bins_to_show = bins_to_show(bin_count, sample_rate);
    let mut data = vec![0u8; bin_count];

    let width = f64::from(SPECTROGRAM_WIDTH);
    let height = f64::from(SPECTROGRAM_HEIGHT);

    tracing::debug!(
        "spectrogram render loop starting: bin_count={bin_count}, bins_to_show={bins_to_show}, sample_rate={sample_rate}"
    );

    let tick = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let tick_handle = tick.clone();

    *tick.borrow_mut() = Some(Closure::new(move || {
        if audio.paused() {
            active.set(false);
            return;
        }

        let duration = audio.duration();
        if duration.is_finite() && duration > 0.0 {
            let progress = (audio.current_time() / duration).clamp(0.0, 1.0);
            let x = progress * width;
            analyser.get_byte_frequency_data(&mut data);
            draw_spectrogram_column(&spec_ctx, &data[..bins_to_show], x, height);
            draw_playhead(&head_ctx, x, width, height);
        }

        request_animation_frame(tick_handle.borrow().as_ref().unwrap());
    }));

    request_animation_frame(tick.borrow().as_ref().unwrap());
}

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
    }
}

fn bins_to_show(bin_count: usize, sample_rate: f64) -> usize {
    let nyquist = sample_rate / 2.0;
    let fraction = (MAX_DISPLAY_FREQUENCY_HZ / nyquist).clamp(0.0, 1.0);
    ((bin_count as f64) * fraction).ceil().clamp(1.0, bin_count as f64) as usize
}

fn draw_spectrogram_column(ctx: &CanvasRenderingContext2d, data: &[u8], x: f64, height: f64) {
    let row_height = height / data.len() as f64;
    for (i, &magnitude) in data.iter().enumerate() {
        let y = height - (i as f64 + 1.0) * row_height;
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(&magnitude_color(magnitude)));
        ctx.fill_rect(x, y, 1.0, row_height.max(1.0));
    }
}

fn draw_playhead(ctx: &CanvasRenderingContext2d, x: f64, width: f64, height: f64) {
    ctx.clear_rect(0.0, 0.0, width, height);
    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(&rgb_string(COLOR_RED_DARK)));
    ctx.fill_rect(x, 0.0, 1.5, height);
}

fn magnitude_color(magnitude: u8) -> String {
    const STOPS: [(f64, (u8, u8, u8)); 3] = [
        (0.0, COLOR_BLUE_LIGHT),
        (0.55, COLOR_BLUE),
        (1.0, COLOR_PINK_DARK),
    ];
    let t = f64::from(magnitude) / 255.0;
    let (mut lo, mut hi) = (STOPS[0], STOPS[STOPS.len() - 1]);
    for window in STOPS.windows(2) {
        if t >= window[0].0 && t <= window[1].0 {
            lo = window[0];
            hi = window[1];
            break;
        }
    }
    let span = (hi.0 - lo.0).max(f64::EPSILON);
    let local_t = ((t - lo.0) / span).clamp(0.0, 1.0);
    let lerp = |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * local_t).round() as u8;
    rgb_string((
        lerp(lo.1 .0, hi.1 .0),
        lerp(lo.1 .1, hi.1 .1),
        lerp(lo.1 .2, hi.1 .2),
    ))
}

fn rgb_string((r, g, b): (u8, u8, u8)) -> String {
    format!("rgb({r}, {g}, {b})")
}

fn get_2d_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .flatten()?
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

fn clear_canvas(canvas: Option<&HtmlCanvasElement>) {
    let Some(canvas) = canvas else { return };
    if let Some(ctx) = get_2d_context(canvas) {
        ctx.clear_rect(0.0, 0.0, f64::from(canvas.width()), f64::from(canvas.height()));
    }
}
