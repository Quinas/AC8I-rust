use chip::{Chip8, HEIGHT, WIDTH};
use graphics::{color, Rectangle};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::{clear, PistonWindow as Window};
use std::fs::File;
use std::io::Read;
use ChipSound;

pub struct System {
  pub chip: Chip8,
  pub app: App,
}

pub struct App {
  pub window: Window,
}

impl ChipSound for App {
  fn make_sound(&self) {
    println!("Beep!");
  }
}

fn decode_key(button: &Button) -> Option<u8> {
  match button {
    Button::Keyboard(key) => match key {
      keyboard::Key::D1 => Some(1),
      keyboard::Key::D2 => Some(2),
      keyboard::Key::D3 => Some(3),
      keyboard::Key::D4 => Some(0xC),
      keyboard::Key::Q => Some(4),
      keyboard::Key::W => Some(5),
      keyboard::Key::E => Some(6),
      keyboard::Key::R => Some(0xD),
      keyboard::Key::A => Some(7),
      keyboard::Key::S => Some(8),
      keyboard::Key::D => Some(9),
      keyboard::Key::F => Some(0xE),
      keyboard::Key::Z => Some(0xA),
      keyboard::Key::X => Some(0),
      keyboard::Key::C => Some(0xB),
      keyboard::Key::V => Some(0xF),
      _ => None,
    },
    _ => None,
  }
}

const SIZE: u32 = 15;

impl System {
  pub fn new() -> System {
    System {
      chip: Chip8::new(),
      app: App {
        window: WindowSettings::new("chip8", [WIDTH * SIZE, HEIGHT * SIZE])
          .exit_on_esc(true)
          .build()
          .unwrap(),
      },
    }
  }

  pub fn reset(&mut self) {
    self.chip.reset();
  }

  pub fn emulate(&mut self) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut self.app.window) {
      if let Some(r) = e.press_args() {
        if let Some(r) = decode_key(&r) {
          self.chip.press_key(r);
        }
      }
      if let Some(r) = e.release_args() {
        if let Some(r) = decode_key(&r) {
          self.chip.release_key(r);
        }
      }

      self.chip.emulate_cycle(&self.app, 0.001);

      let chip = &self.chip;

      self.app.window.draw_2d(&e, |c, g| {
        clear(color::BLACK, g);
        for x in 0..WIDTH {
          for y in 0..HEIGHT {
            if chip.display[x as usize][y as usize] {
              Rectangle::new(color::WHITE).draw(
                [
                  (x * SIZE) as f64,
                  (y * SIZE) as f64,
                  SIZE as f64,
                  SIZE as f64,
                ],
                &c.draw_state,
                c.transform,
                g,
              );
            }
          }
        }
      });
    }
  }

  pub fn load_game(&mut self, name: &str) {
    self.reset();
    let path = "games/".to_string() + name;
    let file = File::open(path).unwrap();
    for (ind, byte) in file.bytes().enumerate() {
      self.chip.memory[ind + 0x200] = byte.unwrap();
    }
  }
}
