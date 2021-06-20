// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use std::ffi::c_void;
extern crate va_list;
use super::*;

pub const SDL_WINDOWPOS_UNDEFINED_MASK: u32 = 0x1FFF0000;
/* SDL_WINDOWPOS_UNDEFINED_DISPLAY(X)(SDL_WINDOWPOS_UNDEFINED_MASK|(X)) */
//SDL_WINDOWPOS_UNDEFINED SDL_WINDOWPOS_UNDEFINED_DISPLAY(0)
/* SDL_WINDOWPOS_ISUNDEFINED(X)(((X)&0xFFFF0000)==SDL_WINDOWPOS_UNDEFINED_MASK) */
pub const SDL_WINDOWPOS_CENTERED_MASK: u32 = 0x2FFF0000;
pub const fn SDL_WINDOWPOS_CENTERED_DISPLAY(X: u32) -> u32 {
    SDL_WINDOWPOS_CENTERED_MASK | (X)
}
pub const SDL_WINDOWPOS_CENTERED: u32 = SDL_WINDOWPOS_CENTERED_DISPLAY(0);
/* SDL_WINDOWPOS_ISCENTERED(X)(((X)&0xFFFF0000)==SDL_WINDOWPOS_CENTERED_MASK) */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_DisplayMode {
    pub format: u32,
    pub w: i32,
    pub h: i32,
    pub refresh_rate: i32,
    pub driverdata: *mut c_void,
}
pub type SDL_Window = c_void;
pub const SDL_WINDOW_FULLSCREEN: i32 = 0x1;
pub const SDL_WINDOW_OPENGL: i32 = 0x2;
pub const SDL_WINDOW_SHOWN: i32 = 0x4;
pub const SDL_WINDOW_HIDDEN: i32 = 0x8;
pub const SDL_WINDOW_BORDERLESS: i32 = 0x10;
pub const SDL_WINDOW_RESIZABLE: i32 = 0x20;
pub const SDL_WINDOW_MINIMIZED: i32 = 0x40;
pub const SDL_WINDOW_MAXIMIZED: i32 = 0x80;
pub const SDL_WINDOW_MOUSE_GRABBED: i32 = 0x100;
pub const SDL_WINDOW_INPUT_FOCUS: i32 = 0x200;
pub const SDL_WINDOW_MOUSE_FOCUS: i32 = 0x400;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: i32 = 0x1001;
pub const SDL_WINDOW_FOREIGN: i32 = 0x800;
pub const SDL_WINDOW_ALLOW_HIGHDPI: i32 = 0x2000;
pub const SDL_WINDOW_MOUSE_CAPTURE: i32 = 0x4000;
pub const SDL_WINDOW_ALWAYS_ON_TOP: i32 = 0x8000;
pub const SDL_WINDOW_SKIP_TASKBAR: i32 = 0x10000;
pub const SDL_WINDOW_UTILITY: i32 = 0x20000;
pub const SDL_WINDOW_TOOLTIP: i32 = 0x40000;
pub const SDL_WINDOW_POPUP_MENU: i32 = 0x80000;
pub const SDL_WINDOW_KEYBOARD_GRABBED: i32 = 0x100000;
pub const SDL_WINDOW_VULKAN: i32 = 0x10000000;
pub const SDL_WINDOW_METAL: i32 = 0x20000000;
pub const SDL_WINDOW_INPUT_GRABBED: i32 = 0x100;
pub const SDL_WINDOWEVENT_NONE: i32 = 0;
pub const SDL_WINDOWEVENT_SHOWN: i32 = 0x1;
pub const SDL_WINDOWEVENT_HIDDEN: i32 = 0x2;
pub const SDL_WINDOWEVENT_EXPOSED: i32 = 0x3;
pub const SDL_WINDOWEVENT_MOVED: i32 = 0x4;
pub const SDL_WINDOWEVENT_RESIZED: i32 = 0x5;
pub const SDL_WINDOWEVENT_SIZE_CHANGED: i32 = 0x6;
pub const SDL_WINDOWEVENT_MINIMIZED: i32 = 0x7;
pub const SDL_WINDOWEVENT_MAXIMIZED: i32 = 0x8;
pub const SDL_WINDOWEVENT_RESTORED: i32 = 0x9;
pub const SDL_WINDOWEVENT_ENTER: i32 = 0xa;
pub const SDL_WINDOWEVENT_LEAVE: i32 = 0xb;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: i32 = 0xc;
pub const SDL_WINDOWEVENT_FOCUS_LOST: i32 = 0xd;
pub const SDL_WINDOWEVENT_CLOSE: i32 = 0xe;
pub const SDL_WINDOWEVENT_TAKE_FOCUS: i32 = 0xf;
pub const SDL_WINDOWEVENT_HIT_TEST: i32 = 0x10;
pub const SDL_DISPLAYEVENT_NONE: i32 = 0;
pub const SDL_DISPLAYEVENT_ORIENTATION: i32 = 0x1;
pub const SDL_DISPLAYEVENT_CONNECTED: i32 = 0x2;
pub const SDL_DISPLAYEVENT_DISCONNECTED: i32 = 0x3;
pub const SDL_ORIENTATION_UNKNOWN: i32 = 0;
pub const SDL_ORIENTATION_LANDSCAPE: i32 = 0x1;
pub const SDL_ORIENTATION_LANDSCAPE_FLIPPED: i32 = 0x2;
pub const SDL_ORIENTATION_PORTRAIT: i32 = 0x3;
pub const SDL_ORIENTATION_PORTRAIT_FLIPPED: i32 = 0x4;
pub const SDL_GL_RED_SIZE: i32 = 0;
pub const SDL_GL_GREEN_SIZE: i32 = 0x1;
pub const SDL_GL_BLUE_SIZE: i32 = 0x2;
pub const SDL_GL_ALPHA_SIZE: i32 = 0x3;
pub const SDL_GL_BUFFER_SIZE: i32 = 0x4;
pub const SDL_GL_DOUBLEBUFFER: i32 = 0x5;
pub const SDL_GL_DEPTH_SIZE: i32 = 0x6;
pub const SDL_GL_STENCIL_SIZE: i32 = 0x7;
pub const SDL_GL_ACCUM_RED_SIZE: i32 = 0x8;
pub const SDL_GL_ACCUM_GREEN_SIZE: i32 = 0x9;
pub const SDL_GL_ACCUM_BLUE_SIZE: i32 = 0xa;
pub const SDL_GL_ACCUM_ALPHA_SIZE: i32 = 0xb;
pub const SDL_GL_STEREO: i32 = 0xc;
pub const SDL_GL_MULTISAMPLEBUFFERS: i32 = 0xd;
pub const SDL_GL_MULTISAMPLESAMPLES: i32 = 0xe;
pub const SDL_GL_ACCELERATED_VISUAL: i32 = 0xf;
pub const SDL_GL_RETAINED_BACKING: i32 = 0x10;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: i32 = 0x11;
pub const SDL_GL_CONTEXT_MINOR_VERSION: i32 = 0x12;
pub const SDL_GL_CONTEXT_EGL: i32 = 0x13;
pub const SDL_GL_CONTEXT_FLAGS: i32 = 0x14;
pub const SDL_GL_CONTEXT_PROFILE_MASK: i32 = 0x15;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: i32 = 0x16;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: i32 = 0x17;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: i32 = 0x18;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: i32 = 0x19;
pub const SDL_GL_CONTEXT_NO_ERROR: i32 = 0x1a;
pub const SDL_GL_CONTEXT_PROFILE_CORE: i32 = 0x1;
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: i32 = 0x2;
pub const SDL_GL_CONTEXT_PROFILE_ES: i32 = 0x4;
pub const SDL_GL_CONTEXT_DEBUG_FLAG: i32 = 0x1;
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: i32 = 0x2;
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: i32 = 0x4;
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: i32 = 0x8;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE: i32 = 0;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: i32 = 0x1;
pub const SDL_GL_CONTEXT_RESET_NO_NOTIFICATION: i32 = 0;
pub const SDL_GL_CONTEXT_RESET_LOSE_CONTEXT: i32 = 0x1;
pub const SDL_HITTEST_NORMAL: i32 = 0;
pub const SDL_HITTEST_DRAGGABLE: i32 = 0x1;
pub const SDL_HITTEST_RESIZE_TOPLEFT: i32 = 0x2;
pub const SDL_HITTEST_RESIZE_TOP: i32 = 0x3;
pub const SDL_HITTEST_RESIZE_TOPRIGHT: i32 = 0x4;
pub const SDL_HITTEST_RESIZE_RIGHT: i32 = 0x5;
pub const SDL_HITTEST_RESIZE_BOTTOMRIGHT: i32 = 0x6;
pub const SDL_HITTEST_RESIZE_BOTTOM: i32 = 0x7;
pub const SDL_HITTEST_RESIZE_BOTTOMLEFT: i32 = 0x8;
pub const SDL_HITTEST_RESIZE_LEFT: i32 = 0x9;

