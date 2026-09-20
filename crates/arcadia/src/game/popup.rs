use std::ptr;

use crate::offsets;

pub const POPUP_DELETE_CONFIRM: u64 = 0x58ff_ff17_f737_7bb7;

pub const RESULT_NONE: i32 = 0;
pub const RESULT_YES: i32 = 2;

#[repr(C)]
struct ArgVector {
    begin: *const *const u16,
    end: *const *const u16,
    capacity: *const *const u16,
}

#[repr(C)]
struct PopupHolder {
    unk0: *mut u8,
    manager: *mut PopupManager,
    unk2: *mut u8,
}

#[repr(C)]
struct PopupManager {
    unk0: [u8; 0xf8],
    state: i32,
    unkfc: [u8; 0x118 - 0xfc],
    result: i32,
    unk11c: [u8; 0x142 - 0x11c],
    popup_open: u8,
}

const _: () = {
    assert!(std::mem::offset_of!(PopupHolder, manager) == 8);
    assert!(std::mem::offset_of!(PopupManager, state) == 0xf8);
    assert!(std::mem::offset_of!(PopupManager, result) == 0x118);
    assert!(std::mem::offset_of!(PopupManager, popup_open) == 0x142);
};

#[skyline::from_offset(offsets::popup_open())]
unsafe fn app_popup_manager_open_popup(holder: *mut PopupHolder, id: u64, args: *const ArgVector);

unsafe fn holder() -> *mut PopupHolder {
    *(patterns::offset_to_addr(offsets::g_popup_holder()) as *const *mut PopupHolder)
}

unsafe fn manager() -> *mut PopupManager {
    match holder().as_ref() {
        Some(holder) => holder.manager,
        None => ptr::null_mut(),
    }
}

pub unsafe fn open(id: u64, argument: *const u16) -> bool {
    let holder = holder();
    if holder.is_null() {
        return false;
    }

    let arguments = [argument];
    let vector = ArgVector {
        begin: arguments.as_ptr(),
        end: arguments.as_ptr().add(1),
        capacity: arguments.as_ptr().add(1),
    };

    app_popup_manager_open_popup(holder, id, &vector);
    true
}

pub unsafe fn is_open() -> bool {
    match manager().as_ref() {
        Some(manager) => manager.state != 0 || manager.popup_open != 0,
        None => false,
    }
}

pub unsafe fn result() -> i32 {
    match manager().as_ref() {
        Some(manager) => manager.result,
        None => RESULT_NONE,
    }
}
