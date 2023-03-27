use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;

static QHD: POINT = POINT { x: 2560, y: 1440 };
static FHD: POINT = POINT { x: 1920, y: 1080 };
static BOUNDARIES: [i32; 4] = [-290, 150, 1228, 1630];

fn position_main_to_side(ppos: POINT, pos: POINT) -> POINT {
    POINT {
        x: pos.x,
        y: (ppos.y as f64 / (QHD.y as f64 / FHD.y as f64) as f64) as i32 + BOUNDARIES[1]
    }
}

fn position_side_to_main(ppos: POINT, pos: POINT) -> POINT {
    POINT {
        x: pos.x,
        y: ((ppos.y - BOUNDARIES[1]) as f64 * (QHD.y as f64/ FHD.y as f64)) as i32
    }
}

fn in_main(pos: POINT) -> bool {
    0 <= pos.x && pos.x < QHD.x
}

fn is_switching_main_to_side(pos: POINT, prev: POINT) -> bool {
    in_main(prev) && !in_main(pos)
}

fn is_switching_side_to_main(pos: POINT, prev: POINT) -> bool {
    !in_main(prev) && in_main(pos)
}

fn touching_void_from_side(pos: POINT) -> bool {
    pos.y < BOUNDARIES[1] || BOUNDARIES[2] < pos.y
}

fn is_invalid_border_crossing(pos: POINT, prev: POINT) -> bool {
    !in_main(prev) && in_main(pos) && touching_void_from_side(prev)
}

fn main() {
    unsafe extern "system" fn mouse_proc(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
        static mut PREVIOUS_POS: POINT = POINT { x: 0, y: 0 };
        if code >= HC_ACTION as i32 && wp == WM_MOUSEMOVE as usize {

            let mut send_message = false;

            let mut set_cursor_pos = |pos: POINT| {
                SetCursorPos(pos.x, pos.y);
                send_message = false;
                1
            };
            let mut force_cursor_pos = |pos: POINT| {
                set_cursor_pos(pos);
                PREVIOUS_POS = pos;
                1
            };

            let p = *(lp as *const MSLLHOOKSTRUCT);
            let pos = p.pt;
            if is_invalid_border_crossing(pos, PREVIOUS_POS) {
                set_cursor_pos(POINT { y: pos.y, ..PREVIOUS_POS })
            } else if is_switching_main_to_side(pos, PREVIOUS_POS) {
                force_cursor_pos(position_main_to_side(PREVIOUS_POS, pos))
            } else if is_switching_side_to_main(pos, PREVIOUS_POS) {
                force_cursor_pos(position_side_to_main(PREVIOUS_POS, pos))
            } else {
                PREVIOUS_POS = pos;
                0
            }
        } else {
            CallNextHookEx(HHOOK::default(), code, wp, lp)
        }
    }

    unsafe {
        SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), HINSTANCE::default(), 0);
        loop {
            GetMessageW(std::ptr::null_mut(), 0, 0, 0);
        }
    }
}