#[link(name = "SDL2", kind = "static")]
extern "C" {

    pub fn SDL_GetNumVideoDrivers() -> i32;

    /// * index: 
    pub fn SDL_GetVideoDriver(
        index: i32,
    ) -> *mut i8;

    /// * driver_name: 
    pub fn SDL_VideoInit(
        driver_name: *const i8,
    ) -> i32;

    pub fn SDL_VideoQuit() -> c_void;

    pub fn SDL_GetCurrentVideoDriver() -> *mut i8;

    pub fn SDL_GetNumVideoDisplays() -> i32;

    /// * displayIndex: 
    pub fn SDL_GetDisplayName(
        displayIndex: i32,
    ) -> *mut i8;

    /// * displayIndex: 
    /// * rect: 
    pub fn SDL_GetDisplayBounds(
        displayIndex: i32,
        rect: *mut SDL_Rect,
    ) -> i32;

    /// * displayIndex: 
    /// * rect: 
    pub fn SDL_GetDisplayUsableBounds(
        displayIndex: i32,
        rect: *mut SDL_Rect,
    ) -> i32;

    /// * displayIndex: 
    /// * ddpi: 
    /// * hdpi: 
    /// * vdpi: 
    pub fn SDL_GetDisplayDPI(
        displayIndex: i32,
        ddpi: *mut f32,
        hdpi: *mut f32,
        vdpi: *mut f32,
    ) -> i32;

    /// * displayIndex: 
    pub fn SDL_GetDisplayOrientation(
        displayIndex: i32,
    ) -> i32;

    /// * displayIndex: 
    pub fn SDL_GetNumDisplayModes(
        displayIndex: i32,
    ) -> i32;

    /// * displayIndex: 
    /// * modeIndex: 
    /// * mode: 
    pub fn SDL_GetDisplayMode(
        displayIndex: i32,
        modeIndex: i32,
        mode: *mut SDL_DisplayMode,
    ) -> i32;

    /// * displayIndex: 
    /// * mode: 
    pub fn SDL_GetDesktopDisplayMode(
        displayIndex: i32,
        mode: *mut SDL_DisplayMode,
    ) -> i32;

    /// * displayIndex: 
    /// * mode: 
    pub fn SDL_GetCurrentDisplayMode(
        displayIndex: i32,
        mode: *mut SDL_DisplayMode,
    ) -> i32;

    /// * displayIndex: 
    /// * mode: 
    /// * closest: 
    pub fn SDL_GetClosestDisplayMode(
        displayIndex: i32,
        mode: *const SDL_DisplayMode,
        closest: *mut SDL_DisplayMode,
    ) -> *mut SDL_DisplayMode;

    /// * window: 
    pub fn SDL_GetWindowDisplayIndex(
        window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    /// * mode: 
    pub fn SDL_SetWindowDisplayMode(
        window: *mut SDL_Window,
        mode: *const SDL_DisplayMode,
    ) -> i32;

    /// * window: 
    /// * mode: 
    pub fn SDL_GetWindowDisplayMode(
        window: *mut SDL_Window,
        mode: *mut SDL_DisplayMode,
    ) -> i32;

    /// * window: 
    pub fn SDL_GetWindowPixelFormat(
        window: *mut SDL_Window,
    ) -> u32;

    /// * title: 
    /// * x: 
    /// * y: 
    /// * w: 
    /// * h: 
    /// * flags: 
    pub fn SDL_CreateWindow(
        title: *const i8,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> *mut SDL_Window;

    /// * data: 
    pub fn SDL_CreateWindowFrom(
        data: *const c_void,
    ) -> *mut SDL_Window;

    /// * window: 
    pub fn SDL_GetWindowID(
        window: *mut SDL_Window,
    ) -> u32;

    /// * id: 
    pub fn SDL_GetWindowFromID(
        id: u32,
    ) -> *mut SDL_Window;

    /// * window: 
    pub fn SDL_GetWindowFlags(
        window: *mut SDL_Window,
    ) -> u32;

    /// * window: 
    /// * title: 
    pub fn SDL_SetWindowTitle(
        window: *mut SDL_Window,
        title: *const i8,
    ) -> c_void;

    /// * window: 
    pub fn SDL_GetWindowTitle(
        window: *mut SDL_Window,
    ) -> *mut i8;

    /// * window: 
    /// * icon: 
    pub fn SDL_SetWindowIcon(
        window: *mut SDL_Window,
        icon: *mut SDL_Surface,
    ) -> c_void;

    /// * window: 
    /// * name: 
    /// * userdata: 
    pub fn SDL_SetWindowData(
        window: *mut SDL_Window,
        name: *const i8,
        userdata: *mut c_void,
    ) -> *mut c_void;

    /// * window: 
    /// * name: 
    pub fn SDL_GetWindowData(
        window: *mut SDL_Window,
        name: *const i8,
    ) -> *mut c_void;

    /// * window: 
    /// * x: 
    /// * y: 
    pub fn SDL_SetWindowPosition(
        window: *mut SDL_Window,
        x: i32,
        y: i32,
    ) -> c_void;

    /// * window: 
    /// * x: 
    /// * y: 
    pub fn SDL_GetWindowPosition(
        window: *mut SDL_Window,
        x: *mut i32,
        y: *mut i32,
    ) -> c_void;

    /// * window: 
    /// * w: 
    /// * h: 
    pub fn SDL_SetWindowSize(
        window: *mut SDL_Window,
        w: i32,
        h: i32,
    ) -> c_void;

    /// * window: 
    /// * w: 
    /// * h: 
    pub fn SDL_GetWindowSize(
        window: *mut SDL_Window,
        w: *mut i32,
        h: *mut i32,
    ) -> c_void;

    /// * window: 
    /// * top: 
    /// * left: 
    /// * bottom: 
    /// * right: 
    pub fn SDL_GetWindowBordersSize(
        window: *mut SDL_Window,
        top: *mut i32,
        left: *mut i32,
        bottom: *mut i32,
        right: *mut i32,
    ) -> i32;

    /// * window: 
    /// * min_w: 
    /// * min_h: 
    pub fn SDL_SetWindowMinimumSize(
        window: *mut SDL_Window,
        min_w: i32,
        min_h: i32,
    ) -> c_void;

    /// * window: 
    /// * w: 
    /// * h: 
    pub fn SDL_GetWindowMinimumSize(
        window: *mut SDL_Window,
        w: *mut i32,
        h: *mut i32,
    ) -> c_void;

    /// * window: 
    /// * max_w: 
    /// * max_h: 
    pub fn SDL_SetWindowMaximumSize(
        window: *mut SDL_Window,
        max_w: i32,
        max_h: i32,
    ) -> c_void;

    /// * window: 
    /// * w: 
    /// * h: 
    pub fn SDL_GetWindowMaximumSize(
        window: *mut SDL_Window,
        w: *mut i32,
        h: *mut i32,
    ) -> c_void;

    /// * window: 
    /// * bordered: 
    pub fn SDL_SetWindowBordered(
        window: *mut SDL_Window,
        bordered: i32,
    ) -> c_void;

    /// * window: 
    /// * resizable: 
    pub fn SDL_SetWindowResizable(
        window: *mut SDL_Window,
        resizable: i32,
    ) -> c_void;

    /// * window: 
    /// * on_top: 
    pub fn SDL_SetWindowAlwaysOnTop(
        window: *mut SDL_Window,
        on_top: i32,
    ) -> c_void;

    /// * window: 
    pub fn SDL_ShowWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    pub fn SDL_HideWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    pub fn SDL_RaiseWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    pub fn SDL_MaximizeWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    pub fn SDL_MinimizeWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    pub fn SDL_RestoreWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * window: 
    /// * flags: 
    pub fn SDL_SetWindowFullscreen(
        window: *mut SDL_Window,
        flags: u32,
    ) -> i32;

    /// * window: 
    pub fn SDL_GetWindowSurface(
        window: *mut SDL_Window,
    ) -> *mut SDL_Surface;

    /// * window: 
    pub fn SDL_UpdateWindowSurface(
        window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    /// * rects: 
    /// * numrects: 
    pub fn SDL_UpdateWindowSurfaceRects(
        window: *mut SDL_Window,
        rects: *const SDL_Rect,
        numrects: i32,
    ) -> i32;

    /// * window: 
    /// * grabbed: 
    pub fn SDL_SetWindowGrab(
        window: *mut SDL_Window,
        grabbed: i32,
    ) -> c_void;

    /// * window: 
    /// * grabbed: 
    pub fn SDL_SetWindowKeyboardGrab(
        window: *mut SDL_Window,
        grabbed: i32,
    ) -> c_void;

    /// * window: 
    /// * grabbed: 
    pub fn SDL_SetWindowMouseGrab(
        window: *mut SDL_Window,
        grabbed: i32,
    ) -> c_void;

    /// * window: 
    pub fn SDL_GetWindowGrab(
        window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    pub fn SDL_GetWindowKeyboardGrab(
        window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    pub fn SDL_GetWindowMouseGrab(
        window: *mut SDL_Window,
    ) -> i32;

    pub fn SDL_GetGrabbedWindow() -> *mut SDL_Window;

    /// * window: 
    /// * brightness: 
    pub fn SDL_SetWindowBrightness(
        window: *mut SDL_Window,
        brightness: f32,
    ) -> i32;

    /// * window: 
    pub fn SDL_GetWindowBrightness(
        window: *mut SDL_Window,
    ) -> f32;

    /// * window: 
    /// * opacity: 
    pub fn SDL_SetWindowOpacity(
        window: *mut SDL_Window,
        opacity: f32,
    ) -> i32;

    /// * window: 
    /// * out_opacity: 
    pub fn SDL_GetWindowOpacity(
        window: *mut SDL_Window,
        out_opacity: *mut f32,
    ) -> i32;

    /// * modal_window: 
    /// * parent_window: 
    pub fn SDL_SetWindowModalFor(
        modal_window: *mut SDL_Window,
        parent_window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    pub fn SDL_SetWindowInputFocus(
        window: *mut SDL_Window,
    ) -> i32;

    /// * window: 
    /// * red: 
    /// * green: 
    /// * blue: 
    pub fn SDL_SetWindowGammaRamp(
        window: *mut SDL_Window,
        red: *const u16,
        green: *const u16,
        blue: *const u16,
    ) -> i32;

    /// * window: 
    /// * red: 
    /// * green: 
    /// * blue: 
    pub fn SDL_GetWindowGammaRamp(
        window: *mut SDL_Window,
        red: *mut u16,
        green: *mut u16,
        blue: *mut u16,
    ) -> i32;

    /// * window: 
    /// * callback: 
    /// * callback_data: 
    pub fn SDL_SetWindowHitTest(
        window: *mut SDL_Window,
        callback: extern fn(*mut SDL_Window,*mut SDL_Point,*mut c_void,) -> i32,
        callback_data: *mut c_void,
    ) -> i32;

    /// * window: 
    /// * flash_count: 
    pub fn SDL_FlashWindow(
        window: *mut SDL_Window,
        flash_count: u32,
    ) -> i32;

    /// * window: 
    pub fn SDL_DestroyWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    pub fn SDL_IsScreenSaverEnabled() -> i32;

    pub fn SDL_EnableScreenSaver() -> c_void;

    pub fn SDL_DisableScreenSaver() -> c_void;

    /// * path: 
    pub fn SDL_GL_LoadLibrary(
        path: *const i8,
    ) -> i32;

    /// * proc: 
    pub fn SDL_GL_GetProcAddress(
        proc: *const i8,
    ) -> *mut c_void;

    pub fn SDL_GL_UnloadLibrary() -> c_void;

    /// * extension: 
    pub fn SDL_GL_ExtensionSupported(
        extension: *const i8,
    ) -> i32;

    pub fn SDL_GL_ResetAttributes() -> c_void;

    /// * attr: 
    /// * value: 
    pub fn SDL_GL_SetAttribute(
        attr: i32,
        value: i32,
    ) -> i32;

    /// * attr: 
    /// * value: 
    pub fn SDL_GL_GetAttribute(
        attr: i32,
        value: *mut i32,
    ) -> i32;

    /// * window: 
    pub fn SDL_GL_CreateContext(
        window: *mut SDL_Window,
    ) -> *mut c_void;

    /// * window: 
    /// * context: 
    pub fn SDL_GL_MakeCurrent(
        window: *mut SDL_Window,
        context: *mut c_void,
    ) -> i32;

    pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;

    pub fn SDL_GL_GetCurrentContext() -> *mut c_void;

    /// * window: 
    /// * w: 
    /// * h: 
    pub fn SDL_GL_GetDrawableSize(
        window: *mut SDL_Window,
        w: *mut i32,
        h: *mut i32,
    ) -> c_void;

    /// * interval: 
    pub fn SDL_GL_SetSwapInterval(
        interval: i32,
    ) -> i32;

    pub fn SDL_GL_GetSwapInterval() -> i32;

    /// * window: 
    pub fn SDL_GL_SwapWindow(
        window: *mut SDL_Window,
    ) -> c_void;

    /// * context: 
    pub fn SDL_GL_DeleteContext(
        context: *mut c_void,
    ) -> c_void;
}
