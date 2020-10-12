use winit::{
    event::Event, event::WindowEvent, event_loop::ControlFlow, event_loop::EventLoop,
    window::Window,
};

pub fn run(event_loop: EventLoop<()>, window: Window) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::DeviceEvent { device_id, event } => match event {
                winit::event::DeviceEvent::Added => {}
                winit::event::DeviceEvent::Removed => {}
                winit::event::DeviceEvent::MouseMotion { delta } => {}
                winit::event::DeviceEvent::MouseWheel { delta } => {}
                winit::event::DeviceEvent::Motion { axis, value } => {}
                winit::event::DeviceEvent::Button { button, state } => {}
                winit::event::DeviceEvent::Key(key) => {
                    if let Some(keycode) = key.virtual_keycode {
                        match keycode {
                            winit::event::VirtualKeyCode::Key1 => {}
                            winit::event::VirtualKeyCode::Key2 => {}
                            winit::event::VirtualKeyCode::Key3 => {}
                            winit::event::VirtualKeyCode::Key4 => {}
                            winit::event::VirtualKeyCode::Key5 => {}
                            winit::event::VirtualKeyCode::Key6 => {}
                            winit::event::VirtualKeyCode::Key7 => {}
                            winit::event::VirtualKeyCode::Key8 => {}
                            winit::event::VirtualKeyCode::Key9 => {}
                            winit::event::VirtualKeyCode::Key0 => {}
                            winit::event::VirtualKeyCode::A => {}
                            winit::event::VirtualKeyCode::B => {}
                            winit::event::VirtualKeyCode::C => {}
                            winit::event::VirtualKeyCode::D => {}
                            winit::event::VirtualKeyCode::E => {}
                            winit::event::VirtualKeyCode::F => {}
                            winit::event::VirtualKeyCode::G => {}
                            winit::event::VirtualKeyCode::H => {}
                            winit::event::VirtualKeyCode::I => {}
                            winit::event::VirtualKeyCode::J => {}
                            winit::event::VirtualKeyCode::K => {}
                            winit::event::VirtualKeyCode::L => {}
                            winit::event::VirtualKeyCode::M => {}
                            winit::event::VirtualKeyCode::N => {}
                            winit::event::VirtualKeyCode::O => {}
                            winit::event::VirtualKeyCode::P => {}
                            winit::event::VirtualKeyCode::Q => {}
                            winit::event::VirtualKeyCode::R => {}
                            winit::event::VirtualKeyCode::S => {}
                            winit::event::VirtualKeyCode::T => {}
                            winit::event::VirtualKeyCode::U => {}
                            winit::event::VirtualKeyCode::V => {}
                            winit::event::VirtualKeyCode::W => {}
                            winit::event::VirtualKeyCode::X => {}
                            winit::event::VirtualKeyCode::Y => {}
                            winit::event::VirtualKeyCode::Z => {}
                            winit::event::VirtualKeyCode::Escape => {
                                *control_flow = ControlFlow::Exit
                            }
                            winit::event::VirtualKeyCode::F1 => {}
                            winit::event::VirtualKeyCode::F2 => {}
                            winit::event::VirtualKeyCode::F3 => {}
                            winit::event::VirtualKeyCode::F4 => {}
                            winit::event::VirtualKeyCode::F5 => {}
                            winit::event::VirtualKeyCode::F6 => {}
                            winit::event::VirtualKeyCode::F7 => {}
                            winit::event::VirtualKeyCode::F8 => {}
                            winit::event::VirtualKeyCode::F9 => {}
                            winit::event::VirtualKeyCode::F10 => {}
                            winit::event::VirtualKeyCode::F11 => {}
                            winit::event::VirtualKeyCode::F12 => {}
                            winit::event::VirtualKeyCode::F13 => {}
                            winit::event::VirtualKeyCode::F14 => {}
                            winit::event::VirtualKeyCode::F15 => {}
                            winit::event::VirtualKeyCode::F16 => {}
                            winit::event::VirtualKeyCode::F17 => {}
                            winit::event::VirtualKeyCode::F18 => {}
                            winit::event::VirtualKeyCode::F19 => {}
                            winit::event::VirtualKeyCode::F20 => {}
                            winit::event::VirtualKeyCode::F21 => {}
                            winit::event::VirtualKeyCode::F22 => {}
                            winit::event::VirtualKeyCode::F23 => {}
                            winit::event::VirtualKeyCode::F24 => {}
                            winit::event::VirtualKeyCode::Snapshot => {}
                            winit::event::VirtualKeyCode::Scroll => {}
                            winit::event::VirtualKeyCode::Pause => {}
                            winit::event::VirtualKeyCode::Insert => {}
                            winit::event::VirtualKeyCode::Home => {}
                            winit::event::VirtualKeyCode::Delete => {}
                            winit::event::VirtualKeyCode::End => {}
                            winit::event::VirtualKeyCode::PageDown => {}
                            winit::event::VirtualKeyCode::PageUp => {}
                            winit::event::VirtualKeyCode::Left => {}
                            winit::event::VirtualKeyCode::Up => {}
                            winit::event::VirtualKeyCode::Right => {}
                            winit::event::VirtualKeyCode::Down => {}
                            winit::event::VirtualKeyCode::Back => {}
                            winit::event::VirtualKeyCode::Return => {}
                            winit::event::VirtualKeyCode::Space => {}
                            winit::event::VirtualKeyCode::Compose => {}
                            winit::event::VirtualKeyCode::Caret => {}
                            winit::event::VirtualKeyCode::Numlock => {}
                            winit::event::VirtualKeyCode::Numpad0 => {}
                            winit::event::VirtualKeyCode::Numpad1 => {}
                            winit::event::VirtualKeyCode::Numpad2 => {}
                            winit::event::VirtualKeyCode::Numpad3 => {}
                            winit::event::VirtualKeyCode::Numpad4 => {}
                            winit::event::VirtualKeyCode::Numpad5 => {}
                            winit::event::VirtualKeyCode::Numpad6 => {}
                            winit::event::VirtualKeyCode::Numpad7 => {}
                            winit::event::VirtualKeyCode::Numpad8 => {}
                            winit::event::VirtualKeyCode::Numpad9 => {}
                            winit::event::VirtualKeyCode::NumpadAdd => {}
                            winit::event::VirtualKeyCode::NumpadDivide => {}
                            winit::event::VirtualKeyCode::NumpadDecimal => {}
                            winit::event::VirtualKeyCode::NumpadComma => {}
                            winit::event::VirtualKeyCode::NumpadEnter => {}
                            winit::event::VirtualKeyCode::NumpadEquals => {}
                            winit::event::VirtualKeyCode::NumpadMultiply => {}
                            winit::event::VirtualKeyCode::NumpadSubtract => {}
                            winit::event::VirtualKeyCode::AbntC1 => {}
                            winit::event::VirtualKeyCode::AbntC2 => {}
                            winit::event::VirtualKeyCode::Apostrophe => {}
                            winit::event::VirtualKeyCode::Apps => {}
                            winit::event::VirtualKeyCode::Asterisk => {}
                            winit::event::VirtualKeyCode::At => {}
                            winit::event::VirtualKeyCode::Ax => {}
                            winit::event::VirtualKeyCode::Backslash => {}
                            winit::event::VirtualKeyCode::Calculator => {}
                            winit::event::VirtualKeyCode::Capital => {}
                            winit::event::VirtualKeyCode::Colon => {}
                            winit::event::VirtualKeyCode::Comma => {}
                            winit::event::VirtualKeyCode::Convert => {}
                            winit::event::VirtualKeyCode::Equals => {}
                            winit::event::VirtualKeyCode::Grave => {}
                            winit::event::VirtualKeyCode::Kana => {}
                            winit::event::VirtualKeyCode::Kanji => {}
                            winit::event::VirtualKeyCode::LAlt => {}
                            winit::event::VirtualKeyCode::LBracket => {}
                            winit::event::VirtualKeyCode::LControl => {}
                            winit::event::VirtualKeyCode::LShift => {}
                            winit::event::VirtualKeyCode::LWin => {}
                            winit::event::VirtualKeyCode::Mail => {}
                            winit::event::VirtualKeyCode::MediaSelect => {}
                            winit::event::VirtualKeyCode::MediaStop => {}
                            winit::event::VirtualKeyCode::Minus => {}
                            winit::event::VirtualKeyCode::Mute => {}
                            winit::event::VirtualKeyCode::MyComputer => {}
                            winit::event::VirtualKeyCode::NavigateForward => {}
                            winit::event::VirtualKeyCode::NavigateBackward => {}
                            winit::event::VirtualKeyCode::NextTrack => {}
                            winit::event::VirtualKeyCode::NoConvert => {}
                            winit::event::VirtualKeyCode::OEM102 => {}
                            winit::event::VirtualKeyCode::Period => {}
                            winit::event::VirtualKeyCode::PlayPause => {}
                            winit::event::VirtualKeyCode::Plus => {}
                            winit::event::VirtualKeyCode::Power => {}
                            winit::event::VirtualKeyCode::PrevTrack => {}
                            winit::event::VirtualKeyCode::RAlt => {}
                            winit::event::VirtualKeyCode::RBracket => {}
                            winit::event::VirtualKeyCode::RControl => {}
                            winit::event::VirtualKeyCode::RShift => {}
                            winit::event::VirtualKeyCode::RWin => {}
                            winit::event::VirtualKeyCode::Semicolon => {}
                            winit::event::VirtualKeyCode::Slash => {}
                            winit::event::VirtualKeyCode::Sleep => {}
                            winit::event::VirtualKeyCode::Stop => {}
                            winit::event::VirtualKeyCode::Sysrq => {}
                            winit::event::VirtualKeyCode::Tab => {}
                            winit::event::VirtualKeyCode::Underline => {}
                            winit::event::VirtualKeyCode::Unlabeled => {}
                            winit::event::VirtualKeyCode::VolumeDown => {}
                            winit::event::VirtualKeyCode::VolumeUp => {}
                            winit::event::VirtualKeyCode::Wake => {}
                            winit::event::VirtualKeyCode::WebBack => {}
                            winit::event::VirtualKeyCode::WebFavorites => {}
                            winit::event::VirtualKeyCode::WebForward => {}
                            winit::event::VirtualKeyCode::WebHome => {}
                            winit::event::VirtualKeyCode::WebRefresh => {}
                            winit::event::VirtualKeyCode::WebSearch => {}
                            winit::event::VirtualKeyCode::WebStop => {}
                            winit::event::VirtualKeyCode::Yen => {}
                            winit::event::VirtualKeyCode::Copy => {}
                            winit::event::VirtualKeyCode::Paste => {}
                            winit::event::VirtualKeyCode::Cut => {}
                        }
                    }
                }
                winit::event::DeviceEvent::Text { codepoint } => {}
            },
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
            _ => {}
        }
    });
}
