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
}

impl SimulationState {
    /// sets the value of a register. Note that we don't currently support writing to half registers
    pub fn set_register_value(&mut self, register: Register, value: u16) {
        match register {
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

    pub fn pretty_string(&self) -> String {
        format!(
            "ax: {:#06X}({})\nbx: {:#06X}({})\ncx: {:#06X}({})\ndx: {:#06X}({})\nsp: {:#06X}({})\nbp: {:#06X}({})\nsi: {:#06X}({})\ndi: {:#06X}({})\n",
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
        )
    }
}

/// returns a string indicating a change in simulation state
pub fn get_sim_state_diff(before: &SimulationState, after: &SimulationState) -> String {
    let mut result = String::new();

    if before.ax != after.ax {
        result.push_str(&format!("ax: {} -> {}\n", before.ax, after.ax));
    }
    if before.bx != after.bx {
        result.push_str(&format!("bx: {} -> {}\n", before.bx, after.bx));
    }
    if before.cx != after.cx {
        result.push_str(&format!("cx: {} -> {}\n", before.cx, after.cx));
    }
    if before.dx != after.dx {
        result.push_str(&format!("dx: {} -> {}\n", before.dx, after.dx));
    }
    if before.sp != after.sp {
        result.push_str(&format!("sp: {} -> {}\n", before.sp, after.sp));
    }
    if before.bp != after.bp {
        result.push_str(&format!("bp: {} -> {}\n", before.bp, after.bp));
    }
    if before.si != after.si {
        result.push_str(&format!("si: {} -> {}\n", before.si, after.si));
    }
    if before.di != after.di {
        result.push_str(&format!("di: {} -> {}\n", before.di, after.di));
    }

    result
}
