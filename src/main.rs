pub mod point;
pub mod vector;
pub mod scene;
mod render;
extern crate image;

use scene::{Scene, Colour, Sphere, Element, Plane, Intersection, DirectionalLight,
    SphericalLight, Light};
use point::Point;
use vector::Vector3;
use render::{Ray, Intersectable};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba, Pixel};




pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 255);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime_ray(x, y, scene);

            let intersection = scene.trace(&ray);
            let color = intersection.map(|i| to_rgba(&get_colour(scene, &ray, &i)))
                .unwrap_or(black);
            img.put_pixel(x, y, color);
        }
    }
    img
}

fn to_rgba(colour: &Colour) -> Rgba<u8> {
    Rgba::from_channels((colour.red * 255.0) as u8, (colour.green * 255.0) as u8, (colour.blue * 255.0) as u8, 0)
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 5.0,
            colour: Colour {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };


    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn get_colour(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Colour {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    
    let surface_normal = intersection.elements.surface_normal(&hit_point);

    let mut colour = Colour {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    for light in &scene.light {
        let direction_to_light = light.direction_from(&hit_point);
        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() ||
                       shadow_intersection.unwrap().distance > light.distance(&hit_point);
        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };

        let light_power = (surface_normal.dot_prod(&direction_to_light) as f32).max(0.0) *
                          light_intensity;
        
        let light_reflected = intersection.elements.albedo() / std::f32::consts::PI;

        let light_colour = light.colour() * light_power * light_reflected;
        colour = colour + (intersection.elements.colour() * &light_colour);

    }
    colour.clamp()
}

fn main() {
    let mut elements = Vec::new();
    let mut lights = Vec::new();

    let sp = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        radius: 1.0,
        colour: Colour {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        },
        albedo: 0.18,
    };

    let sp1 = Sphere {
        center: Point {
            x: -3.0,
            y: 1.0,
            z: -6.0,
        },
        radius: 2.0,
        colour: Colour {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        },
        albedo: 0.18,
    };

    let sp2 = Sphere {
        center: Point {
            x: 2.0,
            y: 2.0,
            z: -4.0,
        },
        radius: 2.25,
        colour: Colour {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        },
        albedo: 0.18,
    };

    let pl = Plane {
        origin: Point {
            x: 0.0,
            y: -2.0,
            z: 0.0,
        },
        normal: Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        colour: Colour {
            red: 0.2,
            green: 0.2,
            blue: 0.2,
        },
        albedo: 0.18,
    };

    let pl2 = Plane {
        origin: Point {
            x: 0.0,
            y: 0.0,
            z: -20.0,
        },
        normal: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        colour: Colour {
            red: 0.6,
            green: 0.8,
            blue: 1.0,
        },
        albedo: 0.18,
    };

    let li = DirectionalLight {
        direction: Vector3 {
            x: 0.25,
            y: 0.0,
            z: -2.0,
        },
        
        colour: Colour {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        },
        intensity: 20.0,
    };

    let li2 = SphericalLight {
        position: Point {
            x: -2.0,
            y: 10.0,
            z: -3.0,
        },
        
        colour: Colour {
            red: 3.0,
            green: 0.8,
            blue: 0.3,
        },
        intensity: 40000.0,
    };

    elements.push(Element::Sphere(sp));
    elements.push(Element::Sphere(sp1));
    elements.push(Element::Sphere(sp2));
    elements.push(Element::Plane(pl));
    elements.push(Element::Plane(pl2));

    lights.push(Light::Directional(li));
    lights.push(Light::Spherical(li2));

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: elements,
        light: lights,
        shadow_bias: 0.0001,
    };

    let img: DynamicImage = render(&scene);
    img.save("test.png").unwrap();

}
