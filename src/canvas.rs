use crate::tuples::{self, Tuple, TupleKind};
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            data: vec![
                Tuple {
                    x: 0., // red
                    y: 0., // green
                    z: 0., // blue
                    w: TupleKind::Color,
                };
                width * height
            ],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        if ((x) + (y) * self.width) < self.width * self.height {
            self.data[(x) + (y - 1) * self.width] = color;
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
            let pix = &pixel.clamp(255., 0., 255.);
            buffer.push_str(&pix.x.to_string());
            buffer.push_str(" ");
            buffer.push_str(&pix.y.to_string());
            buffer.push_str(" ");
            buffer.push_str(&pix.z.to_string());
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
        let c1 = Tuple {
            x: 1.5,
            y: 0.,
            z: 0.,
            w: TupleKind::Color,
        };
        let c2 = Tuple {
            x: 0.,
            y: 0.5,
            z: 0.,
            w: TupleKind::Color,
        };
        let c3 = Tuple {
            x: -0.5,
            y: 0.,
            z: 1.,
            w: TupleKind::Color,
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
            position: Tuple {
                x: 0.,
                y: 1.,
                z: 0.,
                w: TupleKind::Point,
            },
            velocity: Tuple {
                x: 1.,
                y: 1.8,
                z: 0.,
                w: TupleKind::Vector,
            }
            .normalize()
                * 11.25,
        };
        let env = Environment {
            gravity: Tuple {
                x: 0.,
                y: -0.1,
                z: 0.,
                w: TupleKind::Vector,
            },
            wind: Tuple {
                x: -0.01,
                y: 0.,
                z: 0.,
                w: TupleKind::Vector,
            },
        };
        let mut canvas = Canvas::new(900, 550);
        let color = Tuple {
            x: 1.,
            y: 1.,
            z: 1.,
            w: TupleKind::Color,
        };
        let mut i = 0;
        while i < 450 {
            projectile = tick(env, projectile);
            i += 1;
            //println!("{:#?}", projectile);
            let x = projectile.position.x.floor() as usize;
            let y = projectile.position.y.floor() as usize;
            println!(
                "{}*{}={}   {} {}",
                x,
                y,
                x * y,
                canvas.height - y,
                x * (canvas.height - y)
            );
            canvas.write_pixel(x, canvas.height - y, color);
        }
        fs::write("foo.ppm", canvas.to_ppm()).unwrap();
    }
}
