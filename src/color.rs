use std::simd::prelude::*;

pub struct Spd<'a>(pub &'a [(f32, f32)]);

pub struct Xyz(f32x4);

impl<'a> From<Spd<'a>> for Xyz {
    fn from(value: Spd) -> Self {
        let mut xyz = f32x4::default();
        for &(wavelength, radiation) in value.0 {
            xyz += f32x4::splat(radiation)
                * f32x4::from([
                    xfit_1931(wavelength),
                    yfit_1931(wavelength),
                    zfit_1931(wavelength),
                    0.0,
                ]);
        }
        Xyz(xyz)
    }
}

impl Xyz {
    pub fn to_linear(self, color_space: ColorSpace) -> [f32; 4] {
        let mut rgb = simd_swizzle!(color_space.xyz, [0, 4, 8, 12]) * f32x4::splat(self.0[0]);
        rgb += simd_swizzle!(color_space.xyz, [1, 5, 9, 13]) * f32x4::splat(self.0[1]);
        rgb += simd_swizzle!(color_space.xyz, [2, 6, 10, 14]) * f32x4::splat(self.0[2]);
        return rgb.to_array();
    }

    pub fn to_nonlinear(self, color_space: ColorSpace) -> [f32; 4] {
        let mut rgb = simd_swizzle!(color_space.xyz, [0, 4, 8, 12]) * f32x4::splat(self.0[0]);
        rgb += simd_swizzle!(color_space.xyz, [1, 5, 9, 13]) * f32x4::splat(self.0[1]);
        rgb += simd_swizzle!(color_space.xyz, [2, 6, 10, 14]) * f32x4::splat(self.0[2]);
        for x in rgb.as_mut_array() {
            *x = (color_space.oetf)(*x);
        }
        return rgb.to_array();
    }
}

pub struct ColorSpace {
    xyz: f32x16,
    oetf: fn(f32) -> f32,
}

impl ColorSpace {
    const fn new(r: [f32; 2], g: [f32; 2], b: [f32; 2], w: [f32; 2], oetf: fn(f32) -> f32) -> Self {
        let r2 = 1.0 - (r[0] + r[1]);
        let g2 = 1.0 - (g[0] + g[1]);
        let b2 = 1.0 - (b[0] + b[1]);
        let w2 = 1.0 - (w[0] + w[1]);

        let mut rx = (g[1] * b2) - (b[1] * g2);
        let mut ry = (b[0] * g2) - (g[0] * b2);
        let mut rz = (g[0] * b[1]) - (b[0] * g[1]);
        let rw = ((rx * w[0]) + (ry * w[1]) + (rz * w2)) / w[1];
        rx = rx / rw;
        ry = ry / rw;
        rz = rz / rw;

        let mut gx = (b[1] * r2) - (r[1] * b2);
        let mut gy = (r[0] * b2) - (b[0] * r2);
        let mut gz = (b[0] * r[1]) - (r[0] * b[1]);
        let gw = ((gx * w[0]) + (gy * w[1]) + (gz * w2)) / w[1];
        gx = gx / gw;
        gy = gy / gw;
        gz = gz / gw;

        let mut bx = (r[1] * g2) - (g[1] * r2);
        let mut by = (g[0] * r2) - (r[0] * g2);
        let mut bz = (r[0] * g[1]) - (g[0] * r[1]);
        let bw = ((bx * w[0]) + (by * w[1]) + (bz * w2)) / w[1];
        bx = bx / bw;
        by = by / bw;
        bz = bz / bw;

        Self {
            xyz: f32x16::from_array([
                rx, ry, rz, 0.0, gx, gy, gz, 0.0, bx, by, bz, 0.0, 0.0, 0.0, 0.0, 1.0,
            ]),
            oetf,
        }
    }
}

// Wyman, Chris; Sloan, Peter-Pike; Shirley, Peter (July 12, 2013). "Simple Analytic Approximations to the CIE XYZ Color Matching Functions". Journal of Computer Graphics Techniques. 2 (2): 1-11. ISSN 2331-7418.
fn xfit_1931(wave: f32) -> f32 {
    let t1 = (wave - 442.0) * if wave < 442.0 { 0.0624 } else { 0.0374 };
    let t2 = (wave - 599.8) * if wave < 599.8 { 0.0264 } else { 0.0323 };
    let t3 = (wave - 501.1) * if wave < 501.1 { 0.0490 } else { 0.0382 };
    0.362 * (-0.5 * t1 * t1).exp() + (1.056 * (-0.5 * t2 * t2).exp())
        - (0.065 * (-0.5 * t3 * t3).exp())
}

fn yfit_1931(wave: f32) -> f32 {
    let t1 = (wave - 568.8) * if wave < 568.8 { 0.0213 } else { 0.0247 };
    let t2 = (wave - 530.9) * if wave < 530.9 { 0.0613 } else { 0.0322 };
    0.821 * (-0.5 * t1 * t1).exp() + (0.286 * (-0.5 * t2 * t2).exp())
}

fn zfit_1931(wave: f32) -> f32 {
    let t1 = (wave - 437.0) * if wave < 437.0 { 0.0845 } else { 0.0278 };
    let t2 = (wave - 459.0) * if wave < 459.0 { 0.0385 } else { 0.0725 };
    1.217 * (-0.5 * t1 * t1).exp() + (0.681 * (-0.5 * t2 * t2).exp())
}

const D65: [f32; 2] = [0.3127, 0.3290];

// ITU-R BT.709
fn bt709_oetf(value: f32) -> f32 {
    if value < 0.018 {
        4.5 * value
    } else {
        1.099 * value.powf(0.45) - 0.099
    }
}

pub const BT709: ColorSpace =
    ColorSpace::new([0.64, 0.33], [0.30, 0.60], [0.15, 0.06], D65, bt709_oetf);

// W3C sRGB
fn srgb_oetf(value: f32) -> f32 {
    if value <= 0.00304 {
        12.92 * value
    } else {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    }
}

pub const SRGB: ColorSpace =
    ColorSpace::new([0.64, 0.33], [0.30, 0.60], [0.15, 0.06], D65, srgb_oetf);
