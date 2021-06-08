// this is generated.
use std::ffi::c_void;
extern crate va_list;

type ImDrawListSharedData = c_void;
type ImFontBuilderIO = c_void;
type ImGuiContext = c_void;
type ImGuiCol = i32;
type ImGuiCond = i32;
type ImGuiDataType = i32;
type ImGuiDir = i32;
type ImGuiKey = i32;
type ImGuiNavInput = i32;
type ImGuiMouseButton = i32;
type ImGuiMouseCursor = i32;
type ImGuiSortDirection = i32;
type ImGuiStyleVar = i32;
type ImGuiTableBgTarget = i32;
type ImDrawFlags = i32;
type ImDrawListFlags = i32;
type ImFontAtlasFlags = i32;
type ImGuiBackendFlags = i32;
type ImGuiButtonFlags = i32;
type ImGuiColorEditFlags = i32;
type ImGuiConfigFlags = i32;
type ImGuiComboFlags = i32;
type ImGuiDockNodeFlags = i32;
type ImGuiDragDropFlags = i32;
type ImGuiFocusedFlags = i32;
type ImGuiHoveredFlags = i32;
type ImGuiInputTextFlags = i32;
type ImGuiKeyModFlags = i32;
type ImGuiPopupFlags = i32;
type ImGuiSelectableFlags = i32;
type ImGuiSliderFlags = i32;
type ImGuiTabBarFlags = i32;
type ImGuiTabItemFlags = i32;
type ImGuiTableFlags = i32;
type ImGuiTableColumnFlags = i32;
type ImGuiTableRowFlags = i32;
type ImGuiTreeNodeFlags = i32;
type ImGuiViewportFlags = i32;
type ImGuiWindowFlags = i32;
type ImTextureID = *mut c_void;
type ImGuiID = u32;
type ImWchar16 = u16;
type ImWchar32 = u32;
type ImWchar = ImWchar16;
type ImS8 = i8;
type ImU8 = u8;
type ImS16 = i16;
type ImU16 = u16;
type ImS32 = i32;
type ImU32 = u32;
type ImS64 = i64;
type ImU64 = u64;

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImVec2 {
    x: f32,
    y: f32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiWindowFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiInputTextFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTreeNodeFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiPopupFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiSelectableFlags_ {
    None = 0,
    DontClosePopups = 0x1,
    SpanAllColumns = 0x2,
    AllowDoubleClick = 0x4,
    Disabled = 0x8,
    AllowItemOverlap = 0x10,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiComboFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTabBarFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTabItemFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTableFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTableColumnFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTableRowFlags_ {
    None = 0,
    Headers = 0x1,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiTableBgTarget_ {
    None = 0,
    RowBg0 = 0x1,
    RowBg1 = 0x2,
    CellBg = 0x3,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiFocusedFlags_ {
    None = 0,
    ChildWindows = 0x1,
    RootWindow = 0x2,
    AnyWindow = 0x4,
    RootAndChildWindows = 0x3,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiHoveredFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiDockNodeFlags_ {
    None = 0,
    KeepAliveOnly = 0x1,
    NoDockingInCentralNode = 0x4,
    PassthruCentralNode = 0x8,
    NoSplit = 0x10,
    NoResize = 0x20,
    AutoHideTabBar = 0x40,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiDragDropFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiDataType_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiDir_ {
    None = -1,
    Left = 0,
    Right = 0x1,
    Up = 0x2,
    Down = 0x3,
    COUNT = 0x4,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiSortDirection_ {
    None = 0,
    Ascending = 0x1,
    Descending = 0x2,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiKey_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiKeyModFlags_ {
    None = 0,
    Ctrl = 0x1,
    Shift = 0x2,
    Alt = 0x4,
    Super = 0x8,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiNavInput_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiConfigFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiBackendFlags_ {
    None = 0,
    HasGamepad = 0x1,
    HasMouseCursors = 0x2,
    HasSetMousePos = 0x4,
    RendererHasVtxOffset = 0x8,
    PlatformHasViewports = 0x400,
    HasMouseHoveredViewport = 0x800,
    RendererHasViewports = 0x1000,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiCol_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiStyleVar_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiButtonFlags_ {
    None = 0,
    MouseButtonLeft = 0x1,
    MouseButtonRight = 0x2,
    MouseButtonMiddle = 0x4,
    MouseButtonMask_ = 0x7,
    // MouseButtonDefault_ = 0x1,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiColorEditFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiSliderFlags_ {
    None = 0,
    AlwaysClamp = 0x10,
    Logarithmic = 0x20,
    NoRoundToFormat = 0x40,
    NoInput = 0x80,
    InvalidMask_ = 0x7000000f,
    // ClampOnInput = 0x10,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiMouseButton_ {
    Left = 0,
    Right = 0x1,
    Middle = 0x2,
    COUNT = 0x5,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiMouseCursor_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiCond_ {
    None = 0,
    Always = 0x1,
    Once = 0x2,
    FirstUseEver = 0x4,
    Appearing = 0x8,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiStyle {
    Alpha: f32,
    WindowPadding: ImVec2,
    WindowRounding: f32,
    WindowBorderSize: f32,
    WindowMinSize: ImVec2,
    WindowTitleAlign: ImVec2,
    WindowMenuButtonPosition: ImGuiDir,
    ChildRounding: f32,
    ChildBorderSize: f32,
    PopupRounding: f32,
    PopupBorderSize: f32,
    FramePadding: ImVec2,
    FrameRounding: f32,
    FrameBorderSize: f32,
    ItemSpacing: ImVec2,
    ItemInnerSpacing: ImVec2,
    CellPadding: ImVec2,
    TouchExtraPadding: ImVec2,
    IndentSpacing: f32,
    ColumnsMinSpacing: f32,
    ScrollbarSize: f32,
    ScrollbarRounding: f32,
    GrabMinSize: f32,
    GrabRounding: f32,
    LogSliderDeadzone: f32,
    TabRounding: f32,
    TabBorderSize: f32,
    TabMinWidthForCloseButton: f32,
    ColorButtonPosition: ImGuiDir,
    ButtonTextAlign: ImVec2,
    SelectableTextAlign: ImVec2,
    DisplayWindowPadding: ImVec2,
    DisplaySafeAreaPadding: ImVec2,
    MouseCursorScale: f32,
    AntiAliasedLines: bool,
    AntiAliasedLinesUseTex: bool,
    AntiAliasedFill: bool,
    CurveTessellationTol: f32,
    CircleTessellationMaxError: f32,
    Colors: [ImVec4; 55],
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiIO {
    ConfigFlags: ImGuiConfigFlags,
    BackendFlags: ImGuiBackendFlags,
    DisplaySize: ImVec2,
    DeltaTime: f32,
    IniSavingRate: f32,
    IniFilename: *mut i8,
    LogFilename: *mut i8,
    MouseDoubleClickTime: f32,
    MouseDoubleClickMaxDist: f32,
    MouseDragThreshold: f32,
    KeyMap: [i32; 22],
    KeyRepeatDelay: f32,
    KeyRepeatRate: f32,
    UserData: *mut c_void,
    Fonts: *mut ImFontAtlas,
    FontGlobalScale: f32,
    FontAllowUserScaling: bool,
    FontDefault: *mut ImFont,
    DisplayFramebufferScale: ImVec2,
    ConfigDockingNoSplit: bool,
    ConfigDockingAlwaysTabBar: bool,
    ConfigDockingTransparentPayload: bool,
    ConfigViewportsNoAutoMerge: bool,
    ConfigViewportsNoTaskBarIcon: bool,
    ConfigViewportsNoDecoration: bool,
    ConfigViewportsNoDefaultParent: bool,
    MouseDrawCursor: bool,
    ConfigMacOSXBehaviors: bool,
    ConfigInputTextCursorBlink: bool,
    ConfigDragClickToInputText: bool,
    ConfigWindowsResizeFromEdges: bool,
    ConfigWindowsMoveFromTitleBarOnly: bool,
    ConfigMemoryCompactTimer: f32,
    BackendPlatformName: *mut i8,
    BackendRendererName: *mut i8,
    BackendPlatformUserData: *mut c_void,
    BackendRendererUserData: *mut c_void,
    BackendLanguageUserData: *mut c_void,
    GetClipboardTextFn: *mut extern fn(*mut c_void,) -> *mut i8,
    SetClipboardTextFn: *mut extern fn(*mut c_void,*mut i8,) -> c_void,
    ClipboardUserData: *mut c_void,
    MousePos: ImVec2,
    MouseDown: [bool; 5],
    MouseWheel: f32,
    MouseWheelH: f32,
    MouseHoveredViewport: ImGuiID,
    KeyCtrl: bool,
    KeyShift: bool,
    KeyAlt: bool,
    KeySuper: bool,
    KeysDown: [bool; 512],
    NavInputs: [f32; 21],
    WantCaptureMouse: bool,
    WantCaptureKeyboard: bool,
    WantTextInput: bool,
    WantSetMousePos: bool,
    WantSaveIniSettings: bool,
    NavActive: bool,
    NavVisible: bool,
    Framerate: f32,
    MetricsRenderVertices: i32,
    MetricsRenderIndices: i32,
    MetricsRenderWindows: i32,
    MetricsActiveWindows: i32,
    MetricsActiveAllocations: i32,
    MouseDelta: ImVec2,
    KeyMods: ImGuiKeyModFlags,
    MousePosPrev: ImVec2,
    MouseClickedPos: [ImVec2; 5],
    MouseClickedTime: [f64; 5],
    MouseClicked: [bool; 5],
    MouseDoubleClicked: [bool; 5],
    MouseReleased: [bool; 5],
    MouseDownOwned: [bool; 5],
    MouseDownWasDoubleClick: [bool; 5],
    MouseDownDuration: [f32; 5],
    MouseDownDurationPrev: [f32; 5],
    MouseDragMaxDistanceAbs: [ImVec2; 5],
    MouseDragMaxDistanceSqr: [f32; 5],
    KeysDownDuration: [f32; 512],
    KeysDownDurationPrev: [f32; 512],
    NavInputsDownDuration: [f32; 21],
    NavInputsDownDurationPrev: [f32; 21],
    PenPressure: f32,
    InputQueueSurrogate: ImWchar16,
    InputQueueCharacters: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiInputTextCallbackData {
    EventFlag: ImGuiInputTextFlags,
    Flags: ImGuiInputTextFlags,
    UserData: *mut c_void,
    EventChar: ImWchar,
    EventKey: ImGuiKey,
    Buf: *mut i8,
    BufTextLen: i32,
    BufSize: i32,
    BufDirty: bool,
    CursorPos: i32,
    SelectionStart: i32,
    SelectionEnd: i32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiSizeCallbackData {
    UserData: *mut c_void,
    Pos: ImVec2,
    CurrentSize: ImVec2,
    DesiredSize: ImVec2,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiWindowClass {
    ClassId: ImGuiID,
    ParentViewportId: ImGuiID,
    ViewportFlagsOverrideSet: ImGuiViewportFlags,
    ViewportFlagsOverrideClear: ImGuiViewportFlags,
    TabItemFlagsOverrideSet: ImGuiTabItemFlags,
    DockNodeFlagsOverrideSet: ImGuiDockNodeFlags,
    DockNodeFlagsOverrideClear: ImGuiDockNodeFlags,
    DockingAlwaysTabBar: bool,
    DockingAllowUnclassed: bool,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiPayload {
    Data: *mut c_void,
    DataSize: i32,
    SourceId: ImGuiID,
    SourceParentId: ImGuiID,
    DataFrameCount: i32,
    DataType: [i8; 33],
    Preview: bool,
    Delivery: bool,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiTableColumnSortSpecs {
    ColumnUserID: ImGuiID,
    ColumnIndex: ImS16,
    SortOrder: ImS16,
    SortDirection: ImGuiSortDirection,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiTableSortSpecs {
    Specs: *mut ImGuiTableColumnSortSpecs,
    SpecsCount: i32,
    SpecsDirty: bool,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiOnceUponAFrame {
    RefFrame: i32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiTextFilter {
    InputBuf: [i8; 256],
    Filters: *mut c_void,
    CountGrep: i32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiTextBuffer {
    Buf: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiStorage {
    Data: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiListClipper {
    DisplayStart: i32,
    DisplayEnd: i32,
    ItemsCount: i32,
    StepNo: i32,
    ItemsFrozen: i32,
    ItemsHeight: f32,
    StartPosY: f32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImColor {
    Value: ImVec4,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawCmd {
    ClipRect: ImVec4,
    TextureId: ImTextureID,
    VtxOffset: u32,
    IdxOffset: u32,
    ElemCount: u32,
    UserCallback: extern fn(*mut ImDrawList,*mut ImDrawCmd,) -> c_void,
    UserCallbackData: *mut c_void,
}
type ImDrawIdx = u16;

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawVert {
    pos: ImVec2,
    uv: ImVec2,
    col: ImU32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawCmdHeader {
    ClipRect: ImVec4,
    TextureId: ImTextureID,
    VtxOffset: u32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawChannel {
    _CmdBuffer: *mut c_void,
    _IdxBuffer: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawListSplitter {
    _Current: i32,
    _Count: i32,
    _Channels: *mut c_void,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImDrawFlags_ {
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

#[allow(non_snake_case)]
#[repr(i32)]
enum ImDrawListFlags_ {
    None = 0,
    AntiAliasedLines = 0x1,
    AntiAliasedLinesUseTex = 0x2,
    AntiAliasedFill = 0x4,
    AllowVtxOffset = 0x8,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawList {
    CmdBuffer: *mut c_void,
    IdxBuffer: *mut c_void,
    VtxBuffer: *mut c_void,
    Flags: ImDrawListFlags,
    _VtxCurrentIdx: u32,
    _Data: *mut ImDrawListSharedData,
    _OwnerName: *mut i8,
    _VtxWritePtr: *mut ImDrawVert,
    _IdxWritePtr: *mut ImDrawIdx,
    _ClipRectStack: *mut c_void,
    _TextureIdStack: *mut c_void,
    _Path: *mut c_void,
    _CmdHeader: ImDrawCmdHeader,
    _Splitter: ImDrawListSplitter,
    _FringeScale: f32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImDrawData {
    Valid: bool,
    CmdListsCount: i32,
    TotalIdxCount: i32,
    TotalVtxCount: i32,
    CmdLists: *mut *mut ImDrawList,
    DisplayPos: ImVec2,
    DisplaySize: ImVec2,
    FramebufferScale: ImVec2,
    OwnerViewport: *mut ImGuiViewport,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFontConfig {
    FontData: *mut c_void,
    FontDataSize: i32,
    FontDataOwnedByAtlas: bool,
    FontNo: i32,
    SizePixels: f32,
    OversampleH: i32,
    OversampleV: i32,
    PixelSnapH: bool,
    GlyphExtraSpacing: ImVec2,
    GlyphOffset: ImVec2,
    GlyphRanges: *mut ImWchar,
    GlyphMinAdvanceX: f32,
    GlyphMaxAdvanceX: f32,
    MergeMode: bool,
    FontBuilderFlags: u32,
    RasterizerMultiply: f32,
    EllipsisChar: ImWchar,
    Name: [i8; 40],
    DstFont: *mut ImFont,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFontGlyph {
    Colored: u32,
    Visible: u32,
    Codepoint: u32,
    AdvanceX: f32,
    X0: f32,
    Y0: f32,
    X1: f32,
    Y1: f32,
    U0: f32,
    V0: f32,
    U1: f32,
    V1: f32,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFontGlyphRangesBuilder {
    UsedChars: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFontAtlasCustomRect {
    Width: u16,
    Height: u16,
    X: u16,
    Y: u16,
    GlyphID: u32,
    GlyphAdvanceX: f32,
    GlyphOffset: ImVec2,
    Font: *mut ImFont,
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImFontAtlasFlags_ {
    None = 0,
    NoPowerOfTwoHeight = 0x1,
    NoMouseCursors = 0x2,
    NoBakedLines = 0x4,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFontAtlas {
    Flags: ImFontAtlasFlags,
    TexID: ImTextureID,
    TexDesiredWidth: i32,
    TexGlyphPadding: i32,
    Locked: bool,
    TexPixelsUseColors: bool,
    TexPixelsAlpha8: *mut u8,
    TexPixelsRGBA32: *mut u32,
    TexWidth: i32,
    TexHeight: i32,
    TexUvScale: ImVec2,
    TexUvWhitePixel: ImVec2,
    Fonts: *mut c_void,
    CustomRects: *mut c_void,
    ConfigData: *mut c_void,
    TexUvLines: [ImVec4; 64],
    FontBuilderIO: *mut ImFontBuilderIO,
    FontBuilderFlags: u32,
    PackIdMouseCursors: i32,
    PackIdLines: i32,
}
type CustomRect = ImFontAtlasCustomRect;
type GlyphRangesBuilder = ImFontGlyphRangesBuilder;

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImFont {
    IndexAdvanceX: *mut c_void,
    FallbackAdvanceX: f32,
    FontSize: f32,
    IndexLookup: *mut c_void,
    Glyphs: *mut c_void,
    FallbackGlyph: *mut ImFontGlyph,
    ContainerAtlas: *mut ImFontAtlas,
    ConfigData: *mut ImFontConfig,
    ConfigDataCount: i16,
    FallbackChar: ImWchar,
    EllipsisChar: ImWchar,
    DirtyLookupTables: bool,
    Scale: f32,
    Ascent: f32,
    Descent: f32,
    MetricsTotalSurface: i32,
    Used4kPagesMap: [ImU8; 2],
}

#[allow(non_snake_case)]
#[repr(i32)]
enum ImGuiViewportFlags_ {
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

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiViewport {
    ID: ImGuiID,
    Flags: ImGuiViewportFlags,
    Pos: ImVec2,
    Size: ImVec2,
    WorkPos: ImVec2,
    WorkSize: ImVec2,
    DpiScale: f32,
    ParentViewportId: ImGuiID,
    DrawData: *mut ImDrawData,
    RendererUserData: *mut c_void,
    PlatformUserData: *mut c_void,
    PlatformHandle: *mut c_void,
    PlatformHandleRaw: *mut c_void,
    PlatformRequestMove: bool,
    PlatformRequestResize: bool,
    PlatformRequestClose: bool,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiPlatformIO {
    Platform_CreateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_DestroyWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_ShowWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_SetWindowPos: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    Platform_GetWindowPos: *mut extern fn(*mut ImGuiViewport,) -> ImVec2,
    Platform_SetWindowSize: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    Platform_GetWindowSize: *mut extern fn(*mut ImGuiViewport,) -> ImVec2,
    Platform_SetWindowFocus: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_GetWindowFocus: *mut extern fn(*mut ImGuiViewport,) -> bool,
    Platform_GetWindowMinimized: *mut extern fn(*mut ImGuiViewport,) -> bool,
    Platform_SetWindowTitle: *mut extern fn(*mut ImGuiViewport,*mut i8,) -> c_void,
    Platform_SetWindowAlpha: *mut extern fn(*mut ImGuiViewport,f32,) -> c_void,
    Platform_UpdateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_RenderWindow: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    Platform_SwapBuffers: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    Platform_GetWindowDpiScale: *mut extern fn(*mut ImGuiViewport,) -> f32,
    Platform_OnChangedViewport: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Platform_SetImeInputPos: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    Platform_CreateVkSurface: *mut extern fn(*mut ImGuiViewport,ImU64,*mut c_void,*mut ImU64,) -> i32,
    Renderer_CreateWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Renderer_DestroyWindow: *mut extern fn(*mut ImGuiViewport,) -> c_void,
    Renderer_SetWindowSize: *mut extern fn(*mut ImGuiViewport,ImVec2,) -> c_void,
    Renderer_RenderWindow: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    Renderer_SwapBuffers: *mut extern fn(*mut ImGuiViewport,*mut c_void,) -> c_void,
    Monitors: *mut c_void,
    Viewports: *mut c_void,
}

#[allow(non_snake_case)]        
#[repr(C)]
pub struct ImGuiPlatformMonitor {
    MainPos: ImVec2,
    MainSize: ImVec2,
    WorkPos: ImVec2,
    WorkSize: ImVec2,
    DpiScale: f32,
}
type ImDrawCornerFlags = ImDrawFlags;

#[allow(non_snake_case)]
#[repr(i32)]
enum ImDrawCornerFlags_ {
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

#[allow(non_upper_case_globals, non_snake_case)]        
#[link(name = "imgui", kind = "dylib")]
extern "C" {

    /// * shared_font_atlas: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?CreateContext@ImGui@@YAPEAUImGuiContext@@PEAUImFontAtlas@@@Z"]
    pub fn CreateContext(
        shared_font_atlas: *mut ImFontAtlas,
    ) -> *mut ImGuiContext;

    /// * ctx: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?DestroyContext@ImGui@@YAXPEAUImGuiContext@@@Z"]
    pub fn DestroyContext(
        ctx: *mut ImGuiContext,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCurrentContext@ImGui@@YAPEAUImGuiContext@@XZ"]
    pub fn GetCurrentContext() -> *mut ImGuiContext;

    /// * ctx: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetCurrentContext@ImGui@@YAXPEAUImGuiContext@@@Z"]
    pub fn SetCurrentContext(
        ctx: *mut ImGuiContext,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetIO@ImGui@@YAAEAUImGuiIO@@XZ"]
    pub fn GetIO() -> *mut ImGuiIO;

    #[allow(non_snake_case)]        
    #[link_name = "?GetStyle@ImGui@@YAAEAUImGuiStyle@@XZ"]
    pub fn GetStyle() -> *mut ImGuiStyle;

    #[allow(non_snake_case)]        
    #[link_name = "?NewFrame@ImGui@@YAXXZ"]
    pub fn NewFrame() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?EndFrame@ImGui@@YAXXZ"]
    pub fn EndFrame() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?Render@ImGui@@YAXXZ"]
    pub fn Render() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetDrawData@ImGui@@YAPEAUImDrawData@@XZ"]
    pub fn GetDrawData() -> *mut ImDrawData;

    /// * p_open: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?ShowDemoWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowDemoWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * p_open: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?ShowMetricsWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowMetricsWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * p_open: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?ShowAboutWindow@ImGui@@YAXPEA_N@Z"]
    pub fn ShowAboutWindow(
        p_open: *mut bool,
    ) -> c_void;

    /// * ref: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?ShowStyleEditor@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn ShowStyleEditor(
        r#ref: *mut ImGuiStyle,
    ) -> c_void;

    /// * label: 
    #[allow(non_snake_case)]        
    #[link_name = "?ShowStyleSelector@ImGui@@YA_NPEBD@Z"]
    pub fn ShowStyleSelector(
        label: *const i8,
    ) -> bool;

    /// * label: 
    #[allow(non_snake_case)]        
    #[link_name = "?ShowFontSelector@ImGui@@YAXPEBD@Z"]
    pub fn ShowFontSelector(
        label: *const i8,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?ShowUserGuide@ImGui@@YAXXZ"]
    pub fn ShowUserGuide() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetVersion@ImGui@@YAPEBDXZ"]
    pub fn GetVersion() -> *mut i8;

    /// * dst: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?StyleColorsDark@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsDark(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * dst: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?StyleColorsLight@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsLight(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * dst: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?StyleColorsClassic@ImGui@@YAXPEAUImGuiStyle@@@Z"]
    pub fn StyleColorsClassic(
        dst: *mut ImGuiStyle,
    ) -> c_void;

    /// * name: 
    /// * p_open: NULL
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?Begin@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn Begin(
        name: *const i8,
        p_open: *mut bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?End@ImGui@@YAXXZ"]
    pub fn End() -> c_void;

    /// * str_id: 
    /// * size: ImVec2(0,0)
    /// * border: false
    /// * flags: 0
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?BeginChild@ImGui@@YA_NIAEBUImVec2@@_NH@Z"]
    pub fn BeginChild_(
        id: ImGuiID,
        size: *const ImVec2,
        border: bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndChild@ImGui@@YAXXZ"]
    pub fn EndChild() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?IsWindowAppearing@ImGui@@YA_NXZ"]
    pub fn IsWindowAppearing() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsWindowCollapsed@ImGui@@YA_NXZ"]
    pub fn IsWindowCollapsed() -> bool;

    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?IsWindowFocused@ImGui@@YA_NH@Z"]
    pub fn IsWindowFocused(
        flags: ImGuiFocusedFlags,
    ) -> bool;

    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?IsWindowHovered@ImGui@@YA_NH@Z"]
    pub fn IsWindowHovered(
        flags: ImGuiHoveredFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetWindowDrawList() -> *mut ImDrawList;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowDpiScale@ImGui@@YAMXZ"]
    pub fn GetWindowDpiScale() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowPos() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowSize@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowSize() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowWidth@ImGui@@YAMXZ"]
    pub fn GetWindowWidth() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowHeight@ImGui@@YAMXZ"]
    pub fn GetWindowHeight() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowViewport@ImGui@@YAPEAUImGuiViewport@@XZ"]
    pub fn GetWindowViewport() -> *mut ImGuiViewport;

    /// * pos: 
    /// * cond: 0
    /// * pivot: ImVec2(0,0)
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowPos@ImGui@@YAXAEBUImVec2@@H0@Z"]
    pub fn SetNextWindowPos(
        pos: *const ImVec2,
        cond: ImGuiCond,
        pivot: *const ImVec2,
    ) -> c_void;

    /// * size: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowSize@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetNextWindowSize(
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * size_min: 
    /// * size_max: 
    /// * custom_callback: NULL
    /// * custom_callback_data: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowSizeConstraints@ImGui@@YAXAEBUImVec2@@0P6AXPEAUImGuiSizeCallbackData@@@ZPEAX@Z"]
    pub fn SetNextWindowSizeConstraints(
        size_min: *const ImVec2,
        size_max: *const ImVec2,
        custom_callback: extern fn(*mut ImGuiSizeCallbackData,) -> c_void,
        custom_callback_data: *mut c_void,
    ) -> c_void;

    /// * size: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowContentSize@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetNextWindowContentSize(
        size: *const ImVec2,
    ) -> c_void;

    /// * collapsed: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowCollapsed@ImGui@@YAX_NH@Z"]
    pub fn SetNextWindowCollapsed(
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowFocus@ImGui@@YAXXZ"]
    pub fn SetNextWindowFocus() -> c_void;

    /// * alpha: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowBgAlpha@ImGui@@YAXM@Z"]
    pub fn SetNextWindowBgAlpha(
        alpha: f32,
    ) -> c_void;

    /// * viewport_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowViewport@ImGui@@YAXI@Z"]
    pub fn SetNextWindowViewport(
        viewport_id: ImGuiID,
    ) -> c_void;

    /// * pos: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowPos@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetWindowPos(
        pos: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * size: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowSize@ImGui@@YAXAEBUImVec2@@H@Z"]
    pub fn SetWindowSize(
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * collapsed: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowCollapsed@ImGui@@YAX_NH@Z"]
    pub fn SetWindowCollapsed(
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowFocus@ImGui@@YAXXZ"]
    pub fn SetWindowFocus() -> c_void;

    /// * scale: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowFontScale@ImGui@@YAXM@Z"]
    pub fn SetWindowFontScale(
        scale: f32,
    ) -> c_void;

    /// * name: 
    /// * pos: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowPos@ImGui@@YAXPEBDAEBUImVec2@@H@Z"]
    pub fn SetWindowPos_(
        name: *const i8,
        pos: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    /// * size: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowSize@ImGui@@YAXPEBDAEBUImVec2@@H@Z"]
    pub fn SetWindowSize_(
        name: *const i8,
        size: *const ImVec2,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    /// * collapsed: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowCollapsed@ImGui@@YAXPEBD_NH@Z"]
    pub fn SetWindowCollapsed_(
        name: *const i8,
        collapsed: bool,
        cond: ImGuiCond,
    ) -> c_void;

    /// * name: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetWindowFocus@ImGui@@YAXPEBD@Z"]
    pub fn SetWindowFocus_(
        name: *const i8,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetContentRegionAvail@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetContentRegionAvail() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetContentRegionMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetContentRegionMax() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowContentRegionMin@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowContentRegionMin() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowContentRegionMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetWindowContentRegionMax() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowContentRegionWidth@ImGui@@YAMXZ"]
    pub fn GetWindowContentRegionWidth() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetScrollX@ImGui@@YAMXZ"]
    pub fn GetScrollX() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetScrollY@ImGui@@YAMXZ"]
    pub fn GetScrollY() -> f32;

    /// * scroll_x: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollX@ImGui@@YAXM@Z"]
    pub fn SetScrollX(
        scroll_x: f32,
    ) -> c_void;

    /// * scroll_y: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollY@ImGui@@YAXM@Z"]
    pub fn SetScrollY(
        scroll_y: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetScrollMaxX@ImGui@@YAMXZ"]
    pub fn GetScrollMaxX() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetScrollMaxY@ImGui@@YAMXZ"]
    pub fn GetScrollMaxY() -> f32;

    /// * center_x_ratio: 0.5f
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollHereX@ImGui@@YAXM@Z"]
    pub fn SetScrollHereX(
        center_x_ratio: f32,
    ) -> c_void;

    /// * center_y_ratio: 0.5f
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollHereY@ImGui@@YAXM@Z"]
    pub fn SetScrollHereY(
        center_y_ratio: f32,
    ) -> c_void;

    /// * local_x: 
    /// * center_x_ratio: 0.5f
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollFromPosX@ImGui@@YAXMM@Z"]
    pub fn SetScrollFromPosX(
        local_x: f32,
        center_x_ratio: f32,
    ) -> c_void;

    /// * local_y: 
    /// * center_y_ratio: 0.5f
    #[allow(non_snake_case)]        
    #[link_name = "?SetScrollFromPosY@ImGui@@YAXMM@Z"]
    pub fn SetScrollFromPosY(
        local_y: f32,
        center_y_ratio: f32,
    ) -> c_void;

    /// * font: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushFont@ImGui@@YAXPEAUImFont@@@Z"]
    pub fn PushFont(
        font: *mut ImFont,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopFont@ImGui@@YAXXZ"]
    pub fn PopFont() -> c_void;

    /// * idx: 
    /// * col: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushStyleColor@ImGui@@YAXHI@Z"]
    pub fn PushStyleColor(
        idx: ImGuiCol,
        col: ImU32,
    ) -> c_void;

    /// * idx: 
    /// * col: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushStyleColor@ImGui@@YAXHAEBUImVec4@@@Z"]
    pub fn PushStyleColor_(
        idx: ImGuiCol,
        col: *const ImVec4,
    ) -> c_void;

    /// * count: 1
    #[allow(non_snake_case)]        
    #[link_name = "?PopStyleColor@ImGui@@YAXH@Z"]
    pub fn PopStyleColor(
        count: i32,
    ) -> c_void;

    /// * idx: 
    /// * val: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushStyleVar@ImGui@@YAXHM@Z"]
    pub fn PushStyleVar(
        idx: ImGuiStyleVar,
        val: f32,
    ) -> c_void;

    /// * idx: 
    /// * val: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushStyleVar@ImGui@@YAXHAEBUImVec2@@@Z"]
    pub fn PushStyleVar_(
        idx: ImGuiStyleVar,
        val: *const ImVec2,
    ) -> c_void;

    /// * count: 1
    #[allow(non_snake_case)]        
    #[link_name = "?PopStyleVar@ImGui@@YAXH@Z"]
    pub fn PopStyleVar(
        count: i32,
    ) -> c_void;

    /// * allow_keyboard_focus: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushAllowKeyboardFocus@ImGui@@YAX_N@Z"]
    pub fn PushAllowKeyboardFocus(
        allow_keyboard_focus: bool,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopAllowKeyboardFocus@ImGui@@YAXXZ"]
    pub fn PopAllowKeyboardFocus() -> c_void;

    /// * repeat: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushButtonRepeat@ImGui@@YAX_N@Z"]
    pub fn PushButtonRepeat(
        repeat: bool,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopButtonRepeat@ImGui@@YAXXZ"]
    pub fn PopButtonRepeat() -> c_void;

    /// * item_width: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushItemWidth@ImGui@@YAXM@Z"]
    pub fn PushItemWidth(
        item_width: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopItemWidth@ImGui@@YAXXZ"]
    pub fn PopItemWidth() -> c_void;

    /// * item_width: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextItemWidth@ImGui@@YAXM@Z"]
    pub fn SetNextItemWidth(
        item_width: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?CalcItemWidth@ImGui@@YAMXZ"]
    pub fn CalcItemWidth() -> f32;

    /// * wrap_local_pos_x: 0.0f
    #[allow(non_snake_case)]        
    #[link_name = "?PushTextWrapPos@ImGui@@YAXM@Z"]
    pub fn PushTextWrapPos(
        wrap_local_pos_x: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopTextWrapPos@ImGui@@YAXXZ"]
    pub fn PopTextWrapPos() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFont@ImGui@@YAPEAUImFont@@XZ"]
    pub fn GetFont() -> *mut ImFont;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFontSize@ImGui@@YAMXZ"]
    pub fn GetFontSize() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFontTexUvWhitePixel@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetFontTexUvWhitePixel() -> ImVec2;

    /// * idx: 
    /// * alpha_mul: 1.0f
    #[allow(non_snake_case)]        
    #[link_name = "?GetColorU32@ImGui@@YAIHM@Z"]
    pub fn GetColorU32(
        idx: ImGuiCol,
        alpha_mul: f32,
    ) -> ImU32;

    /// * col: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetColorU32@ImGui@@YAIAEBUImVec4@@@Z"]
    pub fn GetColorU32_(
        col: *const ImVec4,
    ) -> ImU32;

    /// * col: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetColorU32@ImGui@@YAII@Z"]
    pub fn GetColorU32__(
        col: ImU32,
    ) -> ImU32;

    /// * idx: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetStyleColorVec4@ImGui@@YAAEBUImVec4@@H@Z"]
    pub fn GetStyleColorVec4(
        idx: ImGuiCol,
    ) -> *mut ImVec4;

    #[allow(non_snake_case)]        
    #[link_name = "?Separator@ImGui@@YAXXZ"]
    pub fn Separator() -> c_void;

    /// * offset_from_start_x: 0.0f
    /// * spacing: -1.0f
    #[allow(non_snake_case)]        
    #[link_name = "?SameLine@ImGui@@YAXMM@Z"]
    pub fn SameLine(
        offset_from_start_x: f32,
        spacing: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?NewLine@ImGui@@YAXXZ"]
    pub fn NewLine() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?Spacing@ImGui@@YAXXZ"]
    pub fn Spacing() -> c_void;

    /// * size: 
    #[allow(non_snake_case)]        
    #[link_name = "?Dummy@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn Dummy(
        size: *const ImVec2,
    ) -> c_void;

    /// * indent_w: 0.0f
    #[allow(non_snake_case)]        
    #[link_name = "?Indent@ImGui@@YAXM@Z"]
    pub fn Indent(
        indent_w: f32,
    ) -> c_void;

    /// * indent_w: 0.0f
    #[allow(non_snake_case)]        
    #[link_name = "?Unindent@ImGui@@YAXM@Z"]
    pub fn Unindent(
        indent_w: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?BeginGroup@ImGui@@YAXXZ"]
    pub fn BeginGroup() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?EndGroup@ImGui@@YAXXZ"]
    pub fn EndGroup() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCursorPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorPos() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCursorPosX@ImGui@@YAMXZ"]
    pub fn GetCursorPosX() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCursorPosY@ImGui@@YAMXZ"]
    pub fn GetCursorPosY() -> f32;

    /// * local_pos: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetCursorPos@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetCursorPos(
        local_pos: *const ImVec2,
    ) -> c_void;

    /// * local_x: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetCursorPosX@ImGui@@YAXM@Z"]
    pub fn SetCursorPosX(
        local_x: f32,
    ) -> c_void;

    /// * local_y: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetCursorPosY@ImGui@@YAXM@Z"]
    pub fn SetCursorPosY(
        local_y: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCursorStartPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorStartPos() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetCursorScreenPos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetCursorScreenPos() -> ImVec2;

    /// * pos: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetCursorScreenPos@ImGui@@YAXAEBUImVec2@@@Z"]
    pub fn SetCursorScreenPos(
        pos: *const ImVec2,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?AlignTextToFramePadding@ImGui@@YAXXZ"]
    pub fn AlignTextToFramePadding() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetTextLineHeight@ImGui@@YAMXZ"]
    pub fn GetTextLineHeight() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetTextLineHeightWithSpacing@ImGui@@YAMXZ"]
    pub fn GetTextLineHeightWithSpacing() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFrameHeight@ImGui@@YAMXZ"]
    pub fn GetFrameHeight() -> f32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFrameHeightWithSpacing@ImGui@@YAMXZ"]
    pub fn GetFrameHeightWithSpacing() -> f32;

    /// * str_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushID@ImGui@@YAXPEBD@Z"]
    pub fn PushID(
        str_id: *const i8,
    ) -> c_void;

    /// * str_id_begin: 
    /// * str_id_end: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushID@ImGui@@YAXPEBD0@Z"]
    pub fn PushID_(
        str_id_begin: *const i8,
        str_id_end: *const i8,
    ) -> c_void;

    /// * ptr_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushID@ImGui@@YAXPEBX@Z"]
    pub fn PushID__(
        ptr_id: *const c_void,
    ) -> c_void;

    /// * int_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushID@ImGui@@YAXH@Z"]
    pub fn PushID___(
        int_id: i32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopID@ImGui@@YAXXZ"]
    pub fn PopID() -> c_void;

    /// * str_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetID@ImGui@@YAIPEBD@Z"]
    pub fn GetID(
        str_id: *const i8,
    ) -> ImGuiID;

    /// * str_id_begin: 
    /// * str_id_end: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetID@ImGui@@YAIPEBD0@Z"]
    pub fn GetID_(
        str_id_begin: *const i8,
        str_id_end: *const i8,
    ) -> ImGuiID;

    /// * ptr_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetID@ImGui@@YAIPEBX@Z"]
    pub fn GetID__(
        ptr_id: *const c_void,
    ) -> ImGuiID;

    /// * text: 
    /// * text_end: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?TextUnformatted@ImGui@@YAXPEBD0@Z"]
    pub fn TextUnformatted(
        text: *const i8,
        text_end: *const i8,
    ) -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?Text@ImGui@@YAXPEBDZZ"]
    pub fn Text(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextV@ImGui@@YAXPEBDH@Z"]
    pub fn TextV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * col: 
    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextColored@ImGui@@YAXAEBUImVec4@@PEBDZZ"]
    pub fn TextColored(
        col: *const ImVec4,
        fmt: *const i8,
    ) -> c_void;

    /// * col: 
    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextColoredV@ImGui@@YAXAEBUImVec4@@PEBDH@Z"]
    pub fn TextColoredV(
        col: *const ImVec4,
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextDisabled@ImGui@@YAXPEBDZZ"]
    pub fn TextDisabled(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextDisabledV@ImGui@@YAXPEBDH@Z"]
    pub fn TextDisabledV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextWrapped@ImGui@@YAXPEBDZZ"]
    pub fn TextWrapped(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TextWrappedV@ImGui@@YAXPEBDH@Z"]
    pub fn TextWrappedV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * label: 
    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?LabelText@ImGui@@YAXPEBD0ZZ"]
    pub fn LabelText(
        label: *const i8,
        fmt: *const i8,
    ) -> c_void;

    /// * label: 
    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?LabelTextV@ImGui@@YAXPEBD0H@Z"]
    pub fn LabelTextV(
        label: *const i8,
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?BulletText@ImGui@@YAXPEBDZZ"]
    pub fn BulletText(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?BulletTextV@ImGui@@YAXPEBDH@Z"]
    pub fn BulletTextV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * label: 
    /// * size: ImVec2(0,0)
    #[allow(non_snake_case)]        
    #[link_name = "?Button@ImGui@@YA_NPEBDAEBUImVec2@@@Z"]
    pub fn Button(
        label: *const i8,
        size: *const ImVec2,
    ) -> bool;

    /// * label: 
    #[allow(non_snake_case)]        
    #[link_name = "?SmallButton@ImGui@@YA_NPEBD@Z"]
    pub fn SmallButton(
        label: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * size: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?InvisibleButton@ImGui@@YA_NPEBDAEBUImVec2@@H@Z"]
    pub fn InvisibleButton(
        str_id: *const i8,
        size: *const ImVec2,
        flags: ImGuiButtonFlags,
    ) -> bool;

    /// * str_id: 
    /// * dir: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?Checkbox@ImGui@@YA_NPEBDPEA_N@Z"]
    pub fn Checkbox(
        label: *const i8,
        v: *mut bool,
    ) -> bool;

    /// * label: 
    /// * flags: 
    /// * flags_value: 
    #[allow(non_snake_case)]        
    #[link_name = "?CheckboxFlags@ImGui@@YA_NPEBDPEAHH@Z"]
    pub fn CheckboxFlags(
        label: *const i8,
        flags: *mut i32,
        flags_value: i32,
    ) -> bool;

    /// * label: 
    /// * flags: 
    /// * flags_value: 
    #[allow(non_snake_case)]        
    #[link_name = "?CheckboxFlags@ImGui@@YA_NPEBDPEAII@Z"]
    pub fn CheckboxFlags_(
        label: *const i8,
        flags: *mut u32,
        flags_value: u32,
    ) -> bool;

    /// * label: 
    /// * active: 
    #[allow(non_snake_case)]        
    #[link_name = "?RadioButton@ImGui@@YA_NPEBD_N@Z"]
    pub fn RadioButton(
        label: *const i8,
        active: bool,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_button: 
    #[allow(non_snake_case)]        
    #[link_name = "?RadioButton@ImGui@@YA_NPEBDPEAHH@Z"]
    pub fn RadioButton_(
        label: *const i8,
        v: *mut i32,
        v_button: i32,
    ) -> bool;

    /// * fraction: 
    /// * size_arg: ImVec2(-FLT_MIN,0)
    /// * overlay: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?ProgressBar@ImGui@@YAXMAEBUImVec2@@PEBD@Z"]
    pub fn ProgressBar(
        fraction: f32,
        size_arg: *const ImVec2,
        overlay: *const i8,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?Bullet@ImGui@@YAXXZ"]
    pub fn Bullet() -> c_void;

    /// * label: 
    /// * preview_value: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginCombo@ImGui@@YA_NPEBD0H@Z"]
    pub fn BeginCombo(
        label: *const i8,
        preview_value: *const i8,
        flags: ImGuiComboFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndCombo@ImGui@@YAXXZ"]
    pub fn EndCombo() -> c_void;

    /// * label: 
    /// * current_item: 
    /// * items: 
    /// * items_count: 
    /// * popup_max_height_in_items: -1
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?Combo@ImGui@@YA_NPEBDPEAHP6A_NPEAXHPEAPEBD@Z2HH@Z"]
    pub fn Combo__(
        label: *const i8,
        current_item: *mut i32,
        items_getter: *mut extern fn(*mut c_void,i32,*mut *mut i8,) -> bool,
        data: *mut c_void,
        items_count: i32,
        popup_max_height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * v_speed: 1.0f
    /// * v_min: 0.0f
    /// * v_max: 0.0f
    /// * format: "%.3f"
    /// * flags: 0
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?InputInt2@ImGui@@YA_NPEBDQEAHH@Z"]
    pub fn InputInt2(
        label: *const i8,
        v: [i32; 2],
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?InputInt3@ImGui@@YA_NPEBDQEAHH@Z"]
    pub fn InputInt3(
        label: *const i8,
        v: [i32; 3],
        flags: ImGuiInputTextFlags,
    ) -> bool;

    /// * label: 
    /// * v: 
    /// * flags: 0
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?ColorEdit3@ImGui@@YA_NPEBDQEAMH@Z"]
    pub fn ColorEdit3(
        label: *const i8,
        col: [f32; 3],
        flags: ImGuiColorEditFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?ColorEdit4@ImGui@@YA_NPEBDQEAMH@Z"]
    pub fn ColorEdit4(
        label: *const i8,
        col: [f32; 4],
        flags: ImGuiColorEditFlags,
    ) -> bool;

    /// * label: 
    /// * col: 
    /// * flags: 0
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?ColorButton@ImGui@@YA_NPEBDAEBUImVec4@@HUImVec2@@@Z"]
    pub fn ColorButton(
        desc_id: *const i8,
        col: *const ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;

    /// * flags: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetColorEditOptions@ImGui@@YAXH@Z"]
    pub fn SetColorEditOptions(
        flags: ImGuiColorEditFlags,
    ) -> c_void;

    /// * label: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNode@ImGui@@YA_NPEBD@Z"]
    pub fn TreeNode(
        label: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNode@ImGui@@YA_NPEBD0ZZ"]
    pub fn TreeNode_(
        str_id: *const i8,
        fmt: *const i8,
    ) -> bool;

    /// * ptr_id: 
    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNode@ImGui@@YA_NPEBXPEBDZZ"]
    pub fn TreeNode__(
        ptr_id: *const c_void,
        fmt: *const i8,
    ) -> bool;

    /// * str_id: 
    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeV@ImGui@@YA_NPEBD0H@Z"]
    pub fn TreeNodeV(
        str_id: *const i8,
        fmt: *const i8,
        args: i32,
    ) -> bool;

    /// * ptr_id: 
    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeV@ImGui@@YA_NPEBXPEBDH@Z"]
    pub fn TreeNodeV_(
        ptr_id: *const c_void,
        fmt: *const i8,
        args: i32,
    ) -> bool;

    /// * label: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeEx@ImGui@@YA_NPEBDH@Z"]
    pub fn TreeNodeEx(
        label: *const i8,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * str_id: 
    /// * flags: 
    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeEx@ImGui@@YA_NPEBDH0ZZ"]
    pub fn TreeNodeEx_(
        str_id: *const i8,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
    ) -> bool;

    /// * ptr_id: 
    /// * flags: 
    /// * fmt: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeExV@ImGui@@YA_NPEBDH0H@Z"]
    pub fn TreeNodeExV(
        str_id: *const i8,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
        args: i32,
    ) -> bool;

    /// * ptr_id: 
    /// * flags: 
    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreeNodeExV@ImGui@@YA_NPEBXHPEBDH@Z"]
    pub fn TreeNodeExV_(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const i8,
        args: i32,
    ) -> bool;

    /// * str_id: 
    #[allow(non_snake_case)]        
    #[link_name = "?TreePush@ImGui@@YAXPEBD@Z"]
    pub fn TreePush(
        str_id: *const i8,
    ) -> c_void;

    /// * ptr_id: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?TreePush@ImGui@@YAXPEBX@Z"]
    pub fn TreePush_(
        ptr_id: *const c_void,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?TreePop@ImGui@@YAXXZ"]
    pub fn TreePop() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetTreeNodeToLabelSpacing@ImGui@@YAMXZ"]
    pub fn GetTreeNodeToLabelSpacing() -> f32;

    /// * label: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?CollapsingHeader@ImGui@@YA_NPEBDH@Z"]
    pub fn CollapsingHeader(
        label: *const i8,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * label: 
    /// * p_visible: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?CollapsingHeader@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn CollapsingHeader_(
        label: *const i8,
        p_visible: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;

    /// * is_open: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextItemOpen@ImGui@@YAX_NH@Z"]
    pub fn SetNextItemOpen(
        is_open: bool,
        cond: ImGuiCond,
    ) -> c_void;

    /// * label: 
    /// * selected: false
    /// * flags: 0
    /// * size: ImVec2(0,0)
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?Selectable@ImGui@@YA_NPEBDPEA_NHAEBUImVec2@@@Z"]
    pub fn Selectable_(
        label: *const i8,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: *const ImVec2,
    ) -> bool;

    /// * label: 
    /// * size: ImVec2(0,0)
    #[allow(non_snake_case)]        
    #[link_name = "?BeginListBox@ImGui@@YA_NPEBDAEBUImVec2@@@Z"]
    pub fn BeginListBox(
        label: *const i8,
        size: *const ImVec2,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndListBox@ImGui@@YAXXZ"]
    pub fn EndListBox() -> c_void;

    /// * label: 
    /// * current_item: 
    /// * items: 
    /// * items_count: 
    /// * height_in_items: -1
    #[allow(non_snake_case)]        
    #[link_name = "?ListBox@ImGui@@YA_NPEBDPEAHQEBQEBDHH@Z"]
    pub fn ListBox(
        label: *const i8,
        current_item: *mut i32,
        items: *mut *mut i8,
        items_count: i32,
        height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * current_item: 
    /// * items_getter: 
    /// * data: 
    /// * items_count: 
    /// * height_in_items: -1
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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

    /// * label: 
    /// * values_getter: 
    /// * data: 
    /// * values_count: 
    /// * values_offset: 0
    /// * overlay_text: NULL
    /// * scale_min: FLT_MAX
    /// * scale_max: FLT_MAX
    /// * graph_size: ImVec2(0,0)
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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

    /// * prefix: 
    /// * b: 
    #[allow(non_snake_case)]        
    #[link_name = "?Value@ImGui@@YAXPEBD_N@Z"]
    pub fn Value(
        prefix: *const i8,
        b: bool,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    #[allow(non_snake_case)]        
    #[link_name = "?Value@ImGui@@YAXPEBDH@Z"]
    pub fn Value_(
        prefix: *const i8,
        v: i32,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    #[allow(non_snake_case)]        
    #[link_name = "?Value@ImGui@@YAXPEBDI@Z"]
    pub fn Value__(
        prefix: *const i8,
        v: u32,
    ) -> c_void;

    /// * prefix: 
    /// * v: 
    /// * float_format: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?Value@ImGui@@YAXPEBDM0@Z"]
    pub fn Value___(
        prefix: *const i8,
        v: f32,
        float_format: *const i8,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?BeginMenuBar@ImGui@@YA_NXZ"]
    pub fn BeginMenuBar() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndMenuBar@ImGui@@YAXXZ"]
    pub fn EndMenuBar() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?BeginMainMenuBar@ImGui@@YA_NXZ"]
    pub fn BeginMainMenuBar() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndMainMenuBar@ImGui@@YAXXZ"]
    pub fn EndMainMenuBar() -> c_void;

    /// * label: 
    /// * enabled: true
    #[allow(non_snake_case)]        
    #[link_name = "?BeginMenu@ImGui@@YA_NPEBD_N@Z"]
    pub fn BeginMenu(
        label: *const i8,
        enabled: bool,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndMenu@ImGui@@YAXXZ"]
    pub fn EndMenu() -> c_void;

    /// * label: 
    /// * shortcut: NULL
    /// * selected: false
    /// * enabled: true
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?MenuItem@ImGui@@YA_NPEBD0PEA_N_N@Z"]
    pub fn MenuItem_(
        label: *const i8,
        shortcut: *const i8,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?BeginTooltip@ImGui@@YAXXZ"]
    pub fn BeginTooltip() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?EndTooltip@ImGui@@YAXXZ"]
    pub fn EndTooltip() -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetTooltip@ImGui@@YAXPEBDZZ"]
    pub fn SetTooltip(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetTooltipV@ImGui@@YAXPEBDH@Z"]
    pub fn SetTooltipV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * str_id: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginPopup@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopup(
        str_id: *const i8,
        flags: ImGuiWindowFlags,
    ) -> bool;

    /// * name: 
    /// * p_open: NULL
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginPopupModal@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn BeginPopupModal(
        name: *const i8,
        p_open: *mut bool,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndPopup@ImGui@@YAXXZ"]
    pub fn EndPopup() -> c_void;

    /// * str_id: 
    /// * popup_flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?OpenPopup@ImGui@@YAXPEBDH@Z"]
    pub fn OpenPopup(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    /// * id: 
    /// * popup_flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?OpenPopup@ImGui@@YAXIH@Z"]
    pub fn OpenPopup_(
        id: ImGuiID,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[allow(non_snake_case)]        
    #[link_name = "?OpenPopupOnItemClick@ImGui@@YAXPEBDH@Z"]
    pub fn OpenPopupOnItemClick(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?CloseCurrentPopup@ImGui@@YAXXZ"]
    pub fn CloseCurrentPopup() -> c_void;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[allow(non_snake_case)]        
    #[link_name = "?BeginPopupContextItem@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextItem(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[allow(non_snake_case)]        
    #[link_name = "?BeginPopupContextWindow@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextWindow(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: NULL
    /// * popup_flags: 1
    #[allow(non_snake_case)]        
    #[link_name = "?BeginPopupContextVoid@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginPopupContextVoid(
        str_id: *const i8,
        popup_flags: ImGuiPopupFlags,
    ) -> bool;

    /// * str_id: 
    /// * flags: 0
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?BeginTable@ImGui@@YA_NPEBDHHAEBUImVec2@@M@Z"]
    pub fn BeginTable(
        str_id: *const i8,
        column: i32,
        flags: ImGuiTableFlags,
        outer_size: *const ImVec2,
        inner_width: f32,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndTable@ImGui@@YAXXZ"]
    pub fn EndTable() -> c_void;

    /// * row_flags: 0
    /// * min_row_height: 0.0f
    #[allow(non_snake_case)]        
    #[link_name = "?TableNextRow@ImGui@@YAXHM@Z"]
    pub fn TableNextRow(
        row_flags: ImGuiTableRowFlags,
        min_row_height: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?TableNextColumn@ImGui@@YA_NXZ"]
    pub fn TableNextColumn() -> bool;

    /// * column_n: 
    #[allow(non_snake_case)]        
    #[link_name = "?TableSetColumnIndex@ImGui@@YA_NH@Z"]
    pub fn TableSetColumnIndex(
        column_n: i32,
    ) -> bool;

    /// * label: 
    /// * flags: 0
    /// * init_width_or_weight: 0.0f
    /// * user_id: 0
    #[allow(non_snake_case)]        
    #[link_name = "?TableSetupColumn@ImGui@@YAXPEBDHMI@Z"]
    pub fn TableSetupColumn(
        label: *const i8,
        flags: ImGuiTableColumnFlags,
        init_width_or_weight: f32,
        user_id: ImGuiID,
    ) -> c_void;

    /// * cols: 
    /// * rows: 
    #[allow(non_snake_case)]        
    #[link_name = "?TableSetupScrollFreeze@ImGui@@YAXHH@Z"]
    pub fn TableSetupScrollFreeze(
        cols: i32,
        rows: i32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?TableHeadersRow@ImGui@@YAXXZ"]
    pub fn TableHeadersRow() -> c_void;

    /// * label: 
    #[allow(non_snake_case)]        
    #[link_name = "?TableHeader@ImGui@@YAXPEBD@Z"]
    pub fn TableHeader(
        label: *const i8,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?TableGetSortSpecs@ImGui@@YAPEAUImGuiTableSortSpecs@@XZ"]
    pub fn TableGetSortSpecs() -> *mut ImGuiTableSortSpecs;

    #[allow(non_snake_case)]        
    #[link_name = "?TableGetColumnCount@ImGui@@YAHXZ"]
    pub fn TableGetColumnCount() -> i32;

    #[allow(non_snake_case)]        
    #[link_name = "?TableGetColumnIndex@ImGui@@YAHXZ"]
    pub fn TableGetColumnIndex() -> i32;

    #[allow(non_snake_case)]        
    #[link_name = "?TableGetRowIndex@ImGui@@YAHXZ"]
    pub fn TableGetRowIndex() -> i32;

    /// * column_n: -1
    #[allow(non_snake_case)]        
    #[link_name = "?TableGetColumnName@ImGui@@YAPEBDH@Z"]
    pub fn TableGetColumnName(
        column_n: i32,
    ) -> *mut i8;

    /// * column_n: -1
    #[allow(non_snake_case)]        
    #[link_name = "?TableGetColumnFlags@ImGui@@YAHH@Z"]
    pub fn TableGetColumnFlags(
        column_n: i32,
    ) -> ImGuiTableColumnFlags;

    /// * column_n: 
    /// * v: 
    #[allow(non_snake_case)]        
    #[link_name = "?TableSetColumnEnabled@ImGui@@YAXH_N@Z"]
    pub fn TableSetColumnEnabled(
        column_n: i32,
        v: bool,
    ) -> c_void;

    /// * target: 
    /// * color: 
    /// * column_n: -1
    #[allow(non_snake_case)]        
    #[link_name = "?TableSetBgColor@ImGui@@YAXHIH@Z"]
    pub fn TableSetBgColor(
        target: ImGuiTableBgTarget,
        color: ImU32,
        column_n: i32,
    ) -> c_void;

    /// * count: 1
    /// * id: NULL
    /// * border: true
    #[allow(non_snake_case)]        
    #[link_name = "?Columns@ImGui@@YAXHPEBD_N@Z"]
    pub fn Columns(
        count: i32,
        id: *const i8,
        border: bool,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?NextColumn@ImGui@@YAXXZ"]
    pub fn NextColumn() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetColumnIndex@ImGui@@YAHXZ"]
    pub fn GetColumnIndex() -> i32;

    /// * column_index: -1
    #[allow(non_snake_case)]        
    #[link_name = "?GetColumnWidth@ImGui@@YAMH@Z"]
    pub fn GetColumnWidth(
        column_index: i32,
    ) -> f32;

    /// * column_index: 
    /// * width: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetColumnWidth@ImGui@@YAXHM@Z"]
    pub fn SetColumnWidth(
        column_index: i32,
        width: f32,
    ) -> c_void;

    /// * column_index: -1
    #[allow(non_snake_case)]        
    #[link_name = "?GetColumnOffset@ImGui@@YAMH@Z"]
    pub fn GetColumnOffset(
        column_index: i32,
    ) -> f32;

    /// * column_index: 
    /// * offset_x: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetColumnOffset@ImGui@@YAXHM@Z"]
    pub fn SetColumnOffset(
        column_index: i32,
        offset_x: f32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetColumnsCount@ImGui@@YAHXZ"]
    pub fn GetColumnsCount() -> i32;

    /// * str_id: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginTabBar@ImGui@@YA_NPEBDH@Z"]
    pub fn BeginTabBar(
        str_id: *const i8,
        flags: ImGuiTabBarFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndTabBar@ImGui@@YAXXZ"]
    pub fn EndTabBar() -> c_void;

    /// * label: 
    /// * p_open: NULL
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginTabItem@ImGui@@YA_NPEBDPEA_NH@Z"]
    pub fn BeginTabItem(
        label: *const i8,
        p_open: *mut bool,
        flags: ImGuiTabItemFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndTabItem@ImGui@@YAXXZ"]
    pub fn EndTabItem() -> c_void;

    /// * label: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?TabItemButton@ImGui@@YA_NPEBDH@Z"]
    pub fn TabItemButton(
        label: *const i8,
        flags: ImGuiTabItemFlags,
    ) -> bool;

    /// * tab_or_docked_window_label: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetTabItemClosed@ImGui@@YAXPEBD@Z"]
    pub fn SetTabItemClosed(
        tab_or_docked_window_label: *const i8,
    ) -> c_void;

    /// * id: 
    /// * size: ImVec2(0,0)
    /// * flags: 0
    /// * window_class: NULL
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?DockSpaceOverViewport@ImGui@@YAIPEBUImGuiViewport@@HPEBUImGuiWindowClass@@@Z"]
    pub fn DockSpaceOverViewport(
        viewport: *const ImGuiViewport,
        flags: ImGuiDockNodeFlags,
        window_class: *const ImGuiWindowClass,
    ) -> ImGuiID;

    /// * dock_id: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowDockID@ImGui@@YAXIH@Z"]
    pub fn SetNextWindowDockID(
        dock_id: ImGuiID,
        cond: ImGuiCond,
    ) -> c_void;

    /// * window_class: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetNextWindowClass@ImGui@@YAXPEBUImGuiWindowClass@@@Z"]
    pub fn SetNextWindowClass(
        window_class: *const ImGuiWindowClass,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetWindowDockID@ImGui@@YAIXZ"]
    pub fn GetWindowDockID() -> ImGuiID;

    #[allow(non_snake_case)]        
    #[link_name = "?IsWindowDocked@ImGui@@YA_NXZ"]
    pub fn IsWindowDocked() -> bool;

    /// * auto_open_depth: -1
    #[allow(non_snake_case)]        
    #[link_name = "?LogToTTY@ImGui@@YAXH@Z"]
    pub fn LogToTTY(
        auto_open_depth: i32,
    ) -> c_void;

    /// * auto_open_depth: -1
    /// * filename: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?LogToFile@ImGui@@YAXHPEBD@Z"]
    pub fn LogToFile(
        auto_open_depth: i32,
        filename: *const i8,
    ) -> c_void;

    /// * auto_open_depth: -1
    #[allow(non_snake_case)]        
    #[link_name = "?LogToClipboard@ImGui@@YAXH@Z"]
    pub fn LogToClipboard(
        auto_open_depth: i32,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?LogFinish@ImGui@@YAXXZ"]
    pub fn LogFinish() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?LogButtons@ImGui@@YAXXZ"]
    pub fn LogButtons() -> c_void;

    /// * fmt: 
    #[allow(non_snake_case)]        
    #[link_name = "?LogText@ImGui@@YAXPEBDZZ"]
    pub fn LogText(
        fmt: *const i8,
    ) -> c_void;

    /// * fmt: 
    /// * args: 
    #[allow(non_snake_case)]        
    #[link_name = "?LogTextV@ImGui@@YAXPEBDH@Z"]
    pub fn LogTextV(
        fmt: *const i8,
        args: i32,
    ) -> c_void;

    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?BeginDragDropSource@ImGui@@YA_NH@Z"]
    pub fn BeginDragDropSource(
        flags: ImGuiDragDropFlags,
    ) -> bool;

    /// * type: 
    /// * data: 
    /// * sz: 
    /// * cond: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetDragDropPayload@ImGui@@YA_NPEBDPEBX_KH@Z"]
    pub fn SetDragDropPayload(
        r#type: *const i8,
        data: *const c_void,
        sz: usize,
        cond: ImGuiCond,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndDragDropSource@ImGui@@YAXXZ"]
    pub fn EndDragDropSource() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?BeginDragDropTarget@ImGui@@YA_NXZ"]
    pub fn BeginDragDropTarget() -> bool;

    /// * type: 
    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?AcceptDragDropPayload@ImGui@@YAPEBUImGuiPayload@@PEBDH@Z"]
    pub fn AcceptDragDropPayload(
        r#type: *const i8,
        flags: ImGuiDragDropFlags,
    ) -> *mut ImGuiPayload;

    #[allow(non_snake_case)]        
    #[link_name = "?EndDragDropTarget@ImGui@@YAXXZ"]
    pub fn EndDragDropTarget() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetDragDropPayload@ImGui@@YAPEBUImGuiPayload@@XZ"]
    pub fn GetDragDropPayload() -> *mut ImGuiPayload;

    /// * clip_rect_min: 
    /// * clip_rect_max: 
    /// * intersect_with_current_clip_rect: 
    #[allow(non_snake_case)]        
    #[link_name = "?PushClipRect@ImGui@@YAXAEBUImVec2@@0_N@Z"]
    pub fn PushClipRect(
        clip_rect_min: *const ImVec2,
        clip_rect_max: *const ImVec2,
        intersect_with_current_clip_rect: bool,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?PopClipRect@ImGui@@YAXXZ"]
    pub fn PopClipRect() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?SetItemDefaultFocus@ImGui@@YAXXZ"]
    pub fn SetItemDefaultFocus() -> c_void;

    /// * offset: 0
    #[allow(non_snake_case)]        
    #[link_name = "?SetKeyboardFocusHere@ImGui@@YAXH@Z"]
    pub fn SetKeyboardFocusHere(
        offset: i32,
    ) -> c_void;

    /// * flags: 0
    #[allow(non_snake_case)]        
    #[link_name = "?IsItemHovered@ImGui@@YA_NH@Z"]
    pub fn IsItemHovered(
        flags: ImGuiHoveredFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemActive@ImGui@@YA_NXZ"]
    pub fn IsItemActive() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemFocused@ImGui@@YA_NXZ"]
    pub fn IsItemFocused() -> bool;

    /// * mouse_button: 0
    #[allow(non_snake_case)]        
    #[link_name = "?IsItemClicked@ImGui@@YA_NH@Z"]
    pub fn IsItemClicked(
        mouse_button: ImGuiMouseButton,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemVisible@ImGui@@YA_NXZ"]
    pub fn IsItemVisible() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemEdited@ImGui@@YA_NXZ"]
    pub fn IsItemEdited() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemActivated@ImGui@@YA_NXZ"]
    pub fn IsItemActivated() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemDeactivated@ImGui@@YA_NXZ"]
    pub fn IsItemDeactivated() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemDeactivatedAfterEdit@ImGui@@YA_NXZ"]
    pub fn IsItemDeactivatedAfterEdit() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsItemToggledOpen@ImGui@@YA_NXZ"]
    pub fn IsItemToggledOpen() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsAnyItemHovered@ImGui@@YA_NXZ"]
    pub fn IsAnyItemHovered() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsAnyItemActive@ImGui@@YA_NXZ"]
    pub fn IsAnyItemActive() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsAnyItemFocused@ImGui@@YA_NXZ"]
    pub fn IsAnyItemFocused() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?GetItemRectMin@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectMin() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetItemRectMax@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectMax() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetItemRectSize@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetItemRectSize() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?SetItemAllowOverlap@ImGui@@YAXXZ"]
    pub fn SetItemAllowOverlap() -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetMainViewport@ImGui@@YAPEAUImGuiViewport@@XZ"]
    pub fn GetMainViewport() -> *mut ImGuiViewport;

    /// * size: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsRectVisible@ImGui@@YA_NAEBUImVec2@@@Z"]
    pub fn IsRectVisible(
        size: *const ImVec2,
    ) -> bool;

    /// * rect_min: 
    /// * rect_max: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsRectVisible@ImGui@@YA_NAEBUImVec2@@0@Z"]
    pub fn IsRectVisible_(
        rect_min: *const ImVec2,
        rect_max: *const ImVec2,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?GetTime@ImGui@@YANXZ"]
    pub fn GetTime() -> f64;

    #[allow(non_snake_case)]        
    #[link_name = "?GetFrameCount@ImGui@@YAHXZ"]
    pub fn GetFrameCount() -> i32;

    #[allow(non_snake_case)]        
    #[link_name = "?GetBackgroundDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetBackgroundDrawList() -> *mut ImDrawList;

    #[allow(non_snake_case)]        
    #[link_name = "?GetForegroundDrawList@ImGui@@YAPEAUImDrawList@@XZ"]
    pub fn GetForegroundDrawList() -> *mut ImDrawList;

    /// * viewport: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetBackgroundDrawList@ImGui@@YAPEAUImDrawList@@PEAUImGuiViewport@@@Z"]
    pub fn GetBackgroundDrawList_(
        viewport: *mut ImGuiViewport,
    ) -> *mut ImDrawList;

    /// * viewport: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetForegroundDrawList@ImGui@@YAPEAUImDrawList@@PEAUImGuiViewport@@@Z"]
    pub fn GetForegroundDrawList_(
        viewport: *mut ImGuiViewport,
    ) -> *mut ImDrawList;

    #[allow(non_snake_case)]        
    #[link_name = "?GetDrawListSharedData@ImGui@@YAPEAUImDrawListSharedData@@XZ"]
    pub fn GetDrawListSharedData() -> *mut ImDrawListSharedData;

    /// * idx: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetStyleColorName@ImGui@@YAPEBDH@Z"]
    pub fn GetStyleColorName(
        idx: ImGuiCol,
    ) -> *mut i8;

    /// * storage: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetStateStorage@ImGui@@YAXPEAUImGuiStorage@@@Z"]
    pub fn SetStateStorage(
        storage: *mut ImGuiStorage,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetStateStorage@ImGui@@YAPEAUImGuiStorage@@XZ"]
    pub fn GetStateStorage() -> *mut ImGuiStorage;

    /// * items_count: 
    /// * items_height: 
    /// * out_items_display_start: 
    /// * out_items_display_end: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?BeginChildFrame@ImGui@@YA_NIAEBUImVec2@@H@Z"]
    pub fn BeginChildFrame(
        id: ImGuiID,
        size: *const ImVec2,
        flags: ImGuiWindowFlags,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?EndChildFrame@ImGui@@YAXXZ"]
    pub fn EndChildFrame() -> c_void;

    /// * text: 
    /// * text_end: NULL
    /// * hide_text_after_double_hash: false
    /// * wrap_width: -1.0f
    #[allow(non_snake_case)]        
    #[link_name = "?CalcTextSize@ImGui@@YA?AUImVec2@@PEBD0_NM@Z"]
    pub fn CalcTextSize(
        text: *const i8,
        text_end: *const i8,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2;

    /// * in: 
    #[allow(non_snake_case)]        
    #[link_name = "?ColorConvertU32ToFloat4@ImGui@@YA?AUImVec4@@I@Z"]
    pub fn ColorConvertU32ToFloat4(
        r#in: ImU32,
    ) -> ImVec4;

    /// * in: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?GetKeyIndex@ImGui@@YAHH@Z"]
    pub fn GetKeyIndex(
        imgui_key: ImGuiKey,
    ) -> i32;

    /// * user_key_index: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsKeyDown@ImGui@@YA_NH@Z"]
    pub fn IsKeyDown(
        user_key_index: i32,
    ) -> bool;

    /// * user_key_index: 
    /// * repeat: true
    #[allow(non_snake_case)]        
    #[link_name = "?IsKeyPressed@ImGui@@YA_NH_N@Z"]
    pub fn IsKeyPressed(
        user_key_index: i32,
        repeat: bool,
    ) -> bool;

    /// * user_key_index: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsKeyReleased@ImGui@@YA_NH@Z"]
    pub fn IsKeyReleased(
        user_key_index: i32,
    ) -> bool;

    /// * key_index: 
    /// * repeat_delay: 
    /// * rate: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetKeyPressedAmount@ImGui@@YAHHMM@Z"]
    pub fn GetKeyPressedAmount(
        key_index: i32,
        repeat_delay: f32,
        rate: f32,
    ) -> i32;

    /// * want_capture_keyboard_value: true
    #[allow(non_snake_case)]        
    #[link_name = "?CaptureKeyboardFromApp@ImGui@@YAX_N@Z"]
    pub fn CaptureKeyboardFromApp(
        want_capture_keyboard_value: bool,
    ) -> c_void;

    /// * button: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseDown@ImGui@@YA_NH@Z"]
    pub fn IsMouseDown(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * button: 
    /// * repeat: false
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseClicked@ImGui@@YA_NH_N@Z"]
    pub fn IsMouseClicked(
        button: ImGuiMouseButton,
        repeat: bool,
    ) -> bool;

    /// * button: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseReleased@ImGui@@YA_NH@Z"]
    pub fn IsMouseReleased(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * button: 
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseDoubleClicked@ImGui@@YA_NH@Z"]
    pub fn IsMouseDoubleClicked(
        button: ImGuiMouseButton,
    ) -> bool;

    /// * r_min: 
    /// * r_max: 
    /// * clip: true
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseHoveringRect@ImGui@@YA_NAEBUImVec2@@0_N@Z"]
    pub fn IsMouseHoveringRect(
        r_min: *const ImVec2,
        r_max: *const ImVec2,
        clip: bool,
    ) -> bool;

    /// * mouse_pos: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?IsMousePosValid@ImGui@@YA_NPEBUImVec2@@@Z"]
    pub fn IsMousePosValid(
        mouse_pos: *const ImVec2,
    ) -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?IsAnyMouseDown@ImGui@@YA_NXZ"]
    pub fn IsAnyMouseDown() -> bool;

    #[allow(non_snake_case)]        
    #[link_name = "?GetMousePos@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetMousePos() -> ImVec2;

    #[allow(non_snake_case)]        
    #[link_name = "?GetMousePosOnOpeningCurrentPopup@ImGui@@YA?AUImVec2@@XZ"]
    pub fn GetMousePosOnOpeningCurrentPopup() -> ImVec2;

    /// * button: 
    /// * lock_threshold: -1.0f
    #[allow(non_snake_case)]        
    #[link_name = "?IsMouseDragging@ImGui@@YA_NHM@Z"]
    pub fn IsMouseDragging(
        button: ImGuiMouseButton,
        lock_threshold: f32,
    ) -> bool;

    /// * button: 0
    /// * lock_threshold: -1.0f
    #[allow(non_snake_case)]        
    #[link_name = "?GetMouseDragDelta@ImGui@@YA?AUImVec2@@HM@Z"]
    pub fn GetMouseDragDelta(
        button: ImGuiMouseButton,
        lock_threshold: f32,
    ) -> ImVec2;

    /// * button: 0
    #[allow(non_snake_case)]        
    #[link_name = "?ResetMouseDragDelta@ImGui@@YAXH@Z"]
    pub fn ResetMouseDragDelta(
        button: ImGuiMouseButton,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetMouseCursor@ImGui@@YAHXZ"]
    pub fn GetMouseCursor() -> ImGuiMouseCursor;

    /// * cursor_type: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetMouseCursor@ImGui@@YAXH@Z"]
    pub fn SetMouseCursor(
        cursor_type: ImGuiMouseCursor,
    ) -> c_void;

    /// * want_capture_mouse_value: true
    #[allow(non_snake_case)]        
    #[link_name = "?CaptureMouseFromApp@ImGui@@YAX_N@Z"]
    pub fn CaptureMouseFromApp(
        want_capture_mouse_value: bool,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetClipboardText@ImGui@@YAPEBDXZ"]
    pub fn GetClipboardText() -> *mut i8;

    /// * text: 
    #[allow(non_snake_case)]        
    #[link_name = "?SetClipboardText@ImGui@@YAXPEBD@Z"]
    pub fn SetClipboardText(
        text: *const i8,
    ) -> c_void;

    /// * ini_filename: 
    #[allow(non_snake_case)]        
    #[link_name = "?LoadIniSettingsFromDisk@ImGui@@YAXPEBD@Z"]
    pub fn LoadIniSettingsFromDisk(
        ini_filename: *const i8,
    ) -> c_void;

    /// * ini_data: 
    /// * ini_size: 0
    #[allow(non_snake_case)]        
    #[link_name = "?LoadIniSettingsFromMemory@ImGui@@YAXPEBD_K@Z"]
    pub fn LoadIniSettingsFromMemory(
        ini_data: *const i8,
        ini_size: usize,
    ) -> c_void;

    /// * ini_filename: 
    #[allow(non_snake_case)]        
    #[link_name = "?SaveIniSettingsToDisk@ImGui@@YAXPEBD@Z"]
    pub fn SaveIniSettingsToDisk(
        ini_filename: *const i8,
    ) -> c_void;

    /// * out_ini_size: NULL
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
    #[link_name = "?SetAllocatorFunctions@ImGui@@YAXP6APEAX_KPEAX@ZP6AX11@Z1@Z"]
    pub fn SetAllocatorFunctions(
        alloc_func: extern fn(usize,*mut c_void,) -> *mut c_void,
        free_func: extern fn(*mut c_void,*mut c_void,) -> c_void,
        user_data: *mut c_void,
    ) -> c_void;

    /// * p_alloc_func: 
    /// * p_free_func: 
    /// * p_user_data: 
    #[allow(non_snake_case)]        
    #[link_name = "?GetAllocatorFunctions@ImGui@@YAXPEAP6APEAX_KPEAX@ZPEAP6AX11@ZPEAPEAX@Z"]
    pub fn GetAllocatorFunctions(
        p_alloc_func: *mut extern fn(usize,*mut c_void,) -> *mut c_void,
        p_free_func: *mut extern fn(*mut c_void,*mut c_void,) -> c_void,
        p_user_data: *mut *mut c_void,
    ) -> c_void;

    /// * size: 
    #[allow(non_snake_case)]        
    #[link_name = "?MemAlloc@ImGui@@YAPEAX_K@Z"]
    pub fn MemAlloc(
        size: usize,
    ) -> *mut c_void;

    /// * ptr: 
    #[allow(non_snake_case)]        
    #[link_name = "?MemFree@ImGui@@YAXPEAX@Z"]
    pub fn MemFree(
        ptr: *mut c_void,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?GetPlatformIO@ImGui@@YAAEAUImGuiPlatformIO@@XZ"]
    pub fn GetPlatformIO() -> *mut ImGuiPlatformIO;

    #[allow(non_snake_case)]        
    #[link_name = "?UpdatePlatformWindows@ImGui@@YAXXZ"]
    pub fn UpdatePlatformWindows() -> c_void;

    /// * platform_render_arg: NULL
    /// * renderer_render_arg: NULL
    #[allow(non_snake_case)]        
    #[link_name = "?RenderPlatformWindowsDefault@ImGui@@YAXPEAX0@Z"]
    pub fn RenderPlatformWindowsDefault(
        platform_render_arg: *mut c_void,
        renderer_render_arg: *mut c_void,
    ) -> c_void;

    #[allow(non_snake_case)]        
    #[link_name = "?DestroyPlatformWindows@ImGui@@YAXXZ"]
    pub fn DestroyPlatformWindows() -> c_void;

    /// * id: 
    #[allow(non_snake_case)]        
    #[link_name = "?FindViewportByID@ImGui@@YAPEAUImGuiViewport@@I@Z"]
    pub fn FindViewportByID(
        id: ImGuiID,
    ) -> *mut ImGuiViewport;

    /// * platform_handle: 
    #[allow(non_snake_case)]        
    #[link_name = "?FindViewportByPlatformHandle@ImGui@@YAPEAUImGuiViewport@@PEAX@Z"]
    pub fn FindViewportByPlatformHandle(
        platform_handle: *mut c_void,
    ) -> *mut ImGuiViewport;

    /// * label: 
    /// * items_count: 
    /// * height_in_items: -1
    #[allow(non_snake_case)]        
    #[link_name = "?ListBoxHeader@ImGui@@YA_NPEBDHH@Z"]
    pub fn ListBoxHeader(
        label: *const i8,
        items_count: i32,
        height_in_items: i32,
    ) -> bool;

    /// * label: 
    /// * data_type: 
    /// * p_data: 
    /// * v_speed: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
    /// * data_type: 
    /// * p_data: 
    /// * p_min: 
    /// * p_max: 
    /// * format: 
    /// * power: 
    #[allow(non_snake_case)]        
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
    #[allow(non_snake_case)]        
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
}
