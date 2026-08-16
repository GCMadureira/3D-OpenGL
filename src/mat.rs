use std::ops::{IndexMut, Index, Add, Sub, Mul, AddAssign, SubAssign, MulAssign, Div, DivAssign};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4([f32; 4]);
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }

    pub fn zero() -> Self {
        Self([0.0, 0.0, 0.0, 0.0])
    }
    
    pub fn normalize_self(&mut self) -> &mut Self {
        let len = f32::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2] + self[3] * self[3]);

        if len == 0.0 {
            panic!("Zero length vector provided, cannot normalize!");
        }

        self[0] /= len;
        self[1] /= len;
        self[2] /= len;
        self[3] /= len;

        self
    }

    pub fn normalize(mut self) -> Self {
        let len = f32::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2] + self[3] * self[3]);

        if len == 0.0 {
            panic!("Zero length vector provided, cannot normalize!");
        }

        self[0] /= len;
        self[1] /= len;
        self[2] /= len;
        self[3] /= len;

        self
    }

    // ignores the 4th value and assumes both vectors are 3D
    pub fn cross_mult(&self, other: &Self) -> Self {
        Vec4([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
            0.0
        ])
    }

    pub fn dot_mult(&self, other: &Self) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }

    pub fn add(&mut self, other: &Self) -> &mut Self {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
        self[3] += other[3];

        self
    }

    pub fn sub(&mut self, other: &Self) -> &mut Self {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
        self[3] -= other[3];

        self
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, ind: usize) -> &Self::Output {
        &self.0[ind]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, ind: usize) -> &mut Self::Output {
        &mut self.0[ind]
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self([self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]])
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self([self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]])
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs.post_multiply(&self)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs
        )
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        Vec4::new(
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs
        )
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4([Vec4; 4]);
impl Mat4 {
    pub fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self([x, y, z, w])
    }

    pub fn as_ptr(&self) -> *const f32 {
        self as *const Mat4 as *const f32
    }

    pub fn identity() -> Self {
        Self([
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }
    
    pub fn pre_multiply(&self, v: &Vec4) -> Vec4 {
        Vec4::new(
            self[0][0] * v[0] + self[1][0] * v[1] + self[2][0] * v[2] + self[3][0] * v[3],
            self[0][1] * v[0] + self[1][1] * v[1] + self[2][1] * v[2] + self[3][1] * v[3],
            self[0][2] * v[0] + self[1][2] * v[1] + self[2][2] * v[2] + self[3][2] * v[3],
            self[0][3] * v[0] + self[1][3] * v[1] + self[2][3] * v[2] + self[3][3] * v[3]

        )
    }

    pub fn post_multiply(&self, v: &Vec4) -> Vec4 {
        Vec4::new(
            v[0] * self[0][0] + v[1] * self[0][1] + v[2] * self[0][2] + v[3] * self[0][3],
            v[0] * self[1][0] + v[1] * self[1][1] + v[2] * self[1][2] + v[3] * self[1][3],
            v[0] * self[2][0] + v[1] * self[2][1] + v[2] * self[2][2] + v[3] * self[2][3],
            v[0] * self[3][0] + v[1] * self[3][1] + v[2] * self[3][2] + v[3] * self[3][3]
        )
    }

    pub fn multiply(&self, other: &Mat4) -> Self {
        let x = *self * other[0];
        let y = *self * other[1];
        let z = *self * other[2];
        let w = *self * other[3];

        Mat4([x, y, z, w])
    }

    pub fn translate_by(&self, x: f32, y: f32, z: f32) -> Mat4 {
        Self::translate_matrix(x, y, z) * *self
    }

    pub fn scale_by(&self, x: f32, y: f32, z: f32) -> Mat4 {
        Self::scale_matrix(x, y, z) * *self
    }

    pub fn rotate_by_zaxis(&self, angle: f32) -> Mat4 {
        Self::rotate_matrix_zaxis(angle) * *self
    }

    pub fn rotate_by_xaxis(&self, angle: f32) -> Mat4 {
        Self::rotate_matrix_xaxis(angle) * *self
    }

    pub fn rotate_by_yaxis(&self, angle: f32) -> Mat4 {
        Self::rotate_matrix_yaxis(angle) * *self
    }

    pub fn rotate_by(&self, angle: f32, x: f32, y:f32, z: f32) -> Mat4 {
        Self::rotate_matrix(angle, x, y, z) * *self
    }

    pub fn translate_matrix(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(x, y , z, 1.0),
        )
    }

    pub fn scale_matrix(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rotate_matrix_zaxis(angle: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(angle.cos(), angle.sin(), 0.0, 0.0),
            Vec4::new(-angle.sin(), angle.cos(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rotate_matrix_xaxis(angle: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(1.0,0.0, 0.0, 0.0),
            Vec4::new(0.0, angle.cos(), angle.sin(), 0.0),
            Vec4::new(0.0, -angle.sin(), angle.cos(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rotate_matrix_yaxis(angle: f32) -> Mat4 {
        Mat4::new(
            Vec4::new(angle.cos(), 0.0, -angle.sin(), 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(angle.sin(), 0.0, angle.cos(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rotate_matrix(angle: f32, x: f32, y:f32, z: f32) -> Mat4 {
        let len = x.hypot(y).hypot(z);
        if len == 0.0 {
            panic!("Invalid axis provided!");
        }

        // normalize the coords
        let xa = x/len;
        let ya = y/len;
        let za = z/len;

        let s = angle.sin();
        let c = angle.cos();
        let t = 1.0 - c;

        Mat4::new(
            Vec4::new(t*xa*xa + c, t*xa*ya + s*za, t*xa*za - s*ya, 0.0),
            Vec4::new(t*xa*ya - s*za, t*ya*ya + c, t*ya*za + s*xa, 0.0),
            Vec4::new(t*xa*za + s*ya, t*ya*za - s*xa, t*za*za + c, 0.0),
            Vec4::new(0.0, 0.0,0.0, 1.0),
        )
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, ind: usize) -> &Self::Output {
        &self.0[ind]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, ind: usize) -> &mut Self::Output {
        &mut self.0[ind]
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.pre_multiply(&rhs)
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(&rhs)
    }
}

// receives 4D vectors as parameters but ignores the fourth dimension; assumes they are 3D vectors of type (x, y, z, 0)
pub fn view_matrix(eye: &Vec4, target: &Vec4, up: &Vec4) -> Mat4 { 
    let zaxis = *(*eye - *target).normalize_self();
    let xaxis = *up.cross_mult(&zaxis).normalize_self();
    let yaxis = zaxis.cross_mult(&xaxis);

    Mat4::new(
        Vec4::new(xaxis[0], yaxis[0], zaxis[0], 0.0),
        Vec4::new(xaxis[1], yaxis[1], zaxis[1], 0.0),
        Vec4::new(xaxis[2], yaxis[2], zaxis[2], 0.0),
        Vec4::new(-xaxis.dot_mult(eye), -yaxis.dot_mult(eye), -zaxis.dot_mult(eye), 1.0)
    )
}

pub fn projection_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();

    Mat4::new(
        Vec4::new(f / aspect, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, (far + near) / (near - far), -1.0),
        Vec4::new(0.0, 0.0, (2.0 * far * near) / (near - far), 0.0),
    )
}



// tests written by chat because there is no way I am calculating and writing matrixes and vectors by hand
#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;

    fn assert_vec4_eq(a: Vec4, b: Vec4) {
        for i in 0..4 {
            assert!(
                (a[i] - b[i]).abs() < EPS,
                "component {} differs: {} != {}",
                i,
                a[i],
                b[i]
            );
        }
    }

    fn assert_mat4_eq(a: Mat4, b: Mat4) {
        for r in 0..4 {
            assert_vec4_eq(a[r], b[r]);
        }
    }

    #[test]
    fn normalize() {
        let v = Vec4::new(3.0, 4.0, 0.0, 0.0);

        assert_vec4_eq(
            v.normalize(),
            Vec4::new(0.6, 0.8, 0.0, 0.0),
        );
    }

    #[test]
    fn dot_product() {
        let a = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 0.0);

        assert!((a.dot_mult(&b) - 32.0).abs() < EPS);
    }

    #[test]
    fn cross_product() {
        let x = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let y = Vec4::new(0.0, 1.0, 0.0, 0.0);

        assert_vec4_eq(
            x.cross_mult(&y),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
        );
    }

    #[test]
    fn identity_matrix() {
        let v = Vec4::new(5.0, -2.0, 8.0, 1.0);

        assert_vec4_eq(
            Mat4::identity() * v,
            v,
        );
    }

    #[test]
    fn translation() {
        let m = Mat4::identity().translate_by(1.0, 2.0, 3.0);

        let v = Vec4::new(0.0, 0.0, 0.0, 1.0);

        assert_vec4_eq(
            m * v,
            Vec4::new(1.0, 2.0, 3.0, 1.0),
        );
    }

    #[test]
    fn scaling() {
        let m = Mat4::identity().scale_by(2.0, 3.0, 4.0);

        let v = Vec4::new(1.0, 1.0, 1.0, 1.0);

        assert_vec4_eq(
            m * v,
            Vec4::new(2.0, 3.0, 4.0, 1.0),
        );
    }

    #[test]
    fn rotate_z_90() {
        let m = Mat4::identity()
            .rotate_by_zaxis(std::f32::consts::FRAC_PI_2);

        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);

        assert_vec4_eq(
            m * v,
            Vec4::new(0.0, 1.0, 0.0, 1.0),
        );
    }

    #[test]
    fn matrix_multiplication_identity() {
        let m = Mat4::identity().translate_by(1.0, 2.0, 3.0);

        assert_mat4_eq(
            Mat4::identity() * m,
            m,
        );

        assert_mat4_eq(
            m * Mat4::identity(),
            m,
        );
    }

    #[test]
    fn look_at_origin() {
        let eye = Vec4::new(0.0, 0.0, 1.0, 0.0);
        let target = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let up = Vec4::new(0.0, 1.0, 0.0, 0.0);

        let view = view_matrix(&eye, &target, &up);

        let expected = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, 1.0),
        );

        assert_mat4_eq(view, expected);
    }

    #[test]
    fn translate_then_rotate_is_not_rotate_then_translate() {
        let translate = Mat4::identity().translate_by(1.0, 0.0, 0.0);
        let rotate = Mat4::identity().rotate_by_zaxis(std::f32::consts::FRAC_PI_2);

        let p = Vec4::new(1.0, 0.0, 0.0, 1.0);

        let a = (translate * rotate) * p;
        let b = (rotate * translate) * p;

        assert!(
            (a[0] - b[0]).abs() > EPS ||
            (a[1] - b[1]).abs() > EPS
        );
    }
}