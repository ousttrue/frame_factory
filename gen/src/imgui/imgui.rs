// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;

pub type ImDrawListSharedData = c_void;
pub type ImFontBuilderIO = c_void;
pub type ImGuiContext = c_void;
pub type ImGuiCol = i32;
pub type ImGuiCond = i32;
pub type ImGuiDataType = i32;
pub type ImGuiDir = i32;
pub type ImGuiKey = i32;
pub type ImGuiNavInput = i32;
pub type ImGuiMouseButton = i32;
pub type ImGuiMouseCursor = i32;
pub type ImGuiSortDirection = i32;
pub type ImGuiStyleVar = i32;
pub type ImGuiTableBgTarget = i32;
pub type ImDrawFlags = i32;
pub type ImDrawListFlags = i32;
pub type ImFontAtlasFlags = i32;
pub type ImGuiBackendFlags = i32;
pub type ImGuiButtonFlags = i32;
pub type ImGuiColorEditFlags = i32;
pub type ImGuiConfigFlags = i32;
pub type ImGuiComboFlags = i32;
pub type ImGuiDockNodeFlags = i32;
pub type ImGuiDragDropFlags = i32;
pub type ImGuiFocusedFlags = i32;
pub type ImGuiHoveredFlags = i32;
pub type ImGuiInputTextFlags = i32;
pub type ImGuiKeyModFlags = i32;
pub type ImGuiPopupFlags = i32;
pub type ImGuiSelectableFlags = i32;
pub type ImGuiSliderFlags = i32;
pub type ImGuiTabBarFlags = i32;
pub type ImGuiTabItemFlags = i32;
pub type ImGuiTableFlags = i32;
pub type ImGuiTableColumnFlags = i32;
pub type ImGuiTableRowFlags = i32;
pub type ImGuiTreeNodeFlags = i32;
pub type ImGuiViewportFlags = i32;
pub type ImGuiWindowFlags = i32;
pub type ImTextureID = *mut c_void;
pub type ImGuiID = u32;
pub type ImWchar16 = u16;
pub type ImWchar32 = u32;
pub type ImWchar = ImWchar16;
pub type ImS8 = i8;
pub type ImU8 = u8;
pub type ImS16 = i16;
pub type ImU16 = u16;
pub type ImS32 = i32;
pub type ImU32 = u32;
pub type ImS64 = i64;
pub type ImU64 = u64;

#[repr(C)]
pub struct ImVec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct ImVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(i32)]
pub enum ImGuiWindowFlags_ {
    None = 0,
    NoTitleBar = 0x1,
    NoResize = 0x2,
    NoMove = 0x4,
    NoScrollbar = 0x8,
    NoScrollWithMouse = 0x10,
    NoCollapse = 0x20,
    AlwaysAutoResize = 0x40,
    NoBackground = 0x80,
    NoSavedSettings = 0x100,
    NoMouseInputs = 0x200,
    MenuBar = 0x400,
    HorizontalScrollbar = 0x800,
    NoFocusOnAppearing = 0x1000,
    NoBringToFrontOnFocus = 0x2000,
    AlwaysVerticalScrollbar = 0x4000,
    AlwaysHorizontalScrollbar = 0x8000,
    AlwaysUseWindowPadding = 0x10000,
    NoNavInputs = 0x40000,
    NoNavFocus = 0x80000,
    UnsavedDocument = 0x100000,
    NoDocking = 0x200000,
    NoNav = 0xc0000,
    NoDecoration = 0x2b,
    NoInputs = 0xc0200,
    NavFlattened = 0x800000,
    ChildWindow = 0x1000000,
    Tooltip = 0x2000000,
    Popup = 0x4000000,
    Modal = 0x8000000,
    ChildMenu = 0x10000000,
    DockNodeHost = 0x20000000,
}

#[repr(i32)]
pub enum ImGuiInputTextFlags_ {
    None = 0,
    CharsDecimal = 0x1,
    CharsHexadecimal = 0x2,
    CharsUppercase = 0x4,
    CharsNoBlank = 0x8,
    AutoSelectAll = 0x10,
    EnterReturnsTrue = 0x20,
    CallbackCompletion = 0x40,
    CallbackHistory = 0x80,
    CallbackAlways = 0x100,
    CallbackCharFilter = 0x200,
    AllowTabInput = 0x400,
    CtrlEnterForNewLine = 0x800,
    NoHorizontalScroll = 0x1000,
    AlwaysOverwrite = 0x2000,
    ReadOnly = 0x4000,
    Password = 0x8000,
    NoUndoRedo = 0x10000,
    CharsScientific = 0x20000,
    CallbackResize = 0x40000,
    CallbackEdit = 0x80000,
    // AlwaysInsertMode = 0x2000,
}

#[repr(i32)]
pub enum ImGuiTreeNodeFlags_ {
    None = 0,
    Selected = 0x1,
    Framed = 0x2,
    AllowItemOverlap = 0x4,
    NoTreePushOnOpen = 0x8,
    NoAutoOpenOnLog = 0x10,
    DefaultOpen = 0x20,
    OpenOnDoubleClick = 0x40,
    OpenOnArrow = 0x80,
    Leaf = 0x100,
    Bullet = 0x200,
    FramePadding = 0x400,
    SpanAvailWidth = 0x800,
    SpanFullWidth = 0x1000,
    NavLeftJumpsBackHere = 0x2000,
    CollapsingHeader = 0x1a,
}

#[repr(i32)]
pub enum ImGuiPopupFlags_ {
    None = 0,
    // MouseButtonLeft = 0,
    MouseButtonRight = 0x1,
    MouseButtonMiddle = 0x2,
    MouseButtonMask_ = 0x1f,
    // MouseButtonDefault_ = 0x1,
    NoOpenOverExistingPopup = 0x20,
    NoOpenOverItems = 0x40,
    AnyPopupId = 0x80,
    AnyPopupLevel = 0x100,
    AnyPopup = 0x180,
}

#[repr(i32)]
pub enum ImGuiSelectableFlags_ {
    None = 0,
    DontClosePopups = 0x1,
    SpanAllColumns = 0x2,
    AllowDoubleClick = 0x4,
    Disabled = 0x8,
    AllowItemOverlap = 0x10,
}

#[repr(i32)]
pub enum ImGuiComboFlags_ {
    None = 0,
    PopupAlignLeft = 0x1,
    HeightSmall = 0x2,
    HeightRegular = 0x4,
    HeightLarge = 0x8,
    HeightLargest = 0x10,
    NoArrowButton = 0x20,
    NoPreview = 0x40,
    HeightMask_ = 0x1e,
}

#[repr(i32)]
pub enum ImGuiTabBarFlags_ {
    None = 0,
    Reorderable = 0x1,
    AutoSelectNewTabs = 0x2,
    TabListPopupButton = 0x4,
    NoCloseWithMiddleMouseButton = 0x8,
    NoTabListScrollingButtons = 0x10,
    NoTooltip = 0x20,
    FittingPolicyResizeDown = 0x40,
    FittingPolicyScroll = 0x80,
    FittingPolicyMask_ = 0xc0,
    // FittingPolicyDefault_ = 0x40,
}

#[repr(i32)]
pub enum ImGuiTabItemFlags_ {
    None = 0,
    UnsavedDocument = 0x1,
    SetSelected = 0x2,
    NoCloseWithMiddleMouseButton = 0x4,
    NoPushId = 0x8,
    NoTooltip = 0x10,
    NoReorder = 0x20,
    Leading = 0x40,
    Trailing = 0x80,
}

#[repr(i32)]
pub enum ImGuiTableFlags_ {
    None = 0,
    Resizable = 0x1,
    Reorderable = 0x2,
    Hideable = 0x4,
    Sortable = 0x8,
    NoSavedSettings = 0x10,
    ContextMenuInBody = 0x20,
    RowBg = 0x40,
    BordersInnerH = 0x80,
    BordersOuterH = 0x100,
    BordersInnerV = 0x200,
    BordersOuterV = 0x400,
    BordersH = 0x180,
    BordersV = 0x600,
    BordersInner = 0x280,
    BordersOuter = 0x500,
    Borders = 0x780,
    NoBordersInBody = 0x800,
    NoBordersInBodyUntilResize = 0x1000,
    SizingFixedFit = 0x2000,
    SizingFixedSame = 0x4000,
    SizingStretchProp = 0x6000,
    SizingStretchSame = 0x8000,
    NoHostExtendX = 0x10000,
    NoHostExtendY = 0x20000,
    NoKeepColumnsVisible = 0x40000,
    PreciseWidths = 0x80000,
    NoClip = 0x100000,
    PadOuterX = 0x200000,
    NoPadOuterX = 0x400000,
    NoPadInnerX = 0x800000,
    ScrollX = 0x1000000,
    ScrollY = 0x2000000,
    SortMulti = 0x4000000,
    SortTristate = 0x8000000,
    SizingMask_ = 0xe000,
}

#[repr(i32)]
pub enum ImGuiTableColumnFlags_ {
    None = 0,
    DefaultHide = 0x1,
    DefaultSort = 0x2,
    WidthStretch = 0x4,
    WidthFixed = 0x8,
    NoResize = 0x10,
    NoReorder = 0x20,
    NoHide = 0x40,
    NoClip = 0x80,
    NoSort = 0x100,
    NoSortAscending = 0x200,
    NoSortDescending = 0x400,
    NoHeaderWidth = 0x800,
    PreferSortAscending = 0x1000,
    PreferSortDescending = 0x2000,
    IndentEnable = 0x4000,
    IndentDisable = 0x8000,
    IsEnabled = 0x100000,
    IsVisible = 0x200000,
    IsSorted = 0x400000,
    IsHovered = 0x800000,
    WidthMask_ = 0xc,
    IndentMask_ = 0xc000,
    StatusMask_ = 0xf00000,
    NoDirectResize_ = 0x40000000,
}

#[repr(i32)]
pub enum ImGuiTableRowFlags_ {
    None = 0,
    Headers = 0x1,
}

#[repr(i32)]
pub enum ImGuiTableBgTarget_ {
    None = 0,
    RowBg0 = 0x1,
    RowBg1 = 0x2,
    CellBg = 0x3,
}

#[repr(i32)]
pub enum ImGuiFocusedFlags_ {
    None = 0,
    ChildWindows = 0x1,
    RootWindow = 0x2,
    AnyWindow = 0x4,
    RootAndChildWindows = 0x3,
}

#[repr(i32)]
pub enum ImGuiHoveredFlags_ {
    None = 0,
    ChildWindows = 0x1,
    RootWindow = 0x2,
    AnyWindow = 0x4,
    AllowWhenBlockedByPopup = 0x8,
    AllowWhenBlockedByActiveItem = 0x20,
    AllowWhenOverlapped = 0x40,
    AllowWhenDisabled = 0x80,
    RectOnly = 0x68,
    RootAndChildWindows = 0x3,
}

#[repr(i32)]
pub enum ImGuiDockNodeFlags_ {
    None = 0,
    KeepAliveOnly = 0x1,
    NoDockingInCentralNode = 0x4,
    PassthruCentralNode = 0x8,
    NoSplit = 0x10,
    NoResize = 0x20,
    AutoHideTabBar = 0x40,
}

#[repr(i32)]
pub enum ImGuiDragDropFlags_ {
    None = 0,
    SourceNoPreviewTooltip = 0x1,
    SourceNoDisableHover = 0x2,
    SourceNoHoldToOpenOthers = 0x4,
    SourceAllowNullID = 0x8,
    SourceExtern = 0x10,
    SourceAutoExpirePayload = 0x20,
    AcceptBeforeDelivery = 0x400,
    AcceptNoDrawDefaultRect = 0x800,
    AcceptNoPreviewTooltip = 0x1000,
    AcceptPeekOnly = 0xc00,
}

#[repr(i32)]
pub enum ImGuiDataType_ {
    S8 = 0,
    U8 = 0x1,
    S16 = 0x2,
    U16 = 0x3,
    S32 = 0x4,
    U32 = 0x5,
    S64 = 0x6,
    U64 = 0x7,
    Float = 0x8,
    Double = 0x9,
    COUNT = 0xa,
}

#[repr(i32)]
pub enum ImGuiDir_ {
    None = -1,
    Left = 0,
    Right = 0x1,
    Up = 0x2,
    Down = 0x3,
    COUNT = 0x4,
}

#[repr(i32)]
pub enum ImGuiSortDirection_ {
    None = 0,
    Ascending = 0x1,
    Descending = 0x2,
}

#[repr(i32)]
pub enum ImGuiKey_ {
    Tab = 0,
    LeftArrow = 0x1,
    RightArrow = 0x2,
    UpArrow = 0x3,
    DownArrow = 0x4,
    PageUp = 0x5,
    PageDown = 0x6,
    Home = 0x7,
    End = 0x8,
    Insert = 0x9,
    Delete = 0xa,
    Backspace = 0xb,
    Space = 0xc,
    Enter = 0xd,
    Escape = 0xe,
    KeyPadEnter = 0xf,
    A = 0x10,
    C = 0x11,
    V = 0x12,
    X = 0x13,
    Y = 0x14,
    Z = 0x15,
    COUNT = 0x16,
}

#[repr(i32)]
pub enum ImGuiKeyModFlags_ {
    None = 0,
    Ctrl = 0x1,
    Shift = 0x2,
    Alt = 0x4,
    Super = 0x8,
}

#[repr(i32)]
pub enum ImGuiNavInput_ {
    Activate = 0,
    Cancel = 0x1,
    Input = 0x2,
    Menu = 0x3,
    DpadLeft = 0x4,
    DpadRight = 0x5,
    DpadUp = 0x6,
    DpadDown = 0x7,
    LStickLeft = 0x8,
    LStickRight = 0x9,
    LStickUp = 0xa,
    LStickDown = 0xb,
    FocusPrev = 0xc,
    FocusNext = 0xd,
    TweakSlow = 0xe,
    TweakFast = 0xf,
    KeyMenu_ = 0x10,
    KeyLeft_ = 0x11,
    KeyRight_ = 0x12,
    KeyUp_ = 0x13,
    KeyDown_ = 0x14,
    COUNT = 0x15,
    // InternalStart_ = 0x10,
}

#[repr(i32)]
pub enum ImGuiConfigFlags_ {
    None = 0,
    NavEnableKeyboard = 0x1,
    NavEnableGamepad = 0x2,
    NavEnableSetMousePos = 0x4,
    NavNoCaptureKeyboard = 0x8,
    NoMouse = 0x10,
    NoMouseCursorChange = 0x20,
    DockingEnable = 0x40,
    ViewportsEnable = 0x400,
    DpiEnableScaleViewports = 0x4000,
    DpiEnableScaleFonts = 0x8000,
    IsSRGB = 0x100000,
    IsTouchScreen = 0x200000,
}

#[repr(i32)]
pub enum ImGuiBackendFlags_ {
    None = 0,
    HasGamepad = 0x1,
    HasMouseCursors = 0x2,
    HasSetMousePos = 0x4,
    RendererHasVtxOffset = 0x8,
    PlatformHasViewports = 0x400,
    HasMouseHoveredViewport = 0x800,
    RendererHasViewports = 0x1000,
}

#[repr(i32)]
pub enum ImGuiCol_ {
    Text = 0,
    TextDisabled = 0x1,
    WindowBg = 0x2,
    ChildBg = 0x3,
    PopupBg = 0x4,
    Border = 0x5,
    BorderShadow = 0x6,
    FrameBg = 0x7,
    FrameBgHovered = 0x8,
    FrameBgActive = 0x9,
    TitleBg = 0xa,
    TitleBgActive = 0xb,
    TitleBgCollapsed = 0xc,
    MenuBarBg = 0xd,
    ScrollbarBg = 0xe,
    ScrollbarGrab = 0xf,
    ScrollbarGrabHovered = 0x10,
    ScrollbarGrabActive = 0x11,
    CheckMark = 0x12,
    SliderGrab = 0x13,
    SliderGrabActive = 0x14,
    Button = 0x15,
    ButtonHovered = 0x16,
    ButtonActive = 0x17,
    Header = 0x18,
    HeaderHovered = 0x19,
    HeaderActive = 0x1a,
    Separator = 0x1b,
    SeparatorHovered = 0x1c,
    SeparatorActive = 0x1d,
    ResizeGrip = 0x1e,
    ResizeGripHovered = 0x1f,
    ResizeGripActive = 0x20,
    Tab = 0x21,
    TabHovered = 0x22,
    TabActive = 0x23,
    TabUnfocused = 0x24,
    TabUnfocusedActive = 0x25,
    DockingPreview = 0x26,
    DockingEmptyBg = 0x27,
    PlotLines = 0x28,
    PlotLinesHovered = 0x29,
    PlotHistogram = 0x2a,
    PlotHistogramHovered = 0x2b,
    TableHeaderBg = 0x2c,
    TableBorderStrong = 0x2d,
    TableBorderLight = 0x2e,
    TableRowBg = 0x2f,
    TableRowBgAlt = 0x30,
    TextSelectedBg = 0x31,
    DragDropTarget = 0x32,
    NavHighlight = 0x33,
    NavWindowingHighlight = 0x34,
    NavWindowingDimBg = 0x35,
    ModalWindowDimBg = 0x36,
    COUNT = 0x37,
}

