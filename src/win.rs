use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    fs,
    io::{self, Read, Write},
    mem::{size_of, zeroed},
    path::PathBuf,
    ptr::{null, null_mut},
    time::Instant,
};

use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::{
            KeyboardAndMouse::{GetCapture, ReleaseCapture, SetCapture},
            XboxController::*,
        },
        WindowsAndMessaging::*,
    },
};

use yyquest::{
    game::{Game, Key, SoundCue},
    render::{Frame, BUFFER_HEIGHT, BUFFER_WIDTH, HEIGHT, WIDTH},
};

#[link(name = "winmm")]
extern "system" {
    fn PlaySoundW(sound: *const u16, module: *mut c_void, flags: u32) -> i32;
}

const CLASS: windows_sys::core::PCWSTR = windows_sys::w!("YYquest");
const TRAFFIC_TIMER: usize = 1;
const GAMEPAD_TIMER: usize = 2;
const TRAFFIC_INTERVAL: u32 = 16;
const KEYBOARD: u32 = 1;
const GAMEPAD: u32 = 2;
const MOUSE: u32 = 3;
const BUTTON_SIZE: i32 = 40;
const BUTTON_PITCH: i32 = 46;
const PAD_WIDTH: i32 = 2 * BUTTON_PITCH + BUTTON_SIZE;
const PAD_HEIGHT: i32 = BUTTON_PITCH + BUTTON_SIZE;
const PAD_MARGIN: i32 = 10;
const PAD_KEYS: [(u32, (i32, i32), u16); 4] = [
    (38, (1, 0), '↑' as u16),
    (37, (0, 1), '←' as u16),
    (40, (1, 1), '↓' as u16),
    (39, (2, 1), '→' as u16),
];

thread_local! {
    static GAMEPAD_BUTTONS: Cell<u16> = const { Cell::new(0) };
    static MOUSE_DIRECTION: Cell<Option<u32>> = const { Cell::new(None) };
    static LAST_FRAME: Cell<Instant> = Cell::new(Instant::now());
    static SESSION: RefCell<(Game, Frame)> = RefCell::new((Game::new(), Frame::new()));
    static LAST_SAVE: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
    static SAVE_FAILED: Cell<bool> = const { Cell::new(false) };
    static SOUND_ENABLED: Cell<bool> = const { Cell::new(true) };
}

fn play_game_sound() {
    use std::sync::OnceLock;
    static SOUNDS: OnceLock<[Vec<u16>; 4]> = OnceLock::new();
    const SND_ASYNC: u32 = 0x0001;
    const SND_NODEFAULT: u32 = 0x0002;
    const SND_MEMORY: u32 = 0x0004;
    const SND_NOSTOP: u32 = 0x0010;

    let cue = SESSION.with(|session| session.borrow_mut().0.take_sound());
    let Some(cue) = cue.filter(|_| SOUND_ENABLED.get()) else {
        return;
    };
    let sounds = SOUNDS.get_or_init(|| {
        [
            make_sound(&[587]),
            make_sound(&[196, 147]),
            make_sound(&[523, 659, 784]),
            make_sound(&[392, 523, 659, 784]),
        ]
    });
    let index = match cue {
        SoundCue::Confirm => 0,
        SoundCue::Hit => 1,
        SoundCue::Victory => 2,
        SoundCue::Chapter => 3,
    };
    unsafe {
        let flags = SND_ASYNC
            | SND_MEMORY
            | SND_NODEFAULT
            | if cue == SoundCue::Confirm {
                SND_NOSTOP
            } else {
                0
            };
        PlaySoundW(sounds[index].as_ptr(), null_mut(), flags);
    }
}

fn make_sound(notes: &[u32]) -> Vec<u16> {
    const RATE: u32 = 16_000;
    const NOTE_SAMPLES: u32 = 1_200;
    let size = notes.len() as u32 * NOTE_SAMPLES * 2;
    let mut wave = Vec::new();
    wave.extend(b"RIFF");
    wave.extend((size + 36).to_le_bytes());
    wave.extend(b"WAVEfmt ");
    wave.extend(16_u32.to_le_bytes());
    wave.extend(1_u16.to_le_bytes());
    wave.extend(1_u16.to_le_bytes());
    wave.extend(RATE.to_le_bytes());
    wave.extend((RATE * 2).to_le_bytes());
    wave.extend(2_u16.to_le_bytes());
    wave.extend(16_u16.to_le_bytes());
    wave.extend(b"data");
    wave.extend(size.to_le_bytes());
    for &hz in notes {
        for sample in 0..NOTE_SAMPLES {
            let phase = (sample * hz % RATE) as f32 / RATE as f32;
            let envelope =
                (sample as f32 / 60.0).min(1.0) * (1.0 - sample as f32 / NOTE_SAMPLES as f32);
            let amplitude = ((1.0 - 4.0 * (phase - 0.5).abs()) * envelope * 2_000.0) as i16;
            wave.extend(amplitude.to_le_bytes());
        }
    }
    wave.chunks_exact(2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
        .collect()
}

fn save_path() -> io::Result<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(path).join("YYquest").join("adventure.json"))
        .ok_or_else(|| io::Error::other("LOCALAPPDATA is unavailable"))
}

