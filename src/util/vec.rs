use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::ToPrimitive;

#[derive(Debug, PartialEq, Copy)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.v[0], self.v[1], self.v[2])
    }
}

impl Eq for Vec3 {}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
            ],
        }
    }
}

impl<T> Add<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            v: [
                self.v[0] + rhs.to_f64().unwrap(),
                self.v[1] + rhs.to_f64().unwrap(),
                self.v[2] + rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
            ],
        }
    }
}

impl<T> AddAssign<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    fn add_assign(&mut self, rhs: T) {
        *self = Self {
            v: [
                self.v[0] + rhs.to_f64().unwrap(),
                self.v[1] + rhs.to_f64().unwrap(),
                self.v[2] + rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
            ],
        }
    }
}

impl<T> Sub<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            v: [
                self.v[0] - rhs.to_f64().unwrap(),
                self.v[1] - rhs.to_f64().unwrap(),
                self.v[2] - rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
            ],
        }
    }
}

impl<T> SubAssign<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = Self {
            v: [
                self.v[0] - rhs.to_f64().unwrap(),
                self.v[1] - rhs.to_f64().unwrap(),
                self.v[2] - rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] * rhs.v[0],
                self.v[1] * rhs.v[1],
                self.v[2] * rhs.v[2],
            ],
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            v: [
                self.v[0] * rhs.to_f64().unwrap(),
                self.v[1] * rhs.to_f64().unwrap(),
                self.v[2] * rhs.to_f64().unwrap(),
            ],
        }
    }
}

//TODO use generics here
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return rhs * self;
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return rhs * self;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            v: [
                self.v[0] * rhs.v[0],
                self.v[1] * rhs.v[1],
                self.v[2] * rhs.v[2],
            ],
        }
    }
}

impl<T> MulAssign<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            v: [
                self.v[0] * rhs.to_f64().unwrap(),
                self.v[1] * rhs.to_f64().unwrap(),
                self.v[2] * rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] / rhs.v[0],
                self.v[1] / rhs.v[1],
                self.v[2] / rhs.v[2],
            ],
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            v: [
                self.v[0] / rhs.to_f64().unwrap(),
                self.v[1] / rhs.to_f64().unwrap(),
                self.v[2] / rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            v: [
                self.v[0] / rhs.v[0],
                self.v[1] / rhs.v[1],
                self.v[2] / rhs.v[2],
            ],
        }
    }
}

impl<T> DivAssign<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    fn div_assign(&mut self, rhs: T) {
        *self = Self {
            v: [
                self.v[0] / rhs.to_f64().unwrap(),
                self.v[1] / rhs.to_f64().unwrap(),
                self.v[2] / rhs.to_f64().unwrap(),
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return -1*self;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.v[index];
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.v[index];
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { v: [0.0, 0.0, 0.0] }
    }
}

impl<T> From<T> for Vec3
where
    T: Copy + ToPrimitive,
{
    fn from(value: T) -> Self {
        Self {
            v: [
                value.to_f64().unwrap(),
                value.to_f64().unwrap(),
                value.to_f64().unwrap(),
            ],
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return a.x() * b.x() + a.y() * b.y() + a.z() * b.z();
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3::new(
        a.y() * b.z() - a.z() * b.y(),
        a.z() * b.x() - a.x() * b.z(),
        a.x() * b.y() - a.y() * b.x(),
    );
}

impl Vec3 {
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Self
    where
        T: ToPrimitive,
        U: ToPrimitive,
        V: ToPrimitive,
    {
        Self {
            v: [
                x.to_f64().unwrap(),
                y.to_f64().unwrap(),
                z.to_f64().unwrap(),
            ],
        }
    }

    pub fn x(&self) -> f64 {
        return self.v[0];
    }

    pub fn y(&self) -> f64 {
        return self.v[1];
    }

    pub fn z(&self) -> f64 {
        return self.v[2];
    }

    pub fn v(&self) -> [f64; 3] {
        return self.v;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2];
    }

    pub fn to_normal(&self) -> Self {
        return *self / self.length();
    }
}

pub type Point3 = Vec3;

#[test]
fn create_default_vec3() {
    let a = Vec3::default();
    assert_eq!(a, Vec3::new(0.0, 0.0, 0.0))
}

#[test]
fn create_vec3_from() {
    let a = Vec3::from(10.0);
    assert!(a.x() == a.y() && a.x() == a.z() && a.y() == a.z());
    assert_eq!(a.x(), 10.0)
}

#[test]
fn vec3_length() {
    let a = Vec3::from(1.0);
    assert_eq!(a.length_squared(), 3.0);
    assert_eq!(a.length(), 3.0_f64.sqrt())
}

#[test]
fn vec3_add() {
    let a = Vec3::new(2.0, 0.0, -1.0);
    let b = Vec3::new(0.0, 1.0, 0.5);
    let c = a + b;
    assert_eq!(c, Vec3::new(2.0, 1.0, -0.5))
}

#[test]
fn vec3_sub() {
    let a = Vec3::new(2.0, 0.0, -1.0);
    let b = Vec3::new(0.0, 1.0, 0.5);
    let c = a - b;
    assert_eq!(c, Vec3::new(2.0, -1.0, -1.5))
}

#[test]
fn vec3_mul() {
    let a = Vec3::new(2.0, 0.0, -1.0);
    let b = Vec3::new(0.0, 1.0, 0.5);
    let c = a * b;
    assert_eq!(c, Vec3::new(0.0, 0.0, -0.5))
}

#[test]
fn vec3_div() {
    let a = Vec3::new(2.0, 2.0, 5.0);
    let b = Vec3::new(4.0, 1.0, 0.5);
    let c = a / b;
    assert_eq!(c, Vec3::new(0.5, 2.0, 10.0))
}
