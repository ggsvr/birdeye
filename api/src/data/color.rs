use super::Data;

pub struct ColorData {

    pub colors: [Color; 3],

}

impl ColorData {
    pub fn new(back: Color, front: Color, dest: Color) -> Self {
        Self {
            colors: [
                back,
                front,
                dest
            ]
        }
    }
}

impl Data for ColorData {
    type Inner = Color;

    fn list(&self) -> &[Self::Inner; 3] {
        &self.colors
    }

    fn list_mut(&mut self) -> &mut [Self::Inner; 3] {
        &mut self.colors
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub channels: [u8; 3]
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            channels: [r, g, b]
        }
    }

    pub fn black() -> Self {
        Self {
            channels: [0; 3]
        }
    }
    pub fn white() -> Self {
        Self {
            channels: [255; 3]
        }
    }
    pub fn red() -> Self {
        Self {
            channels: [255, 0, 0]
        }
    }
    pub fn green() -> Self {
        Self {
            channels: [0, 255, 0]
        }
    }
    pub fn blue() -> Self {
        Self {
            channels: [0, 0, 255]
        }
    }

    pub fn r(&self) -> u8 {
        self.channels[0]
    }
    pub fn g(&self) -> u8 {
        self.channels[1]
    }
    pub fn b(&self) -> u8 {
        self.channels[2]
    }
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.channels[0]
    }
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.channels[1]
    }
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.channels[2]
    }
}

// Operations for color

use std::ops;

impl ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] + rhs.channels[0],
                self.channels[1] + rhs.channels[1],
                self.channels[2] + rhs.channels[2],
            ]
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.channels[0] += rhs.channels[0];
        self.channels[1] += rhs.channels[1];
        self.channels[2] += rhs.channels[2];
    }
}

impl ops::Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] - rhs.channels[0],
                self.channels[1] - rhs.channels[1],
                self.channels[2] - rhs.channels[2],
            ]
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.channels[0] += rhs.channels[0];
        self.channels[1] += rhs.channels[1];
        self.channels[2] += rhs.channels[2];
    }
}

impl ops::Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] * rhs.channels[0],
                self.channels[1] * rhs.channels[1],
                self.channels[2] * rhs.channels[2],
            ]
        }
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.channels[0] *= rhs.channels[0];
        self.channels[1] *= rhs.channels[1];
        self.channels[2] *= rhs.channels[2];
    }
}

impl ops::Div for Color {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] / rhs.channels[0],
                self.channels[1] / rhs.channels[1],
                self.channels[2] / rhs.channels[2],
            ]
        }
    }
}

impl ops::DivAssign for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.channels[0] /= rhs.channels[0];
        self.channels[1] /= rhs.channels[1];
        self.channels[2] /= rhs.channels[2];
    }
}