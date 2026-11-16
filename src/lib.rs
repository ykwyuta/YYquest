mod art_sizes;
mod debug;
pub mod game;
mod island;
mod noise;
pub mod render;
mod town;
mod world_map;

#[cfg(target_arch = "wasm32")]
mod web {
    use crate::{
        game::Game,
        render::{Frame, BUFFER_HEIGHT, BUFFER_WIDTH},
    };
    use std::cell::RefCell;

    #[link(wasm_import_module = "screen")]
    extern "C" {
        fn bitmap(pixels: *const u32, width: usize, height: usize);
        fn text(x: i32, y: i32, size: i32, color: u32, utf8: *const u8, len: usize, serif: bool);
    }

    thread_local! {
        static SESSION: RefCell<(Game, Frame)> = RefCell::new((Game::new(), Frame::new()));
    }

    #[no_mangle]
    extern "C" fn draw() {
        SESSION.with(|session| {
            let mut session = session.borrow_mut();
            let (game, frame) = &mut *session;
            frame.draw(game);
            unsafe { bitmap(frame.pixels.as_ptr(), BUFFER_WIDTH, BUFFER_HEIGHT) };
            for label in &frame.labels {
                unsafe {
                    text(
                        label.x,
                        label.y,
                        label.size,
                        label.color,
                        label.text.as_ptr(),
                        label.text.len(),
                        label.serif,
                    );
                }
            }
        });
    }

    #[no_mangle]
    extern "C" fn is_title() -> bool {
        SESSION.with(|session| {
            let session = session.borrow();
            !session.0.map_open && matches!(session.0.scene, crate::game::Scene::Title)
        })
    }

    #[no_mangle]
    extern "C" fn has_map_control() -> bool {
        SESSION.with(|session| {
            let session = session.borrow();
            session.0.map_open || session.0.can_open_map() || session.0.debug_back_control()
        })
    }

    #[no_mangle]
    extern "C" fn animate_traffic(milliseconds: u32) -> bool {
        let redraw = SESSION.with(|session| session.borrow_mut().0.advance_traffic(milliseconds));
        if redraw {
            draw();
        }
        redraw
    }

    #[no_mangle]
    extern "C" fn debug_back_control() -> bool {
        SESSION.with(|session| session.borrow().0.debug_back_control())
    }

    #[no_mangle]
    extern "C" fn key(code: u32, repeat: bool) {
        if SESSION.with(|session| session.borrow_mut().0.input_code(code, repeat)) {
            draw();
        }
    }
}
