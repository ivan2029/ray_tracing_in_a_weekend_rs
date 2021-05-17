#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use crate::cgmath::{angle::Radians, normal::NormalizedVec3, vec3::Vec3};

pub type Matrix3x4 = [[f32; 4]; 3];

fn compose_matrix3x4(
    a: &Matrix3x4,
    b: &Matrix3x4,
) -> Matrix3x4 {
    let mut c = Matrix3x4::default();

    for row in 0..3 {
        for col in 0..3 {
            for k in 0..3 {
                c[row][col] += a[row][k] * b[k][col];
            }
        }
        for k in 0..3 {
            c[row][3] += a[row][k] * b[k][3];
        }
        c[row][3] += a[row][3];
    }

    c
}

fn transpose_sub3x3(a: &Matrix3x4) -> Matrix3x4 {
    [
        [a[0][0], a[1][0], a[2][0], a[0][3]],
        [a[0][1], a[1][1], a[2][1], a[1][3]],
        [a[0][2], a[1][2], a[2][2], a[2][3]],
    ]
}

#[derive(Debug, Clone)]
pub struct Transform {
    forward: Matrix3x4,
    backward: Matrix3x4,
}

impl Transform {
    pub fn identity() -> Transform {
        #[rustfmt::skip]
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        Transform {
            forward: m,
            backward: m,
        }
    }

    pub fn translation(offset: Vec3) -> Transform {
        #[rustfmt::skip]
        let forward = [
            [1.0, 0.0, 0.0, offset.x()],
            [0.0, 1.0, 0.0, offset.y()],
            [0.0, 0.0, 1.0, offset.z()],
        ];

        #[rustfmt::skip]
        let backward = [
            [1.0, 0.0, 0.0, -offset.x()],
            [0.0, 1.0, 0.0, -offset.y()],
            [0.0, 0.0, 1.0, -offset.z()],
        ];

        Transform { forward, backward }
    }

    pub fn rotation_x(angle: Radians) -> Transform {
        let c = angle.0.cos();
        let s = angle.0.sin();

        #[rustfmt::skip]
        let forward = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0,   c,  -s, 0.0],
            [0.0,   s,   c, 0.0],
        ];

        let backward = transpose_sub3x3(&forward);

        Transform { forward, backward }
    }

    pub fn rotation_y(angle: Radians) -> Transform {
        let c = angle.0.cos();
        let s = angle.0.sin();

        #[rustfmt::skip]
        let forward = [
            [  c, 0.0,  -s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [  s, 0.0,   c, 0.0],
        ];

        let backward = transpose_sub3x3(&forward);

        Transform { forward, backward }
    }

    pub fn rotation_z(angle: Radians) -> Transform {
        let c = angle.0.cos();
        let s = angle.0.sin();

        #[rustfmt::skip]
        let forward = [
            [  c,  -s, 0.0, 0.0],
            [  s,   c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];

        let backward = transpose_sub3x3(&forward);

        Transform { forward, backward }
    }

    pub fn rotation(
        angle: Radians,
        axis: NormalizedVec3,
    ) -> Transform {
        let c = angle.0.cos();
        let s = angle.0.sin();

        let compute_row = |v| {
            let a = axis.to_vec3();
            let v_c = a * Vec3::dot(v, a);
            let v_1 = v - v_c;
            let v_2 = Vec3::cross(v_1, a);
            let v_p = v_c + v_1 * c + v_2 * s;

            [v_p.x(), v_p.y(), v_p.z(), 0.0]
        };

        let forward = [
            compute_row(Vec3::X),
            compute_row(Vec3::Y),
            compute_row(Vec3::Z),
        ];

        let backward = transpose_sub3x3(&forward);

        Transform { forward, backward }
    }

    pub fn scale(
        x: f32,
        y: f32,
        z: f32,
    ) -> Transform {
        assert!(x > 0.0);
        assert!(y > 0.0);
        assert!(z > 0.0);

        #[rustfmt::skip]
        let forward = [
            [  x, 0.0, 0.0, 0.0],
            [0.0,   y, 0.0, 0.0],
            [0.0, 0.0,   z, 0.0],
        ];

        #[rustfmt::skip]
        let inverse = [
            [x.recip(),       0.0,       0.0, 0.0],
            [      0.0, y.recip(),       0.0, 0.0],
            [      0.0,       0.0, z.recip(), 0.0],
        ];

        Transform {
            forward,
            backward: inverse,
        }
    }

    pub fn compose(
        a: &Transform,
        b: &Transform,
    ) -> Transform {
        let forward = compose_matrix3x4(a.forward(), b.forward());
        let backward = compose_matrix3x4(b.backward(), a.backward());

        Transform { forward, backward }
    }

    pub fn forward(&self) -> &Matrix3x4 {
        &self.forward
    }

    pub fn backward(&self) -> &Matrix3x4 {
        &self.backward
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            forward: self.backward,
            backward: self.forward,
        }
    }
}
