use num_complex::Complex;
use std::fs::File;
use std::io::{BufWriter, Write};

const SIZE: usize = 340;
const RES: f64 = 0.00001;
const MAX_ITERS: usize = 1500;
const CENTER: Complex<f64> = Complex::new(-0.104894547, 0.927887283);

/// Convierte un número en un emoji de color siguiendo una paleta ordenada
fn get_color_emoji(iters: usize) -> &'static str {
    let colors = ["⬛", "🟦", "🟩", "🟨", "🟧", "🟥", "🟫"];

    if iters == MAX_ITERS {
        return "⬛";
    }

    let index = (iters * colors.len()) / MAX_ITERS;
    colors[index.min(colors.len() - 1)]
}

/// Obtiene la velocidad de escape del fractal de Mandelbrot
fn mandelbrot(c: Complex<f64>) -> usize {
    let mut z = c;
    for i in 0..=MAX_ITERS {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    MAX_ITERS
}

fn main() -> std::io::Result<()> {
    let file = File::create("result.txt")?;
    // Usamos BufWriter para acumular la escritura en memoria y no saturar el disco
    let mut writer = BufWriter::new(file);

    for i in 0..SIZE {
        for j in 0..SIZE {
            let x = (2.0 * j as f64 / SIZE as f64) - 1.0;
            let y = -(2.0 * i as f64 / SIZE as f64) + 1.0;

            let point = Complex::new(x, y) * RES + CENTER;
            let iters = mandelbrot(point);
            let emoji = get_color_emoji(iters);

            writer.write_all(emoji.as_bytes())?;

            if j == SIZE - 1 {
                writer.write_all(b"\n")?;
            }
        }
    }

    Ok(())
}
