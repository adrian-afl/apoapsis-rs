use crate::decimal_vector_3d::DecimalVector3d;
use crate::sin_cos::{cos, f64_to_dbig, sin};
use dashu_float::ops::SquareRoot;
use dashu_float::DBig;
use glam::{DMat3, DMat4, DQuat};
use std::ops::Deref;
use std::sync::LazyLock;

static DBIGHALF: LazyLock<DBig> = LazyLock::new(|| f64_to_dbig(0.5));

#[derive(Debug, Clone)]
pub struct DecimalMatrix3d {
    pub data: [[DBig; 3]; 3],
}

impl DecimalMatrix3d {
    pub fn identity() -> DecimalMatrix3d {
        DecimalMatrix3d {
            data: [
                [DBig::ONE.clone(), DBig::ZERO.clone(), DBig::ZERO.clone()],
                [DBig::ZERO.clone(), DBig::ONE.clone(), DBig::ZERO.clone()],
                [DBig::ZERO.clone(), DBig::ZERO.clone(), DBig::ONE.clone()],
            ],
        }
    }

    pub fn axis_angle(axis: &DecimalVector3d, angle: DBig) -> DecimalMatrix3d {
        // angle is negated to match the Three JS behavior, no idea why
        let c = &cos(-&angle, 32);
        let s = &sin(-&angle, 32);
        let one_minus_c = &DBig::ONE - c;
        DecimalMatrix3d {
            data: [
                [
                    &one_minus_c * &axis.x * &axis.x + c,
                    &one_minus_c * &axis.x * &axis.y - &axis.z * s,
                    &one_minus_c * &axis.z * &axis.x + &axis.y * s,
                ],
                [
                    &one_minus_c * &axis.x * &axis.y + &axis.z * s,
                    &one_minus_c * &axis.y * &axis.y + c,
                    &one_minus_c * &axis.y * &axis.z - &axis.x * s,
                ],
                [
                    &one_minus_c * &axis.z * &axis.x - &axis.y * s,
                    &one_minus_c * &axis.y * &axis.z + &axis.x * s,
                    &one_minus_c * &axis.z * &axis.z + c,
                ],
            ],
        }
    }

    pub fn apply(&self, vector: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::ZERO
                + &self.data[0][0] * &vector.x
                + &self.data[1][0] * &vector.y
                + &self.data[2][0] * &vector.z,

            y: DBig::ZERO
                + &self.data[0][1] * &vector.x
                + &self.data[1][1] * &vector.y
                + &self.data[2][1] * &vector.z,

            z: DBig::ZERO
                + &self.data[0][2] * &vector.x
                + &self.data[1][2] * &vector.y
                + &self.data[2][2] * &vector.z,
        }
    }

    pub fn as_quat(&self) -> [DBig; 4] {
        let f_trace = &self.data[0][0] + &self.data[1][1] + &self.data[2][2];
        let half = DBIGHALF.deref();

        if f_trace > DBig::ZERO {
            let f_root = (f_trace + DBig::ONE).sqrt();
            let w = half * &f_root;
            let half_by_f_root = half / &f_root;
            let x = (&self.data[1][2] - &self.data[2][1]) * &half_by_f_root;
            let y = (&self.data[2][0] - &self.data[0][2]) * &half_by_f_root;
            let z = (&self.data[0][1] - &self.data[1][0]) * &half_by_f_root;
            [x, y, z, w]
        } else {
            let mut i = 0;
            if self.data[1][1] > self.data[0][0] {
                i = 1;
            }
            if self.data[2][2] > self.data[i][i] {
                i = 2;
            }
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;

            let f_root =
                (&self.data[i][i] - &self.data[j][j] - &self.data[k][k] + DBig::ONE).sqrt();
            let mut out = [DBig::ZERO, DBig::ZERO, DBig::ZERO, DBig::ZERO];
            out[i] = half * &f_root;
            let half_by_f_root = half / &f_root;
            out[3] = (&self.data[j][k] - &self.data[k][j]) * &half_by_f_root;
            out[j] = (&self.data[j][i] + &self.data[i][j]) * &half_by_f_root;
            out[k] = (&self.data[k][i] + &self.data[i][k]) * &half_by_f_root;
            out
        }
    }

    pub fn as_dquat(&self) -> DQuat {
        let q = self.as_quat();
        DQuat::from_xyzw(
            q[0].to_f64().value(),
            q[1].to_f64().value(),
            q[2].to_f64().value(),
            q[3].to_f64().value(),
        )
    }

    pub fn as_dmat4(&self) -> DMat4 {
        let mut data: [[f64; 3]; 3] = [[0.0; 3]; 3];
        for x in 0..3 {
            for y in 0..3 {
                data[x][y] = self.data[x][y].to_f64().value();
            }
        }
        DMat4::from_mat3(DMat3::from_cols_array_2d(&data))
    }
}
