use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct PixelRange {
    pub start: usize,
    pub end: usize,
}

impl PixelRange {
    pub fn to_range(self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }

    pub fn split(self) -> (PixelRange, Option<PixelRange>) {
        if self.end - self.start <= 1 { return (self, None); }
        let midpoint = self.start + (self.end - self.start) / 2;
        (PixelRange::from(self.start..midpoint), Some(PixelRange::from(midpoint..self.end)))
    }
}

impl From<Range<usize>> for PixelRange {
    fn from(range: Range<usize>) -> PixelRange {
        PixelRange {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub x: PixelRange,
    pub y: PixelRange,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            x: PixelRange::from(0..width),
            y: PixelRange::from(0..height),
        }
    }

    pub fn split(self) -> (Grid, Option<Grid>) {
        let width = self.x.end - self.x.start;
        let height = self.y.end - self.y.start;
        if width >= height {
            let (split_x, y) = (self.x.split(), self.y);
            let out1 = Grid {
                x: split_x.0,
                y: y.clone(),
            };
            let out2 = split_x.1.map(|x| Grid { x, y });
            (out1, out2)
        } else {
            let (x, split_y) = (self.x, self.y.split());
            let out1 = Grid {
                x: x.clone(),
                y: split_y.0,
            };
            let out2 = split_y.1.map(|y| Grid { x, y, });
            (out1, out2)
        }
    }
}
