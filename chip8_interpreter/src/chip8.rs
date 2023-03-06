
use std::{fs, io, fmt::LowerHex};

use rand::Rng;

struct Cpu {
    registers: [u8; 16],
    register_i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: i8,
    stack: [u16; 16],
}

impl Cpu {
    fn new(initial_instruction: u16) -> Self {
        Self {
            registers: [0; 16],
            register_i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: initial_instruction,
            sp: -1,
            stack: [0; 16],
        }
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    UnknownOpCode(u16, u16)
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction(u16);

impl Instruction {
    fn addr(&self) -> u16 {
        self.0 & ((1 << 12) - 1)
    }

    fn nibble(&self) -> u8 {
        (self.0 & ((1 << 4) - 1)) as u8
    }

    fn x(&self) -> u8 {
        let high_byte = self.0 >> 8;
        (high_byte & ((1 << 4) - 1)) as u8
    }

    fn y(&self) -> u8 {
        ((self.0 >> 4) & ((1 << 4) - 1)) as u8
    }

    fn byte(&self) -> u8 {
        (self.0 & 255) as u8
    }
}

impl LowerHex for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Key {
    Key0 = 0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0 => Key::Key0,
            1 => Key::Key1,
            2 => Key::Key2,
            3 => Key::Key3,
            4 => Key::Key4,
            5 => Key::Key5,
            6 => Key::Key6,
            7 => Key::Key7,
            8 => Key::Key8,
            9 => Key::Key9,
            0xA => Key::A,
            0xB => Key::B,
            0xC => Key::C,
            0xD => Key::D,
            0xE => Key::E,
            0xF => Key::F,
            _ => unreachable!()
        }
    }
}

pub const KEYBOARD_LAYOUT:[[Key; 4]; 4] = [
    [Key::Key1, Key::Key2, Key::Key3, Key::C],
    [Key::Key4, Key::Key5, Key::Key6, Key::D],
    [Key::Key7, Key::Key8, Key::Key9, Key::E],
    [Key::A,    Key::Key0, Key::B,    Key::F],
];

struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard {
    fn new() -> Keyboard {
        Self {
            keys: [false; 16]
        }
    }

    fn set_pressed(&mut self, key: Key, pressed: bool) {
        self.keys[key as usize] = pressed;
    }

    fn is_pressed(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    fn reset(&mut self) {
        self.keys = [false; 16];
    }
}



pub struct Display {
    screen: [u8; 64 * 32],
    width: u8,
    height: u8,
    is_updated: bool
}

impl Display {
    fn new() -> Self {
        Self {
            screen: [0; 64 * 32],
            width: 64,
            height: 32,
            is_updated: true
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.screen
    }

    fn clear(&mut self) {
        self.screen = [0; 64 * 32];
        self.is_updated = true;
    }

    fn draw(&mut self, x: usize, y: usize, byte: u8) {
        self.screen[y * self.width as usize + x] ^= byte;
    }

    pub fn set_is_updated(&mut self, is_updated: bool) {
        self.is_updated = is_updated;
    }

    pub fn is_updated(&self) -> bool {
        self.is_updated
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.screen[y * self.width as usize + x] == 0
    }
}

#[derive(Copy, Clone, Debug)]
enum KeyboardBlockerState {
    None,
    Lock,
    Unlock,
    WaitRelease
}

pub const RAM_SIZE: usize = 4 * 1024;
pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    cpu: Cpu,
    display: Display,
    keyboard: Keyboard,
    wait_key_pressed: KeyboardBlockerState,
    last_key_pressed: Key,
    first_instruction_addr: u16,
    last_instruction: Instruction
}
impl Chip8 {
    pub fn new(initial_instruction: u16) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            cpu: Cpu::new(initial_instruction),
            display: Display::new(),
            keyboard: Keyboard::new(),
            wait_key_pressed: KeyboardBlockerState::None,
            last_key_pressed: Key::Key1,
            first_instruction_addr: initial_instruction,
            last_instruction: Instruction(0)
        }
    }

    pub fn get_ram(&self) -> &[u8] {
        &self.ram
    }

    pub fn get_last_instruction(&self) -> Instruction {
        self.last_instruction
    }

    pub fn play_sound(&self) -> bool {
        self.cpu.sound_timer > 0
    }

    pub fn reset(&mut self) {
        self.display.clear();

        self.keyboard.reset();
        
        self.ram = [0; RAM_SIZE];

        self.cpu.pc = self.first_instruction_addr;
        self.cpu.sp = -1;
        self.cpu.delay_timer = 0;
        self.cpu.sound_timer = 0;
    }

    pub fn get_display(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn set_key_pressed(&mut self, key: Key, pressed: bool) {
        self.keyboard.set_pressed(key, pressed);
        if pressed {
            self.wait_key_pressed = match self.wait_key_pressed {
                KeyboardBlockerState::Lock => KeyboardBlockerState::WaitRelease,
                s => s
            };
            self.last_key_pressed = key;
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn load_program(&mut self, filepath: &str) -> io::Result<()>{
        let content = fs::read(filepath)?;
        for i in 0..content.len() {
            self.ram[self.first_instruction_addr as usize + i] = content[i];
        }
        Ok(())
    }

    pub fn clock(&mut self) -> Result<(), ExecutionError> {

        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }

        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1;
        }

        let instruction = self.fetch();
        self.cpu.pc += 2;

        self.last_instruction = instruction;

        let res = self.execute(instruction);
        
        res
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ExecutionError> {
        let first_bits = instruction.0 & 15;
        let last_bits = instruction.0 >> 12;
        match last_bits {
            0x0 => {
                match first_bits {
                    0x0 => self.instr_00e0(),
                    0xE => self.instr_00ee(),
                    _ => {}
                }
            }
            0x1 => {
                self.instr_1nnn(instruction)
            }
            0x2 => {
                self.instr_2nnn(instruction)
            }
            0x3 => {
                self.instr_3xkk(instruction)
            }
            0x4 => {
                self.instr_4xkk(instruction)
            }
            0x5 => {
                self.instr_5xy0(instruction)
            }
            0x6 => {
                self.instr_6xkk(instruction)
            }
            0x7 => {
                self.instr_7xkk(instruction)
            }
            0x8 => {
                match first_bits {
                    0x0 => {
                        self.instr_8xy0(instruction)
                    }
                    0x1 => {
                        self.instr_8xy1(instruction)
                    }
                    0x2 => {
                        self.instr_8xy2(instruction)
                    }
                    0x3 => {
                        self.instr_8xy3(instruction)
                    }
                    0x4 => {
                        self.instr_8xy4(instruction)
                    }
                    0x5 => {
                        self.instr_8xy5(instruction)
                    }
                    0x6 => {
                        self.instr_8xy6(instruction)
                    }
                    0x7 => {
                        self.instr_8xy7(instruction)
                    }
                    0xE => {
                        self.instr_8xye(instruction)
                    }
                    _ => {}
                }
            }
            0x9 => {
                self.instr_9xy0(instruction)
            }
            0xA => {
                self.instr_annn(instruction)
            }
            0xB => {
                self.instr_bnnn(instruction)
            }
            0xC => {
                self.instr_cxkk(instruction)
            }
            0xD => {
                self.instr_dxyn(instruction)
            }
            0xE => {
                match first_bits {
                    0xE => self.instr_ex9e(instruction),
                    0x1 => self.instr_exa1(instruction),
                    _ => {return Err(ExecutionError::UnknownOpCode(instruction.0, self.cpu.pc))}
                }
            }
            0xF => {
                match instruction.0 & 255 {
                    0x07 => {
                        self.instr_fx07(instruction)
                    }
                    0x0A => {
                        self.instr_fx0a(instruction)
                    }
                    0x15 => {
                        self.instr_fx15(instruction)
                    }
                    0x18 => {
                        self.instr_fx18(instruction)
                    }
                    0x1E => {
                        self.instr_fx1e(instruction)
                    }
                    0x29 => {
                        self.instr_fx29(instruction)
                    }
                    0x33 => {
                        self.instr_fx33(instruction)
                    }
                    0x55 => {
                        self.instr_fx55(instruction)
                    }
                    0x65 => {
                        self.instr_fx65(instruction)
                    }
                    _ => {return Err(ExecutionError::UnknownOpCode(instruction.0, self.cpu.pc))}
                }
            }
            _ => return Err(ExecutionError::UnknownOpCode(instruction.0, self.cpu.pc))
        }

        Ok(())
    }

    fn fetch(&self) -> Instruction {
        let left = (self.ram[self.cpu.pc as usize] as u16) << 8;
        let right = self.ram[self.cpu.pc as usize + 1] as u16;
        Instruction(left | right)
    }

    fn instr_00e0(&mut self) {
        self.display.clear();
    }

    fn instr_00ee(&mut self) {
        self.cpu.pc = self.cpu.stack[self.cpu.sp as usize];
        self.cpu.sp -= 1;
    }

    fn instr_1nnn(&mut self, instr: Instruction) {
        self.cpu.pc = instr.addr();
    }

    fn instr_2nnn(&mut self, instr: Instruction) {
        self.cpu.sp += 1;
        self.cpu.stack[self.cpu.sp as usize] = self.cpu.pc;
        self.cpu.pc = instr.addr();
    }

    fn instr_3xkk(&mut self, instr: Instruction) {
        let x = instr.x();
        let kk = instr.byte();
        let vx = self.cpu.registers[x as usize];
        if vx == kk {
            self.cpu.pc += 2;
        }
    }

    fn instr_4xkk(&mut self, instr: Instruction) {
        let x = instr.x();
        let kk = instr.byte();
        let vx = self.cpu.registers[x as usize];
        if vx != kk {
            self.cpu.pc += 2;
        }
    }

    fn instr_5xy0(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        let vy = self.cpu.registers[instr.y() as usize];
        if vx == vy {
            self.cpu.pc += 2;
        }
    }

    fn instr_6xkk(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] = instr.byte();
    }

    fn instr_7xkk(&mut self, instr: Instruction) {
        let mut vx = self.cpu.registers[instr.x() as usize] as u16;
        vx += instr.byte() as u16;
        self.cpu.registers[instr.x() as usize] = vx as u8;
    }

    fn instr_8xy0(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] = self.cpu.registers[instr.y() as usize];
    }

    fn instr_8xy1(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] |= self.cpu.registers[instr.y() as usize];
    }

    fn instr_8xy2(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] &= self.cpu.registers[instr.y() as usize];
    }

    fn instr_8xy3(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] ^= self.cpu.registers[instr.y() as usize];
    }

    fn instr_8xy4(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize] as u16;
        let vy = self.cpu.registers[instr.y() as usize] as u16;

        let sum = vx + vy;
        self.cpu.registers[0xF as usize] = (sum > 255).into();
        self.cpu.registers[instr.x() as usize] = {
            if sum > 255 {
                (sum & 255) as u8
            }
            else {
                sum as u8
            }
        };
    }

    fn instr_8xy5(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize] as i16;
        let vy = self.cpu.registers[instr.y() as usize] as i16;

        self.cpu.registers[0xF as usize] = (vx > vy).into();
        self.cpu.registers[instr.x() as usize] = (vx - vy) as u8;
    }

    fn instr_8xy6(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        self.cpu.registers[0xF as usize] = vx & 1;
        self.cpu.registers[instr.x() as usize] >>= 1;
    }

    fn instr_8xy7(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize] as i8;
        let vy = self.cpu.registers[instr.y() as usize] as i8;

        self.cpu.registers[0xF as usize] = (vy > vx).into();
        self.cpu.registers[instr.x() as usize] = (vy - vx) as u8;
    }

    fn instr_8xye(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        self.cpu.registers[0xF as usize] = vx & (1 << 7);
        self.cpu.registers[instr.x() as usize] <<= 1;
    }

    fn instr_9xy0(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        let vy = self.cpu.registers[instr.y() as usize];

        if vx != vy {
            self.cpu.pc += 2;
        }
    }

    fn instr_annn(&mut self, instr: Instruction) {
        self.cpu.register_i = instr.addr();
    }

    fn instr_bnnn(&mut self, instr: Instruction) {
        self.cpu.pc = instr.addr() + self.cpu.registers[0] as u16;
    }

    fn instr_cxkk(&mut self, instr: Instruction) {
        let random_byte = rand::thread_rng().gen::<u8>();
        self.cpu.registers[instr.x() as usize] = random_byte & instr.byte();
    }

    fn instr_dxyn(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        let vy = self.cpu.registers[instr.y() as usize];

        self.display.set_is_updated(true);

        for y in 0..instr.nibble() {
            let sprite = self.ram[self.cpu.register_i as usize + y as usize];
            let ypos = vy + y;
            if ypos >= self.display.height {
                break;
            }
            for x in 0..8 {
                let pixel = (sprite >> (7-x)) & 1;
                let xpos = vx + x ;
                if xpos >= self.display.width {
                    break;
                }
                self.display.draw(xpos as usize, ypos as usize, pixel);
                self.cpu.registers[0xF as usize] = 
                    self.display.is_empty(xpos as usize, ypos as usize).into();
            }
        }
    }

    fn instr_ex9e(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        if self.keyboard.is_pressed(vx.into()) {
            self.cpu.pc += 2;
        }
    }

    fn instr_exa1(&mut self, instr: Instruction) {
        let vx = self.cpu.registers[instr.x() as usize];
        if !self.keyboard.is_pressed(vx.into()) {
            self.cpu.pc += 2;
        }
    }

    fn instr_fx07(&mut self, instr: Instruction) {
        self.cpu.registers[instr.x() as usize] = self.cpu.delay_timer;
    }

    fn instr_fx0a(&mut self, instr: Instruction) {

        self.wait_key_pressed = match self.wait_key_pressed {
            KeyboardBlockerState::Unlock => {
                self.cpu.registers[instr.x() as usize] = self.last_key_pressed as u8;
                KeyboardBlockerState::None
            },
            KeyboardBlockerState::None => {
                self.cpu.pc -= 2;
                KeyboardBlockerState::Lock
            },
            KeyboardBlockerState::WaitRelease => {
                self.cpu.pc -= 2;
                if !self.keyboard.is_pressed(self.last_key_pressed) {
                    KeyboardBlockerState::Unlock
                }
                else {
                    KeyboardBlockerState::WaitRelease
                }
            },
            s => {
                self.cpu.pc -= 2;
                s
            }
        }
        
    }

    fn instr_fx15(&mut self, instr: Instruction) {
        self.cpu.delay_timer = self.cpu.registers[instr.x() as usize];
    }

    fn instr_fx18(&mut self, instr: Instruction) {
        self.cpu.sound_timer = self.cpu.registers[instr.x() as usize];
    }

    fn instr_fx1e(&mut self, instr: Instruction) {
        self.cpu.register_i += self.cpu.registers[instr.x() as usize] as u16;

    }

    fn instr_fx29(&mut self, instr: Instruction) {
        self.cpu.register_i = self.cpu.registers[instr.x() as usize] as u16 * 5;
    }

    fn instr_fx33(&mut self, instr: Instruction) {
        let mut vx = self.cpu.registers[instr.x() as usize];
        self.ram[self.cpu.register_i as usize + 2] = vx % 10;
        vx /= 10;

        self.ram[self.cpu.register_i as usize + 1] = vx % 10;
        vx /= 10;

        self.ram[self.cpu.register_i as usize ] = vx % 10;
    }

    fn instr_fx55(&mut self, instr: Instruction) {
        for i in 0..=instr.x() as usize {
            self.ram[self.cpu.register_i as usize + i] = self.cpu.registers[i];
        }
    }

    fn instr_fx65(&mut self, instr: Instruction) {
        for i in 0..=instr.x() as usize {
            self.cpu.registers[i] = self.ram[self.cpu.register_i as usize + i];
        }
    }

}
