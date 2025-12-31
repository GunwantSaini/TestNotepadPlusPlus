//! Command system for menu and keyboard shortcuts
//!
//! Corresponds to the C++ menuCmdID.h and command routing system

/// Command identifier
///
/// Corresponds to the IDM_* constants in C++ (PowerEditor/src/menuCmdID.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandId {
    // File menu commands
    FileNew,
    FileOpen,
    FileSave,
    FileSaveAs,
    FileSaveAll,
    FileClose,
    FileCloseAll,
    FilePrint,
    FileExit,
    FileReload,

    // Edit menu commands
    EditUndo,
    EditRedo,
    EditCut,
    EditCopy,
    EditPaste,
    EditDelete,
    EditSelectAll,
    EditDuplicate,
    EditIndent,
    EditUnindent,
    EditUpperCase,
    EditLowerCase,
    EditLineComment,
    EditBlockComment,

    // Search menu commands
    SearchFind,
    SearchFindNext,
    SearchFindPrevious,
    SearchReplace,
    SearchFindInFiles,
    SearchIncSearch,
    SearchGoToLine,

    // View menu commands
    ViewFullScreen,
    ViewPostIt,
    ViewShowSymbol,
    ViewZoomIn,
    ViewZoomOut,
    ViewZoomRestore,
    ViewSplitHorizontal,
    ViewSplitVertical,
    ViewUnsplit,
    ViewShowWhitespace,
    ViewShowEol,
    ViewShowIndent,
    ViewDocumentMap,
    ViewFunctionList,
    ViewProjectPanel,

    // Encoding commands
    EncodingAnsi,
    EncodingUtf8,
    EncodingUtf8Bom,
    EncodingUtf16Le,
    EncodingUtf16LeBom,
    EncodingUtf16Be,
    EncodingUtf16BeBom,

    // Language commands
    LanguageText,
    LanguageC,
    LanguageCpp,
    LanguageJava,
    LanguagePython,
    LanguageRust,
    LanguageJavascript,
    LanguageHtml,
    LanguageCss,
    LanguageSql,
    LanguageXml,
    LanguageJson,

    // Settings commands
    SettingsPreferences,
    SettingsShortcutMapper,
    SettingsStyleConfigurator,

    // Macro commands
    MacroStartRecord,
    MacroStopRecord,
    MacroPlayback,
    MacroSave,
    MacroRunMultiple,

    // Run commands
    RunRun,
    RunCommand,

    // Plugins commands
    PluginsAdmin,

    // Window commands
    WindowMoveToOtherView,
    WindowCloneToOtherView,
    WindowNext,
    WindowPrevious,

    // Help commands
    HelpAbout,
    HelpOnlineHelp,
    HelpUpdateNpp,

    // Custom command (for plugins, etc.)
    Custom(u32),
}

/// Command structure
#[derive(Debug, Clone)]
pub struct Command {
    pub id: CommandId,
    pub name: String,
    pub description: String,
    pub shortcut: Option<Shortcut>,
}

/// Keyboard shortcut
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortcut {
    pub key: VirtualKey,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

/// Virtual key codes (subset of Windows VK_* constants)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum VirtualKey {
    // Letters
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    // Function keys
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,

    // Special keys
    Escape = 0x1B,
    Enter = 0x0D,
    Tab = 0x09,
    Backspace = 0x08,
    Delete = 0x2E,
    Insert = 0x2D,
    Home = 0x24,
    End = 0x23,
    PageUp = 0x21,
    PageDown = 0x22,

    // Arrow keys
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
}

impl CommandId {
    /// Get the numeric ID for this command (for Win32 menus)
    pub fn to_menu_id(&self) -> u32 {
        match self {
            CommandId::FileNew => 41001,
            CommandId::FileOpen => 41002,
            CommandId::FileSave => 41006,
            CommandId::FileSaveAs => 41007,
            CommandId::FileSaveAll => 41008,
            CommandId::FileClose => 41005,
            CommandId::FileCloseAll => 41009,
            CommandId::FilePrint => 41012,
            CommandId::FileExit => 41004,
            CommandId::FileReload => 41013,

            CommandId::EditUndo => 42001,
            CommandId::EditRedo => 42002,
            CommandId::EditCut => 42003,
            CommandId::EditCopy => 42004,
            CommandId::EditPaste => 42005,
            CommandId::EditDelete => 42006,
            CommandId::EditSelectAll => 42010,

            CommandId::SearchFind => 43001,
            CommandId::SearchFindNext => 43002,
            CommandId::SearchReplace => 43004,
            CommandId::SearchGoToLine => 43020,

            CommandId::ViewFullScreen => 44001,
            CommandId::ViewZoomIn => 44010,
            CommandId::ViewZoomOut => 44011,

            CommandId::Custom(id) => *id,
            _ => 0, // TODO: Complete mapping
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_id_to_menu_id() {
        assert_eq!(CommandId::FileNew.to_menu_id(), 41001);
        assert_eq!(CommandId::FileSave.to_menu_id(), 41006);
    }

    #[test]
    fn test_shortcut() {
        let shortcut = Shortcut {
            key: VirtualKey::S,
            ctrl: true,
            alt: false,
            shift: false,
        };

        assert_eq!(shortcut.key, VirtualKey::S);
        assert!(shortcut.ctrl);
    }
}
