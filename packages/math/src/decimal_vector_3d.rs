use crate::sin_cos::f64_to_dbig;
use dashu_float::DBig;
use dashu_float::ops::SquareRoot;
use glam::DVec3;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecimalVector3d {
    pub x: DBig,
    pub y: DBig,
    pub z: DBig,
}

impl DecimalVector3d {
    pub fn zero() -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::ZERO.clone(),
            y: DBig::ZERO.clone(),
            z: DBig::ZERO.clone(),
        }
    }

    pub fn new(x: DBig, y: DBig, z: DBig) -> DecimalVector3d {
        DecimalVector3d { x, y, z }
    }

    pub fn to_dvec3(&self) -> DVec3 {
        DVec3::new(
            self.x.to_f64().value(),
            self.y.to_f64().value(),
            self.z.to_f64().value(),
        )
    }

    pub fn to_dvec3_with_precision(self, precision: usize) -> DVec3 {
        let x = self.x.with_precision(precision).value();
        let y = self.y.with_precision(precision).value();
        let z = self.z.with_precision(precision).value();
        DVec3::new(x.to_f64().value(), y.to_f64().value(), z.to_f64().value())
    }

    pub fn from_dvec3(vec: DVec3) -> DecimalVector3d {
        DecimalVector3d::from_f64(vec.x, vec.y, vec.z)
    }

    pub fn assign(&mut self, v: &DecimalVector3d) {
        self.x = v.x.clone();
        self.y = v.y.clone();
        self.z = v.z.clone();
    }

    pub fn from_str(x: &str, y: &str, z: &str) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::from_str(x).unwrap(),
            y: DBig::from_str(y).unwrap(),
            z: DBig::from_str(z).unwrap(),
        }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::from_str(x.to_string().as_str()).unwrap(),
            y: DBig::from_str(y.to_string().as_str()).unwrap(),
            z: DBig::from_str(z.to_string().as_str()).unwrap(),
        }
    }

    pub fn length_squared(&self) -> DBig {
        &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    pub fn length(&self) -> DBig {
        let squared = self.length_squared();
        if squared.eq(&DBig::ZERO) {
            DBig::ZERO.clone()
        } else {
            squared.sqrt()
        }
    }

    pub fn distance_to(&self, rhs: &Self) -> DBig {
        let difference = self - rhs;
        difference.length()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        *self /= len;
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn dot(&self, rhs: &Self) -> DBig {
        &self.x * &rhs.x + &self.y * &rhs.y + &self.z * &rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> DecimalVector3d {
        let ax = &self.x;
        let ay = &self.y;
        let az = &self.z;
        let bx = &rhs.x;
        let by = &rhs.y;
        let bz = &rhs.z;

        let x = ay * bz - az * by;
        let y = az * bx - ax * bz;
        let z = ax * by - ay * bx;

        DecimalVector3d { x, y, z }
    }
}

impl fmt::Display for DecimalVector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ x: {}, y: {}, z: {} }}",
            self.x.to_string(),
            self.y.to_string(),
            self.z.to_string()
        )
    }
}

// ADD

impl std::ops::Add<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + &rhs.x,
            y: self.y + &rhs.y,
            z: self.z + &rhs.z,
        }
    }
}

impl std::ops::Add<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x + rhs.x,
            y: &self.y + rhs.y,
            z: &self.z + rhs.z,
        }
    }
}

impl std::ops::Add<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x + &rhs.x,
            y: &self.y + &rhs.y,
            z: &self.z + &rhs.z,
        }
    }
}

impl std::ops::Add<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + &rhs,
            y: self.y + &rhs,
            z: self.z + &rhs,
        }
    }
}

impl std::ops::Add<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Add<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x + &rhs,
            y: &self.y + &rhs,
            z: &self.z + &rhs,
        }
    }
}

impl std::ops::Add<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x + rhs,
            y: &self.y + rhs,
            z: &self.z + rhs,
        }
    }
}

