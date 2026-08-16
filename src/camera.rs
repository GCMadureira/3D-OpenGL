use crate::mat::*;

pub struct Camera {
    matrix: Mat4,
    eye: Vec4,
    target: Vec4,
    up: Vec4,
    changed: bool,
}

impl Camera {
    pub fn new(eye: &Vec4, target: &Vec4, up: &Vec4) -> Self {
        Camera {
            matrix: view_matrix(eye, target, up),
            eye: *eye,
            target: *target,
            up: *up,
            changed: false
        }
    }

    pub fn get_view_matrix(&mut self) -> &Mat4 {
        if self.changed {
            self.matrix = view_matrix(&self.eye, &self.target, &self.up);
            self.changed = false;
        }

        &self.matrix
    }

    pub fn move_horizontal(&mut self, offset: f32) -> &Self {
        let mut xaxis = self.up.cross_mult((self.eye - self.target).normalize_self());
        *xaxis.normalize_self() *= offset;

        self.eye += xaxis;
        self.target += xaxis;

        self.changed = true;
        self
    }

    pub fn move_vertical(&mut self, offset: f32) -> &Self {
        let zaxis = *(self.eye - self.target).normalize_self();
        let mut xaxis = self.up.cross_mult(&zaxis);
        let yaxis = zaxis.cross_mult(xaxis.normalize_self()) * offset;

        self.eye += yaxis;
        self.target += yaxis;

        self.changed = true;
        self
    }

    pub fn move_depth(&mut self, offset: f32) -> &Self {
        let zaxis = *(self.eye - self.target).normalize_self() * offset;

        self.eye += zaxis;
        self.target += zaxis;

        self.changed = true;
        self
    }

    pub fn rotate_horizontal(&mut self, angle: f32) -> &Self {
        let new_target = self.target - self.eye;

        let rot_matrix = Mat4::rotate_matrix(angle, self.up[0], self.up[1], self.up[2]);
        let new_target = rot_matrix * new_target;

        self.target = new_target + self.eye;

        self.changed = true;
        self
    }

    pub fn rotate_vertical(&mut self, angle: f32) -> &Self {
        let new_target = self.target - self.eye;
        let mut xaxis = self.up.cross_mult((self.eye - self.target).normalize_self());
        xaxis.normalize_self();

        let rot_matrix = Mat4::rotate_matrix(angle, xaxis[0], xaxis[1], xaxis[2]);
        let new_target = rot_matrix * new_target;

        self.target = new_target + self.eye;
        self.up = rot_matrix * self.up;

        self.changed = true;
        self
    }

    pub fn rotate_roll(&mut self, angle: f32) -> &Self {
        let zaxis = *(self.eye - self.target).normalize_self();

        let rot_matrix = Mat4::rotate_matrix(angle, zaxis[0], zaxis[1], zaxis[2]);

        self.up = rot_matrix * self.up;

        self.changed = true;
        self
    }
}