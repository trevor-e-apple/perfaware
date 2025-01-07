/*
There are four 16-bit registers that could be addressed as both 16-bits and 8-bit sections (ax, bx, cx, dx).
There are four 16-bit registers that could only be used in their entirety (sp, bp, si, di).
 */

use crate::common_assembly::Register;

#[derive(Default, Clone)]
pub struct SimulationState {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,

    pub sign_flag: bool,
    pub zero_flag: bool,

    pub ip: u16,
}

impl SimulationState {
    pub fn get_register_value(&self, register: Register) -> u16 {
        match register {
            Register::Al => self.ax & 0xFF,
            Register::Cl => self.cx & 0xFF,
            Register::Dl => self.dx & 0xFF,
            Register::Bl => self.bx & 0xFF,
            Register::Ah => (self.ax & 0xFF00) >> 8,
            Register::Ch => (self.cx & 0xFF00) >> 8,
            Register::Dh => (self.dx & 0xFF00) >> 8,
            Register::Bh => (self.bx & 0xFF00) >> 8,
            Register::Ax => self.ax,
            Register::Cx => self.cx,
            Register::Dx => self.dx,
            Register::Bx => self.bx,
            Register::Sp => self.sp,
            Register::Bp => self.bp,
            Register::Si => self.si,
            Register::Di => self.di,
        }
    }

    /// sets the value of a register. Note that we don't currently support writing to half registers
    pub fn set_register_value(&mut self, register: Register, value: u16) {
        match register {
            Register::Al => {
                self.ax = (self.ax | 0xFF) & ((value & 0xFF) | 0xFF00);
            }
            Register::Cl => {
                self.cx = (self.cx | 0xFF) & ((value & 0xFF) | 0xFF00);
            }
            Register::Dl => {
                self.dx = (self.dx | 0xFF) & ((value & 0xFF) | 0xFF00);
            }
            Register::Bl => {
                self.bx = (self.bx | 0xFF) & ((value & 0xFF) | 0xFF00);
            }
            Register::Ah => {
                self.ax = (self.ax | 0xFF00) & ((value << 8) | 0x00FF);
            }
            Register::Ch => {
                self.cx = (self.cx | 0xFF00) & ((value << 8) | 0x00FF);
            }
            Register::Dh => {
                self.dx = (self.dx | 0xFF00) & ((value << 8) | 0x00FF);
            }
            Register::Bh => {
                self.bx = (self.bx | 0xFF00) & ((value << 8) | 0x00FF);
            }
            Register::Ax => {
                self.ax = value;
            }
            Register::Cx => {
                self.cx = value;
            }
            Register::Dx => {
                self.dx = value;
            }
            Register::Bx => {
                self.bx = value;
            }
            Register::Sp => {
                self.sp = value;
            }
            Register::Bp => {
                self.bp = value;
            }
            Register::Si => {
                self.si = value;
            }
            Register::Di => {
                self.di = value;
            }
            _ => {} // not simulated
        };
    }

    /// sets arithmetic flags based on the value in value
    pub fn set_flags(&mut self, value: u16) {
        if (value & 0x80) > 0 {
            self.sign_flag = true;
        } else {
            self.sign_flag = false;
        }

        if value == 0 {
            self.zero_flag = true;
        } else {
            self.zero_flag = false;
        }
    }

    pub fn pretty_string(&self) -> String {
        let mut result = format!(
            concat!(
                "ax: {:#06X}({})\n",
                "bx: {:#06X}({})\n",
                "cx: {:#06X}({})\n",
                "dx: {:#06X}({})\n",
                "sp: {:#06X}({})\n",
                "bp: {:#06X}({})\n",
                "si: {:#06X}({})\n",
                "di: {:#06X}({})\n",
            ),
            self.ax,
            self.ax,
            self.bx,
            self.bx,
            self.cx,
            self.cx,
            self.dx,
            self.dx,
            self.sp,
            self.sp,
            self.bp,
            self.bp,
            self.si,
            self.si,
            self.di,
            self.di
        );

        result.push_str("Flags: ");
        add_flags_string(self, &mut result);

        result
    }
}

/// add flags string to the mutable string passed in as an argument
fn add_flags_string(sim_state: &SimulationState, result: &mut String) {
    if sim_state.sign_flag {
        result.push_str("S");
    }
    if sim_state.zero_flag {
        result.push_str("Z");
    }
}

/// returns a string indicating a change in simulation state
pub fn get_sim_state_diff(before: &SimulationState, after: &SimulationState) -> String {
    let mut result = String::new();

    if before.ax != after.ax {
        result.push_str(&format!("ax: {:#06X} -> {:#06X} ", before.ax, after.ax));
    }
    if before.bx != after.bx {
        result.push_str(&format!("bx: {:#06X} -> {:#06X} ", before.bx, after.bx));
    }
    if before.cx != after.cx {
        result.push_str(&format!("cx: {:#06X} -> {:#06X} ", before.cx, after.cx));
    }
    if before.dx != after.dx {
        result.push_str(&format!("dx: {:#06X} -> {:#06X} ", before.dx, after.dx));
    }
    if before.sp != after.sp {
        result.push_str(&format!("sp: {:#06X} -> {:#06X} ", before.sp, after.sp));
    }
    if before.bp != after.bp {
        result.push_str(&format!("bp: {:#06X} -> {:#06X} ", before.bp, after.bp));
    }
    if before.si != after.si {
        result.push_str(&format!("si: {:#06X} -> {:#06X} ", before.si, after.si));
    }
    if before.di != after.di {
        result.push_str(&format!("di: {:#06X} -> {:#06X} ", before.di, after.di));
    }
    if before.ip != after.ip {
        result.push_str(&format!("ip: {:#06X} -> {:#06X} ", before.ip, after.ip));
    }

    if before.sign_flag != after.sign_flag || before.zero_flag != after.zero_flag {
        result.push_str("Flags: ");
        add_flags_string(&before, &mut result);
        result.push_str(" -> ");
        add_flags_string(&after, &mut result);
    }

    result.push_str("\n");

    result
}
