/*
 * rustboy/cpu.rs
 *
 * This is the CPU of the GameBoy emulator. A technical specification
 * of the GameBoy can be found here: http://problemkaputt.de/pandocs.htm
 *
 */

/*
  16bit Hi   Lo   Name/Function
  AF    A    -    Accumulator & Flags
  BC    B    C    BC
  DE    D    E    DE
  HL    H    L    HL
  SP    -    -    Stack Pointer
  PC    -    -    Program Counter/Pointer
*/
struct Registers {
    // 8-bit registers:
    a: u8,      // acc and flags
    b: u8,      // BC
    c: u8,
    d: u8,      // DE
    e: u8,
    h: u8,      // HL
    l: u8,
    // 16-bit registers:
    af: u16,    // acc and flags
    bc: u16,    // BC
    de: u16,    // DE
    hl: u16,    // HL
    // pointers:
    sp: u16,    // stack pointer
    pc: u16     // program counter / instruction pointer
}

struct CPU;