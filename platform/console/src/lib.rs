//! Native-console host shared by libnx and VitaSDK. The engine's VM, native
//! display tree, resource decoder, text rasterizer and Kira mixer stay shared.
use ethornell_app::host_runtime::{HostConfig, HostRuntime};
use ethornell_render::{NativeDraw, NativeGpu, Renderer};
use std::{
    cell::RefCell,
    ffi::{CStr, c_char, c_void},
    path::PathBuf,
};
#[cfg(any(target_os = "horizon", target_os = "vita"))]
mod files;
thread_local! {static ERROR:RefCell<String>=const{RefCell::new(String::new())};}
fn error(message: impl ToString) -> i32 {
    ERROR.with(|e| *e.borrow_mut() = message.to_string());
    -1
}
unsafe fn string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("null string".into());
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(str::to_owned)
        .map_err(|e| e.to_string())
}
unsafe fn copy_text(ptr: *mut c_char, size: usize, text: &str) {
    if ptr.is_null() || size == 0 {
        return;
    }
    let n = text.len().min(size - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(text.as_ptr(), ptr.cast(), n);
        ptr.add(n).write(0);
    }
}
unsafe extern "C" {
    fn ethornell_gpu_create(w: u32, h: u32, rgba: *const u8) -> u64;
    fn ethornell_gpu_update(texture: u64, w: u32, h: u32, rgba: *const u8) -> i32;
    fn ethornell_gpu_destroy(texture: u64);
    fn ethornell_gpu_begin(clear: *const f32) -> i32;
    fn ethornell_gpu_draw(draw: *const NativeDraw) -> i32;
    fn ethornell_gpu_end() -> i32;
}
struct Session {
    host: HostRuntime,
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_create(
    game: *const c_char,
    saves: *const c_char,
    font: *const c_char,
    w: u32,
    h: u32,
) -> *mut c_void {
    let create = || -> Result<Session, String> {
        let game = PathBuf::from(unsafe { string(game) }?);
        let user = PathBuf::from(unsafe { string(saves) }?);
        let font = PathBuf::from(unsafe { string(font) }?);
        std::fs::create_dir_all(&user).map_err(|e| e.to_string())?;
        // Unattended test runs: an input script beside the game directory.
        let script = game
            .parent()
            .and_then(|dir| std::fs::read_to_string(dir.join("input.txt")).ok());
        #[cfg(any(target_os = "horizon", target_os = "vita"))]
        files::install(&game, &user).map_err(|e| e.to_string())?;
        let mut host = HostRuntime::create(HostConfig {
            resource_root: game,
            user_root: user,
            font_path: Some(font),
            font_data: None,
            font_face_index: 0,
            gbk: false,
            backend: 0,
            strict: true,
            trace: false,
            silent: false,
        })
        .map_err(|e| e.to_string())?;
        let gpu = NativeGpu {
            create: ethornell_gpu_create,
            update: ethornell_gpu_update,
            destroy: ethornell_gpu_destroy,
            begin: ethornell_gpu_begin,
            draw: ethornell_gpu_draw,
            end: ethornell_gpu_end,
        };
        let renderer = Renderer::new_native(w, h, host.font(), gpu).map_err(|e| e.to_string())?;
        host.attach_renderer(renderer, w, h, 1)
            .map_err(|e| e.to_string())?;
        if let Some(script) = script {
            host.set_input_script(script.trim()).map_err(|e| e.to_string())?;
        }
        Ok(Session { host })
    };
    match create() {
        Ok(s) => Box::into_raw(Box::new(s)).cast(),
        Err(e) => {
            error(e);
            std::ptr::null_mut()
        }
    }
}
fn with(ptr: *mut c_void, f: impl FnOnce(&mut Session) -> Result<(), String>) -> i32 {
    if ptr.is_null() {
        return error("null engine");
    }
    // Shell retains this Box on its VM thread and never uses it after destroy.
    match f(unsafe { &mut *ptr.cast::<Session>() }) {
        Ok(()) => 0,
        Err(e) => error(e),
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn ethornell_console_step(ptr: *mut c_void, now: u64) -> i32 {
    with(ptr, |s| {
        s.host.step(now).map_err(|e| e.to_string())?;
        #[cfg(any(target_os = "horizon", target_os = "vita"))]
        if let Some(error) = files::take_persist_error() {
            return Err(error);
        }
        Ok(())
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn ethornell_console_input(
    ptr: *mut c_void,
    kind: u32,
    x: f32,
    y: f32,
    code: i32,
    value: f32,
) -> i32 {
    with(ptr, |s| {
        s.host
            .input(kind, x, y, code, value)
            .map_err(|e| e.to_string())
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn ethornell_console_pause(ptr: *mut c_void, paused: bool) -> i32 {
    with(ptr, |s| {
        s.host.pause(u32::from(paused)).map_err(|e| e.to_string())
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn ethornell_console_close(ptr: *mut c_void) -> i32 {
    with(ptr, |s| {
        s.host
            .scope(|h| h.request_close())
            .map_err(|e| e.to_string())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_destroy(ptr: *mut c_void) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr.cast::<Session>()) });
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_audio(samples: *mut i16, frames: usize) {
    if samples.is_null() || frames == 0 || frames > 65536 {
        return;
    }
    let out = unsafe { std::slice::from_raw_parts_mut(samples, frames * 2) };
    #[cfg(any(target_os = "horizon", target_os = "vita"))]
    ethornell_audio::pump::render_i16(out);
    #[cfg(not(any(target_os = "horizon", target_os = "vita")))]
    out.fill(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_event(
    ptr: *mut c_void,
    request: *mut u64,
    code: *mut i32,
    text: *mut c_char,
    capacity: usize,
) -> i32 {
    if ptr.is_null() || request.is_null() || code.is_null() {
        return error("null event arguments");
    }
    let session = unsafe { &mut *ptr.cast::<Session>() };
    let Some(event) = session.host.poll_event() else {
        return 0;
    };
    unsafe {
        request.write(event.request_id);
        code.write(event.code);
        copy_text(text, capacity, &event.text);
    }
    if event.kind == 2 {
        session.host.clear_input();
        if let Err(e) = session.host.show_console_notice(Some(format!(
            "{}\n\nA / Cross: Yes    B / Circle: No",
            event.text
        ))) {
            return error(e);
        }
    }
    event.kind as i32
}
#[unsafe(no_mangle)]
pub extern "C" fn ethornell_console_respond(ptr: *mut c_void, request: u64, result: i32) -> i32 {
    with(ptr, |s| {
        s.host.clear_input();
        s.host
            .show_console_notice(None)
            .map_err(|e| e.to_string())?;
        s.host
            .scope(|h| h.complete_request(request, result))
            .map_err(|e| e.to_string())
    })
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_dialog(
    ptr: *mut c_void,
    request: u64,
    json: *const c_char,
) -> i32 {
    match unsafe { string(json) } {
        Ok(json) => with(ptr, |s| {
            s.host
                .scope(|h| h.complete_dialog(request, &json))
                .map_err(|e| e.to_string())
        }),
        Err(e) => error(e),
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_diagnostics(
    ptr: *mut c_void,
    text: *mut c_char,
    size: usize,
) {
    if ptr.is_null() {
        return;
    }
    let s = unsafe { &*ptr.cast::<Session>() };
    unsafe {
        copy_text(text, size, &s.host.diagnostics());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ethornell_console_last_error(text: *mut c_char, size: usize) {
    ERROR.with(|e| unsafe { copy_text(text, size, &e.borrow()) });
}
