pub struct Present {
    l: i32,
    w: i32,
    h: i32
}

impl Present {
    pub fn new((l, w, h): (i32, i32, i32)) -> Present { Present { l, w, h} }

    fn smallest_sides(&self) -> (i32, i32) {
        let mut sorted = vec![self.l, self.w, self.h];
        sorted.sort();
        (sorted[0], sorted[1])
    }

    pub fn surface_area(&self) -> i32  {
        let area = (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l);
        let sides = self.smallest_sides();
        area + (sides.0 * sides.1)
    }

    pub fn ribbon(&self) -> i32 {
        let sides = self.smallest_sides();
        (sides.0 * 2) + (sides.1 * 2) + (self.l * self.w * self.h)
    }
}