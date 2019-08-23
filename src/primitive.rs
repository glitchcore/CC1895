use std::f32;

pub trait Primitive {
    fn draw(&self, t: f32, fs: f32) -> (f32, f32);
}

#[derive(Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn scale(point: (f32, f32), factor: (f32, f32)) -> (f32, f32) {
    (point.0 * factor.0, point.1 * factor.1)
}

pub fn shift(point: (f32, f32), vector: (f32, f32)) -> (f32, f32) {
    (point.0 + vector.0, point.1 + vector.1)
}

pub fn rotate(point: (f32, f32), angle: f32) -> (f32, f32) {
    (
        point.0 * angle.cos() - point.1 * angle.sin(),
        point.0 * angle.sin() + point.1 * angle.cos()
    )
}

pub fn interp(a: f32, b: f32, p: f32) -> f32 {
    b - p * (b - a)
}

#[allow(dead_code)]
pub struct Rect {
    width: f32,
    height: f32,

    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

#[allow(dead_code)]
impl Rect {
    pub const fn new(width: f32, height: f32) -> Self {
        return Rect {
            width,
            height,

            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        };
    }
}

impl Primitive for Rect {
    fn draw(&self, t: f32, _fs: f32) -> (f32, f32) {

        let p = t * 4.0;

        let x = -self.width / 2.0;
        let y = -self.height / 2.0;
        
        let (point_x, point_y) = match p {
            d if d >= 0.0 && d < 1.0 => (x + p * self.width, y),
            d if d >= 1.0 && d < 2.0 => (x + self.width, y + (p - 1.0) * self.height),
            d if d >= 2.0 && d < 3.0 => (x + (3.0 - p) * self.width, y + self.height),
            d if d >= 3.0 && d < 4.0 => (x,  (4.0 - p) * self.height + y),
            _ => (0.0, 0.0)
        };

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);


        return (point_x, point_y);
    }
}

#[derive(Default)]
pub struct Line {
    begin: Point,
    end: Point,

    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Line {
    pub const fn new(begin: Point, end: Point) -> Self {
        Line {
            begin,
            end,
            rotate: 0.0,
            shift: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

impl Primitive for Line {
    fn draw(&self, t: f32, _fs: f32) -> (f32, f32) {
        let (point_x, point_y) = (
            self.begin.x + (self.end.x - self.begin.x) * t,
            self.begin.y + (self.end.y - self.begin.y) * t,
        );

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);

        return (point_x, point_y);
    }
}

pub struct Ellipse {
    a: f32,
    b: f32,

    pub begin: f32,
    pub end: f32,

    pub rotate: f32,
    pub shift: (f32, f32),
    pub scale: (f32, f32)
}

impl Ellipse {
    pub const fn new(c: Point, a: f32, b: f32) -> Self {
        Ellipse {
            a,
            b,
            begin: 0.0,
            end: 1.0,
            rotate: 0.0,
            shift: (c.x, c.y),
            scale: (1.0, 1.0),
        }
    }
}

impl Primitive for Ellipse {
    fn draw(&self, t: f32, _fs: f32) -> (f32, f32) {
        let p = self.begin + (self.end - self.begin) * t;

        let (point_x, point_y) = (
            self.a * (p * 2.0 * f32::consts::PI).sin(),
            self.b * (p * 2.0 * f32::consts::PI).cos()
        );

        let (point_x, point_y) = scale((point_x, point_y), self.scale);
        let (point_x, point_y) = rotate((point_x, point_y), self.rotate);
        let (point_x, point_y) = shift((point_x, point_y), self.shift);

        return (point_x, point_y);
    }
}