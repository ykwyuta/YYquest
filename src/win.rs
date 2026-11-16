use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    mem::{size_of, zeroed},
    ptr::{null, null_mut},
};

use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::{Input::XboxController::*, WindowsAndMessaging::*},
};

use yyquest::{
    game::{Game, Key},
    render::{Frame, BUFFER_HEIGHT, BUFFER_WIDTH, HEIGHT, WIDTH},
};

const CLASS: windows_sys::core::PCWSTR = windows_sys::w!("YYquest");
const TRAFFIC_TIMER: usize = 1;
const GAMEPAD_TIMER: usize = 2;
const TRAFFIC_INTERVAL: u32 = 70;

thread_local! {
    static GAMEPAD_BUTTONS: Cell<u16> = const { Cell::new(0) };
    static SESSION: RefCell<(Game, Frame)> = RefCell::new((Game::new(), Frame::new()));
}

pub fn run() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            lpszClassName: CLASS,
            ..zeroed()
        };
        if RegisterClassW(&class) == 0 {
            return;
        }
        let title = windows_sys::w!("残響のアルカディア");
        let window = CreateWindowExW(
            0,
            CLASS,
            title,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            816,
            640,
            null_mut(),
            null_mut(),
            instance,
            null(),
        );
        if window.is_null() {
            return;
        }

        SetTimer(window, TRAFFIC_TIMER, TRAFFIC_INTERVAL, None);
        SetTimer(window, GAMEPAD_TIMER, 16, None);
        ShowWindow(window, SW_SHOW);
        let mut message: MSG = zeroed();
        while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_KEYDOWN => {
            let code = wparam as u32;
            if Key::from_code(code).is_some() {
                let repeat = lparam & (1 << 30) != 0;
                if SESSION.with(|session| session.borrow_mut().0.input_code(code, repeat)) {
                    InvalidateRect(window, null(), 0);
                }
                return 0;
            }
        }
        WM_TIMER if wparam == GAMEPAD_TIMER => {
            if GetForegroundWindow() == window {
                if poll_gamepad() {
                    InvalidateRect(window, null(), 0);
                }
            } else {
                GAMEPAD_BUTTONS.set(0);
            }
            return 0;
        }
        WM_TIMER if wparam == TRAFFIC_TIMER => {
            if IsIconic(window) == 0
                && SESSION.with(|session| session.borrow_mut().0.advance_traffic(TRAFFIC_INTERVAL))
            {
                InvalidateRect(window, null(), 0);
            }
            return 0;
        }
        WM_PAINT => {
            let mut paint: PAINTSTRUCT = zeroed();
            let dc = BeginPaint(window, &mut paint);
            let mut client: RECT = zeroed();
            GetClientRect(window, &mut client);
            SESSION.with(|session| {
                let mut session = session.borrow_mut();
                let (game, frame) = &mut *session;
                frame.draw(game);
                paint_frame(dc, client, frame);
            });
            EndPaint(window, &paint);
            return 0;
        }
        WM_ERASEBKGND => return 1,
        WM_DESTROY => {
            KillTimer(window, TRAFFIC_TIMER);
            KillTimer(window, GAMEPAD_TIMER);
            PostQuitMessage(0);
            return 0;
        }
        _ => {}
    }
    DefWindowProcW(window, message, wparam, lparam)
}

unsafe fn poll_gamepad() -> bool {
    let mut state: XINPUT_STATE = zeroed();
    let buttons = (0..4)
        .find(|&index| XInputGetState(index, &mut state) == 0)
        .map_or(0, |_| state.Gamepad.wButtons);
    let pressed = buttons & !GAMEPAD_BUTTONS.replace(buttons);
    if pressed == 0 {
        return false;
    }
    SESSION.with(|session| {
        let mut session = session.borrow_mut();
        for (button, code) in [
            (XINPUT_GAMEPAD_DPAD_UP, 38),
            (XINPUT_GAMEPAD_DPAD_DOWN, 40),
            (XINPUT_GAMEPAD_DPAD_LEFT, 37),
            (XINPUT_GAMEPAD_DPAD_RIGHT, 39),
            (XINPUT_GAMEPAD_B, 27),
            (XINPUT_GAMEPAD_A, 13),
        ] {
            if pressed & button != 0 {
                session.0.input_code(code, false);
            }
        }
    });
    true
}

unsafe fn paint_frame(dc: HDC, client: RECT, frame: &Frame) {
    let scale =
        (f64::from(client.right) / WIDTH as f64).min(f64::from(client.bottom) / HEIGHT as f64);
    if scale <= 0.0 {
        return;
    }
    let width = (WIDTH as f64 * scale) as i32;
    let height = (HEIGHT as f64 * scale) as i32;
    let left = (client.right - width) / 2;
    let top = (client.bottom - height) / 2;

    let bitmap = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: BUFFER_WIDTH as i32,
            biHeight: -(BUFFER_HEIGHT as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            ..zeroed()
        },
        ..zeroed()
    };

    // Compose text and pixels off-screen before presenting the frame.
    let buffer = CreateCompatibleDC(dc);
    if buffer.is_null() {
        return;
    }
    let surface = CreateCompatibleBitmap(dc, client.right, client.bottom);
    if surface.is_null() {
        DeleteDC(buffer);
        return;
    }
    let previous = SelectObject(buffer, surface);
    FillRect(buffer, &client, GetStockObject(BLACK_BRUSH));
    SetStretchBltMode(buffer, HALFTONE as i32);
    SetBrushOrgEx(buffer, left, top, null_mut());
    StretchDIBits(
        buffer,
        left,
        top,
        width,
        height,
        0,
        0,
        BUFFER_WIDTH as i32,
        BUFFER_HEIGHT as i32,
        frame.pixels.as_ptr().cast::<c_void>(),
        &bitmap,
        DIB_RGB_COLORS,
        SRCCOPY,
    );
    SetBkMode(buffer, TRANSPARENT as i32);
    let font_name = windows_sys::w!("Yu Gothic UI");
    for label in &frame.labels {
        let font = CreateFontW(
            -(f64::from(label.size) * scale).round() as i32,
            0,
            0,
            0,
            FW_SEMIBOLD as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET.into(),
            0,
            0,
            0,
            0,
            font_name,
        );
        let previous_font = SelectObject(buffer, font);
        let color = (label.color & 0x00ff00) | (label.color >> 16) | ((label.color & 0xff) << 16);
        SetTextColor(buffer, color);
        let text: Vec<_> = label.text.encode_utf16().collect();
        TextOutW(
            buffer,
            left + (f64::from(label.x) * scale) as i32,
            top + (f64::from(label.y) * scale) as i32,
            text.as_ptr(),
            text.len() as i32,
        );
        SelectObject(buffer, previous_font);
        DeleteObject(font);
    }
    BitBlt(dc, 0, 0, client.right, client.bottom, buffer, 0, 0, SRCCOPY);
    SelectObject(buffer, previous);
    DeleteObject(surface);
    DeleteDC(buffer);
}