#[repr(i32)]
pub enum ImGuiStyleVar_ {
    Alpha = 0,
    WindowPadding = 0x1,
    WindowRounding = 0x2,
    WindowBorderSize = 0x3,
    WindowMinSize = 0x4,
    WindowTitleAlign = 0x5,
    ChildRounding = 0x6,
    ChildBorderSize = 0x7,
    PopupRounding = 0x8,
    PopupBorderSize = 0x9,
    FramePadding = 0xa,
    FrameRounding = 0xb,
    FrameBorderSize = 0xc,
    ItemSpacing = 0xd,
    ItemInnerSpacing = 0xe,
    IndentSpacing = 0xf,
    CellPadding = 0x10,
    ScrollbarSize = 0x11,
    ScrollbarRounding = 0x12,
    GrabMinSize = 0x13,
    GrabRounding = 0x14,
    TabRounding = 0x15,
    ButtonTextAlign = 0x16,
    SelectableTextAlign = 0x17,
    COUNT = 0x18,
}

#[repr(i32)]
pub enum ImGuiButtonFlags_ {
    None = 0,
    MouseButtonLeft = 0x1,
    MouseButtonRight = 0x2,
    MouseButtonMiddle = 0x4,
    MouseButtonMask_ = 0x7,
    // MouseButtonDefault_ = 0x1,
}

#[repr(i32)]
pub enum ImGuiColorEditFlags_ {
    None = 0,
    NoAlpha = 0x2,
    NoPicker = 0x4,
    NoOptions = 0x8,
    NoSmallPreview = 0x10,
    NoInputs = 0x20,
    NoTooltip = 0x40,
    NoLabel = 0x80,
    NoSidePreview = 0x100,
    NoDragDrop = 0x200,
    NoBorder = 0x400,
    AlphaBar = 0x10000,
    AlphaPreview = 0x20000,
    AlphaPreviewHalf = 0x40000,
    HDR = 0x80000,
    DisplayRGB = 0x100000,
    DisplayHSV = 0x200000,
    DisplayHex = 0x400000,
    Uint8 = 0x800000,
    Float = 0x1000000,
    PickerHueBar = 0x2000000,
    PickerHueWheel = 0x4000000,
    InputRGB = 0x8000000,
    InputHSV = 0x10000000,
    _OptionsDefault = 0xa900000,
    _DisplayMask = 0x700000,
    _DataTypeMask = 0x1800000,
    _PickerMask = 0x6000000,
    _InputMask = 0x18000000,
    // RGB = 0x100000,
    // HSV = 0x200000,
    // HEX = 0x400000,
}

#[repr(i32)]
pub enum ImGuiSliderFlags_ {
    None = 0,
    AlwaysClamp = 0x10,
    Logarithmic = 0x20,
    NoRoundToFormat = 0x40,
    NoInput = 0x80,
    InvalidMask_ = 0x7000000f,
    // ClampOnInput = 0x10,
}

#[repr(i32)]
pub enum ImGuiMouseButton_ {
    Left = 0,
    Right = 0x1,
    Middle = 0x2,
    COUNT = 0x5,
}

#[repr(i32)]
pub enum ImGuiMouseCursor_ {
    None = -1,
    Arrow = 0,
    TextInput = 0x1,
    ResizeAll = 0x2,
    ResizeNS = 0x3,
    ResizeEW = 0x4,
    ResizeNESW = 0x5,
    ResizeNWSE = 0x6,
    Hand = 0x7,
    NotAllowed = 0x8,
    COUNT = 0x9,
}

#[repr(i32)]
pub enum ImGuiCond_ {
    None = 0,
    Always = 0x1,
    Once = 0x2,
    FirstUseEver = 0x4,
    Appearing = 0x8,
}

#[repr(C)]
pub struct ImGuiStyle {
    pub Alpha: f32,
    pub WindowPadding: ImVec2,
    pub WindowRounding: f32,
    pub WindowBorderSize: f32,
    pub WindowMinSize: ImVec2,
    pub WindowTitleAlign: ImVec2,
    pub WindowMenuButtonPosition: ImGuiDir,
    pub ChildRounding: f32,
    pub ChildBorderSize: f32,
    pub PopupRounding: f32,
    pub PopupBorderSize: f32,
    pub FramePadding: ImVec2,
    pub FrameRounding: f32,
    pub FrameBorderSize: f32,
    pub ItemSpacing: ImVec2,
    pub ItemInnerSpacing: ImVec2,
    pub CellPadding: ImVec2,
    pub TouchExtraPadding: ImVec2,
    pub IndentSpacing: f32,
    pub ColumnsMinSpacing: f32,
    pub ScrollbarSize: f32,
    pub ScrollbarRounding: f32,
    pub GrabMinSize: f32,
    pub GrabRounding: f32,
    pub LogSliderDeadzone: f32,
    pub TabRounding: f32,
    pub TabBorderSize: f32,
    pub TabMinWidthForCloseButton: f32,
    pub ColorButtonPosition: ImGuiDir,
    pub ButtonTextAlign: ImVec2,
    pub SelectableTextAlign: ImVec2,
    pub DisplayWindowPadding: ImVec2,
    pub DisplaySafeAreaPadding: ImVec2,
    pub MouseCursorScale: f32,
    pub AntiAliasedLines: bool,
    pub AntiAliasedLinesUseTex: bool,
    pub AntiAliasedFill: bool,
    pub CurveTessellationTol: f32,
    pub CircleTessellationMaxError: f32,
    pub Colors: [ImVec4; 55],
}

#[repr(C)]
pub struct ImGuiIO {
    pub ConfigFlags: ImGuiConfigFlags,
    pub BackendFlags: ImGuiBackendFlags,
    pub DisplaySize: ImVec2,
    pub DeltaTime: f32,
    pub IniSavingRate: f32,
    pub IniFilename: *mut i8,
    pub LogFilename: *mut i8,
    pub MouseDoubleClickTime: f32,
    pub MouseDoubleClickMaxDist: f32,
    pub MouseDragThreshold: f32,
    pub KeyMap: [i32; 22],
    pub KeyRepeatDelay: f32,
    pub KeyRepeatRate: f32,
    pub UserData: *mut c_void,
    pub Fonts: *mut ImFontAtlas,
    pub FontGlobalScale: f32,
    pub FontAllowUserScaling: bool,
    pub FontDefault: *mut ImFont,
    pub DisplayFramebufferScale: ImVec2,
    pub ConfigDockingNoSplit: bool,
    pub ConfigDockingAlwaysTabBar: bool,
    pub ConfigDockingTransparentPayload: bool,
    pub ConfigViewportsNoAutoMerge: bool,
    pub ConfigViewportsNoTaskBarIcon: bool,
    pub ConfigViewportsNoDecoration: bool,
    pub ConfigViewportsNoDefaultParent: bool,
    pub MouseDrawCursor: bool,
    pub ConfigMacOSXBehaviors: bool,
    pub ConfigInputTextCursorBlink: bool,
    pub ConfigDragClickToInputText: bool,
    pub ConfigWindowsResizeFromEdges: bool,
    pub ConfigWindowsMoveFromTitleBarOnly: bool,
    pub ConfigMemoryCompactTimer: f32,
    pub BackendPlatformName: *mut i8,
    pub BackendRendererName: *mut i8,
    pub BackendPlatformUserData: *mut c_void,
    pub BackendRendererUserData: *mut c_void,
    pub BackendLanguageUserData: *mut c_void,
    pub GetClipboardTextFn: *mut extern fn(*mut c_void,) -> *mut i8,
    pub SetClipboardTextFn: *mut extern fn(*mut c_void,*mut i8,) -> c_void,
    pub ClipboardUserData: *mut c_void,
    pub MousePos: ImVec2,
    pub MouseDown: [bool; 5],
    pub MouseWheel: f32,
    pub MouseWheelH: f32,
    pub MouseHoveredViewport: ImGuiID,
    pub KeyCtrl: bool,
    pub KeyShift: bool,
    pub KeyAlt: bool,
    pub KeySuper: bool,
    pub KeysDown: [bool; 512],
    pub NavInputs: [f32; 21],
    pub WantCaptureMouse: bool,
    pub WantCaptureKeyboard: bool,
    pub WantTextInput: bool,
    pub WantSetMousePos: bool,
    pub WantSaveIniSettings: bool,
    pub NavActive: bool,
    pub NavVisible: bool,
    pub Framerate: f32,
    pub MetricsRenderVertices: i32,
    pub MetricsRenderIndices: i32,
    pub MetricsRenderWindows: i32,
    pub MetricsActiveWindows: i32,
    pub MetricsActiveAllocations: i32,
    pub MouseDelta: ImVec2,
    pub KeyMods: ImGuiKeyModFlags,
    pub MousePosPrev: ImVec2,
    pub MouseClickedPos: [ImVec2; 5],
    pub MouseClickedTime: [f64; 5],
    pub MouseClicked: [bool; 5],
    pub MouseDoubleClicked: [bool; 5],
    pub MouseReleased: [bool; 5],
    pub MouseDownOwned: [bool; 5],
    pub MouseDownWasDoubleClick: [bool; 5],
    pub MouseDownDuration: [f32; 5],
    pub MouseDownDurationPrev: [f32; 5],
    pub MouseDragMaxDistanceAbs: [ImVec2; 5],
    pub MouseDragMaxDistanceSqr: [f32; 5],
    pub KeysDownDuration: [f32; 512],
    pub KeysDownDurationPrev: [f32; 512],
    pub NavInputsDownDuration: [f32; 21],
    pub NavInputsDownDurationPrev: [f32; 21],
    pub PenPressure: f32,
    pub InputQueueSurrogate: ImWchar16,
    pub InputQueueCharacters: *mut c_void,
}

#[repr(C)]
pub struct ImGuiInputTextCallbackData {
    pub EventFlag: ImGuiInputTextFlags,
    pub Flags: ImGuiInputTextFlags,
    pub UserData: *mut c_void,
    pub EventChar: ImWchar,
    pub EventKey: ImGuiKey,
    pub Buf: *mut i8,
    pub BufTextLen: i32,
    pub BufSize: i32,
    pub BufDirty: bool,
    pub CursorPos: i32,
    pub SelectionStart: i32,
    pub SelectionEnd: i32,
}

#[repr(C)]
pub struct ImGuiSizeCallbackData {
    pub UserData: *mut c_void,
    pub Pos: ImVec2,
    pub CurrentSize: ImVec2,
    pub DesiredSize: ImVec2,
}

#[repr(C)]
pub struct ImGuiWindowClass {
    pub ClassId: ImGuiID,
    pub ParentViewportId: ImGuiID,
    pub ViewportFlagsOverrideSet: ImGuiViewportFlags,
    pub ViewportFlagsOverrideClear: ImGuiViewportFlags,
    pub TabItemFlagsOverrideSet: ImGuiTabItemFlags,
    pub DockNodeFlagsOverrideSet: ImGuiDockNodeFlags,
    pub DockNodeFlagsOverrideClear: ImGuiDockNodeFlags,
    pub DockingAlwaysTabBar: bool,
    pub DockingAllowUnclassed: bool,
}

#[repr(C)]
pub struct ImGuiPayload {
    pub Data: *mut c_void,
    pub DataSize: i32,
    pub SourceId: ImGuiID,
    pub SourceParentId: ImGuiID,
    pub DataFrameCount: i32,
    pub DataType: [i8; 33],
    pub Preview: bool,
    pub Delivery: bool,
}

#[repr(C)]
pub struct ImGuiTableColumnSortSpecs {
    pub ColumnUserID: ImGuiID,
    pub ColumnIndex: ImS16,
    pub SortOrder: ImS16,
    pub SortDirection: ImGuiSortDirection,
}

#[repr(C)]
pub struct ImGuiTableSortSpecs {
    pub Specs: *mut ImGuiTableColumnSortSpecs,
    pub SpecsCount: i32,
    pub SpecsDirty: bool,
}

#[repr(C)]
pub struct ImGuiOnceUponAFrame {
    pub RefFrame: i32,
}

#[repr(C)]
pub struct ImGuiTextFilter {
    pub InputBuf: [i8; 256],
    pub Filters: *mut c_void,
    pub CountGrep: i32,
}

#[repr(C)]
pub struct ImGuiTextBuffer {
    pub Buf: *mut c_void,
}

#[repr(C)]
pub struct ImGuiStorage {
    pub Data: *mut c_void,
}

#[repr(C)]
pub struct ImGuiListClipper {
    pub DisplayStart: i32,
    pub DisplayEnd: i32,
    pub ItemsCount: i32,
    pub StepNo: i32,
    pub ItemsFrozen: i32,
    pub ItemsHeight: f32,
    pub StartPosY: f32,
}

#[repr(C)]
pub struct ImColor {
    pub Value: ImVec4,
}

#[repr(C)]
pub struct ImDrawCmd {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: u32,
    pub IdxOffset: u32,
    pub ElemCount: u32,
    pub UserCallback: extern fn(*mut ImDrawList,*mut ImDrawCmd,) -> c_void,
    pub UserCallbackData: *mut c_void,
}
pub type ImDrawIdx = u16;

#[repr(C)]
pub struct ImDrawVert {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}

#[repr(C)]
pub struct ImDrawCmdHeader {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: u32,
}

#[repr(C)]
pub struct ImDrawChannel {
    pub _CmdBuffer: *mut c_void,
    pub _IdxBuffer: *mut c_void,
}

#[repr(C)]
pub struct ImDrawListSplitter {
    pub _Current: i32,
    pub _Count: i32,
    pub _Channels: *mut c_void,
}

#[repr(i32)]
pub enum ImDrawFlags_ {
    None = 0,
    Closed = 0x1,
    RoundCornersTopLeft = 0x10,
    RoundCornersTopRight = 0x20,
    RoundCornersBottomLeft = 0x40,
    RoundCornersBottomRight = 0x80,
    RoundCornersNone = 0x100,
    RoundCornersTop = 0x30,
    RoundCornersBottom = 0xc0,
    RoundCornersLeft = 0x50,
    RoundCornersRight = 0xa0,
    RoundCornersAll = 0xf0,
    // RoundCornersDefault_ = 0xf0,
    RoundCornersMask_ = 0x1f0,
}

#[repr(i32)]
pub enum ImDrawListFlags_ {
    None = 0,
    AntiAliasedLines = 0x1,
    AntiAliasedLinesUseTex = 0x2,
    AntiAliasedFill = 0x4,
    AllowVtxOffset = 0x8,
}

#[repr(C)]
pub struct ImDrawList {
    pub CmdBuffer: *mut c_void,
    pub IdxBuffer: *mut c_void,
    pub VtxBuffer: *mut c_void,
    pub Flags: ImDrawListFlags,
    pub _VtxCurrentIdx: u32,
    pub _Data: *mut ImDrawListSharedData,
    pub _OwnerName: *mut i8,
    pub _VtxWritePtr: *mut ImDrawVert,
    pub _IdxWritePtr: *mut ImDrawIdx,
    pub _ClipRectStack: *mut c_void,
    pub _TextureIdStack: *mut c_void,
    pub _Path: *mut c_void,
    pub _CmdHeader: ImDrawCmdHeader,
    pub _Splitter: ImDrawListSplitter,
    pub _FringeScale: f32,
}

#[repr(C)]
pub struct ImDrawData {
    pub Valid: bool,
    pub CmdListsCount: i32,
    pub TotalIdxCount: i32,
    pub TotalVtxCount: i32,
    pub CmdLists: *mut *mut ImDrawList,
    pub DisplayPos: ImVec2,
    pub DisplaySize: ImVec2,
    pub FramebufferScale: ImVec2,
    pub OwnerViewport: *mut ImGuiViewport,
}

