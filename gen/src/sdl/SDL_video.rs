// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;
use super::*;
pub const SDL_WINDOWPOS_UNDEFINED_MASK: u32 = 0x1FFF0000;
// SDL_WINDOWPOS_UNDEFINED_DISPLAY(X)(SDL_WINDOWPOS_UNDEFINED_MASK|(X))
//SDL_WINDOWPOS_UNDEFINED SDL_WINDOWPOS_UNDEFINED_DISPLAY(0)
// SDL_WINDOWPOS_ISUNDEFINED(X)(((X)&0xFFFF0000)==SDL_WINDOWPOS_UNDEFINED_MASK)
pub const SDL_WINDOWPOS_CENTERED_MASK: u32 = 0x2FFF0000;
// SDL_WINDOWPOS_CENTERED_DISPLAY(X)(SDL_WINDOWPOS_CENTERED_MASK|(X))
//SDL_WINDOWPOS_CENTERED SDL_WINDOWPOS_CENTERED_DISPLAY(0)
// SDL_WINDOWPOS_ISCENTERED(X)(((X)&0xFFFF0000)==SDL_WINDOWPOS_CENTERED_MASK)

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

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_WindowFlags {
    SDL_WINDOW_FULLSCREEN = 0x1,
    SDL_WINDOW_OPENGL = 0x2,
    SDL_WINDOW_SHOWN = 0x4,
    SDL_WINDOW_HIDDEN = 0x8,
    SDL_WINDOW_BORDERLESS = 0x10,
    SDL_WINDOW_RESIZABLE = 0x20,
    SDL_WINDOW_MINIMIZED = 0x40,
    SDL_WINDOW_MAXIMIZED = 0x80,
    SDL_WINDOW_MOUSE_GRABBED = 0x100,
    SDL_WINDOW_INPUT_FOCUS = 0x200,
    SDL_WINDOW_MOUSE_FOCUS = 0x400,
    SDL_WINDOW_FULLSCREEN_DESKTOP = 0x1001,
    SDL_WINDOW_FOREIGN = 0x800,
    SDL_WINDOW_ALLOW_HIGHDPI = 0x2000,
    SDL_WINDOW_MOUSE_CAPTURE = 0x4000,
    SDL_WINDOW_ALWAYS_ON_TOP = 0x8000,
    SDL_WINDOW_SKIP_TASKBAR = 0x10000,
    SDL_WINDOW_UTILITY = 0x20000,
    SDL_WINDOW_TOOLTIP = 0x40000,
    SDL_WINDOW_POPUP_MENU = 0x80000,
    SDL_WINDOW_KEYBOARD_GRABBED = 0x100000,
    SDL_WINDOW_VULKAN = 0x10000000,
    SDL_WINDOW_METAL = 0x20000000,
    // SDL_WINDOW_INPUT_GRABBED = 0x100,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_WindowEventID {
    SDL_WINDOWEVENT_NONE = 0,
    SDL_WINDOWEVENT_SHOWN = 0x1,
    SDL_WINDOWEVENT_HIDDEN = 0x2,
    SDL_WINDOWEVENT_EXPOSED = 0x3,
    SDL_WINDOWEVENT_MOVED = 0x4,
    SDL_WINDOWEVENT_RESIZED = 0x5,
    SDL_WINDOWEVENT_SIZE_CHANGED = 0x6,
    SDL_WINDOWEVENT_MINIMIZED = 0x7,
    SDL_WINDOWEVENT_MAXIMIZED = 0x8,
    SDL_WINDOWEVENT_RESTORED = 0x9,
    SDL_WINDOWEVENT_ENTER = 0xa,
    SDL_WINDOWEVENT_LEAVE = 0xb,
    SDL_WINDOWEVENT_FOCUS_GAINED = 0xc,
    SDL_WINDOWEVENT_FOCUS_LOST = 0xd,
    SDL_WINDOWEVENT_CLOSE = 0xe,
    SDL_WINDOWEVENT_TAKE_FOCUS = 0xf,
    SDL_WINDOWEVENT_HIT_TEST = 0x10,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_DisplayEventID {
    SDL_DISPLAYEVENT_NONE = 0,
    SDL_DISPLAYEVENT_ORIENTATION = 0x1,
    SDL_DISPLAYEVENT_CONNECTED = 0x2,
    SDL_DISPLAYEVENT_DISCONNECTED = 0x3,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_DisplayOrientation {
    SDL_ORIENTATION_UNKNOWN = 0,
    SDL_ORIENTATION_LANDSCAPE = 0x1,
    SDL_ORIENTATION_LANDSCAPE_FLIPPED = 0x2,
    SDL_ORIENTATION_PORTRAIT = 0x3,
    SDL_ORIENTATION_PORTRAIT_FLIPPED = 0x4,
}
pub type SDL_GLContext = *mut c_void;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_GLattr {
    SDL_GL_RED_SIZE = 0,
    SDL_GL_GREEN_SIZE = 0x1,
    SDL_GL_BLUE_SIZE = 0x2,
    SDL_GL_ALPHA_SIZE = 0x3,
    SDL_GL_BUFFER_SIZE = 0x4,
    SDL_GL_DOUBLEBUFFER = 0x5,
    SDL_GL_DEPTH_SIZE = 0x6,
    SDL_GL_STENCIL_SIZE = 0x7,
    SDL_GL_ACCUM_RED_SIZE = 0x8,
    SDL_GL_ACCUM_GREEN_SIZE = 0x9,
    SDL_GL_ACCUM_BLUE_SIZE = 0xa,
    SDL_GL_ACCUM_ALPHA_SIZE = 0xb,
    SDL_GL_STEREO = 0xc,
    SDL_GL_MULTISAMPLEBUFFERS = 0xd,
    SDL_GL_MULTISAMPLESAMPLES = 0xe,
    SDL_GL_ACCELERATED_VISUAL = 0xf,
    SDL_GL_RETAINED_BACKING = 0x10,
    SDL_GL_CONTEXT_MAJOR_VERSION = 0x11,
    SDL_GL_CONTEXT_MINOR_VERSION = 0x12,
    SDL_GL_CONTEXT_EGL = 0x13,
    SDL_GL_CONTEXT_FLAGS = 0x14,
    SDL_GL_CONTEXT_PROFILE_MASK = 0x15,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT = 0x16,
    SDL_GL_FRAMEBUFFER_SRGB_CAPABLE = 0x17,
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR = 0x18,
    SDL_GL_CONTEXT_RESET_NOTIFICATION = 0x19,
    SDL_GL_CONTEXT_NO_ERROR = 0x1a,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_GLprofile {
    SDL_GL_CONTEXT_PROFILE_CORE = 0x1,
    SDL_GL_CONTEXT_PROFILE_COMPATIBILITY = 0x2,
    SDL_GL_CONTEXT_PROFILE_ES = 0x4,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_GLcontextFlag {
    SDL_GL_CONTEXT_DEBUG_FLAG = 0x1,
    SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG = 0x2,
    SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG = 0x4,
    SDL_GL_CONTEXT_RESET_ISOLATION_FLAG = 0x8,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_GLcontextReleaseFlag {
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE = 0,
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH = 0x1,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_GLContextResetNotification {
    SDL_GL_CONTEXT_RESET_NO_NOTIFICATION = 0,
    SDL_GL_CONTEXT_RESET_LOSE_CONTEXT = 0x1,
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum SDL_HitTestResult {
    SDL_HITTEST_NORMAL = 0,
    SDL_HITTEST_DRAGGABLE = 0x1,
    SDL_HITTEST_RESIZE_TOPLEFT = 0x2,
    SDL_HITTEST_RESIZE_TOP = 0x3,
    SDL_HITTEST_RESIZE_TOPRIGHT = 0x4,
    SDL_HITTEST_RESIZE_RIGHT = 0x5,
    SDL_HITTEST_RESIZE_BOTTOMRIGHT = 0x6,
    SDL_HITTEST_RESIZE_BOTTOM = 0x7,
    SDL_HITTEST_RESIZE_BOTTOMLEFT = 0x8,
    SDL_HITTEST_RESIZE_LEFT = 0x9,
}

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
    ) -> SDL_DisplayOrientation;

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
        bordered: SDL_bool,
    ) -> c_void;

    /// * window: 
    /// * resizable: 
    pub fn SDL_SetWindowResizable(
        window: *mut SDL_Window,
        resizable: SDL_bool,
    ) -> c_void;

    /// * window: 
    /// * on_top: 
    pub fn SDL_SetWindowAlwaysOnTop(
        window: *mut SDL_Window,
        on_top: SDL_bool,
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
        grabbed: SDL_bool,
    ) -> c_void;

    /// * window: 
    /// * grabbed: 
    pub fn SDL_SetWindowKeyboardGrab(
        window: *mut SDL_Window,
        grabbed: SDL_bool,
    ) -> c_void;

    /// * window: 
    /// * grabbed: 
    pub fn SDL_SetWindowMouseGrab(
        window: *mut SDL_Window,
        grabbed: SDL_bool,
    ) -> c_void;

    /// * window: 
    pub fn SDL_GetWindowGrab(
        window: *mut SDL_Window,
    ) -> SDL_bool;

    /// * window: 
    pub fn SDL_GetWindowKeyboardGrab(
        window: *mut SDL_Window,
    ) -> SDL_bool;

    /// * window: 
    pub fn SDL_GetWindowMouseGrab(
        window: *mut SDL_Window,
    ) -> SDL_bool;

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
        callback: extern fn(*mut SDL_Window,*mut SDL_Point,*mut c_void,) -> SDL_HitTestResult,
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

    pub fn SDL_IsScreenSaverEnabled() -> SDL_bool;

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
    ) -> SDL_bool;

    pub fn SDL_GL_ResetAttributes() -> c_void;

    /// * attr: 
    /// * value: 
    pub fn SDL_GL_SetAttribute(
        attr: SDL_GLattr,
        value: i32,
    ) -> i32;

    /// * attr: 
    /// * value: 
    pub fn SDL_GL_GetAttribute(
        attr: SDL_GLattr,
        value: *mut i32,
    ) -> i32;

    /// * window: 
    pub fn SDL_GL_CreateContext(
        window: *mut SDL_Window,
    ) -> SDL_GLContext;

    /// * window: 
    /// * context: 
    pub fn SDL_GL_MakeCurrent(
        window: *mut SDL_Window,
        context: SDL_GLContext,
    ) -> i32;

    pub fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;

    pub fn SDL_GL_GetCurrentContext() -> SDL_GLContext;

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
        context: SDL_GLContext,
    ) -> c_void;
}