impl std::ops::AddAssign<&DBig> for DecimalVector3d {
    fn add_assign(&mut self, rhs: &DBig) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl std::ops::AddAssign<DBig> for DecimalVector3d {
    fn add_assign(&mut self, rhs: DBig) {
        self.x += &rhs;
        self.y += &rhs;
        self.z += &rhs;
    }
}

impl std::ops::AddAssign<&DecimalVector3d> for DecimalVector3d {
    fn add_assign(&mut self, rhs: &DecimalVector3d) {
        self.x += &rhs.x;
        self.y += &rhs.y;
        self.z += &rhs.z;
    }
}

impl std::ops::AddAssign<DecimalVector3d> for DecimalVector3d {
    fn add_assign(&mut self, rhs: DecimalVector3d) {
        self.x += &rhs.x;
        self.y += &rhs.y;
        self.z += &rhs.z;
    }
}

// SUB

impl std::ops::Sub<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - &rhs.x,
            y: self.y - &rhs.y,
            z: self.z - &rhs.z,
        }
    }
}

impl std::ops::Sub<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x - rhs.x,
            y: &self.y - rhs.y,
            z: &self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
            z: &self.z - &rhs.z,
        }
    }
}

impl std::ops::Sub<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - &rhs,
            y: self.y - &rhs,
            z: self.z - &rhs,
        }
    }
}

impl std::ops::Sub<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl std::ops::Sub<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x - &rhs,
            y: &self.y - &rhs,
            z: &self.z - &rhs,
        }
    }
}

impl std::ops::Sub<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x - rhs,
            y: &self.y - rhs,
            z: &self.z - rhs,
        }
    }
}

impl std::ops::SubAssign<&DBig> for DecimalVector3d {
    fn sub_assign(&mut self, rhs: &DBig) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl std::ops::SubAssign<DBig> for DecimalVector3d {
    fn sub_assign(&mut self, rhs: DBig) {
        self.x -= &rhs;
        self.y -= &rhs;
        self.z -= &rhs;
    }
}

// MUL

impl std::ops::Mul<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * &rhs.x,
            y: self.y * &rhs.y,
            z: self.z * &rhs.z,
        }
    }
}

impl std::ops::Mul<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x * rhs.x,
            y: &self.y * rhs.y,
            z: &self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x * &rhs.x,
            y: &self.y * &rhs.y,
            z: &self.z * &rhs.z,
        }
    }
}

impl std::ops::Mul<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * &rhs,
            y: self.y * &rhs,
            z: self.z * &rhs,
        }
    }
}

impl std::ops::Mul<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x * &rhs,
            y: &self.y * &rhs,
            z: &self.z * &rhs,
        }
    }
}

impl std::ops::Mul<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x * rhs,
            y: &self.y * rhs,
            z: &self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: f64) -> DecimalVector3d {
        let dbig = f64_to_dbig(rhs);
        DecimalVector3d {
            x: &self.x * &dbig,
            y: &self.y * &dbig,
            z: &self.z * &dbig,
        }
    }
}

impl std::ops::MulAssign<&DBig> for DecimalVector3d {
    fn mul_assign(&mut self, rhs: &DBig) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::MulAssign<DBig> for DecimalVector3d {
    fn mul_assign(&mut self, rhs: DBig) {
        self.x *= &rhs;
        self.y *= &rhs;
        self.z *= &rhs;
    }
}

// DIV

impl std::ops::Div<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / &rhs.x,
            y: self.y / &rhs.y,
            z: self.z / &rhs.z,
        }
    }
}

impl std::ops::Div<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x / rhs.x,
            y: &self.y / rhs.y,
            z: &self.z / rhs.z,
        }
    }
}

impl std::ops::Div<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x / &rhs.x,
            y: &self.y / &rhs.y,
            z: &self.z / &rhs.z,
        }
    }
}

impl std::ops::Div<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / &rhs,
            y: self.y / &rhs,
            z: self.z / &rhs,
        }
    }
}

impl std::ops::Div<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Div<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x / &rhs,
            y: &self.y / &rhs,
            z: &self.z / &rhs,
        }
    }
}

impl std::ops::Div<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: &self.x / rhs,
            y: &self.y / rhs,
            z: &self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<&DBig> for DecimalVector3d {
    fn div_assign(&mut self, rhs: &DBig) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::DivAssign<DBig> for DecimalVector3d {
    fn div_assign(&mut self, rhs: DBig) {
        self.x /= &rhs;
        self.y /= &rhs;
        self.z /= &rhs;
    }
}
