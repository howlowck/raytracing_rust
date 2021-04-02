use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub value: [f32 ; 3]
}

pub trait ToRgb {
    fn to_rgb_with_newline(&self) -> String;
}

impl ToRgb for Vec3 {
    fn to_rgb_with_newline(&self) -> String {
        let str_r = (self.value[0].round() as u8).to_string();
        let str_g = (self.value[1].round() as u8).to_string();
        let str_b = (self.value[2].round() as u8).to_string();
        return format!("{}{}", [str_r, str_g, str_b].join(" "), "\n");
    }
}

pub trait Dot {
    fn dot(&self, rhs: Vec3) -> f32;
}

impl Dot for Vec3 {
    fn dot(&self, rhs: Vec3) -> f32 {
        return
            self.value[0] * rhs.value[0] +
            self.value[1] * rhs.value[1] +
            self.value[2] * rhs.value[2];
    }
}

pub trait Cross {
    fn cross(&self, rhs: Vec3) -> Vec3;
}

impl Cross for Vec3 {
    fn cross(&self, rhs: Vec3) -> Vec3 {
        return Vec3 { value: [
            self.value[1] * rhs.value[2] - self.value[2] * rhs.value[1],
            self.value[2] * rhs.value[0] - self.value[0] * rhs.value[2],
            self.value[0] * rhs.value[1] - self.value[1] * rhs.value[0]
        ]}
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, operand: Self) -> Self::Output {
        let a = self.value[0] + operand.value[0];
        let b = self.value[1] + operand.value[1];
        let c = self.value[2] + operand.value[2];
        return Vec3 { value: [a, b, c] };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, operand: Self) -> Self::Output {
        let a = self.value[0] - operand.value[0];
        let b = self.value[1] - operand.value[1];
        let c = self.value[2] - operand.value[2];
        return Vec3 { value: [a, b, c] };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let a = self.value[0] * rhs;
        let b = self.value[1] * rhs;
        let c = self.value[2] * rhs;
        return Vec3 { value: [a, b, c] };
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let a = self * rhs.value[0];
        let b = self * rhs.value[1];
        let c = self * rhs.value[2];
        return Vec3 { value: [a, b, c] };
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.value[0] * rhs.value[0];
        let b = self.value[1] * rhs.value[1];
        let c = self.value[2] * rhs.value[2];
        return Vec3 { value: [a, b, c] };
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Vec3 {
        let fa = self.value[0] / rhs;
        let fb = self.value[1] / rhs;
        let fc = self.value[2] / rhs;
        return Vec3 { value: [fa, fb, fc] };
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        let fa = self.value[0] / rhs;
        let fb = self.value[1] / rhs;
        let fc = self.value[2] / rhs;
        return Vec3 { value: [fa, fb, fc] };
    }
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    return vec / vec.value.len() as f32;
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
    return
        lhs.value[0] * rhs.value[0] +
        lhs.value[1] * rhs.value[1] +
        lhs.value[2] * rhs.value[2];
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction
    }
}