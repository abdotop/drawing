// geometrical_shapes.rs
use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.x, self.y, self.color()).unwrap();
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x0 = self.start.x as f32;
        let y0 = self.start.y as f32;
        let x1 = self.end.x as f32;
        let y1 = self.end.y as f32;

        let dx = x1 - x0;
        let dy = y1 - y0;

        let steps = dx.abs().max(dy.abs()) as i32;

        let color = self.color();

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = (x0 + dx * t) as i32;
            let y = (y0 + dy * t) as i32;
            image.set_pixel(x, y, color.clone()).unwrap();
        }
    }
    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

#[derive(Copy, Clone)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let line1 = Line::new(&self.p1, &self.p2);
        let line2 = Line::new(&self.p2, &self.p3);
        let line3 = Line::new(&self.p3, &self.p1);

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }
    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Self {
            top_left: *top_left,
            bottom_right: *bottom_right,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_right = Point {
            x: self.bottom_right.x,
            y: self.top_left.y,
        };
        let bottom_left = Point {
            x: self.top_left.x,
            y: self.bottom_right.y,
        };

        let line1 = Line::new(&self.top_left, &top_right);
        let line2 = Line::new(&top_right, &self.bottom_right);
        let line3 = Line::new(&self.bottom_right, &bottom_left);
        let line4 = Line::new(&bottom_left, &self.top_left);

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
    }
    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Self {
        Self {
            center: Point::random(width, height),
            radius: rand::thread_rng().gen_range(0..width.min(height) / 2),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let mut x = 0;
        let mut y = self.radius;
        let mut d = 3 - 2 * self.radius;
        let color = self.color();

        while y >= x {
            let points = [
                (self.center.x + x, self.center.y + y),
                (self.center.x - x, self.center.y + y),
                (self.center.x + x, self.center.y - y),
                (self.center.x - x, self.center.y - y),
                (self.center.x + y, self.center.y + x),
                (self.center.x - y, self.center.y + x),
                (self.center.x + y, self.center.y - x),
                (self.center.x - y, self.center.y - x),
            ];

            for &(x, y) in &points {
                if x >= 0 && x < image.width && y >= 0 && y < image.height {
                    image.set_pixel(x, y, color.clone()).unwrap();
                }
            }

            x += 1;

            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
        }
    }
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}
struct Pentagon {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    p5: Point,
}
impl Pentagon {
    pub fn new(p1: &Point, p2: &Point, p3: &Point, p4:&Point,p5:&Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            p3: *p3,
            p4: *p4,
            p5: *p5,
        }
    }
}
impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let line1 = Line::new(&self.p1, &self.p2);
        let line2 = Line::new(&self.p2, &self.p3);
        let line3 = Line::new(&self.p3, &self.p4);
        let line4 = Line::new(&self.p4, &self.p5);
        let line5 = Line::new(&self.p5, &self.p1);

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
        line5.draw(image);
    }
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_point() {
        let mut image = Image::blank(100, 100);
        let point = Point::new(50, 50);
        point.draw(&mut image);
        // Check that the pixel at (50, 50) is not black (assuming black is the default color)
        let pixel_color = image.get_pixel(50, 50).unwrap();
        let black = Color::rgb(0, 0, 0);
        assert_ne!(pixel_color.r, black.r);
        assert_ne!(pixel_color.g, black.g);
        assert_ne!(pixel_color.b, black.b);
    }

    #[test]
    fn test_draw_line() {
        let mut image = Image::blank(100, 100);
        let line = Line::new(&Point::new(0, 0), &Point::new(99, 99));
        line.draw(&mut image);
        // Check that the pixel at (50, 50) is not black
        let pixel_color = image.get_pixel(50, 50).unwrap();
        let black = Color::rgb(0, 0, 0);
        assert_ne!(pixel_color.r, black.r);
        assert_ne!(pixel_color.g, black.g);
        assert_ne!(pixel_color.b, black.b);
    }

    #[test]
    fn test_draw_triangle() {
        let mut image = Image::blank(100, 100);
        let triangle = Triangle::new(&Point::new(0, 0), &Point::new(99, 0), &Point::new(50, 99));
        triangle.draw(&mut image);
        // Check that the pixels at the vertices are not black
        for &point in &[Point::new(0, 0), Point::new(99, 0), Point::new(50, 99)] {
            let pixel_color = image.get_pixel(point.x, point.y).unwrap();
            let black = Color::rgb(0, 0, 0);
            assert_ne!(pixel_color.r, black.r);
            assert_ne!(pixel_color.g, black.g);
            assert_ne!(pixel_color.b, black.b);
        }
    }

    #[test]
    fn test_draw_rectangle() {
        let mut image = Image::blank(100, 100);
        let rectangle = Rectangle::new(&Point::new(0, 0), &Point::new(99, 99));
        rectangle.draw(&mut image);
        // Check that the pixels at the corners are not black
        for &point in &[
            Point::new(0, 0),
            Point::new(99, 0),
            Point::new(0, 99),
            Point::new(99, 99),
        ] {
            let pixel_color = image.get_pixel(point.x, point.y).unwrap();
            let black = Color::rgb(0, 0, 0);
            assert_ne!(pixel_color.r, black.r);
            assert_ne!(pixel_color.g, black.g);
            assert_ne!(pixel_color.b, black.b);
        }
    }
}
