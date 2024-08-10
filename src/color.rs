use parry3d::na::{Matrix3, Matrix4x2, SimdValue, Vector3, Vector4};

pub const SPD_SAMPLES: usize = 16;
pub const SPD_START: f32 = 380.0;
pub const SPD_END: f32 = 750.0;
pub const SPD_INTERVAL: f32 = (SPD_END - SPD_START) / SPD_SAMPLES as f32;

struct Spd([f32; SPD_SAMPLES]);

struct Xyz(Vector3<f32>);

impl From<Spd> for Xyz {
    fn from(value: Spd) -> Self {
        let mut wavelength = SPD_START;
        let mut xyz = Vector3::new(0.0, 0.0, 0.0);
        for power in value.0 {
            let x = {
                let t1 = (wavelength - 442.0) * if wavelength < 442.0 {
                    0.0624
                } else {
                    0.0374
                };
                let t2 = (wavelength - 599.8) * if wavelength < 599.8 {
                    0.0264
                } else {
                    0.0323
                };
                let t3 = (wavelength - 501.1) * if wavelength < 501.1 {
                    0.0490
                } else {
                    0.0382
                };
                0.362 * (-0.5 * t1 * t1).exp() + (1.056 * (-0.5 * t2 * t2).exp()) - (0.065 * (-0.5 * t3 * t3).exp())
            };
            let y = {
                let t1 = (wavelength - 568.8) * if wavelength < 568.8 {
                    0.0213
                } else {
                    0.0247
                };
                let t2 = (wavelength - 530.9) * if wavelength < 530.9 {
                    0.0613
                } else {
                    0.0322
                };
                0.821 * (-0.5 * t1 * t1).exp() + (0.286 * (-0.5 * t2 * t2).exp())
            };
            let z = {
                let t1 = (wavelength - 437.0) * if wavelength < 437.0 {
                    0.0845
                } else {
                    0.0278
                };
                let t2 = (wavelength - 459.0) * if wavelength < 459.0 {
                    0.0385
                } else {
                    0.0725
                };
                1.217 * (-0.5 * t1 * t1).exp() + (0.681 * (-0.5 * t2 * t2).exp())
            };
            wavelength += SPD_INTERVAL;
            xyz += Vector3::new(x, y, z) * power;
        }
        Xyz(xyz)
    }
}

impl Xyz {
    fn rgb(self, color_space: ColorSpace) -> Vector3<f32> {
        color_space.0 * self.0
    }
}

const REC709: ColorSpace = ColorSpace::new([0.3127, 0.3290], [[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]]);

struct ColorSpace(Matrix3<f32>);

impl ColorSpace {
    const fn new(white: [f32; 2], primaries: [[f32; 2]; 3]) -> Self {
        let xr = primaries[0][0];
        let yr = primaries[0][1];
        let xg = primaries[1][0];
        let yg = primaries[1][1];
        let xb = primaries[2][0];
        let yb = primaries[2][1];
        let xw = white[0];
        let yw = white[1];
        let zr = 1.0 - (xr + yr);
        let zg = 1.0 - (xg + yg);
        let zb = 1.0 - (xb + yb);
        let zw = 1.0 - (xw + yw);

        let mut rx = (yg * zb) - (yb * zg);
        let mut ry = (xb * zg) - (xg * zb);
        let mut rz = (xg * yb) - (xb * yg);
        let mut gx = (yb * zr) - (yr * zb);
        let mut gy = (xr * zb) - (xb * zr);
        let mut gz = (xb * yr) - (xr * yb);
        let mut bx = (yr * zg) - (yg * zr);
        let mut by = (xg * zr) - (xr * zg);
        let mut bz = (xr * yg) - (xg * yr);

        let rw = ((rx * xw) + (ry * yw) + (rz * zw)) / yw;
        let gw = ((gx * xw) + (gy * yw) + (gz * zw)) / yw;
        let bw = ((bx * xw) + (by * yw) + (bz * zw)) / yw;
        rx = rx / rw;
        ry = ry / rw;
        rz = rz / rw;
        gx = gx / gw;
        gy = gy / gw;
        gz = gz / gw;
        bx = bx / bw;
        by = by / bw;
        bz = bz / bw;

        Self(Matrix3::new(
            rx, ry, rz,
            gx, gy, gz,
            bx, by, bz,
        ))
    }
}
