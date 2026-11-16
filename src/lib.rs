pub mod game;
mod island;
mod noise;
pub mod render;
mod world_map;

#[cfg(target_arch = "wasm32")]
mod web {
    use crate::{
        game::{Game, Key},
        render::Frame,
    };
    use std::cell::RefCell;

    #[link(wasm_import_module = "screen")]
    extern "C" {
        fn bitmap(pixels: *const u32);
        fn text(x: i32, y: i32, size: i32, color: u32, utf8: *const u8, len: usize);
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
            unsafe { bitmap(frame.pixels.as_ptr()) };
            for label in &frame.labels {
                unsafe {
                    text(
                        label.x,
                        label.y,
                        label.size,
                        label.color,
                        label.text.as_ptr(),
                        label.text.len(),
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
            session.0.map_open || session.0.can_open_map()
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
    extern "C" fn key(code: u32) {
        if let Some(key) = Key::from_code(code) {
            SESSION.with(|session| session.borrow_mut().0.input(key));
            draw();
        }
    }
}
