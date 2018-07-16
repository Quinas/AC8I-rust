use instruction::{decode_opcode, fetch_opcode, OpCode};
use rand;
use ChipSound;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub struct Chip8 {
  pub memory: [u8; 4096],
  pub v: [u8; 16],
  pub index: u16,
  pub pc: u16,
  pub display: [[bool; HEIGHT as usize]; WIDTH as usize],
  pub delay_timer: u8,
  pub sound_timer: u8,
  pub stack: [u16; 16],
  pub sp: u8,
  pub keys: [bool; 16],
  pub draw_flag: bool,
  wait_key: Option<u8>,
}

const SPRITES: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xF0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
  pub fn new() -> Chip8 {
    let mut chip = Chip8 {
      memory: [0; 4096],
      v: [0; 16],
      index: 0,
      pc: 0x200,
      display: [[false; HEIGHT as usize]; WIDTH as usize],
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      sp: 0,
      keys: [false; 16],
      draw_flag: false,
      wait_key: None,
    };
    chip.reset_memory();
    chip
  }

  pub fn reset(&mut self) {
    self.reset_memory();
    self.v = [0; 16];
    self.index = 0;
    self.pc = 0x200;
    self.display = [[false; HEIGHT as usize]; WIDTH as usize];
    self.delay_timer = 0;
    self.sound_timer = 0;
    self.stack = [0; 16];
    self.sp = 0;
    self.keys = [false; 16];
  }

  fn reset_memory(&mut self) {
    self.memory = [0; 4096];
    for (ind, pixel) in SPRITES.iter().enumerate() {
      self.memory[ind] = *pixel;
    }
  }

  pub fn load_game(&mut self) {}

  pub fn emulate_cycle<T: ChipSound>(&mut self, sound: &T, seconds: f64) {
    let num = (seconds * 600.0).round() as u64;

    for _ in 0..num {
      let opcode = fetch_opcode(self);
      if self.sound_timer > 0 {
        if self.sound_timer == 1 {
          sound.make_sound();
        }
        self.sound_timer -= 1;
      }
      if self.delay_timer > 0 {
        self.delay_timer -= 1;
      }
      if self.wait_key == None {
        self.execute_opcode(decode_opcode(opcode));
      }
    }
  }

  pub fn press_key(&mut self, key: u8) {
    self.keys[key as usize] = true;
    if let Some(r) = self.wait_key {
      self.v[r as usize] = key;
      self.wait_key = None;
    }
  }

  pub fn release_key(&mut self, key: u8) {
    self.keys[key as usize] = false;
  }

  pub fn execute_opcode(&mut self, opcode: Option<OpCode>) {
    if let Some(opcode) = opcode {
      match opcode {
        OpCode::CLS => {
          self.display = [[false; HEIGHT as usize]; WIDTH as usize];
          self.pc += 2;
        }
        OpCode::RET => {
          self.sp -= 1;
          self.pc = self.stack[self.sp as usize] + 2;
        }
        OpCode::SYS(..) => {
          self.pc += 2;
        }
        OpCode::JP(addr) => self.pc = addr,
        OpCode::CALL(addr) => {
          self.stack[self.sp as usize] = self.pc;
          self.sp += 1;
          self.pc = addr;
        }
        OpCode::SE(vx, byte) => {
          if self.v[vx as usize] == byte {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::SNE(vx, byte) => {
          if self.v[vx as usize] != byte {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::SE2(vx, vy) => {
          if self.v[vx as usize] == self.v[vy as usize] {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::LD(vx, byte) => {
          self.v[vx as usize] = byte;
          self.pc += 2;
        }
        OpCode::ADD(vx, byte) => {
          self.v[vx as usize] = (self.v[vx as usize] as u16 + byte as u16) as u8;
          self.pc += 2;
        }
        OpCode::LD2(vx, vy) => {
          self.v[vx as usize] = self.v[vy as usize];
          self.pc += 2;
        }
        OpCode::OR(vx, vy) => {
          self.v[vx as usize] |= self.v[vy as usize];
          self.pc += 2;
        }
        OpCode::AND(vx, vy) => {
          self.v[vx as usize] &= self.v[vy as usize];
          self.pc += 2;
        }
        OpCode::XOR(vx, vy) => {
          self.v[vx as usize] ^= self.v[vy as usize];
          self.pc += 2;
        }
        OpCode::ADD2(vx, vy) => {
          let sum = self.v[vy as usize] as u16 + self.v[vx as usize] as u16;
          self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
          self.v[vx as usize] = sum as u8;
          self.pc += 2;
        }
        OpCode::SUB(vx, vy) => {
          let diff = self.v[vx as usize].wrapping_sub(self.v[vy as usize]);
          self.v[0xF] = if self.v[vx as usize] > self.v[vy as usize] {
            1
          } else {
            0
          };
          self.v[vx as usize] = diff as u8;
          self.pc += 2;
        }
        OpCode::SHR(vx, ..) => {
          self.v[0xF] = self.v[vx as usize] % 2;
          self.v[vx as usize] /= 2;
          self.pc += 2;
        }
        OpCode::SUBN(vx, vy) => {
          let diff = self.v[vy as usize] - self.v[vx as usize];
          self.v[0xF] = if self.v[vy as usize] > self.v[vx as usize] {
            1
          } else {
            0
          };
          self.v[vx as usize] = diff as u8;
          self.pc += 2;
        }
        OpCode::SHL(vx, ..) => {
          self.v[0xF] = self.v[vx as usize] >> 7;
          self.v[vx as usize] <<= 1;
          self.pc += 2;
        }
        OpCode::SNE2(vx, vy) => {
          if self.v[vx as usize] != self.v[vy as usize] {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::LD3(addr) => {
          self.index = addr;
          self.pc += 2;
        }
        OpCode::JP2(addr) => self.pc = addr + self.v[0] as u16,
        OpCode::RND(vx, byte) => {
          let r: u8 = rand::random();
          self.v[vx as usize] = r & byte;
          self.pc += 2;
        }
        OpCode::DRW(vx, vy, height) => {
          let x0 = self.v[vx as usize];
          let y0 = self.v[vy as usize];

          self.v[0xF] = 0;
          for y in 0..height {
            if y0 + y >= HEIGHT as u8 {
              break;
            }
            let line = self.memory[(self.index + y as u16) as usize];
            for x in 0..8 {
              if x0 + x >= WIDTH as u8 {
                break;
              }
              if (line >> (7 - x)) & 1 == 1 {
                if self.display[(x0 + x) as usize][(y0 + y) as usize] == true {
                  self.v[0xF] = 1;
                }
                self.display[(x0 + x) as usize][(y0 + y) as usize] =
                  !self.display[(x0 + x) as usize][(y0 + y) as usize];
              }
            }
          }
          self.draw_flag = true;
          self.pc += 2;
        }
        OpCode::SKP(vx) => {
          if self.keys[self.v[vx as usize] as usize] {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::SKNP(vx) => {
          if !self.keys[self.v[vx as usize] as usize] {
            self.pc += 2;
          }
          self.pc += 2;
        }
        OpCode::LD4(vx) => {
          self.v[vx as usize] = self.delay_timer;
          self.pc += 2;
        }
        OpCode::LD5(vx) => {
          self.wait_key = Some(vx);
          self.pc += 2;
        }
        OpCode::LD6(vx) => {
          self.delay_timer = self.v[vx as usize];
          self.pc += 2;
        }
        OpCode::LD7(vx) => {
          self.sound_timer = self.v[vx as usize];
          self.pc += 2;
        }
        OpCode::ADD3(vx) => {
          self.index += self.v[vx as usize] as u16;
          self.pc += 2;
        }
        OpCode::LD8(vx) => {
          self.index = (self.v[vx as usize] as u16) * 5;
          self.pc += 2;
        }
        OpCode::LD9(vx) => {
          self.memory[self.index as usize] = (self.v[vx as usize] / 100) % 10;
          self.memory[self.index as usize + 1] = (self.v[vx as usize] / 10) % 10;
          self.memory[self.index as usize + 2] = self.v[vx as usize] % 10;
          self.pc += 2;
        }
        OpCode::LD10(vx) => {
          for ind in 0..vx + 1 {
            self.memory[(self.index + ind as u16) as usize] = self.v[ind as usize];
          }
          self.pc += 2;
        }
        OpCode::LD11(vx) => {
          for ind in 0..vx + 1 {
            self.v[ind as usize] = self.memory[(self.index + ind as u16) as usize];
          }
          self.pc += 2;
        }
      };
    } else {
      self.pc += 2;
    }
  }
}
