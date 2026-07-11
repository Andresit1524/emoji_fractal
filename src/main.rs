mod viridis;

use image::{Rgb, RgbImage};
use num_complex::Complex;

const SIZE: u32 = 1920;
const RES: f64 = 0.00001;
const MAX_ITERS: u32 = 1500;
const CENTER: Complex<f64> = Complex::new(-0.104894547, 0.927887283);

fn get_color_rgb(iters: u32) -> Rgb<u8> {
    if iters == MAX_ITERS {
        return Rgb(viridis::at(0));
    }

    let index = (iters as usize * viridis::len()) / MAX_ITERS as usize;
    Rgb(viridis::at(index.min(viridis::len() - 1)))
}

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = c;
    for i in 0..=MAX_ITERS {
        if z.norm_sqr() > 4.0 {
            return i;
        }

        z = z * z + c;
    }

    MAX_ITERS
}

fn main() {
    // Creamos la imagen directamente en memoria (búfer de píxeles RGB)
    let mut img = RgbImage::new(SIZE, SIZE);

    for i in 0..SIZE {
        for j in 0..SIZE {
            let x = (2.0 * j as f64 / SIZE as f64) - 1.0;
            let y = -(2.0 * i as f64 / SIZE as f64) + 1.0;

            let point = Complex::new(x, y) * RES + CENTER;
            let iters = mandelbrot(point);
            let pixel_color = if iters == MAX_ITERS {
                Rgb([0, 0, 0])
            } else {
                get_color_rgb(iters)
            };

            // Modificamos el píxel directamente en el buffer
            img.put_pixel(j, i, pixel_color);
        }
    }

    // Guardamos la imagen en disco
    img.save("result.png").expect("Error al guardar la imagen");
    println!("¡Imagen guardada como result.png!");
}
