mod art_sizes;
mod campaign;
mod debug;
mod dialogue;
mod economy;
mod fort;
pub mod game;
mod intro;
mod island;
mod noise;
mod path;
pub mod render;
mod town;
mod walk;
mod world_map;

#[cfg(target_arch = "wasm32")]
mod web {
    use crate::{
        game::Game,
        render::{Frame, BUFFER_HEIGHT},
    };
    use std::cell::RefCell;

    #[link(wasm_import_module = "screen")]
    extern "C" {
        fn bitmap(pixels: *const u32, width: usize, height: usize);
        fn text(x: i32, y: i32, size: i32, color: u32, utf8: *const u8, len: usize, serif: bool);
    }

    thread_local! {
        static SESSION: RefCell<(Game, Frame)> = RefCell::new((Game::new(), Frame::new()));
        static SAVE_BUFFER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    #[no_mangle]
    extern "C" fn checkpoint_size() -> usize {
        let checkpoint = SESSION.with(|session| session.borrow().0.checkpoint());
        SAVE_BUFFER.with(|buffer| {
            let mut buffer = buffer.borrow_mut();
            *buffer = checkpoint.unwrap_or_default();
            buffer.len()
        })
    }

    #[no_mangle]
    extern "C" fn checkpoint_ptr() -> *const u8 {
        SAVE_BUFFER.with(|buffer| buffer.borrow().as_ptr())
    }

    #[no_mangle]
    extern "C" fn prepare_save_load(size: usize) -> *mut u8 {
        if size == 0 || size > Game::MAX_SAVE_BYTES {
            return std::ptr::null_mut();
        }
        SAVE_BUFFER.with(|buffer| {
            let mut buffer = buffer.borrow_mut();
            buffer.resize(size, 0);
            buffer.as_mut_ptr()
        })
    }

    #[no_mangle]
    extern "C" fn offer_continue() -> bool {
        let bytes = SAVE_BUFFER.with(|buffer| std::mem::take(&mut *buffer.borrow_mut()));
        SESSION.with(|session| session.borrow_mut().0.offer_continue(&bytes))
    }

    #[no_mangle]
    extern "C" fn take_sound() -> u32 {
        SESSION.with(|session| {
            session
                .borrow_mut()
                .0
                .take_sound()
                .map_or(0, |cue| cue as u32)
        })
    }

    #[no_mangle]
    extern "C" fn draw() {
        SESSION.with(|session| {
            let mut session = session.borrow_mut();
            let (game, frame) = &mut *session;
            frame.draw(game);
            unsafe { bitmap(frame.pixels.as_ptr(), frame.buffer_width(), BUFFER_HEIGHT) };
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
    extern "C" fn set_view_width(width: usize, minimap_inset: usize) {
        let changed = SESSION.with(|session| {
            session
                .borrow_mut()
                .1
                .set_exploration_view(width, minimap_inset)
        });
        if changed {
            draw();
        }
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
            session.0.economy_menu.is_some()
                || session.0.map_open
                || session.0.can_open_map()
                || session.0.debug_back_control()
        })
    }

    #[no_mangle]
    extern "C" fn reading_dialogue() -> bool {
        SESSION.with(|session| {
            let session = session.borrow();
            let game = &session.0;
            if game.map_open {
                game.world_map.campaign.panel.is_some_and(|panel| !matches!(panel, crate::campaign::VoyagePanel::Consent(_))) || game.world_map.fort.as_ref().is_some_and(|fort| {
                    !matches!(fort.panel, crate::fort::Panel::Walk | crate::fort::Panel::Lamps(_) | crate::fort::Panel::Return(_))
                }) || game.world_map.town.as_ref().is_some_and(|town| {
                    matches!(town.panel, crate::town::Panel::Talk { next, .. } if !matches!(next, crate::town::Followup::Choose(..)))
                })
            } else {
                game.message.is_some() || game.battle.as_ref().is_some_and(|battle| battle.log.is_some())
            }
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
        SESSION.with(|session| {
            let session = session.borrow();
            session.0.economy_menu.is_some()
                || session.0.debug_back_control()
                || session.0.confirm_overwrite
        })
    }

    #[no_mangle]
    extern "C" fn key(code: u32, repeat: bool) {
        if SESSION.with(|session| session.borrow_mut().0.input_code(code, repeat)) {
            draw();
        }
    }

    #[no_mangle]
    extern "C" fn press_direction(code: u32, source: u32) {
        if SESSION.with(|session| session.borrow_mut().0.press_direction(code, source)) {
            draw();
        }
    }

    #[no_mangle]
    extern "C" fn release_direction(code: u32, source: u32) {
        SESSION.with(|session| session.borrow_mut().0.release_direction(code, source));
    }

    #[no_mangle]
    extern "C" fn clear_held_direction() {
        SESSION.with(|session| session.borrow_mut().0.clear_held_direction());
    }
}
