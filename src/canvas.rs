use crate::tuples::{self, Color, Tuple};
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            data: vec![Color::white(); width * height],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        println!("pixel {}:{}", x, y);
        if ((x) + (y) * self.width < self.width * self.height)
            && (y > 0)
            && (x > 0)
            && (x < self.width)
            && (y < self.height)
        {
            self.data[(x) + (y) * self.width] = color;
        }
    }
    pub fn ppm_header(&self) -> String {
        let mut header = "P3".to_string();
        header.push_str("\n");
        header.push_str(&self.width.to_string());
        header.push_str(" ");
        header.push_str(&self.height.to_string());
        header.push_str("\n");
        header.push_str("255 ");
        header.push_str("\n");
        header
    }
    pub fn to_ppm(&self) -> String {
        let mut ppm = self.ppm_header();
        let mut buffer = "".to_string();
        //ppm.push_str(&Vec::<tuples::Tuple>::join(" "));
        for (pos, pixel) in self.data.iter().enumerate() {
            //println!("{}", pos);
            let pix = &pixel.clamp(0., 255.);
            buffer.push_str(&pix.red.to_string());
            buffer.push_str(" ");
            buffer.push_str(&pix.blue.to_string());
            buffer.push_str(" ");
            buffer.push_str(&pix.green.to_string());
            buffer.push_str(" ");
            if buffer.chars().count() > 60 {
                ppm.push_str(&buffer);
                //println!("\n============= {}", buffer.chars().count());
                //println!("ppm : {}", ppm);
                //println!("buffer : {}", buffer);
                ppm.push_str(&"\n\r");
                buffer = "".to_string();
            }
            //println!("{}", buffer);
        }
        ppm.push_str(&buffer);
        ppm.push_str("\r\n");
        ppm
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::tuples::*;

    use super::Canvas;
    #[test]
    fn can_instantiate_canvas() {
        let canvas = Canvas::new(10, 10);
        println!("{:#?}", canvas);
    }
    #[test]
    fn can_gen_header() {
        let canvas = Canvas::new(100, 100);
        let header = canvas.ppm_header();
        println!("{}", header);
    }
    #[test]
    fn canvas_to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color {
            red: 1.5,
            green: 0.,
            blue: 0.,
        };
        let c2 = Color {
            red: 0.,
            green: 0.5,
            blue: 0.,
        };
        let c3 = Color {
            red: -0.5,
            green: 0.,
            blue: 1.,
        };
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        println!("{:#?}", canvas);
        let ppm = canvas.to_ppm();
        println!("{}", &ppm);
    }

    #[test]
    fn ticking() {
        let mut projectile = Projectile {
            position: Tuple::point(0., 1., 0.),
            velocity: Tuple::vector(1., 1.8, 0.).normalize() * 11.25,
        };
        let env = Environment {
            gravity: Tuple::vector(0., -0.2, 0.),
            wind: Tuple::vector(-0.1, 0., 0.),
        };
        let mut canvas = Canvas::new(900, 550);
        let color = Color::green();

        let mut i = 0;
        while i < 450 {
            projectile = tick(env, projectile);
            i += 1;
            //println!("{:#?}", projectile);
            let x = projectile.position.x.floor() as usize;
            let y = projectile.position.y.floor() as usize;
            println!(
                "{}*{}={}   {} ",
                x,
                y,
                x * y,
                if canvas.height > y {
                    canvas.height - y
                } else {
                    0
                },
            );
            canvas.write_pixel(
                x,
                /*
                if canvas.height > y {
                    canvas.height - y
                } else {
                    0
                }*/
                canvas.height - y,
                color,
            );
        }
        fs::write("foo.ppm", canvas.to_ppm()).unwrap();
    }
}
