#[link(name     = "sphere3d"
       , vers   = "0.0"
       , author = "Sébastien Crozet"
       , uuid   = "cf8cfe5d-18ca-40cb-b596-d8790090a56d")];
#[crate_type = "bin"];
#[warn(non_camel_case_types)]

extern mod nalgebra;
extern mod ncollide;
extern mod nrays;

use std::io;
use nalgebra::na::{Iso3, Vec3, Mat4};
use nalgebra::na;
use ncollide::ray::Ray;
use ncollide::geom::{Geom, Ball, Box, Cone, Cylinder};
use nrays::scene_node::{Material, SceneNode};
use nrays::scene::Scene;
use nrays::light::Light;

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    let resolution = na::vec2(1024u, 1024);
    let mut lights = ~[];
    let mut nodes  = ~[];

    {
        lights.push(Light::new(na::vec3(0.0f64, 2.0, 10.0),
                               na::vec3(1.0, 0.0, 0.0)));
        // lights.push(Light::new(na::vec3(-10.0f64, 10.0, 10.0),
        //                        na::vec3(0.0, 1.0, 0.0)));
    }

    {
        let white_material = Material::new(na::vec4(1.0f64, 1.0, 1.0, 1.0));
        // let red_material = Material::new(na::vec4(1.0f64, 0.0, 0.0, 1.0));
        // let blue_material = Material::new(na::vec4(0.0f64, 0.0, 1.0, 1.0));
        // let green_material = Material::new(na::vec4(0.0f64, 1.0, 0.0, 1.0));

        let transform: Iso3<f64> = na::one();

        type G = Geom<f64, Vec3<f64>, Iso3<f64>>;
        let ball: G = Geom::new_ball(Ball::new(1.0f64));
        let box:  G = Geom::new_box(Box::new_with_margin(na::vec3(1.0f64, 1.0, 1.0), 0.0));
        let cone: G = Geom::new_cone(Cone::new_with_margin(1.0f64, 1.0f64, 0.0));
        let cylinder: G = Geom::new_cylinder(Cylinder::new_with_margin(1.0f64, 1.0, 0.0));
        // FIXME: new_capsule is missing from ncollide
        // let capsule: G = Geom::new_capsule(Capsule::new(1.0f64, 1.0f64));

        nodes.push(@SceneNode::new(white_material, na::translated(&transform, &na::vec3(0.0f64, 0.0, 10.0)), ball));
        nodes.push(@SceneNode::new(white_material, na::translated(&transform, &na::vec3(-5.0f64, 0.0, 15.0)), box));
        nodes.push(@SceneNode::new(white_material, na::translated(&transform, &na::vec3(5.0f64, 0.0, 15.0)), cone));
        nodes.push(@SceneNode::new(white_material, na::translated(&transform, &na::vec3(0.0f64, -5.0f64, 15.0)), cylinder));
        // nodes.push(@SceneNode::new(green_material, transform.translated(&na::vec3(0.0f64, 5.0f64, 15.0)), capsule));
    }

    // FIXME: new_perspective is _not_ accessible as a free function.
    let mut perspective = Mat4::new_perspective(
        resolution.x as f64,
        resolution.y as f64,
        45.0f64 * 3.14 / 180.0,
        1.0,
        100000.0);

    na::invert(&mut perspective);

    let scene  = Scene::new(nodes, lights);
    let pixels = scene.render(&resolution, |pt| {
        let device_x = (pt.x as f64 / resolution.x as f64 - 0.5) * 2.0;
        let device_y = (pt.y as f64 / resolution.y as f64 - 0.5) * 2.0;
        let start = na::vec4(device_x, device_y, -1.0, 1.0);
        let end   = na::vec4(device_x, device_y, 1.0, 1.0);
        let h_eye = perspective * start;
        let h_at  = perspective * end;
        let eye: Vec3<f64> = na::from_homogeneous(&h_eye);
        let at:  Vec3<f64> = na::from_homogeneous(&h_at);
        Ray::new(eye, na::normalized(&(at - eye)))
    });

    let file = io::buffered_file_writer(&PosixPath("out.ppm")).expect("Cannot open the output file.");
    pixels.to_ppm(file);
}
