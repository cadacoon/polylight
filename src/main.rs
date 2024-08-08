use eframe::egui;
use eframe::egui::TextureHandle;
use glam::Vec3;
use image::ImageBuffer;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    );

   /* println!("Hello, world!");
    let sphere = Sphere {
        center: Vec3::new(3.0, 1.0, 1.0),
        radius_sq: 1.0 * 1.0,
    };
    // let hit = sphere.intersect(Ray {
    //     o: Vec3::new(0.0, 0.0, 0.0),
    //     d: Vec3::new(0.1, 0.0, 0.01).normalize(),
    // });
    // println!("{:?}", hit);
    let mut image = ImageBuffer::new(256, 256);
    let camera = Camera::new(Vec3::ZERO, Vec3::X, 80.0f32.to_radians());
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let x_frac = x as f32 / 256.0;
        let y_frac = y as f32 / 256.0;
        let ray = camera.trace(x_frac, y_frac);
        //println!("{:?}", ray);
        let hit = sphere.intersect(ray);

        *pixel = image::Rgb(if hit.t > 0.0 {
            let ni = (hit.n * 256.0).as_ivec3();
            [ni.x as u8, ni.y as u8, ni.z as u8]
        } else {
            [0, 0, 0]
        });
    }

    image.save("output.png").unwrap();*/
}

#[derive(Default)]
struct MyApp {
    texture: Option<TextureHandle>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let texture = self.texture.get_or_insert_with(|| ctx.load_texture("", egui::ColorImage::example(), Default::default()));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image((texture.id(), texture.size_vec2()));
        });
    }
}

#[derive(Debug)]
struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.o + t * self.d
    }
}

#[derive(Debug)]
struct Hit {
    t: f32,
    n: Vec3,
}

struct Sphere {
    center: Vec3,
    radius_sq: f32,
}

impl Sphere {
    fn intersect(&self, ray: Ray) -> Hit {
        // let s = self.center - ray.o;
        // let sd = s.dot(ray.d);
        // let ss = s.length_squared();
        // let disc = sd * sd - ss + self.radius_sq;
        // if disc.is_sign_negative() {
        //     return Hit {
        //         t: -1.0,
        //         n: Vec3::ZERO,
        //     };
        // }

        // let disc_sqrt = disc.sqrt();
        // let p1 = sd - disc_sqrt;
        // let p2 = sd + disc_sqrt;
        // let t = if p1 < 0.0 {
        //     p2
        // } else if p2 < 0.0 {
        //     p1
        // } else {
        //     p1.min(p2)
        // };
        // let n = ray.at(t).normalize();
        // Hit { t, n }
        let origin = ray.o - self.center;

        let a = ray.d.length_squared();
        let b = 2.0 * (ray.d.dot(origin));
        let c = origin.length_squared() - self.radius_sq;

        let mut delta = b * b - 4.0 * a * c;
        if delta.is_sign_negative() {
            return Hit {
                t: -1.0,
                n: Vec3::ZERO,
            };
        }

        delta = delta.sqrt();
        let p1 = (-b - delta) / (2.0 * a);
        let p2 = (-b + delta) / (2.0 * a);
        if p1 <= f32::EPSILON && p2 <= f32::EPSILON {
            return Hit {
                t: -1.0,
                n: Vec3::ZERO,
            };
        }

        let t = if p1 <= f32::EPSILON {
            p2
        } else if p2 <= f32::EPSILON {
            p1
        } else {
            p1.min(p2)
        };
        let n = ray.at(t).normalize();
        Hit { t, n }
    }
}

// struct Camera {
//     position: Vec3,
//     focal_plane: [Vec3; 4],
// }

// impl Camera {
//     fn new(position: Vec3, target: Vec3, fov: f32) -> Self {
//         let z_axis = (target - position).normalize();
//         let x_axis = (Vec3::new(0.0, 1.0, 1.0).cross(z_axis)).normalize();
//         let y_axis = (z_axis.cross(x_axis)).normalize();

//         println!("{} {} {}", x_axis, y_axis, z_axis);
//         let inverse_view_matrix = [
//             Vec3::new(x_axis.x, y_axis.x, z_axis.x),
//             Vec3::new(x_axis.y, y_axis.y, z_axis.y),
//             Vec3::new(x_axis.z, y_axis.z, z_axis.z),
//         ];

//         let fov = (fov * 0.5).tan();

//         let multiplicand = Vec3::new(-fov, -fov, 1.0);
//         let focal_plane_0 = Vec3::new(
//             multiplicand.dot(inverse_view_matrix[0]),
//             multiplicand.dot(inverse_view_matrix[1]),
//             multiplicand.dot(inverse_view_matrix[2]),
//         );
//         let multiplicand = Vec3::new(fov, -fov, 1.0);
//         let focal_plane_1 = Vec3::new(
//             multiplicand.dot(inverse_view_matrix[0]),
//             multiplicand.dot(inverse_view_matrix[1]),
//             multiplicand.dot(inverse_view_matrix[2]),
//         );
//         let multiplicand = Vec3::new(fov, fov, 1.0);
//         let focal_plane_2 = Vec3::new(
//             multiplicand.dot(inverse_view_matrix[0]),
//             multiplicand.dot(inverse_view_matrix[1]),
//             multiplicand.dot(inverse_view_matrix[2]),
//         );
//         let multiplicand = Vec3::new(-fov, fov, 1.0);
//         let focal_plane_3 = Vec3::new(
//             multiplicand.dot(inverse_view_matrix[0]),
//             multiplicand.dot(inverse_view_matrix[1]),
//             multiplicand.dot(inverse_view_matrix[2]),
//         );
//         println!(
//             "{:?} {:?} {:?} {:?} {:?}",
//             focal_plane_0, focal_plane_1, focal_plane_2, focal_plane_3, inverse_view_matrix
//         );

//         Self {
//             position,
//             focal_plane: [focal_plane_0, focal_plane_1, focal_plane_2, focal_plane_3],
//         }
//     }

//     fn trace(&self, u: f32, v: f32) -> Ray {
//         Ray {
//             o: self.position,
//             d: self.focal_plane[0]
//                 .lerp(self.focal_plane[1], (u + 1.0) * 0.5)
//                 .lerp(
//                     self.focal_plane[3].lerp(self.focal_plane[2], (u + 1.0) * 0.5),
//                     (1.0 - v) * 0.5,
//                 ),
//         }
//     }
// }

struct Camera {
    eye: Vec3,
    dir: Vec3,
    up: Vec3,
    fov: f32,
}

impl Camera {
    fn new(position: Vec3, target: Vec3, fov: f32) -> Self {
        let dir = (target - position).normalize();
        let up = Vec3::Y;
        let up = (up - up.dot(dir) * dir).normalize();
        Self {
            eye: position,
            dir,
            up,
            fov,
        }
    }

    fn trace(&self, x: f32, y: f32) -> Ray {
        let d = (self.fov / 2.0).tan().recip();
        let right = self.dir.cross(self.up).normalize();
        let origin = self.eye;
        let new_dir = d * self.dir + x * right + y * self.up;
        Ray {
            o: origin,
            d: new_dir.normalize(),
        }
    }
}
