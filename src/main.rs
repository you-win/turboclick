use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState};
use mouce::{
    common::{MouseButton, MouseEvent},
    Mouse,
};

fn main() -> anyhow::Result<()> {
    let mut delay = 0.05;

    let args = std::env::args();
    for arg in args {
        if let Ok(v) = arg.parse() {
            delay = v;
        }
    }

    let can_toggle = Arc::new(AtomicBool::new(false));
    let should_turbo = Arc::new(AtomicBool::new(false));
    let should_run = Arc::new(AtomicBool::new(true));

    let sr = should_run.clone();
    ctrlc::set_handler(move || {
        println!("Stopping!");
        sr.store(false, Ordering::Relaxed);
    })?;

    let device_query = DeviceState::new();
    let mut mouse = Mouse::new();

    let st = should_turbo.clone();
    mouse.hook(Box::new(move |event| match event {
        MouseEvent::Press(b) => match b {
            MouseButton::Left => {
                if !device_query
                    .get_keys()
                    .contains(&device_query::Keycode::Key8)
                    || !can_toggle.load(Ordering::Relaxed)
                {
                    return;
                }
                st.store(!st.load(Ordering::Relaxed), Ordering::Relaxed);

                can_toggle.store(false, Ordering::Relaxed);
            }
            _ => {}
        },
        MouseEvent::Release(b) => match b {
            MouseButton::Left => {
                if !device_query
                    .get_keys()
                    .contains(&device_query::Keycode::Key8)
                    && !can_toggle.load(Ordering::Relaxed)
                {
                    can_toggle.store(true, Ordering::Relaxed);
                }
            }
            _ => {}
        },
        _ => {}
    }))?;

    while should_run.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_secs_f32(delay));

        if should_turbo.load(Ordering::Relaxed) {
            mouse.click_button(&MouseButton::Left)?;
        }
    }

    mouse.unhook_all()?;

    Ok(())
}