#[repr(C)]
pub struct ImFontConfig {
    pub FontData: *mut c_void,
    pub FontDataSize: i32,
    pub FontDataOwnedByAtlas: bool,
    pub FontNo: i32,
    pub SizePixels: f32,
    pub OversampleH: i32,
    pub OversampleV: i32,
    pub PixelSnapH: bool,
    pub GlyphExtraSpacing: ImVec2,
    pub GlyphOffset: ImVec2,
    pub GlyphRanges: *mut ImWchar,
    pub GlyphMinAdvanceX: f32,
    pub GlyphMaxAdvanceX: f32,
    pub MergeMode: bool,
    pub FontBuilderFlags: u32,
    pub RasterizerMultiply: f32,
    pub EllipsisChar: ImWchar,
    pub Name: [i8; 40],
    pub DstFont: *mut ImFont,
}

#[repr(C)]
pub struct ImFontGlyph {
    pub Colored: u32,
    pub Visible: u32,
    pub Codepoint: u32,
    pub AdvanceX: f32,
    pub X0: f32,
    pub Y0: f32,
    pub X1: f32,
    pub Y1: f32,
    pub U0: f32,
    pub V0: f32,
    pub U1: f32,
    pub V1: f32,
}

#[repr(C)]
pub struct ImFontGlyphRangesBuilder {
    pub UsedChars: *mut c_void,
}

#[repr(C)]
pub struct ImFontAtlasCustomRect {
    pub Width: u16,
    pub Height: u16,
    pub X: u16,
    pub Y: u16,
    pub GlyphID: u32,
    pub GlyphAdvanceX: f32,
    pub GlyphOffset: ImVec2,
    pub Font: *mut ImFont,
}

#[repr(i32)]
pub enum ImFontAtlasFlags_ {
    None = 0,
    NoPowerOfTwoHeight = 0x1,
    NoMouseCursors = 0x2,
    NoBakedLines = 0x4,
}

#[repr(C)]
pub struct ImFontAtlas {
    pub Flags: ImFontAtlasFlags,
    pub TexID: ImTextureID,
    pub TexDesiredWidth: i32,
    pub TexGlyphPadding: i32,
    pub Locked: bool,
    pub TexPixelsUseColors: bool,
    pub TexPixelsAlpha8: *mut u8,
    pub TexPixelsRGBA32: *mut u32,
    pub TexWidth: i32,
    pub TexHeight: i32,
    pub TexUvScale: ImVec2,
    pub TexUvWhitePixel: ImVec2,
    pub Fonts: *mut c_void,
    pub CustomRects: *mut c_void,
    pub ConfigData: *mut c_void,
    pub TexUvLines: [ImVec4; 64],
    pub FontBuilderIO: *mut ImFontBuilderIO,
    pub FontBuilderFlags: u32,
    pub PackIdMouseCursors: i32,
    pub PackIdLines: i32,
}
pub type CustomRect = ImFontAtlasCustomRect;
pub type GlyphRangesBuilder = ImFontGlyphRangesBuilder;

#[repr(C)]
pub struct ImFont {
    pub IndexAdvanceX: *mut c_void,
    pub FallbackAdvanceX: f32,
    pub FontSize: f32,
    pub IndexLookup: *mut c_void,
    pub Glyphs: *mut c_void,
    pub FallbackGlyph: *mut ImFontGlyph,
    pub ContainerAtlas: *mut ImFontAtlas,
    pub ConfigData: *mut ImFontConfig,
    pub ConfigDataCount: i16,
    pub FallbackChar: ImWchar,
    pub EllipsisChar: ImWchar,
    pub DirtyLookupTables: bool,
    pub Scale: f32,
    pub Ascent: f32,
    pub Descent: f32,
    pub MetricsTotalSurface: i32,
    pub Used4kPagesMap: [ImU8; 2],
}

#[repr(i32)]
pub enum ImGuiViewportFlags_ {
    None = 0,
    IsPlatformWindow = 0x1,
    IsPlatformMonitor = 0x2,
    OwnedByApp = 0x4,
    NoDecoration = 0x8,
    NoTaskBarIcon = 0x10,
    NoFocusOnAppearing = 0x20,
    NoFocusOnClick = 0x40,
    NoInputs = 0x80,
    NoRendererClear = 0x100,
    TopMost = 0x200,
    Minimized = 0x400,
    NoAutoMerge = 0x800,
    CanHostOtherWindows = 0x1000,
}

#[repr(C)]
pub struct ImGuiViewport {
    pub ID: ImGuiID,
    pub Flags: ImGuiViewportFlags,
    pub Pos: ImVec2,
    pub Size: ImVec2,
    pub WorkPos: ImVec2,
    pub WorkSize: ImVec2,
    pub DpiScale: f32,
    pub ParentViewportId: ImGuiID,
    pub DrawData: *mut ImDrawData,
    pub RendererUserData: *mut c_void,
    pub PlatformUserData: *mut c_void,
    pub PlatformHandle: *mut c_void,
    pub PlatformHandleRaw: *mut c_void,
    pub PlatformRequestMove: bool,
    pub PlatformRequestResize: bool,
    pub PlatformRequestClose: bool,
}

#[repr(C)]
pub struct ImGuiPlatformIO {
    pub Platform_CreateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_DestroyWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_ShowWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_SetWindowPos: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    pub Platform_GetWindowPos: *mut extern fn(*mut ImGuiViewport,) -> ImVec2,
    pub Platform_SetWindowSize: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    pub Platform_GetWindowSize: *mut extern fn(*mut ImGuiViewport,) -> ImVec2,
    pub Platform_SetWindowFocus: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_GetWindowFocus: *mut extern fn(*mut ImGuiViewport,) -> bool,
    pub Platform_GetWindowMinimized: *mut extern fn(*mut ImGuiViewport,) -> bool,
    pub Platform_SetWindowTitle: *mut extern fn(*mut ImGuiViewport,*mut i8,) -> c_void,
    pub Platform_SetWindowAlpha: *mut extern fn(*mut ImGuiViewport,f32,) -> c_void,
    pub Platform_UpdateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_RenderWindow: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    pub Platform_SwapBuffers: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    pub Platform_GetWindowDpiScale: *mut extern fn(*mut ImGuiViewport,) -> f32,
    pub Platform_OnChangedViewport: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Platform_SetImeInputPos: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    pub Platform_CreateVkSurface: *mut extern fn(*mut ImGuiViewport,ImU64,*mut c_void,*mut ImU64,) -> i32,
    pub Renderer_CreateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Renderer_DestroyWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    pub Renderer_SetWindowSize: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    pub Renderer_RenderWindow: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    pub Renderer_SwapBuffers: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    pub Monitors: *mut c_void,
    pub Viewports: *mut c_void,
}

#[repr(C)]
pub struct ImGuiPlatformMonitor {
    pub MainPos: ImVec2,
    pub MainSize: ImVec2,
    pub WorkPos: ImVec2,
    pub WorkSize: ImVec2,
    pub DpiScale: f32,
}
pub type ImDrawCornerFlags = ImDrawFlags;

#[repr(i32)]
pub enum ImDrawCornerFlags_ {
    None = 0x100,
    TopLeft = 0x10,
    TopRight = 0x20,
    BotLeft = 0x40,
    BotRight = 0x80,
    All = 0xf0,
    Top = 0x30,
    Bot = 0xc0,
    Left = 0x50,
    Right = 0xa0,
}

