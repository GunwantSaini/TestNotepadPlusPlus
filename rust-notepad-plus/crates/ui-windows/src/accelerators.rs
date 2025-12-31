//! Keyboard accelerators for menu shortcuts

use notepad_core::CommandId;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateAcceleratorTableW, TranslateAcceleratorW, ACCEL, ACCEL_VIRT_FLAGS, FCONTROL, HACCEL,
    MSG,
};

// FVIRTKEY constant
const FVIRTKEY: ACCEL_VIRT_FLAGS = ACCEL_VIRT_FLAGS(0x01);

/// Create accelerator table for keyboard shortcuts
pub fn create_accelerators() -> Result<HACCEL, windows::core::Error> {
    // Define keyboard shortcuts
    // Format: ACCEL { fVirt: flags, key: virtual_key_code, cmd: command_id }
    let accelerators = vec![
        // File menu
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'N' as u16,
            cmd: CommandId::FileNew.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'O' as u16,
            cmd: CommandId::FileOpen.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'S' as u16,
            cmd: CommandId::FileSave.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'W' as u16,
            cmd: CommandId::FileClose.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'P' as u16,
            cmd: CommandId::FilePrint.to_menu_id() as u16,
        },
        // Edit menu
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'Z' as u16,
            cmd: CommandId::EditUndo.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'Y' as u16,
            cmd: CommandId::EditRedo.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'X' as u16,
            cmd: CommandId::EditCut.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'C' as u16,
            cmd: CommandId::EditCopy.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'V' as u16,
            cmd: CommandId::EditPaste.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'A' as u16,
            cmd: CommandId::EditSelectAll.to_menu_id() as u16,
        },
        // Search menu
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'F' as u16,
            cmd: CommandId::SearchFind.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'H' as u16,
            cmd: CommandId::SearchReplace.to_menu_id() as u16,
        },
        ACCEL {
            fVirt: FCONTROL | FVIRTKEY,
            key: 'G' as u16,
            cmd: CommandId::SearchGoToLine.to_menu_id() as u16,
        },
        // F3 for Find Next
        ACCEL {
            fVirt: FVIRTKEY, // FVIRTKEY only
            key: 0x72,       // VK_F3
            cmd: CommandId::SearchFindNext.to_menu_id() as u16,
        },
    ];

    unsafe {
        let haccel = CreateAcceleratorTableW(&accelerators)?;
        if haccel.is_invalid() {
            return Err(windows::core::Error::from_win32());
        }
        Ok(haccel)
    }
}

/// Translate accelerator in message loop
pub fn translate_accelerator(hwnd: HWND, haccel: HACCEL, msg: &mut MSG) -> bool {
    unsafe { TranslateAcceleratorW(hwnd, haccel, msg) != 0 }
}