fn load_adventure() -> io::Result<()> {
    let file = match fs::File::open(save_path()?) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    let mut bytes = Vec::new();
    file.take(Game::MAX_SAVE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if !SESSION.with(|session| session.borrow_mut().0.offer_continue(&bytes)) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid save"));
    }
    LAST_SAVE.replace(Some(bytes));
    Ok(())
}

fn write_adventure(bytes: &[u8]) -> io::Result<()> {
    let path = save_path()?;
    fs::create_dir_all(path.parent().unwrap())?;
    let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
    let result = (|| {
        let mut file = fs::File::create(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary, &path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

fn save_adventure(window: HWND) {
    let Some(bytes) = SESSION.with(|session| session.borrow().0.checkpoint()) else {
        return;
    };
    if LAST_SAVE.with(|last| last.borrow().as_deref() == Some(bytes.as_slice())) {
        return;
    }
    if write_adventure(&bytes).is_ok() {
        LAST_SAVE.replace(Some(bytes));
        SAVE_FAILED.set(false);
    } else if !SAVE_FAILED.replace(true) {
        unsafe {
            MessageBoxW(
                window,
                windows_sys::w!("冒険の記録を保存できませんでした。"),
                CLASS,
                MB_OK | MB_ICONWARNING,
            );
        }
    }
}

struct Layout {
    game: RECT,
    pad: (i32, i32),
}

impl Layout {
    fn new(client: RECT) -> Self {
        let width = client.right;
        let height = client.bottom;
        let side_width = PAD_WIDTH + 2 * PAD_MARGIN;
        let side = width >= height * WIDTH as i32 / HEIGHT as i32 + side_width;
        let (area_x, area_width, area_height, pad) = if side {
            (
                side_width,
                width - side_width,
                height,
                (PAD_MARGIN, height - PAD_HEIGHT - PAD_MARGIN),
            )
        } else {
            let row = height - PAD_HEIGHT - 2 * PAD_MARGIN;
            (0, width, row, ((width - PAD_WIDTH) / 2, row + PAD_MARGIN))
        };
        let scale = (f64::from(area_width) / WIDTH as f64)
            .min(f64::from(area_height) / HEIGHT as f64)
            .max(0.0);
        let game_width = (WIDTH as f64 * scale) as i32;
        let game_height = (HEIGHT as f64 * scale) as i32;
        let left = area_x + (area_width - game_width) / 2;
        let top = (area_height - game_height) / 2;
        Self {
            game: RECT {
                left,
                top,
                right: left + game_width,
                bottom: top + game_height,
            },
            pad,
        }
    }

    fn buttons(&self) -> impl Iterator<Item = (u32, RECT, u16)> + '_ {
        PAD_KEYS.into_iter().map(|(code, (col, row), glyph)| {
            let left = self.pad.0 + col * BUTTON_PITCH;
            let top = self.pad.1 + row * BUTTON_PITCH;
            (
                code,
                RECT {
                    left,
                    top,
                    right: left + BUTTON_SIZE,
                    bottom: top + BUTTON_SIZE,
                },
                glyph,
            )
        })
    }
}

unsafe fn create_app_icon(
    instance: windows_sys::Win32::Foundation::HINSTANCE,
    side: i32,
    bgra: &[u8],
) -> HICON {
    let and_mask = vec![0_u8; ((side as usize + 7) / 8) * side as usize];
    CreateIcon(
        instance,
        side,
        side,
        1,
        32,
        and_mask.as_ptr(),
        bgra.as_ptr(),
    )
}

pub fn run() {
    const ICON_16: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/app-icon-16.bgra"));
    const ICON_32: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/app-icon-32.bgra"));
    unsafe {
        let instance = GetModuleHandleW(null());
        let big_icon = create_app_icon(instance, 32, ICON_32);
        let small_icon = create_app_icon(instance, 16, ICON_16);
        let class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            hIcon: big_icon,
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
        if !small_icon.is_null() {
            SendMessageW(
                window,
                WM_SETICON,
                ICON_SMALL as WPARAM,
                small_icon as LPARAM,
            );
        }

        if load_adventure().is_err() {
            MessageBoxW(
                window,
                windows_sys::w!("冒険の記録を読み込めませんでした。"),
                title,
                MB_OK | MB_ICONWARNING,
            );
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
        WM_GETMINMAXINFO => {
            let sizing = &mut *(lparam as *mut MINMAXINFO);
            sizing.ptMinTrackSize.x = sizing.ptMinTrackSize.x.max(320);
            sizing.ptMinTrackSize.y = sizing.ptMinTrackSize.y.max(300);
            return 0;
        }
        WM_LBUTTONDOWN => {
            let mut client = RECT::default();
            GetClientRect(window, &mut client);
            let x = i32::from(lparam as i16);
            let y = i32::from((lparam >> 16) as i16);
            if let Some((code, _, _)) = Layout::new(client).buttons().find(|(_, rect, _)| {
                (rect.left..rect.right).contains(&x) && (rect.top..rect.bottom).contains(&y)
            }) {
                SetCapture(window);
                MOUSE_DIRECTION.set(Some(code));
                SESSION.with(|session| session.borrow_mut().0.press_direction(code, MOUSE));
                InvalidateRect(window, null(), 0);
            }
            return 0;
        }
        WM_LBUTTONUP | WM_CANCELMODE => {
            release_mouse(window);
            if GetCapture() == window {
                ReleaseCapture();
            }
            return 0;
        }
        WM_CAPTURECHANGED => {
            release_mouse(window);
            return 0;
        }
        WM_KEYDOWN => {
            let code = wparam as u32;
            if code == 0x71 && lparam & (1 << 30) == 0 {
                if SOUND_ENABLED.replace(!SOUND_ENABLED.get()) {
                    PlaySoundW(null(), null_mut(), 0);
                }
                return 0;
            }
            if let Some(key) = Key::from_code(code) {
                let repeat = lparam & (1 << 30) != 0;
                if !repeat && key == Key::Confirm {
                    save_adventure(window);
                }
                if !repeat
                    && SESSION.with(|session| {
                        let mut session = session.borrow_mut();
                        session.0.press_direction(code, KEYBOARD)
                            || session.0.input_code(code, false)
                    })
                {
                    InvalidateRect(window, null(), 0);
                    play_game_sound();
                    if matches!(key, Key::Confirm | Key::Back | Key::Map | Key::Journal) {
                        save_adventure(window);
                    }
                }
                return 0;
            }
        }
        WM_KEYUP => {
            SESSION.with(|session| {
                session
                    .borrow_mut()
                    .0
                    .release_direction(wparam as u32, KEYBOARD)
            });
        }
        WM_KILLFOCUS => {
            SESSION.with(|session| session.borrow_mut().0.clear_held_direction());
            release_mouse(window);
            if GetCapture() == window {
                ReleaseCapture();
            }
        }
        WM_TIMER if wparam == GAMEPAD_TIMER => {
            if GetForegroundWindow() == window {
                if poll_gamepad(window) {
                    InvalidateRect(window, null(), 0);
                }
            } else {
                GAMEPAD_BUTTONS.set(0);
            }
            return 0;
        }
        WM_TIMER if wparam == TRAFFIC_TIMER => {
            let now = Instant::now();
            let elapsed =
                LAST_FRAME.with(|last| now.duration_since(last.replace(now)).as_millis()) as u32;
            if IsIconic(window) == 0
                && SESSION.with(|session| session.borrow_mut().0.advance_traffic(elapsed))
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
        WM_CLOSE => save_adventure(window),
        _ => {}
    }
    DefWindowProcW(window, message, wparam, lparam)
}

unsafe fn release_mouse(window: HWND) {
    if let Some(code) = MOUSE_DIRECTION.take() {
        SESSION.with(|session| session.borrow_mut().0.release_direction(code, MOUSE));
        InvalidateRect(window, null(), 0);
    }
}

unsafe fn poll_gamepad(window: HWND) -> bool {
    let mut state: XINPUT_STATE = zeroed();
    let buttons = (0..4)
        .find(|&index| XInputGetState(index, &mut state) == 0)
        .map_or(0, |_| state.Gamepad.wButtons);
    let previous = GAMEPAD_BUTTONS.replace(buttons);
    if buttons == previous {
        return false;
    }
    let pressed = buttons & !previous;
    if pressed & XINPUT_GAMEPAD_A != 0 {
        save_adventure(window);
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
            if (buttons ^ previous) & button == 0 {
                continue;
            }
            if buttons & button == 0 {
                session.0.release_direction(code, GAMEPAD);
            } else if !session.0.press_direction(code, GAMEPAD) {
                session.0.input_code(code, false);
            }
        }
    });
    play_game_sound();
    if pressed & (XINPUT_GAMEPAD_A | XINPUT_GAMEPAD_B) != 0 {
        save_adventure(window);
    }
    true
}

unsafe fn paint_frame(dc: HDC, client: RECT, frame: &Frame) {
    let layout = Layout::new(client);
    let RECT {
        left,
        top,
        right,
        bottom,
    } = layout.game;
    let width = right - left;
    let height = bottom - top;
    let scale = f64::from(width) / WIDTH as f64;
    if scale <= 0.0 {
        return;
    }
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
    let background = CreateSolidBrush(0x1e1410);
    FillRect(buffer, &client, background);
    DeleteObject(background);
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
    for label in &frame.labels {
        let font_name = if label.serif {
            windows_sys::w!("Yu Mincho")
        } else {
            windows_sys::w!("Yu Gothic UI")
        };
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
    paint_dpad(buffer, &layout);
    BitBlt(dc, 0, 0, client.right, client.bottom, buffer, 0, 0, SRCCOPY);
    SelectObject(buffer, previous);
    DeleteObject(surface);
    DeleteDC(buffer);
}

unsafe fn paint_dpad(dc: HDC, layout: &Layout) {
    let font = CreateFontW(
        -23,
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
        windows_sys::w!("Segoe UI Symbol"),
    );
    let previous_font = SelectObject(dc, font);
    let outline = CreatePen(PS_SOLID, 1, 0x82725e);
    let previous_pen = SelectObject(dc, outline);
    let normal = CreateSolidBrush(0x362821);
    let pressed = CreateSolidBrush(0x55467c);
    SetTextColor(dc, 0xd6e8fc);
    let held = MOUSE_DIRECTION.get();
    for (code, mut rect, glyph) in layout.buttons() {
        let previous_brush = SelectObject(dc, if held == Some(code) { pressed } else { normal });
        RoundRect(dc, rect.left, rect.top, rect.right, rect.bottom, 18, 18);
        SelectObject(dc, previous_brush);
        DrawTextW(
            dc,
            &glyph,
            1,
            &mut rect,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    }
    SelectObject(dc, previous_pen);
    SelectObject(dc, previous_font);
    DeleteObject(normal);
    DeleteObject(pressed);
    DeleteObject(outline);
    DeleteObject(font);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpad_click_and_keyboard_arrows_move_through_the_same_window() {
        SOUND_ENABLED.set(false);
        for (width, height) in [(800, 600), (1300, 600), (320, 260)] {
            let layout = Layout::new(RECT {
                right: width,
                bottom: height,
                ..RECT::default()
            });
            assert!(layout.buttons().all(|(_, button, _)| {
                button.left >= 0
                    && button.right <= width
                    && button.top >= 0
                    && button.bottom <= height
                    && (button.right <= layout.game.left || button.top >= layout.game.bottom)
            }));
        }
        unsafe {
            let window = CreateWindowExW(
                0,
                windows_sys::w!("STATIC"),
                null(),
                WS_POPUP,
                0,
                0,
                800,
                600,
                null_mut(),
                null_mut(),
                GetModuleHandleW(null()),
                null(),
            );
            assert!(!window.is_null());
            SESSION.with(|session| {
                let game = &mut session.borrow_mut().0;
                game.input(Key::Confirm);
                game.input(Key::Confirm);
            });

            let mut client = RECT::default();
            GetClientRect(window, &mut client);
            let (_, up, _) = Layout::new(client)
                .buttons()
                .find(|(code, _, _)| *code == 38)
                .unwrap();
            let point = ((up.top + 20) << 16 | (up.left + 20)) as LPARAM;
            window_proc(window, WM_LBUTTONDOWN, 0, point);
            window_proc(window, WM_LBUTTONUP, 0, point);
            SESSION.with(|session| {
                let game = &mut session.borrow_mut().0;
                game.advance_traffic(300);
                assert_eq!((game.x, game.y), (3, 3));
            });
            window_proc(window, WM_KEYDOWN, 38, 0);
            window_proc(window, WM_KEYUP, 38, 0);
            SESSION.with(|session| {
                let game = &session.borrow().0;
                assert_eq!((game.x, game.y), (3, 2));
            });
            DestroyWindow(window);
        }
    }
}