#[link(name = "imgui_static", kind = "static")]
extern "C" {

    /// * data: 
    #[link_name = ""]
    pub fn ImGuiInputTextCallback(
        data: *mut ImGuiInputTextCallbackData,
    ) -> i32;

    /// * data: 
    #[link_name = ""]
    pub fn ImGuiSizeCallback(
        data: *mut ImGuiSizeCallbackData,
    ) -> c_void;

    /// * sz: 
    /// * user_data: 
    #[link_name = ""]
    pub fn ImGuiMemAllocFunc(
        sz: usize,
        user_data: *mut c_void,
    ) -> *mut c_void;

    /// * ptr: 
    /// * user_data: 
    #[link_name = ""]
    pub fn ImGuiMemFreeFunc(
        ptr: *mut c_void,
        user_data: *mut c_void,
    ) -> c_void;

    /// * shared_font_atlas: NULL
    #[link_name = "?CreateContext@ImGui@@YAPEAUImGuiContext@@PEAUImFontAtlas@@@Z"]
    pub fn CreateContext(
        shared_font_atlas: *mut ImFontAtlas,
    ) -> *mut ImGuiContext;

    /// * ctx: NULL
    #[link_name = "?DestroyContext@ImGui@@YAXPEAUImGuiContext@@@Z"]
    pub fn DestroyContext(
        ctx: *mut ImGuiContext,
    ) -> c_void;

    #[link_name = "?GetCurrentContext@ImGui@@YAPEAUImGuiContext@@XZ"]
    pub fn GetCurrentContext() -> *mut ImGuiContext;

    /// * ctx: 
    #[link_name = "?SetCurrentContext@ImGui@@YAXPEAUImGuiContext@@@Z"]
    pub fn SetCurrentContext(
        ctx: *mut ImGuiContext,
    ) -> c_void;

    #[link_name = "?GetIO@ImGui@@YAAEAUImGuiIO@@XZ"]
    pub fn GetIO() -> *mut ImGuiIO;

    #[link_name = "?GetStyle@ImGui@@YAAEAUImGuiStyle@@XZ"]
    pub fn GetStyle() -> *mut ImGuiStyle;

    #[link_name = "?NewFrame@ImGui@@YAXXZ"]
    pub fn NewFrame() -> c_void;

    #[link_name = "?EndFrame@ImGui@@YAXXZ"]
    pub fn EndFrame() -> c_void;

    #[link_name = "?Render@ImGui@@YAXXZ"]
    pub fn Render() -> c_void;

    #[link_name = "?GetDrawData@ImGui@@YAPEAUImDrawData@@XZ"]
    pub fn GetDrawData() -> *mut ImDrawData;

    /// * p_open: NULL
    #[link_name = "?ShowDemoWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowDemoWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * p_open: NULL
    #[link_name = "?ShowMetricsWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowMetricsWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * p_open: NULL
    #[link_name = "?ShowAboutWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowAboutWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * ref: NULL
    #[link_name = "?ShowStyleEditor@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn ShowStyleEditor(
        r#ref: *mut ImGuiStyle,
    ) -> c_void;

    /// * label: 
    #[link_name = "?ShowStyleSelector@ImGui@@YA_NPEBD@Z"]
    pub fn ShowStyleSelector(
        label: *const i8,
    ) -> bool;

    /// * label: 
    #[link_name = "?ShowFontSelector@ImGui@@YAXPEBD@Z"]
    pub fn ShowFontSelector(
        label: *const i8,
    ) -> c_void;

    #[link_name = "?ShowUserGuide@ImGui@@YAXXZ"]
    pub fn ShowUserGuide() -> c_void;

    #[link_name = "?GetVersion@ImGui@@YAPEBDXZ"]
    pub fn GetVersion() -> *mut i8;

    /// * dst: NULL
    #[link_name = "?StyleColorsDark@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsDark(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * dst: NULL
    #[link_name = "?StyleColorsLight@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsLight(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * dst: NULL
    #[link_name = "?StyleColorsClassic@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsClassic(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * name: 
    /// * p_open: NULL
    /// * flags: 0
    #[link_name = "?Begin@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn Begin(
        name: *const i8,
        p_open: *mut bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[link_name = "?End@ImGui@@YAXXZ"]
    pub fn End() -> c_void;

    /// * str_id: 
    /// * size: ImVec2(0,0)
    /// * border: false
    /// * flags: 0
    #[link_name = "?BeginChild@ImGui@@YA_NPEBDAEBUImVec2@@_NH@Z"]
    pub fn BeginChild(
        str_id: *const i8,
        size: *const ImVec2,
        border: bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    /// * id: 
    /// * size: ImVec2(0,0)
    /// * border: false
    /// * flags: 0
    #[link_name = "?BeginChild@ImGui@@YA_NIAEBUImVec2@@_NH@Z"]
    pub fn BeginChild_(
        id: ImGuiID,
        size: *const ImVec2,
        border: bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[link_name = "?EndChild@ImGui@@YAXXZ"]
    pub fn EndChild() -> c_void;

    #[link_name = "?IsWindowAppearing@ImGui@@YA_NXZ"]
    pub fn IsWindowAppearing() -> bool;

    #[link_name = "?IsWindowCollapsed@ImGui@@YA_NXZ"]
    pub fn IsWindowCollapsed() -> bool;

    /// * flags: 0
    #[link_name = "?IsWindowFocused@ImGui@@YA_NH@Z"]
    pub fn IsWindowFocused(
        flags: ImGuiFocusedFlags,
    ) -> bool;

    /// * flags: 0
    #[link_name = "?IsWindowHovered@ImGui@@YA_NH@Z"]
    pub fn IsWindowHovered(
        flags: ImGuiHoveredFlags,
    ) -> bool;

    #[link_name = "?GetWindowDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetWindowDrawList() -> *mut ImDrawList;

    #[link_name = "?GetWindowDpiScale@ImGui@@YAMXZ"]
    pub fn GetWindowDpiScale() -> f32;

    #[link_name = "?GetWindowPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowPos() -> ImVec2;

    #[link_name = "?GetWindowSize@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowSize() -> ImVec2;

    #[link_name = "?GetWindowWidth@ImGui@@YAMXZ"]
    pub fn GetWindowWidth() -> f32;

    #[link_name = "?GetWindowHeight@ImGui@@YAMXZ"]
    pub fn GetWindowHeight() -> f32;

    #[link_name = "?GetWindowViewport@ImGui@@YAPEAUImGuiViewport@@XZ"]
    pub fn GetWindowViewport() -> *mut ImGuiViewport;

    /// * pos: 
    /// * cond: 0
    /// * pivot: ImVec2(0,0)
    #[link_name = "?SetNextWindowPos@ImGui@@YAXAEBUImVec2@@H0@Z"]
    pub fn SetNextWindowPos(
        pos: *const ImVec2,
        cond: ImGuiCond,
        pivot: *const ImVec2,
    ) -> c_void;

    /// * size: 
    /// * cond: 0
    #[link_name = "?SetNextWindowSize@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetNextWindowSize(
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * size_min: 
    /// * size_max: 
    /// * custom_callback: NULL
    /// * custom_callback_data: NULL
    #[link_name = "?SetNextWindowSizeConstraints@ImGui@@YAXAEBUImVec2@@0P6AXPEAUImGuiSizeCallbackData@@@ZPEAX@Z"]
    pub fn SetNextWindowSizeConstraints(
        size_min: *const ImVec2,
        size_max: *const ImVec2,
        custom_callback: extern fn(*mut ImGuiSizeCallbackData,) -> c_void,
        custom_callback_data: *mut c_void,
    ) -> c_void;

    /// * size: 
    #[link_name = "?SetNextWindowContentSize@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetNextWindowContentSize(
        size: *const ImVec2,
    ) -> c_void;

    /// * collapsed: 
    /// * cond: 0
    #[link_name = "?SetNextWindowCollapsed@ImGui@@YAX_NH@Z"]
    pub fn SetNextWindowCollapsed(
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    #[link_name = "?SetNextWindowFocus@ImGui@@YAXXZ"]
    pub fn SetNextWindowFocus() -> c_void;

    /// * alpha: 
    #[link_name = "?SetNextWindowBgAlpha@ImGui@@YAXM@Z"]
    pub fn SetNextWindowBgAlpha(
        alpha: f32,
    ) -> c_void;

    /// * viewport_id: 
    #[link_name = "?SetNextWindowViewport@ImGui@@YAXI@Z"]
    pub fn SetNextWindowViewport(
        viewport_id: ImGuiID,
    ) -> c_void;

    /// * pos: 
    /// * cond: 0
    #[link_name = "?SetWindowPos@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetWindowPos(
        pos: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * size: 
    /// * cond: 0
    #[link_name = "?SetWindowSize@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetWindowSize(
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * collapsed: 
    /// * cond: 0
    #[link_name = "?SetWindowCollapsed@ImGui@@YAX_NH@Z"]
    pub fn SetWindowCollapsed(
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    #[link_name = "?SetWindowFocus@ImGui@@YAXXZ"]
    pub fn SetWindowFocus() -> c_void;

    /// * scale: 
    #[link_name = "?SetWindowFontScale@ImGui@@YAXM@Z"]
    pub fn SetWindowFontScale(
        scale: f32,
    ) -> c_void;

    /// * name: 
    /// * pos: 
    /// * cond: 0
    #[link_name = "?SetWindowPos@ImGui@@YAXPEBDAEBUImVec2@@H@Z"]
    pub fn SetWindowPos_(
        name: *const i8,
        pos: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    /// * size: 
    /// * cond: 0
    #[link_name = "?SetWindowSize@ImGui@@YAXPEBDAEBUImVec2@@H@Z"]
    pub fn SetWindowSize_(
        name: *const i8,
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    /// * collapsed: 
    /// * cond: 0
    #[link_name = "?SetWindowCollapsed@ImGui@@YAXPEBD_NH@Z"]
    pub fn SetWindowCollapsed_(
        name: *const i8,
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    #[link_name = "?SetWindowFocus@ImGui@@YAXPEBD@Z"]
    pub fn SetWindowFocus_(
        name: *const i8,
    ) -> c_void;

    #[link_name = "?GetContentRegionAvail@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetContentRegionAvail() -> ImVec2;

    #[link_name = "?GetContentRegionMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetContentRegionMax() -> ImVec2;

    #[link_name = "?GetWindowContentRegionMin@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowContentRegionMin() -> ImVec2;

    #[link_name = "?GetWindowContentRegionMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowContentRegionMax() -> ImVec2;

    #[link_name = "?GetWindowContentRegionWidth@ImGui@@YAMXZ"]
    pub fn GetWindowContentRegionWidth() -> f32;

    #[link_name = "?GetScrollX@ImGui@@YAMXZ"]
    pub fn GetScrollX() -> f32;

    #[link_name = "?GetScrollY@ImGui@@YAMXZ"]
    pub fn GetScrollY() -> f32;

    /// * scroll_x: 
    #[link_name = "?SetScrollX@ImGui@@YAXM@Z"]
    pub fn SetScrollX(
        scroll_x: f32,
    ) -> c_void;

    /// * scroll_y: 
    #[link_name = "?SetScrollY@ImGui@@YAXM@Z"]
    pub fn SetScrollY(
        scroll_y: f32,
    ) -> c_void;

    #[link_name = "?GetScrollMaxX@ImGui@@YAMXZ"]
    pub fn GetScrollMaxX() -> f32;

    #[link_name = "?GetScrollMaxY@ImGui@@YAMXZ"]
    pub fn GetScrollMaxY() -> f32;

    /// * center_x_ratio: 0.5f
    #[link_name = "?SetScrollHereX@ImGui@@YAXM@Z"]
    pub fn SetScrollHereX(
        center_x_ratio: f32,
    ) -> c_void;

    /// * center_y_ratio: 0.5f
    #[link_name = "?SetScrollHereY@ImGui@@YAXM@Z"]
    pub fn SetScrollHereY(
        center_y_ratio: f32,
    ) -> c_void;

    /// * local_x: 
    /// * center_x_ratio: 0.5f
    #[link_name = "?SetScrollFromPosX@ImGui@@YAXMM@Z"]
    pub fn SetScrollFromPosX(
        local_x: f32,
        center_x_ratio: f32,
    ) -> c_void;

    /// * local_y: 
    /// * center_y_ratio: 0.5f
    #[link_name = "?SetScrollFromPosY@ImGui@@YAXMM@Z"]
    pub fn SetScrollFromPosY(
        local_y: f32,
        center_y_ratio: f32,
    ) -> c_void;

    /// * font: 
    #[link_name = "?PushFont@ImGui@@YAXPEAUImFont@@@Z"]
    pub fn PushFont(
        font: *mut ImFont,
    ) -> c_void;

    #[link_name = "?PopFont@ImGui@@YAXXZ"]
    pub fn PopFont() -> c_void;

    /// * idx: 
    /// * col: 
    #[link_name = "?PushStyleColor@ImGui@@YAXHI@Z"]
    pub fn PushStyleColor(
        idx: ImGuiCol,
        col: ImU32,
    ) -> c_void;

    /// * idx: 
    /// * col: 
    #[link_name = "?PushStyleColor@ImGui@@YAXHAEBUImVec4@@@Z"]
    pub fn PushStyleColor_(
        idx: ImGuiCol,
        col: *const ImVec4,
    ) -> c_void;

    /// * count: 1
    #[link_name = "?PopStyleColor@ImGui@@YAXH@Z"]
    pub fn PopStyleColor(
        count: i32,
    ) -> c_void;

    /// * idx: 
    /// * val: 
    #[link_name = "?PushStyleVar@ImGui@@YAXHM@Z"]
    pub fn PushStyleVar(
        idx: ImGuiStyleVar,
        val: f32,
    ) -> c_void;

    /// * idx: 
    /// * val: 
    #[link_name = "?PushStyleVar@ImGui@@YAXHAEBUImVec2@@@Z"]
    pub fn PushStyleVar_(
        idx: ImGuiStyleVar,
        val: *const ImVec2,
    ) -> c_void;

    /// * count: 1
    #[link_name = "?PopStyleVar@ImGui@@YAXH@Z"]
    pub fn PopStyleVar(
        count: i32,
    ) -> c_void;

    /// * allow_keyboard_focus: 
    #[link_name = "?PushAllowKeyboardFocus@ImGui@@YAX_N@Z"]
    pub fn PushAllowKeyboardFocus(
        allow_keyboard_focus: bool,
    ) -> c_void;

    #[link_name = "?PopAllowKeyboardFocus@ImGui@@YAXXZ"]
    pub fn PopAllowKeyboardFocus() -> c_void;

    /// * repeat: 
    #[link_name = "?PushButtonRepeat@ImGui@@YAX_N@Z"]
    pub fn PushButtonRepeat(
        repeat: bool,
    ) -> c_void;

    #[link_name = "?PopButtonRepeat@ImGui@@YAXXZ"]
    pub fn PopButtonRepeat() -> c_void;

    /// * item_width: 
    #[link_name = "?PushItemWidth@ImGui@@YAXM@Z"]
    pub fn PushItemWidth(
        item_width: f32,
    ) -> c_void;

    #[link_name = "?PopItemWidth@ImGui@@YAXXZ"]
    pub fn PopItemWidth() -> c_void;

    /// * item_width: 
    #[link_name = "?SetNextItemWidth@ImGui@@YAXM@Z"]
    pub fn SetNextItemWidth(
        item_width: f32,
    ) -> c_void;

    #[link_name = "?CalcItemWidth@ImGui@@YAMXZ"]
    pub fn CalcItemWidth() -> f32;

    /// * wrap_local_pos_x: 0.0f
    #[link_name = "?PushTextWrapPos@ImGui@@YAXM@Z"]
    pub fn PushTextWrapPos(
        wrap_local_pos_x: f32,
    ) -> c_void;

    #[link_name = "?PopTextWrapPos@ImGui@@YAXXZ"]
    pub fn PopTextWrapPos() -> c_void;

    #[link_name = "?GetFont@ImGui@@YAPEAUImFont@@XZ"]
    pub fn GetFont() -> *mut ImFont;

    #[link_name = "?GetFontSize@ImGui@@YAMXZ"]
    pub fn GetFontSize() -> f32;

    #[link_name = "?GetFontTexUvWhitePixel@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetFontTexUvWhitePixel() -> ImVec2;

    /// * idx: 
    /// * alpha_mul: 1.0f
    #[link_name = "?GetColorU32@ImGui@@YAIHM@Z"]
    pub fn GetColorU32(
        idx: ImGuiCol,
        alpha_mul: f32,
    ) -> ImU32;

    /// * col: 
    #[link_name = "?GetColorU32@ImGui@@YAIAEBUImVec4@@@Z"]
    pub fn GetColorU32_(
        col: *const ImVec4,
    ) -> ImU32;

    /// * col: 
    #[link_name = "?GetColorU32@ImGui@@YAII@Z"]
    pub fn GetColorU32__(
        col: ImU32,
    ) -> ImU32;

    /// * idx: 
    #[link_name = "?GetStyleColorVec4@ImGui@@YAAEBUImVec4@@H@Z"]
    pub fn GetStyleColorVec4(
        idx: ImGuiCol,
    ) -> *mut ImVec4;

    #[link_name = "?Separator@ImGui@@YAXXZ"]
    pub fn Separator() -> c_void;

    /// * offset_from_start_x: 0.0f
    /// * spacing: -1.0f
    #[link_name = "?SameLine@ImGui@@YAXMM@Z"]
    pub fn SameLine(
        offset_from_start_x: f32,
        spacing: f32,
    ) -> c_void;

    #[link_name = "?NewLine@ImGui@@YAXXZ"]
    pub fn NewLine() -> c_void;

    #[link_name = "?Spacing@ImGui@@YAXXZ"]
    pub fn Spacing() -> c_void;

    /// * size: 
    #[link_name = "?Dummy@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn Dummy(
        size: *const ImVec2,
    ) -> c_void;

    /// * indent_w: 0.0f
    #[link_name = "?Indent@ImGui@@YAXM@Z"]
    pub fn Indent(
        indent_w: f32,
    ) -> c_void;

    /// * indent_w: 0.0f
    #[link_name = "?Unindent@ImGui@@YAXM@Z"]
    pub fn Unindent(
        indent_w: f32,
    ) -> c_void;

    #[link_name = "?BeginGroup@ImGui@@YAXXZ"]
    pub fn BeginGroup() -> c_void;

    #[link_name = "?EndGroup@ImGui@@YAXXZ"]
    pub fn EndGroup() -> c_void;

    #[link_name = "?GetCursorPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorPos() -> ImVec2;

    #[link_name = "?GetCursorPosX@ImGui@@YAMXZ"]
    pub fn GetCursorPosX() -> f32;

    #[link_name = "?GetCursorPosY@ImGui@@YAMXZ"]
    pub fn GetCursorPosY() -> f32;

    /// * local_pos: 
    #[link_name = "?SetCursorPos@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetCursorPos(
        local_pos: *const ImVec2,
    ) -> c_void;

    /// * local_x: 
    #[link_name = "?SetCursorPosX@ImGui@@YAXM@Z"]
    pub fn SetCursorPosX(
        local_x: f32,
    ) -> c_void;

    /// * local_y: 
    #[link_name = "?SetCursorPosY@ImGui@@YAXM@Z"]
    pub fn SetCursorPosY(
        local_y: f32,
    ) -> c_void;

    #[link_name = "?GetCursorStartPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorStartPos() -> ImVec2;

    #[link_name = "?GetCursorScreenPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorScreenPos() -> ImVec2;

    /// * pos: 
    #[link_name = "?SetCursorScreenPos@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetCursorScreenPos(
        pos: *const ImVec2,
    ) -> c_void;

    #[link_name = "?AlignTextToFramePadding@ImGui@@YAXXZ"]
    pub fn AlignTextToFramePadding() -> c_void;

    #[link_name = "?GetTextLineHeight@ImGui@@YAMXZ"]
    pub fn GetTextLineHeight() -> f32;

    #[link_name = "?GetTextLineHeightWithSpacing@ImGui@@YAMXZ"]
    pub fn GetTextLineHeightWithSpacing() -> f32;

    #[link_name = "?GetFrameHeight@ImGui@@YAMXZ"]
    pub fn GetFrameHeight() -> f32;

    #[link_name = "?GetFrameHeightWithSpacing@ImGui@@YAMXZ"]
    pub fn GetFrameHeightWithSpacing() -> f32;

    /// * str_id: 
    #[link_name = "?PushID@ImGui@@YAXPEBD@Z"]
    pub fn PushID(
        str_id: *const i8,
    ) -> c_void;

    /// * str_id_begin: 
    /// * str_id_end: 
    #[link_name = "?PushID@ImGui@@YAXPEBD0@Z"]
    pub fn PushID_(
        str_id_begin: *const i8,
        str_id_end: *const i8,
    ) -> c_void;

    /// * ptr_id: 
    #[link_name = "?PushID@ImGui@@YAXPEBX@Z"]
    pub fn PushID__(
        ptr_id: *const c_void,
    ) -> c_void;

    /// * int_id: 
    #[link_name = "?PushID@ImGui@@YAXH@Z"]
    pub fn PushID___(
        int_id: i32,
    ) -> c_void;

    #[link_name = "?PopID@ImGui@@YAXXZ"]
    pub fn PopID() -> c_void;

    /// * str_id: 
    #[link_name = "?GetID@ImGui@@YAIPEBD@Z"]
    pub fn GetID(
        str_id: *const i8,
    ) -> ImGuiID;

    /// * str_id_begin: 
    /// * str_id_end: 
    #[link_name = "?GetID@ImGui@@YAIPEBD0@Z"]
    pub fn GetID_(
        str_id_begin: *const i8,
        str_id_end: *const i8,
    ) -> ImGuiID;

    /// * ptr_id: 
    #[link_name = "?GetID@ImGui@@YAIPEBX@Z"]
    pub fn GetID__(
        ptr_id: *const c_void,
    ) -> ImGuiID;

    /// * text: 
    /// * text_end: NULL
    #[link_name = "?TextUnformatted@ImGui@@YAXPEBD0@Z"]
    pub fn TextUnformatted(
        text: *const i8,
        text_end: *const i8,
    ) -> c_void;

    /// * fmt: 
    #[link_name = "?Text@ImGui@@YAXPEBDZZ"]
    pub fn Text(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?TextV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn TextV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * col: 
    /// * fmt: 
    #[link_name = "?TextColored@ImGui@@YAXAEBUImVec4@@PEBDZZ"]
    pub fn TextColored(
        col: *const ImVec4,
        fmt: *const i8,
    ) -> c_void;

    /// * col: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?TextColoredV@ImGui@@YAXAEBUImVec4@@PEBDPEAD@Z"]
    pub fn TextColoredV(
        col: *const ImVec4,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * fmt: 
    #[link_name = "?TextDisabled@ImGui@@YAXPEBDZZ"]
    pub fn TextDisabled(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?TextDisabledV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn TextDisabledV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * fmt: 
    #[link_name = "?TextWrapped@ImGui@@YAXPEBDZZ"]
    pub fn TextWrapped(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?TextWrappedV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn TextWrappedV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * label: 
    /// * fmt: 
    #[link_name = "?LabelText@ImGui@@YAXPEBD0ZZ"]
    pub fn LabelText(
        label: *const i8,
        fmt: *const i8,
    ) -> c_void;

    /// * label: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?LabelTextV@ImGui@@YAXPEBD0PEAD@Z"]
    pub fn LabelTextV(
        label: *const i8,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * fmt: 
    #[link_name = "?BulletText@ImGui@@YAXPEBDZZ"]
    pub fn BulletText(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?BulletTextV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn BulletTextV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * label: 
    /// * size: ImVec2(0,0)
    #[link_name = "?Button@ImGui@@YA_NPEBDAEBUImVec2@@@Z"]
    pub fn Button(
        label: *const i8,
        size: *const ImVec2,
    ) -> bool;

    /// * label: 
    #[link_name = "?SmallButton@ImGui@@YA_NPEBD@Z"]
    pub fn SmallButton(
        label: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * size: 
    /// * flags: 0
    #[link_name = "?InvisibleButton@ImGui@@YA_NPEBDAEBUImVec2@@H@Z"]
    pub fn InvisibleButton(
        str_id: *const i8,
        size: *const ImVec2,
        flags: ImGuiButtonFlags,
    ) -> bool;

    /// * str_id: 
    /// * dir: 
    #[link_name = "?ArrowButton@ImGui@@YA_NPEBDH@Z"]
    pub fn ArrowButton(
        str_id: *const i8,
        dir: ImGuiDir,
    ) -> bool;

    /// * user_texture_id: 
    /// * size: 
    /// * uv0: ImVec2(0,0)
    /// * uv1: ImVec2(1,1)
    /// * tint_col: ImVec4(1,1,1,1)
    /// * border_col: ImVec4(0,0,0,0)
    #[link_name = "?Image@ImGui@@YAXPEAXAEBUImVec2@@11AEBUImVec4@@2@Z"]
    pub fn Image(
        user_texture_id: ImTextureID,
        size: *const ImVec2,
        uv0: *const ImVec2,
        uv1: *const ImVec2,
        tint_col: *const ImVec4,
        border_col: *const ImVec4,
    ) -> c_void;

    /// * user_texture_id: 
    /// * size: 
    /// * uv0: ImVec2(0,0)
    /// * uv1: ImVec2(1,1)
    /// * frame_padding: -1
    /// * bg_col: ImVec4(0,0,0,0)
    /// * tint_col: ImVec4(1,1,1,1)
    #[link_name = "?ImageButton@ImGui@@YA_NPEAXAEBUImVec2@@11HAEBUImVec4@@2@Z"]
    pub fn ImageButton(
        user_texture_id: ImTextureID,
        size: *const ImVec2,
        uv0: *const ImVec2,
        uv1: *const ImVec2,
        frame_padding: i32,
        bg_col: *const ImVec4,
        tint_col: *const ImVec4,
    ) -> bool;

    /// * label: 
    /// * v: 
    #[link_name = "?Checkbox@ImGui@@YA_NPEBDPEA_N@Z"]
    pub fn Checkbox(
        label: *const i8,
        v: *mut bool,
    ) -> bool;

    /// * label: 
    /// * flags: 
    /// * flags_value: 
    #[link_name = "?CheckboxFlags@ImGui@@YA_NPEBDPEAHH@Z"]
    pub fn CheckboxFlags(
        label: *const i8,
        flags: *mut i32,
        flags_value: i32,
    ) -> bool;

    /// * label: 
    /// * flags: 
    /// * flags_value: 
    #[link_name = "?CheckboxFlags@ImGui@@YA_NPEBDPEAII@Z"]
    pub fn CheckboxFlags_(
        label: *const i8,
        flags: *mut u32,
        flags_value: u32,
    ) -> bool;

    /// * label: 
    /// * active: 
    #[link_name = "?RadioButton@ImGui@@YA_NPEBD_N@Z"]
    pub fn RadioButton(
        label: *const i8,
        active: bool,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_button: 
    #[link_name = "?RadioButton@ImGui@@YA_NPEBDPEAHH@Z"]
    pub fn RadioButton_(
        label: *const i8,
        v: *mut i32,
        v_button: i32,
    ) -> bool;

    /// * fraction: 
    /// * size_arg: ImVec2(-FLT_MIN,0)
    /// * overlay: NULL
    #[link_name = "?ProgressBar@ImGui@@YAXMAEBUImVec2@@PEBD@Z"]
    pub fn ProgressBar(
        fraction: f32,
        size_arg: *const ImVec2,
        overlay: *const i8,
    ) -> c_void;

    #[link_name = "?Bullet@ImGui@@YAXXZ"]
    pub fn Bullet() -> c_void;

    /// * label: 
    /// * preview_value: 
    /// * flags: 0
    #[link_name = "?BeginCombo@ImGui@@YA_NPEBD0H@Z"]
    pub fn BeginCombo(
        label: *const i8,
        preview_value: *const i8,
        flags: ImGuiComboFlags,
    ) -> bool;

    #[link_name = "?EndCombo@ImGui@@YAXXZ"]
    pub fn EndCombo() -> c_void;

    /// * label: 
    /// * current_item: 
    /// * items: 
    /// * items_count: 
    /// * popup_max_height_in_items: -1
    #[link_name = "?Combo@ImGui@@YA_NPEBDPEAHQEBQEBDHH@Z"]
    pub fn Combo(
        label: *const i8,
        current_item: *mut i32,
        items: *mut *mut i8,
        items_count: i32,
        popup_max_height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * current_item: 
    /// * items_separated_by_zeros: 
    /// * popup_max_height_in_items: -1
    #[link_name = "?Combo@ImGui@@YA_NPEBDPEAH0H@Z"]
    pub fn Combo_(
        label: *const i8,
        current_item: *mut i32,
        items_separated_by_zeros: *const i8,
        popup_max_height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * current_item: 
    /// * items_getter: 
    /// * data: 
    /// * items_count: 
    /// * popup_max_height_in_items: -1
    #[link_name = "?Combo@ImGui@@YA_NPEBDPEAHP6A_NPEAXHPEAPEBD@Z2HH@Z"]
    pub fn Combo__(
        label: *const i8,
        current_item: *mut i32,
        items_getter: *mut extern fn(*mut c_void,i32,*mut *mut i8,) -> bool,
        data: *mut c_void,
        items_count: i32,
        popup_max_height_in_items: i32,
    ) -> bool;

    /// * data: 
    /// * idx: 
    /// * out_text: 
    #[link_name = "?items_getter@?1??Combo@ImGui@@YA_NPEBDPEAHP6A_NPEAXHPEAPEBD@Z2HH@Z@3P6A_N2H3@ZEA"]
    pub fn items_getter(
        data: *mut c_void,
        idx: i32,
        out_text: *const *mut i8,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?DragFloat@ImGui@@YA_NPEBDPEAMMMM0H@Z"]
    pub fn DragFloat(
        label: *const i8,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?DragFloat2@ImGui@@YA_NPEBDQEAMMMM0H@Z"]
    pub fn DragFloat2(
        label: *const i8,
        v: [f32; 2],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?DragFloat3@ImGui@@YA_NPEBDQEAMMMM0H@Z"]
    pub fn DragFloat3(
        label: *const i8,
        v: [f32; 3],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?DragFloat4@ImGui@@YA_NPEBDQEAMMMM0H@Z"]
    pub fn DragFloat4(
        label: *const i8,
        v: [f32; 4],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v_current_min: 
    /// * v_current_max: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * format_max: NULL
    /// * flags: 0
    #[link_name = "?DragFloatRange2@ImGui@@YA_NPEBDPEAM1MMM00H@Z"]
    pub fn DragFloatRange2(
        label: *const i8,
        v_current_min: *mut f32,
        v_current_max: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        format_max: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0
    /// * v_max: 0
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?DragInt@ImGui@@YA_NPEBDPEAHMHH0H@Z"]
    pub fn DragInt(
        label: *const i8,
        v: *mut i32,
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0
    /// * v_max: 0
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?DragInt2@ImGui@@YA_NPEBDQEAHMHH0H@Z"]
    pub fn DragInt2(
        label: *const i8,
        v: [i32; 2],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0
    /// * v_max: 0
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?DragInt3@ImGui@@YA_NPEBDQEAHMHH0H@Z"]
    pub fn DragInt3(
        label: *const i8,
        v: [i32; 3],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0
    /// * v_max: 0
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?DragInt4@ImGui@@YA_NPEBDQEAHMHH0H@Z"]
    pub fn DragInt4(
        label: *const i8,
        v: [i32; 4],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v_current_min: 
    /// * v_current_max: 
    /// * v_speed: 1.0f
    /// * v_min: 0
    /// * v_max: 0
    /// * format: "%d"
    /// * format_max: NULL
    /// * flags: 0
    #[link_name = "?DragIntRange2@ImGui@@YA_NPEBDPEAH1MHH00H@Z"]
    pub fn DragIntRange2(
        label: *const i8,
        v_current_min: *mut i32,
        v_current_max: *mut i32,
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        format_max: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * v_speed: 1.0f
    /// * p_min: NULL
    /// * p_max: NULL
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?DragScalar@ImGui@@YA_NPEBDHPEAXMPEBX20H@Z"]
    pub fn DragScalar(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        v_speed: f32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * components: 
    /// * v_speed: 1.0f
    /// * p_min: NULL
    /// * p_max: NULL
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?DragScalarN@ImGui@@YA_NPEBDHPEAXHMPEBX20H@Z"]
    pub fn DragScalarN(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        components: i32,
        v_speed: f32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?SliderFloat@ImGui@@YA_NPEBDPEAMMM0H@Z"]
    pub fn SliderFloat(
        label: *const i8,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?SliderFloat2@ImGui@@YA_NPEBDQEAMMM0H@Z"]
    pub fn SliderFloat2(
        label: *const i8,
        v: [f32; 2],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?SliderFloat3@ImGui@@YA_NPEBDQEAMMM0H@Z"]
    pub fn SliderFloat3(
        label: *const i8,
        v: [f32; 3],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?SliderFloat4@ImGui@@YA_NPEBDQEAMMM0H@Z"]
    pub fn SliderFloat4(
        label: *const i8,
        v: [f32; 4],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v_rad: 
    /// * v_degrees_min: -360.0f
    /// * v_degrees_max: +360.0f
    /// * format: "%.0f deg"
    /// * flags: 0
    #[link_name = "?SliderAngle@ImGui@@YA_NPEBDPEAMMM0H@Z"]
    pub fn SliderAngle(
        label: *const i8,
        v_rad: *mut f32,
        v_degrees_min: f32,
        v_degrees_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?SliderInt@ImGui@@YA_NPEBDPEAHHH0H@Z"]
    pub fn SliderInt(
        label: *const i8,
        v: *mut i32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?SliderInt2@ImGui@@YA_NPEBDQEAHHH0H@Z"]
    pub fn SliderInt2(
        label: *const i8,
        v: [i32; 2],
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?SliderInt3@ImGui@@YA_NPEBDQEAHHH0H@Z"]
    pub fn SliderInt3(
        label: *const i8,
        v: [i32; 3],
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?SliderInt4@ImGui@@YA_NPEBDQEAHHH0H@Z"]
    pub fn SliderInt4(
        label: *const i8,
        v: [i32; 4],
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * p_min: 
    /// * p_max: 
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?SliderScalar@ImGui@@YA_NPEBDHPEAXPEBX20H@Z"]
    pub fn SliderScalar(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * components: 
    /// * p_min: 
    /// * p_max: 
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?SliderScalarN@ImGui@@YA_NPEBDHPEAXHPEBX20H@Z"]
    pub fn SliderScalarN(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        components: i32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * size: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?VSliderFloat@ImGui@@YA_NPEBDAEBUImVec2@@PEAMMM0H@Z"]
    pub fn VSliderFloat(
        label: *const i8,
        size: *const ImVec2,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * size: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: "%d"
    /// * flags: 0
    #[link_name = "?VSliderInt@ImGui@@YA_NPEBDAEBUImVec2@@PEAHHH0H@Z"]
    pub fn VSliderInt(
        label: *const i8,
        size: *const ImVec2,
        v: *mut i32,
        v_min: i32,
        v_max: i32,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * size: 
    /// * data_type: 
    /// * p_data: 
    /// * p_min: 
    /// * p_max: 
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?VSliderScalar@ImGui@@YA_NPEBDAEBUImVec2@@HPEAXPEBX30H@Z"]
    pub fn VSliderScalar(
        label: *const i8,
        size: *const ImVec2,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        flags: ImGuiSliderFlags,
    ) -> bool;

    /// * label: 
    /// * buf: 
    /// * buf_size: 
    /// * flags: 0
    /// * callback: NULL
    /// * user_data: NULL
    #[link_name = "?InputText@ImGui@@YA_NPEBDPEAD_KHP6AHPEAUImGuiInputTextCallbackData@@@ZPEAX@Z"]
    pub fn InputText(
        label: *const i8,
        buf: *mut i8,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: extern fn(*mut ImGuiInputTextCallbackData,) -> i32,
        user_data: *mut c_void,
    ) -> bool;

    /// * label: 
    /// * buf: 
    /// * buf_size: 
    /// * size: ImVec2(0,0)
    /// * flags: 0
    /// * callback: NULL
    /// * user_data: NULL
    #[link_name = "?InputTextMultiline@ImGui@@YA_NPEBDPEAD_KAEBUImVec2@@HP6AHPEAUImGuiInputTextCallbackData@@@ZPEAX@Z"]
    pub fn InputTextMultiline(
        label: *const i8,
        buf: *mut i8,
        buf_size: usize,
        size: *const ImVec2,
        flags: ImGuiInputTextFlags,
        callback: extern fn(*mut ImGuiInputTextCallbackData,) -> i32,
        user_data: *mut c_void,
    ) -> bool;

    /// * label: 
    /// * hint: 
    /// * buf: 
    /// * buf_size: 
    /// * flags: 0
    /// * callback: NULL
    /// * user_data: NULL
    #[link_name = "?InputTextWithHint@ImGui@@YA_NPEBD0PEAD_KHP6AHPEAUImGuiInputTextCallbackData@@@ZPEAX@Z"]
    pub fn InputTextWithHint(
        label: *const i8,
        hint: *const i8,
        buf: *mut i8,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: extern fn(*mut ImGuiInputTextCallbackData,) -> i32,
        user_data: *mut c_void,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * step: 0.0f
    /// * step_fast: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?InputFloat@ImGui@@YA_NPEBDPEAMMM0H@Z"]
    pub fn InputFloat(
        label: *const i8,
        v: *mut f32,
        step: f32,
        step_fast: f32,
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?InputFloat2@ImGui@@YA_NPEBDQEAM0H@Z"]
    pub fn InputFloat2(
        label: *const i8,
        v: [f32; 2],
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?InputFloat3@ImGui@@YA_NPEBDQEAM0H@Z"]
    pub fn InputFloat3(
        label: *const i8,
        v: [f32; 3],
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * format: "%.3f"
    /// * flags: 0
    #[link_name = "?InputFloat4@ImGui@@YA_NPEBDQEAM0H@Z"]
    pub fn InputFloat4(
        label: *const i8,
        v: [f32; 4],
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * step: 1
    /// * step_fast: 100
    /// * flags: 0
    #[link_name = "?InputInt@ImGui@@YA_NPEBDPEAHHHH@Z"]
    pub fn InputInt(
        label: *const i8,
        v: *mut i32,
        step: i32,
        step_fast: i32,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * flags: 0
    #[link_name = "?InputInt2@ImGui@@YA_NPEBDQEAHH@Z"]
    pub fn InputInt2(
        label: *const i8,
        v: [i32; 2],
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * flags: 0
    #[link_name = "?InputInt3@ImGui@@YA_NPEBDQEAHH@Z"]
    pub fn InputInt3(
        label: *const i8,
        v: [i32; 3],
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * flags: 0
    #[link_name = "?InputInt4@ImGui@@YA_NPEBDQEAHH@Z"]
    pub fn InputInt4(
        label: *const i8,
        v: [i32; 4],
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * step: 0.0
    /// * step_fast: 0.0
    /// * format: "%.6f"
    /// * flags: 0
    #[link_name = "?InputDouble@ImGui@@YA_NPEBDPEANNN0H@Z"]
    pub fn InputDouble(
        label: *const i8,
        v: *mut f64,
        step: f64,
        step_fast: f64,
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * p_step: NULL
    /// * p_step_fast: NULL
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?InputScalar@ImGui@@YA_NPEBDHPEAXPEBX20H@Z"]
    pub fn InputScalar(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        p_step: *const c_void,
        p_step_fast: *const c_void,
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * components: 
    /// * p_step: NULL
    /// * p_step_fast: NULL
    /// * format: NULL
    /// * flags: 0
    #[link_name = "?InputScalarN@ImGui@@YA_NPEBDHPEAXHPEBX20H@Z"]
    pub fn InputScalarN(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        components: i32,
        p_step: *const c_void,
        p_step_fast: *const c_void,
        format: *const i8,
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    #[link_name = "?ColorEdit3@ImGui@@YA_NPEBDQEAMH@Z"]
    pub fn ColorEdit3(
        label: *const i8,
        col: [f32; 3],
        flags: ImGuiColorEditFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    #[link_name = "?ColorEdit4@ImGui@@YA_NPEBDQEAMH@Z"]
    pub fn ColorEdit4(
        label: *const i8,
        col: [f32; 4],
        flags: ImGuiColorEditFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    #[link_name = "?ColorPicker3@ImGui@@YA_NPEBDQEAMH@Z"]
    pub fn ColorPicker3(
        label: *const i8,
        col: [f32; 3],
        flags: ImGuiColorEditFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    /// * ref_col: NULL
    #[link_name = "?ColorPicker4@ImGui@@YA_NPEBDQEAMHPEBM@Z"]
    pub fn ColorPicker4(
        label: *const i8,
        col: [f32; 4],
        flags: ImGuiColorEditFlags,
        ref_col: *const f32,
    ) -> bool;

    /// * desc_id: 
    /// * col: 
    /// * flags: 0
    /// * size: ImVec2(0,0)
    #[link_name = "?ColorButton@ImGui@@YA_NPEBDAEBUImVec4@@HUImVec2@@@Z"]
    pub fn ColorButton(
        desc_id: *const i8,
        col: *const ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;

    /// * flags: 
    #[link_name = "?SetColorEditOptions@ImGui@@YAXH@Z"]
    pub fn SetColorEditOptions(
        flags: ImGuiColorEditFlags,
    ) -> c_void;

    /// * label: 
    #[link_name = "?TreeNode@ImGui@@YA_NPEBD@Z"]
    pub fn TreeNode(
        label: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * fmt: 
    #[link_name = "?TreeNode@ImGui@@YA_NPEBD0ZZ"]
    pub fn TreeNode_(
        str_id: *const i8,
        fmt: *const i8,
    ) -> bool;

    /// * ptr_id: 
    /// * fmt: 
    #[link_name = "?TreeNode@ImGui@@YA_NPEBXPEBDZZ"]
    pub fn TreeNode__(
        ptr_id: *const c_void,
        fmt: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?TreeNodeV@ImGui@@YA_NPEBD0PEAD@Z"]
    pub fn TreeNodeV(
        str_id: *const i8,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> bool;

    /// * ptr_id: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?TreeNodeV@ImGui@@YA_NPEBXPEBDPEAD@Z"]
    pub fn TreeNodeV_(
        ptr_id: *const c_void,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> bool;

    /// * label: 
    /// * flags: 0
    #[link_name = "?TreeNodeEx@ImGui@@YA_NPEBDH@Z"]
    pub fn TreeNodeEx(
        label: *const i8,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * str_id: 
    /// * flags: 
    /// * fmt: 
    #[link_name = "?TreeNodeEx@ImGui@@YA_NPEBDH0ZZ"]
    pub fn TreeNodeEx_(
        str_id: *const i8,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
    ) -> bool;

    /// * ptr_id: 
    /// * flags: 
    /// * fmt: 
    #[link_name = "?TreeNodeEx@ImGui@@YA_NPEBXHPEBDZZ"]
    pub fn TreeNodeEx__(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * flags: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?TreeNodeExV@ImGui@@YA_NPEBDH0PEAD@Z"]
    pub fn TreeNodeExV(
        str_id: *const i8,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> bool;

    /// * ptr_id: 
    /// * flags: 
    /// * fmt: 
    /// * args: 
    #[link_name = "?TreeNodeExV@ImGui@@YA_NPEBXHPEBDPEAD@Z"]
    pub fn TreeNodeExV_(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
        args: va_list::VaList,
    ) -> bool;

    /// * str_id: 
    #[link_name = "?TreePush@ImGui@@YAXPEBD@Z"]
    pub fn TreePush(
        str_id: *const i8,
    ) -> c_void;

    /// * ptr_id: NULL
    #[link_name = "?TreePush@ImGui@@YAXPEBX@Z"]
    pub fn TreePush_(
        ptr_id: *const c_void,
    ) -> c_void;

    #[link_name = "?TreePop@ImGui@@YAXXZ"]
    pub fn TreePop() -> c_void;

    #[link_name = "?GetTreeNodeToLabelSpacing@ImGui@@YAMXZ"]
    pub fn GetTreeNodeToLabelSpacing() -> f32;

    /// * label: 
    /// * flags: 0
    #[link_name = "?CollapsingHeader@ImGui@@YA_NPEBDH@Z"]
    pub fn CollapsingHeader(
        label: *const i8,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * label: 
    /// * p_visible: 
    /// * flags: 0
    #[link_name = "?CollapsingHeader@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn CollapsingHeader_(
        label: *const i8,
        p_visible: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * is_open: 
    /// * cond: 0
    #[link_name = "?SetNextItemOpen@ImGui@@YAX_NH@Z"]
    pub fn SetNextItemOpen(
        is_open: bool,
        cond: ImGuiCond,
    ) -> c_void;

    /// * label: 
    /// * selected: false
    /// * flags: 0
    /// * size: ImVec2(0,0)
    #[link_name = "?Selectable@ImGui@@YA_NPEBD_NHAEBUImVec2@@@Z"]
    pub fn Selectable(
        label: *const i8,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: *const ImVec2,
    ) -> bool;

    /// * label: 
    /// * p_selected: 
    /// * flags: 0
    /// * size: ImVec2(0,0)
    #[link_name = "?Selectable@ImGui@@YA_NPEBDPEA_NHAEBUImVec2@@@Z"]
    pub fn Selectable_(
        label: *const i8,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: *const ImVec2,
    ) -> bool;

    /// * label: 
    /// * size: ImVec2(0,0)
    #[link_name = "?BeginListBox@ImGui@@YA_NPEBDAEBUImVec2@@@Z"]
    pub fn BeginListBox(
        label: *const i8,
        size: *const ImVec2,
    ) -> bool;

    #[link_name = "?EndListBox@ImGui@@YAXXZ"]
    pub fn EndListBox() -> c_void;

    /// * label: 
    /// * current_item: 
    /// * items: 
    /// * items_count: 
    /// * height_in_items: -1
    #[link_name = "?ListBox@ImGui@@YA_NPEBDPEAHQEBQEBDHH@Z"]
    pub fn ListBox(
        label: *const i8,
        current_item: *mut i32,
        items: *mut *mut i8,
        items_count: i32,
        height_in_items: i32,
    ) -> bool;

    /// * data: 
    /// * idx: 
    /// * out_text: 
    #[link_name = "?items_getter@?1??ListBox@ImGui@@YA_NPEBDPEAHP6A_NPEAXHPEAPEBD@Z2HH@Z@3P6A_N2H3@ZEA"]
    pub fn items_getter_(
        data: *mut c_void,
        idx: i32,
        out_text: *const *mut i8,
    ) -> bool;

    /// * label: 
    /// * current_item: 
    /// * items_getter: 
    /// * data: 
    /// * items_count: 
    /// * height_in_items: -1
    #[link_name = "?ListBox@ImGui@@YA_NPEBDPEAHP6A_NPEAXHPEAPEBD@Z2HH@Z"]
    pub fn ListBox_(
        label: *const i8,
        current_item: *mut i32,
        items_getter: *mut extern fn(*mut c_void,i32,*mut *mut i8,) -> bool,
        data: *mut c_void,
        items_count: i32,
        height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * values: 
    /// * values_count: 
    /// * values_offset: 0
    /// * overlay_text: NULL
    /// * scale_min: FLT_MAX
    /// * scale_max: FLT_MAX
    /// * graph_size: ImVec2(0,0)
    /// * stride: sizeof(float)
    #[link_name = "?PlotLines@ImGui@@YAXPEBDPEBMHH0MMUImVec2@@H@Z"]
    pub fn PlotLines(
        label: *const i8,
        values: *const f32,
        values_count: i32,
        values_offset: i32,
        overlay_text: *const i8,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: i32,
    ) -> c_void;

    /// * data: 
    /// * idx: 
    #[link_name = "?values_getter@?1??PlotLines@ImGui@@YAXPEBDP6AMPEAXH@Z1HH0MMUImVec2@@@Z@3P6AM1H@ZEA"]
    pub fn values_getter(
        data: *mut c_void,
        idx: i32,
    ) -> f32;

    /// * label: 
    /// * values_getter: 
    /// * data: 
    /// * values_count: 
    /// * values_offset: 0
    /// * overlay_text: NULL
    /// * scale_min: FLT_MAX
    /// * scale_max: FLT_MAX
    /// * graph_size: ImVec2(0,0)
    #[link_name = "?PlotLines@ImGui@@YAXPEBDP6AMPEAXH@Z1HH0MMUImVec2@@@Z"]
    pub fn PlotLines_(
        label: *const i8,
        values_getter: *mut extern fn(*mut c_void,i32,) -> f32,
        data: *mut c_void,
        values_count: i32,
        values_offset: i32,
        overlay_text: *const i8,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
    ) -> c_void;

    /// * label: 
    /// * values: 
    /// * values_count: 
    /// * values_offset: 0
    /// * overlay_text: NULL
    /// * scale_min: FLT_MAX
    /// * scale_max: FLT_MAX
    /// * graph_size: ImVec2(0,0)
    /// * stride: sizeof(float)
    #[link_name = "?PlotHistogram@ImGui@@YAXPEBDPEBMHH0MMUImVec2@@H@Z"]
    pub fn PlotHistogram(
        label: *const i8,
        values: *const f32,
        values_count: i32,
        values_offset: i32,
        overlay_text: *const i8,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: i32,
    ) -> c_void;

    /// * label: 
    /// * values_getter: 
    /// * data: 
    /// * values_count: 
    /// * values_offset: 0
    /// * overlay_text: NULL
    /// * scale_min: FLT_MAX
    /// * scale_max: FLT_MAX
    /// * graph_size: ImVec2(0,0)
    #[link_name = "?PlotHistogram@ImGui@@YAXPEBDP6AMPEAXH@Z1HH0MMUImVec2@@@Z"]
    pub fn PlotHistogram_(
        label: *const i8,
        values_getter: *mut extern fn(*mut c_void,i32,) -> f32,
        data: *mut c_void,
        values_count: i32,
        values_offset: i32,
        overlay_text: *const i8,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
    ) -> c_void;

    /// * data: 
    /// * idx: 
    #[link_name = "?values_getter@?1??PlotHistogram@ImGui@@YAXPEBDP6AMPEAXH@Z1HH0MMUImVec2@@@Z@3P6AM1H@ZEA"]
    pub fn values_getter_(
        data: *mut c_void,
        idx: i32,
    ) -> f32;

    /// * prefix: 
    /// * b: 
    #[link_name = "?Value@ImGui@@YAXPEBD_N@Z"]
    pub fn Value(
        prefix: *const i8,
        b: bool,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    #[link_name = "?Value@ImGui@@YAXPEBDH@Z"]
    pub fn Value_(
        prefix: *const i8,
        v: i32,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    #[link_name = "?Value@ImGui@@YAXPEBDI@Z"]
    pub fn Value__(
        prefix: *const i8,
        v: u32,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    /// * float_format: NULL
    #[link_name = "?Value@ImGui@@YAXPEBDM0@Z"]
    pub fn Value___(
        prefix: *const i8,
        v: f32,
        float_format: *const i8,
    ) -> c_void;

    #[link_name = "?BeginMenuBar@ImGui@@YA_NXZ"]
    pub fn BeginMenuBar() -> bool;

    #[link_name = "?EndMenuBar@ImGui@@YAXXZ"]
    pub fn EndMenuBar() -> c_void;

    #[link_name = "?BeginMainMenuBar@ImGui@@YA_NXZ"]
    pub fn BeginMainMenuBar() -> bool;

    #[link_name = "?EndMainMenuBar@ImGui@@YAXXZ"]
    pub fn EndMainMenuBar() -> c_void;

    /// * label: 
    /// * enabled: true
    #[link_name = "?BeginMenu@ImGui@@YA_NPEBD_N@Z"]
    pub fn BeginMenu(
        label: *const i8,
        enabled: bool,
    ) -> bool;

    #[link_name = "?EndMenu@ImGui@@YAXXZ"]
    pub fn EndMenu() -> c_void;

    /// * label: 
    /// * shortcut: NULL
    /// * selected: false
    /// * enabled: true
    #[link_name = "?MenuItem@ImGui@@YA_NPEBD0_N1@Z"]
    pub fn MenuItem(
        label: *const i8,
        shortcut: *const i8,
        selected: bool,
        enabled: bool,
    ) -> bool;

    /// * label: 
    /// * shortcut: 
    /// * p_selected: 
    /// * enabled: true
    #[link_name = "?MenuItem@ImGui@@YA_NPEBD0PEA_N_N@Z"]
    pub fn MenuItem_(
        label: *const i8,
        shortcut: *const i8,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;

    #[link_name = "?BeginTooltip@ImGui@@YAXXZ"]
    pub fn BeginTooltip() -> c_void;

    #[link_name = "?EndTooltip@ImGui@@YAXXZ"]
    pub fn EndTooltip() -> c_void;

    /// * fmt: 
    #[link_name = "?SetTooltip@ImGui@@YAXPEBDZZ"]
    pub fn SetTooltip(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?SetTooltipV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn SetTooltipV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * str_id: 
    /// * flags: 0
    #[link_name = "?BeginPopup@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopup(
        str_id: *const i8,
        flags: ImGuiWindowFlags,
    ) -> bool;

    /// * name: 
    /// * p_open: NULL
    /// * flags: 0
    #[link_name = "?BeginPopupModal@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn BeginPopupModal(
        name: *const i8,
        p_open: *mut bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[link_name = "?EndPopup@ImGui@@YAXXZ"]
    pub fn EndPopup() -> c_void;

    /// * str_id: 
    /// * popup_flags: 0
    #[link_name = "?OpenPopup@ImGui@@YAXPEBDH@Z"]
    pub fn OpenPopup(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    /// * id: 
    /// * popup_flags: 0
    #[link_name = "?OpenPopup@ImGui@@YAXIH@Z"]
    pub fn OpenPopup_(
        id: ImGuiID,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[link_name = "?OpenPopupOnItemClick@ImGui@@YAXPEBDH@Z"]
    pub fn OpenPopupOnItemClick(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    #[link_name = "?CloseCurrentPopup@ImGui@@YAXXZ"]
    pub fn CloseCurrentPopup() -> c_void;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[link_name = "?BeginPopupContextItem@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextItem(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[link_name = "?BeginPopupContextWindow@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextWindow(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[link_name = "?BeginPopupContextVoid@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextVoid(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: 
    /// * flags: 0
    #[link_name = "?IsPopupOpen@ImGui@@YA_NPEBDH@Z"]
    pub fn IsPopupOpen(
        str_id: *const i8,
        flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: 
    /// * column: 
    /// * flags: 0
    /// * outer_size: ImVec2(0.0f,0.0f)
    /// * inner_width: 0.0f
    #[link_name = "?BeginTable@ImGui@@YA_NPEBDHHAEBUImVec2@@M@Z"]
    pub fn BeginTable(
        str_id: *const i8,
        column: i32,
        flags: ImGuiTableFlags,
        outer_size: *const ImVec2,
        inner_width: f32,
    ) -> bool;

    #[link_name = "?EndTable@ImGui@@YAXXZ"]
    pub fn EndTable() -> c_void;

    /// * row_flags: 0
    /// * min_row_height: 0.0f
    #[link_name = "?TableNextRow@ImGui@@YAXHM@Z"]
    pub fn TableNextRow(
        row_flags: ImGuiTableRowFlags,
        min_row_height: f32,
    ) -> c_void;

    #[link_name = "?TableNextColumn@ImGui@@YA_NXZ"]
    pub fn TableNextColumn() -> bool;

    /// * column_n: 
    #[link_name = "?TableSetColumnIndex@ImGui@@YA_NH@Z"]
    pub fn TableSetColumnIndex(
        column_n: i32,
    ) -> bool;

    /// * label: 
    /// * flags: 0
    /// * init_width_or_weight: 0.0f
    /// * user_id: 0
    #[link_name = "?TableSetupColumn@ImGui@@YAXPEBDHMI@Z"]
    pub fn TableSetupColumn(
        label: *const i8,
        flags: ImGuiTableColumnFlags,
        init_width_or_weight: f32,
        user_id: ImGuiID,
    ) -> c_void;

    /// * cols: 
    /// * rows: 
    #[link_name = "?TableSetupScrollFreeze@ImGui@@YAXHH@Z"]
    pub fn TableSetupScrollFreeze(
        cols: i32,
        rows: i32,
    ) -> c_void;

    #[link_name = "?TableHeadersRow@ImGui@@YAXXZ"]
    pub fn TableHeadersRow() -> c_void;

    /// * label: 
    #[link_name = "?TableHeader@ImGui@@YAXPEBD@Z"]
    pub fn TableHeader(
        label: *const i8,
    ) -> c_void;

    #[link_name = "?TableGetSortSpecs@ImGui@@YAPEAUImGuiTableSortSpecs@@XZ"]
    pub fn TableGetSortSpecs() -> *mut ImGuiTableSortSpecs;

    #[link_name = "?TableGetColumnCount@ImGui@@YAHXZ"]
    pub fn TableGetColumnCount() -> i32;

    #[link_name = "?TableGetColumnIndex@ImGui@@YAHXZ"]
    pub fn TableGetColumnIndex() -> i32;

    #[link_name = "?TableGetRowIndex@ImGui@@YAHXZ"]
    pub fn TableGetRowIndex() -> i32;

    /// * column_n: -1
    #[link_name = "?TableGetColumnName@ImGui@@YAPEBDH@Z"]
    pub fn TableGetColumnName(
        column_n: i32,
    ) -> *mut i8;

    /// * column_n: -1
    #[link_name = "?TableGetColumnFlags@ImGui@@YAHH@Z"]
    pub fn TableGetColumnFlags(
        column_n: i32,
    ) -> ImGuiTableColumnFlags;

    /// * column_n: 
    /// * v: 
    #[link_name = "?TableSetColumnEnabled@ImGui@@YAXH_N@Z"]
    pub fn TableSetColumnEnabled(
        column_n: i32,
        v: bool,
    ) -> c_void;

    /// * target: 
    /// * color: 
    /// * column_n: -1
    #[link_name = "?TableSetBgColor@ImGui@@YAXHIH@Z"]
    pub fn TableSetBgColor(
        target: ImGuiTableBgTarget,
        color: ImU32,
        column_n: i32,
    ) -> c_void;

    /// * count: 1
    /// * id: NULL
    /// * border: true
    #[link_name = "?Columns@ImGui@@YAXHPEBD_N@Z"]
    pub fn Columns(
        count: i32,
        id: *const i8,
        border: bool,
    ) -> c_void;

    #[link_name = "?NextColumn@ImGui@@YAXXZ"]
    pub fn NextColumn() -> c_void;

    #[link_name = "?GetColumnIndex@ImGui@@YAHXZ"]
    pub fn GetColumnIndex() -> i32;

    /// * column_index: -1
    #[link_name = "?GetColumnWidth@ImGui@@YAMH@Z"]
    pub fn GetColumnWidth(
        column_index: i32,
    ) -> f32;

    /// * column_index: 
    /// * width: 
    #[link_name = "?SetColumnWidth@ImGui@@YAXHM@Z"]
    pub fn SetColumnWidth(
        column_index: i32,
        width: f32,
    ) -> c_void;

    /// * column_index: -1
    #[link_name = "?GetColumnOffset@ImGui@@YAMH@Z"]
    pub fn GetColumnOffset(
        column_index: i32,
    ) -> f32;

    /// * column_index: 
    /// * offset_x: 
    #[link_name = "?SetColumnOffset@ImGui@@YAXHM@Z"]
    pub fn SetColumnOffset(
        column_index: i32,
        offset_x: f32,
    ) -> c_void;

    #[link_name = "?GetColumnsCount@ImGui@@YAHXZ"]
    pub fn GetColumnsCount() -> i32;

    /// * str_id: 
    /// * flags: 0
    #[link_name = "?BeginTabBar@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginTabBar(
        str_id: *const i8,
        flags: ImGuiTabBarFlags,
    ) -> bool;

    #[link_name = "?EndTabBar@ImGui@@YAXXZ"]
    pub fn EndTabBar() -> c_void;

    /// * label: 
    /// * p_open: NULL
    /// * flags: 0
    #[link_name = "?BeginTabItem@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn BeginTabItem(
        label: *const i8,
        p_open: *mut bool,
        flags: ImGuiTabItemFlags,
    ) -> bool;

    #[link_name = "?EndTabItem@ImGui@@YAXXZ"]
    pub fn EndTabItem() -> c_void;

    /// * label: 
    /// * flags: 0
    #[link_name = "?TabItemButton@ImGui@@YA_NPEBDH@Z"]
    pub fn TabItemButton(
        label: *const i8,
        flags: ImGuiTabItemFlags,
    ) -> bool;

    /// * tab_or_docked_window_label: 
    #[link_name = "?SetTabItemClosed@ImGui@@YAXPEBD@Z"]
    pub fn SetTabItemClosed(
        tab_or_docked_window_label: *const i8,
    ) -> c_void;

    /// * id: 
    /// * size: ImVec2(0,0)
    /// * flags: 0
    /// * window_class: NULL
    #[link_name = "?DockSpace@ImGui@@YAIIAEBUImVec2@@HPEBUImGuiWindowClass@@@Z"]
    pub fn DockSpace(
        id: ImGuiID,
        size: *const ImVec2,
        flags: ImGuiDockNodeFlags,
        window_class: *const ImGuiWindowClass,
    ) -> ImGuiID;

    /// * viewport: NULL
    /// * flags: 0
    /// * window_class: NULL
    #[link_name = "?DockSpaceOverViewport@ImGui@@YAIPEBUImGuiViewport@@HPEBUImGuiWindowClass@@@Z"]
    pub fn DockSpaceOverViewport(
        viewport: *const ImGuiViewport,
        flags: ImGuiDockNodeFlags,
        window_class: *const ImGuiWindowClass,
    ) -> ImGuiID;

    /// * dock_id: 
    /// * cond: 0
    #[link_name = "?SetNextWindowDockID@ImGui@@YAXIH@Z"]
    pub fn SetNextWindowDockID(
        dock_id: ImGuiID,
        cond: ImGuiCond,
    ) -> c_void;

    /// * window_class: 
    #[link_name = "?SetNextWindowClass@ImGui@@YAXPEBUImGuiWindowClass@@@Z"]
    pub fn SetNextWindowClass(
        window_class: *const ImGuiWindowClass,
    ) -> c_void;

    #[link_name = "?GetWindowDockID@ImGui@@YAIXZ"]
    pub fn GetWindowDockID() -> ImGuiID;

    #[link_name = "?IsWindowDocked@ImGui@@YA_NXZ"]
    pub fn IsWindowDocked() -> bool;

    /// * auto_open_depth: -1
    #[link_name = "?LogToTTY@ImGui@@YAXH@Z"]
    pub fn LogToTTY(
        auto_open_depth: i32,
    ) -> c_void;

    /// * auto_open_depth: -1
    /// * filename: NULL
    #[link_name = "?LogToFile@ImGui@@YAXHPEBD@Z"]
    pub fn LogToFile(
        auto_open_depth: i32,
        filename: *const i8,
    ) -> c_void;

    /// * auto_open_depth: -1
    #[link_name = "?LogToClipboard@ImGui@@YAXH@Z"]
    pub fn LogToClipboard(
        auto_open_depth: i32,
    ) -> c_void;

    #[link_name = "?LogFinish@ImGui@@YAXXZ"]
    pub fn LogFinish() -> c_void;

    #[link_name = "?LogButtons@ImGui@@YAXXZ"]
    pub fn LogButtons() -> c_void;

    /// * fmt: 
    #[link_name = "?LogText@ImGui@@YAXPEBDZZ"]
    pub fn LogText(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[link_name = "?LogTextV@ImGui@@YAXPEBDPEAD@Z"]
    pub fn LogTextV(
        fmt: *const i8,
        args: va_list::VaList,
    ) -> c_void;

    /// * flags: 0
    #[link_name = "?BeginDragDropSource@ImGui@@YA_NH@Z"]
    pub fn BeginDragDropSource(
        flags: ImGuiDragDropFlags,
    ) -> bool;

    /// * type: 
    /// * data: 
    /// * sz: 
    /// * cond: 0
    #[link_name = "?SetDragDropPayload@ImGui@@YA_NPEBDPEBX_KH@Z"]
    pub fn SetDragDropPayload(
        r#type: *const i8,
        data: *const c_void,
        sz: usize,
        cond: ImGuiCond,
    ) -> bool;

    #[link_name = "?EndDragDropSource@ImGui@@YAXXZ"]
    pub fn EndDragDropSource() -> c_void;

    #[link_name = "?BeginDragDropTarget@ImGui@@YA_NXZ"]
    pub fn BeginDragDropTarget() -> bool;

    /// * type: 
    /// * flags: 0
    #[link_name = "?AcceptDragDropPayload@ImGui@@YAPEBUImGuiPayload@@PEBDH@Z"]
    pub fn AcceptDragDropPayload(
        r#type: *const i8,
        flags: ImGuiDragDropFlags,
    ) -> *mut ImGuiPayload;

    #[link_name = "?EndDragDropTarget@ImGui@@YAXXZ"]
    pub fn EndDragDropTarget() -> c_void;

    #[link_name = "?GetDragDropPayload@ImGui@@YAPEBUImGuiPayload@@XZ"]
    pub fn GetDragDropPayload() -> *mut ImGuiPayload;

    /// * clip_rect_min: 
    /// * clip_rect_max: 
    /// * intersect_with_current_clip_rect: 
    #[link_name = "?PushClipRect@ImGui@@YAXAEBUImVec2@@0_N@Z"]
    pub fn PushClipRect(
        clip_rect_min: *const ImVec2,
        clip_rect_max: *const ImVec2,
        intersect_with_current_clip_rect: bool,
    ) -> c_void;

    #[link_name = "?PopClipRect@ImGui@@YAXXZ"]
    pub fn PopClipRect() -> c_void;

    #[link_name = "?SetItemDefaultFocus@ImGui@@YAXXZ"]
    pub fn SetItemDefaultFocus() -> c_void;

    /// * offset: 0
    #[link_name = "?SetKeyboardFocusHere@ImGui@@YAXH@Z"]
    pub fn SetKeyboardFocusHere(
        offset: i32,
    ) -> c_void;

    /// * flags: 0
    #[link_name = "?IsItemHovered@ImGui@@YA_NH@Z"]
    pub fn IsItemHovered(
        flags: ImGuiHoveredFlags,
    ) -> bool;

    #[link_name = "?IsItemActive@ImGui@@YA_NXZ"]
    pub fn IsItemActive() -> bool;

    #[link_name = "?IsItemFocused@ImGui@@YA_NXZ"]
    pub fn IsItemFocused() -> bool;

    /// * mouse_button: 0
    #[link_name = "?IsItemClicked@ImGui@@YA_NH@Z"]
    pub fn IsItemClicked(
        mouse_button: ImGuiMouseButton,
    ) -> bool;

    #[link_name = "?IsItemVisible@ImGui@@YA_NXZ"]
    pub fn IsItemVisible() -> bool;

    #[link_name = "?IsItemEdited@ImGui@@YA_NXZ"]
    pub fn IsItemEdited() -> bool;

    #[link_name = "?IsItemActivated@ImGui@@YA_NXZ"]
    pub fn IsItemActivated() -> bool;

    #[link_name = "?IsItemDeactivated@ImGui@@YA_NXZ"]
    pub fn IsItemDeactivated() -> bool;

    #[link_name = "?IsItemDeactivatedAfterEdit@ImGui@@YA_NXZ"]
    pub fn IsItemDeactivatedAfterEdit() -> bool;

    #[link_name = "?IsItemToggledOpen@ImGui@@YA_NXZ"]
    pub fn IsItemToggledOpen() -> bool;

    #[link_name = "?IsAnyItemHovered@ImGui@@YA_NXZ"]
    pub fn IsAnyItemHovered() -> bool;

    #[link_name = "?IsAnyItemActive@ImGui@@YA_NXZ"]
    pub fn IsAnyItemActive() -> bool;

    #[link_name = "?IsAnyItemFocused@ImGui@@YA_NXZ"]
    pub fn IsAnyItemFocused() -> bool;

    #[link_name = "?GetItemRectMin@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectMin() -> ImVec2;

    #[link_name = "?GetItemRectMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectMax() -> ImVec2;

    #[link_name = "?GetItemRectSize@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectSize() -> ImVec2;

    #[link_name = "?SetItemAllowOverlap@ImGui@@YAXXZ"]
    pub fn SetItemAllowOverlap() -> c_void;

    #[link_name = "?GetMainViewport@ImGui@@YAPEAUImGuiViewport@@XZ"]
    pub fn GetMainViewport() -> *mut ImGuiViewport;

    /// * size: 
    #[link_name = "?IsRectVisible@ImGui@@YA_NAEBUImVec2@@@Z"]
    pub fn IsRectVisible(
        size: *const ImVec2,
    ) -> bool;

    /// * rect_min: 
    /// * rect_max: 
    #[link_name = "?IsRectVisible@ImGui@@YA_NAEBUImVec2@@0@Z"]
    pub fn IsRectVisible_(
        rect_min: *const ImVec2,
        rect_max: *const ImVec2,
    ) -> bool;

    #[link_name = "?GetTime@ImGui@@YANXZ"]
    pub fn GetTime() -> f64;

    #[link_name = "?GetFrameCount@ImGui@@YAHXZ"]
    pub fn GetFrameCount() -> i32;

    #[link_name = "?GetBackgroundDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetBackgroundDrawList() -> *mut ImDrawList;

    #[link_name = "?GetForegroundDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetForegroundDrawList() -> *mut ImDrawList;

    /// * viewport: 
    #[link_name = "?GetBackgroundDrawList@ImGui@@YAPEAUImDrawList@@PEAUImGuiViewport@@@Z"]
    pub fn GetBackgroundDrawList_(
        viewport: *mut ImGuiViewport,
    ) -> *mut ImDrawList;

    /// * viewport: 
    #[link_name = "?GetForegroundDrawList@ImGui@@YAPEAUImDrawList@@PEAUImGuiViewport@@@Z"]
    pub fn GetForegroundDrawList_(
        viewport: *mut ImGuiViewport,
    ) -> *mut ImDrawList;

    #[link_name = "?GetDrawListSharedData@ImGui@@YAPEAUImDrawListSharedData@@XZ"]
    pub fn GetDrawListSharedData() -> *mut ImDrawListSharedData;

    /// * idx: 
    #[link_name = "?GetStyleColorName@ImGui@@YAPEBDH@Z"]
    pub fn GetStyleColorName(
        idx: ImGuiCol,
    ) -> *mut i8;

    /// * storage: 
    #[link_name = "?SetStateStorage@ImGui@@YAXPEAUImGuiStorage@@@Z"]
    pub fn SetStateStorage(
        storage: *mut ImGuiStorage,
    ) -> c_void;

    #[link_name = "?GetStateStorage@ImGui@@YAPEAUImGuiStorage@@XZ"]
    pub fn GetStateStorage() -> *mut ImGuiStorage;

    /// * items_count: 
    /// * items_height: 
    /// * out_items_display_start: 
    /// * out_items_display_end: 
    #[link_name = "?CalcListClipping@ImGui@@YAXHMPEAH0@Z"]
    pub fn CalcListClipping(
        items_count: i32,
        items_height: f32,
        out_items_display_start: *mut i32,
        out_items_display_end: *mut i32,
    ) -> c_void;

    /// * id: 
    /// * size: 
    /// * flags: 0
    #[link_name = "?BeginChildFrame@ImGui@@YA_NIAEBUImVec2@@H@Z"]
    pub fn BeginChildFrame(
        id: ImGuiID,
        size: *const ImVec2,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[link_name = "?EndChildFrame@ImGui@@YAXXZ"]
    pub fn EndChildFrame() -> c_void;

    /// * text: 
    /// * text_end: NULL
    /// * hide_text_after_double_hash: false
    /// * wrap_width: -1.0f
    #[link_name = "?CalcTextSize@ImGui@@YA?AUImVec2@@PEBD0_NM@Z"]
    pub fn CalcTextSize(
        text: *const i8,
        text_end: *const i8,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2;

    /// * in: 
    #[link_name = "?ColorConvertU32ToFloat4@ImGui@@YA?AUImVec4@@I@Z"]
    pub fn ColorConvertU32ToFloat4(
        r#in: ImU32,
    ) -> ImVec4;

    /// * in: 
    #[link_name = "?ColorConvertFloat4ToU32@ImGui@@YAIAEBUImVec4@@@Z"]
    pub fn ColorConvertFloat4ToU32(
        r#in: *const ImVec4,
    ) -> ImU32;

    /// * r: 
    /// * g: 
    /// * b: 
    /// * out_h: 
    /// * out_s: 
    /// * out_v: 
    #[link_name = "?ColorConvertRGBtoHSV@ImGui@@YAXMMMAEAM00@Z"]
    pub fn ColorConvertRGBtoHSV(
        r: f32,
        g: f32,
        b: f32,
        out_h: *mut f32,
        out_s: *mut f32,
        out_v: *mut f32,
    ) -> c_void;

    /// * h: 
    /// * s: 
    /// * v: 
    /// * out_r: 
    /// * out_g: 
    /// * out_b: 
    #[link_name = "?ColorConvertHSVtoRGB@ImGui@@YAXMMMAEAM00@Z"]
    pub fn ColorConvertHSVtoRGB(
        h: f32,
        s: f32,
        v: f32,
        out_r: *mut f32,
        out_g: *mut f32,
        out_b: *mut f32,
    ) -> c_void;

    /// * imgui_key: 
    #[link_name = "?GetKeyIndex@ImGui@@YAHH@Z"]
    pub fn GetKeyIndex(
        imgui_key: ImGuiKey,
    ) -> i32;

    /// * user_key_index: 
    #[link_name = "?IsKeyDown@ImGui@@YA_NH@Z"]
    pub fn IsKeyDown(
        user_key_index: i32,
    ) -> bool;

    /// * user_key_index: 
    /// * repeat: true
    #[link_name = "?IsKeyPressed@ImGui@@YA_NH_N@Z"]
    pub fn IsKeyPressed(
        user_key_index: i32,
        repeat: bool,
    ) -> bool;

    /// * user_key_index: 
    #[link_name = "?IsKeyReleased@ImGui@@YA_NH@Z"]
    pub fn IsKeyReleased(
        user_key_index: i32,
    ) -> bool;

    /// * key_index: 
    /// * repeat_delay: 
    /// * rate: 
    #[link_name = "?GetKeyPressedAmount@ImGui@@YAHHMM@Z"]
    pub fn GetKeyPressedAmount(
        key_index: i32,
        repeat_delay: f32,
        rate: f32,
    ) -> i32;

    /// * want_capture_keyboard_value: true
    #[link_name = "?CaptureKeyboardFromApp@ImGui@@YAX_N@Z"]
    pub fn CaptureKeyboardFromApp(
        want_capture_keyboard_value: bool,
    ) -> c_void;

    /// * button: 
    #[link_name = "?IsMouseDown@ImGui@@YA_NH@Z"]
    pub fn IsMouseDown(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * button: 
    /// * repeat: false
    #[link_name = "?IsMouseClicked@ImGui@@YA_NH_N@Z"]
    pub fn IsMouseClicked(
        button: ImGuiMouseButton,
        repeat: bool,
    ) -> bool;

    /// * button: 
    #[link_name = "?IsMouseReleased@ImGui@@YA_NH@Z"]
    pub fn IsMouseReleased(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * button: 
    #[link_name = "?IsMouseDoubleClicked@ImGui@@YA_NH@Z"]
    pub fn IsMouseDoubleClicked(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * r_min: 
    /// * r_max: 
    /// * clip: true
    #[link_name = "?IsMouseHoveringRect@ImGui@@YA_NAEBUImVec2@@0_N@Z"]
    pub fn IsMouseHoveringRect(
        r_min: *const ImVec2,
        r_max: *const ImVec2,
        clip: bool,
    ) -> bool;

    /// * mouse_pos: NULL
    #[link_name = "?IsMousePosValid@ImGui@@YA_NPEBUImVec2@@@Z"]
    pub fn IsMousePosValid(
        mouse_pos: *const ImVec2,
    ) -> bool;

    #[link_name = "?IsAnyMouseDown@ImGui@@YA_NXZ"]
    pub fn IsAnyMouseDown() -> bool;

    #[link_name = "?GetMousePos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetMousePos() -> ImVec2;

    #[link_name = "?GetMousePosOnOpeningCurrentPopup@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetMousePosOnOpeningCurrentPopup() -> ImVec2;

    /// * button: 
    /// * lock_threshold: -1.0f
    #[link_name = "?IsMouseDragging@ImGui@@YA_NHM@Z"]
    pub fn IsMouseDragging(
        button: ImGuiMouseButton,
        lock_threshold: f32,
    ) -> bool;

    /// * button: 0
    /// * lock_threshold: -1.0f
    #[link_name = "?GetMouseDragDelta@ImGui@@YA?AUImVec2@@HM@Z"]
    pub fn GetMouseDragDelta(
        button: ImGuiMouseButton,
        lock_threshold: f32,
    ) -> ImVec2;

    /// * button: 0
    #[link_name = "?ResetMouseDragDelta@ImGui@@YAXH@Z"]
    pub fn ResetMouseDragDelta(
        button: ImGuiMouseButton,
    ) -> c_void;

    #[link_name = "?GetMouseCursor@ImGui@@YAHXZ"]
    pub fn GetMouseCursor() -> ImGuiMouseCursor;

    /// * cursor_type: 
    #[link_name = "?SetMouseCursor@ImGui@@YAXH@Z"]
    pub fn SetMouseCursor(
        cursor_type: ImGuiMouseCursor,
    ) -> c_void;

    /// * want_capture_mouse_value: true
    #[link_name = "?CaptureMouseFromApp@ImGui@@YAX_N@Z"]
    pub fn CaptureMouseFromApp(
        want_capture_mouse_value: bool,
    ) -> c_void;

    #[link_name = "?GetClipboardText@ImGui@@YAPEBDXZ"]
    pub fn GetClipboardText() -> *mut i8;

    /// * text: 
    #[link_name = "?SetClipboardText@ImGui@@YAXPEBD@Z"]
    pub fn SetClipboardText(
        text: *const i8,
    ) -> c_void;

    /// * ini_filename: 
    #[link_name = "?LoadIniSettingsFromDisk@ImGui@@YAXPEBD@Z"]
    pub fn LoadIniSettingsFromDisk(
        ini_filename: *const i8,
    ) -> c_void;

    /// * ini_data: 
    /// * ini_size: 0
    #[link_name = "?LoadIniSettingsFromMemory@ImGui@@YAXPEBD_K@Z"]
    pub fn LoadIniSettingsFromMemory(
        ini_data: *const i8,
        ini_size: usize,
    ) -> c_void;

    /// * ini_filename: 
    #[link_name = "?SaveIniSettingsToDisk@ImGui@@YAXPEBD@Z"]
    pub fn SaveIniSettingsToDisk(
        ini_filename: *const i8,
    ) -> c_void;

    /// * out_ini_size: NULL
    #[link_name = "?SaveIniSettingsToMemory@ImGui@@YAPEBDPEA_K@Z"]
    pub fn SaveIniSettingsToMemory(
        out_ini_size: *mut usize,
    ) -> *mut i8;

    /// * version_str: 
    /// * sz_io: 
    /// * sz_style: 
    /// * sz_vec2: 
    /// * sz_vec4: 
    /// * sz_drawvert: 
    /// * sz_drawidx: 
    #[link_name = "?DebugCheckVersionAndDataLayout@ImGui@@YA_NPEBD_K11111@Z"]
    pub fn DebugCheckVersionAndDataLayout(
        version_str: *const i8,
        sz_io: usize,
        sz_style: usize,
        sz_vec2: usize,
        sz_vec4: usize,
        sz_drawvert: usize,
        sz_drawidx: usize,
    ) -> bool;

    /// * alloc_func: 
    /// * free_func: 
    /// * user_data: NULL
    #[link_name = "?SetAllocatorFunctions@ImGui@@YAXP6APEAX_KPEAX@ZP6AX11@Z1@Z"]
    pub fn SetAllocatorFunctions(
        alloc_func: extern fn(usize,*mut c_void,) -> *mut c_void,
        free_func: extern fn(*mut c_void,*mut c_void,) -> c_void,
        user_data: *mut c_void,
    ) -> c_void;

    /// * p_alloc_func: 
    /// * p_free_func: 
    /// * p_user_data: 
    #[link_name = "?GetAllocatorFunctions@ImGui@@YAXPEAP6APEAX_KPEAX@ZPEAP6AX11@ZPEAPEAX@Z"]
    pub fn GetAllocatorFunctions(
        p_alloc_func: *mut extern fn(usize,*mut c_void,) -> *mut c_void,
        p_free_func: *mut extern fn(*mut c_void,*mut c_void,) -> c_void,
        p_user_data: *mut *mut c_void,
    ) -> c_void;

    /// * size: 
    #[link_name = "?MemAlloc@ImGui@@YAPEAX_K@Z"]
    pub fn MemAlloc(
        size: usize,
    ) -> *mut c_void;

    /// * ptr: 
    #[link_name = "?MemFree@ImGui@@YAXPEAX@Z"]
    pub fn MemFree(
        ptr: *mut c_void,
    ) -> c_void;

    #[link_name = "?GetPlatformIO@ImGui@@YAAEAUImGuiPlatformIO@@XZ"]
    pub fn GetPlatformIO() -> *mut ImGuiPlatformIO;

    #[link_name = "?UpdatePlatformWindows@ImGui@@YAXXZ"]
    pub fn UpdatePlatformWindows() -> c_void;

    /// * platform_render_arg: NULL
    /// * renderer_render_arg: NULL
    #[link_name = "?RenderPlatformWindowsDefault@ImGui@@YAXPEAX0@Z"]
    pub fn RenderPlatformWindowsDefault(
        platform_render_arg: *mut c_void,
        renderer_render_arg: *mut c_void,
    ) -> c_void;

    #[link_name = "?DestroyPlatformWindows@ImGui@@YAXXZ"]
    pub fn DestroyPlatformWindows() -> c_void;

    /// * id: 
    #[link_name = "?FindViewportByID@ImGui@@YAPEAUImGuiViewport@@I@Z"]
    pub fn FindViewportByID(
        id: ImGuiID,
    ) -> *mut ImGuiViewport;

    /// * platform_handle: 
    #[link_name = "?FindViewportByPlatformHandle@ImGui@@YAPEAUImGuiViewport@@PEAX@Z"]
    pub fn FindViewportByPlatformHandle(
        platform_handle: *mut c_void,
    ) -> *mut ImGuiViewport;

    // /// * : 
    // /// * : 
    // /// * ptr: 
    // #[link_name = "??2@YAPEAX_KUImNewWrapper@@PEAX@Z"]
    // pub fn operator new(
    //     : usize,
    //     : ImNewWrapper,
    //     ptr: *mut c_void,
    // ) -> *mut c_void;

    // /// * : 
    // /// * : 
    // /// * : 
    // #[link_name = "??3@YAXPEAXUImNewWrapper@@0@Z"]
    // pub fn operator delete(
    //     : *mut c_void,
    //     : ImNewWrapper,
    //     : *mut c_void,
    // ) -> c_void;

    /// * user_data: 
    #[link_name = ""]
    pub fn GetClipboardTextFn(
        user_data: *mut c_void,
    ) -> *mut i8;

    /// * user_data: 
    /// * text: 
    #[link_name = ""]
    pub fn SetClipboardTextFn(
        user_data: *mut c_void,
        text: *const i8,
    ) -> c_void;

    /// * parent_list: 
    /// * cmd: 
    #[link_name = ""]
    pub fn ImDrawCallback(
        parent_list: *const ImDrawList,
        cmd: *const ImDrawCmd,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_CreateWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_DestroyWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_ShowWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    /// * pos: 
    #[link_name = ""]
    pub fn Platform_SetWindowPos(
        vp: *mut ImGuiViewport,
        pos: ImVec2,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_GetWindowPos(
        vp: *mut ImGuiViewport,
    ) -> ImVec2;

    /// * vp: 
    /// * size: 
    #[link_name = ""]
    pub fn Platform_SetWindowSize(
        vp: *mut ImGuiViewport,
        size: ImVec2,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_GetWindowSize(
        vp: *mut ImGuiViewport,
    ) -> ImVec2;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_SetWindowFocus(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_GetWindowFocus(
        vp: *mut ImGuiViewport,
    ) -> bool;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_GetWindowMinimized(
        vp: *mut ImGuiViewport,
    ) -> bool;

    /// * vp: 
    /// * str: 
    #[link_name = ""]
    pub fn Platform_SetWindowTitle(
        vp: *mut ImGuiViewport,
        str: *const i8,
    ) -> c_void;

    /// * vp: 
    /// * alpha: 
    #[link_name = ""]
    pub fn Platform_SetWindowAlpha(
        vp: *mut ImGuiViewport,
        alpha: f32,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_UpdateWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    /// * render_arg: 
    #[link_name = ""]
    pub fn Platform_RenderWindow(
        vp: *mut ImGuiViewport,
        render_arg: *mut c_void,
    ) -> c_void;

    /// * vp: 
    /// * render_arg: 
    #[link_name = ""]
    pub fn Platform_SwapBuffers(
        vp: *mut ImGuiViewport,
        render_arg: *mut c_void,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_GetWindowDpiScale(
        vp: *mut ImGuiViewport,
    ) -> f32;

    /// * vp: 
    #[link_name = ""]
    pub fn Platform_OnChangedViewport(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    /// * pos: 
    #[link_name = ""]
    pub fn Platform_SetImeInputPos(
        vp: *mut ImGuiViewport,
        pos: ImVec2,
    ) -> c_void;

    /// * vp: 
    /// * vk_inst: 
    /// * vk_allocators: 
    /// * out_vk_surface: 
    #[link_name = ""]
    pub fn Platform_CreateVkSurface(
        vp: *mut ImGuiViewport,
        vk_inst: ImU64,
        vk_allocators: *const c_void,
        out_vk_surface: *mut ImU64,
    ) -> i32;

    /// * vp: 
    #[link_name = ""]
    pub fn Renderer_CreateWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    #[link_name = ""]
    pub fn Renderer_DestroyWindow(
        vp: *mut ImGuiViewport,
    ) -> c_void;

    /// * vp: 
    /// * size: 
    #[link_name = ""]
    pub fn Renderer_SetWindowSize(
        vp: *mut ImGuiViewport,
        size: ImVec2,
    ) -> c_void;

    /// * vp: 
    /// * render_arg: 
    #[link_name = ""]
    pub fn Renderer_RenderWindow(
        vp: *mut ImGuiViewport,
        render_arg: *mut c_void,
    ) -> c_void;

    /// * vp: 
    /// * render_arg: 
    #[link_name = ""]
    pub fn Renderer_SwapBuffers(
        vp: *mut ImGuiViewport,
        render_arg: *mut c_void,
    ) -> c_void;

    /// * label: 
    /// * items_count: 
    /// * height_in_items: -1
    #[link_name = "?ListBoxHeader@ImGui@@YA_NPEBDHH@Z"]
    pub fn ListBoxHeader(
        label: *const i8,
        items_count: i32,
        height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * size: ImVec2(0,0)
    #[link_name = "?ListBoxHeader@ImGui@@YA_NPEBDAEBUImVec2@@@Z"]
    pub fn ListBoxHeader_(
        label: *const i8,
        size: *const ImVec2,
    ) -> bool;

    #[link_name = "?ListBoxFooter@ImGui@@YAXXZ"]
    pub fn ListBoxFooter() -> c_void;

    /// * str_id: NULL
    /// * mb: 1
    #[link_name = "?OpenPopupContextItem@ImGui@@YAXPEBDH@Z"]
    pub fn OpenPopupContextItem(
        str_id: *const i8,
        mb: ImGuiMouseButton,
    ) -> c_void;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * v_speed: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragScalar@ImGui@@YA_NPEBDHPEAXMPEBX20M@Z"]
    pub fn DragScalar_(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        v_speed: f32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * components: 
    /// * v_speed: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragScalarN@ImGui@@YA_NPEBDHPEAXHMPEBX20M@Z"]
    pub fn DragScalarN_(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        components: i32,
        v_speed: f32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragFloat@ImGui@@YA_NPEBDPEAMMMM0M@Z"]
    pub fn DragFloat_(
        label: *const i8,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragFloat2@ImGui@@YA_NPEBDQEAMMMM0M@Z"]
    pub fn DragFloat2_(
        label: *const i8,
        v: [f32; 2],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragFloat3@ImGui@@YA_NPEBDQEAMMMM0M@Z"]
    pub fn DragFloat3_(
        label: *const i8,
        v: [f32; 3],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?DragFloat4@ImGui@@YA_NPEBDQEAMMMM0M@Z"]
    pub fn DragFloat4_(
        label: *const i8,
        v: [f32; 4],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderScalar@ImGui@@YA_NPEBDHPEAXPEBX20M@Z"]
    pub fn SliderScalar_(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * components: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderScalarN@ImGui@@YA_NPEBDHPEAXHPEBX20M@Z"]
    pub fn SliderScalarN_(
        label: *const i8,
        data_type: ImGuiDataType,
        p_data: *mut c_void,
        components: i32,
        p_min: *const c_void,
        p_max: *const c_void,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderFloat@ImGui@@YA_NPEBDPEAMMM0M@Z"]
    pub fn SliderFloat_(
        label: *const i8,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderFloat2@ImGui@@YA_NPEBDQEAMMM0M@Z"]
    pub fn SliderFloat2_(
        label: *const i8,
        v: [f32; 2],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderFloat3@ImGui@@YA_NPEBDQEAMMM0M@Z"]
    pub fn SliderFloat3_(
        label: *const i8,
        v: [f32; 3],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_min: 
    /// * v_max: 
    /// * format: 
    /// * power: 
    #[link_name = "?SliderFloat4@ImGui@@YA_NPEBDQEAMMM0M@Z"]
    pub fn SliderFloat4_(
        label: *const i8,
        v: [f32; 4],
        v_min: f32,
        v_max: f32,
        format: *const i8,
        power: f32,
    ) -> bool;

    /// * str_id: 
    /// * mb: 
    /// * over_items: 
    #[link_name = "?BeginPopupContextWindow@ImGui@@YA_NPEBDH_N@Z"]
    pub fn BeginPopupContextWindow_(
        str_id: *const i8,
        mb: ImGuiMouseButton,
        over_items: bool,
    ) -> bool;

    #[link_name = "?TreeAdvanceToLabelPos@ImGui@@YAXXZ"]
    pub fn TreeAdvanceToLabelPos() -> c_void;

    /// * open: 
    /// * cond: 0
    #[link_name = "?SetNextTreeNodeOpen@ImGui@@YAX_NH@Z"]
    pub fn SetNextTreeNodeOpen(
        open: bool,
        cond: ImGuiCond,
    ) -> c_void;

    #[link_name = "?GetContentRegionAvailWidth@ImGui@@YAMXZ"]
    pub fn GetContentRegionAvailWidth() -> f32;

    #[link_name = "?GetOverlayDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetOverlayDrawList() -> *mut ImDrawList;
}
