#![allow(dead_code)]

#[allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct SH4Registers {
    r: [u32; 16],
    pr: u32,
    sr: u32,
    gbr: u32,
    vbr: u32,
    mach: u32,
    macl: u32,
}
