// %=BEGIN DOC
// %title
// 二次元グリッド
//
// %overview
//
// %usage
//
// %require
// ```
// ```
// %verified
// codingame-fall-challenge
// %references
//
// %=END DOC
// %=BEGIN CODE

type PUnit = i32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Pt {
    y: PUnit,
    x: PUnit,
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // println!("{}", 12);
        // println!("{:?}", 12); // debug
        write!(f, "({},{})", self.x, self.y)
    }
}

impl ops::Neg for Pt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}

impl ops::Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl ops::AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl ops::Sub for Pt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl Into<(i32, i32)> for Pt {
    fn into(self) -> (i32, i32) {
        (self.y, self.x)
    }
}

impl From<(usize, usize)> for Pt {
    fn from(p: (usize, usize)) -> Self {
        Self {
            y: p.0 as i32,
            x: p.1 as i32,
        }
    }
}

impl From<(i32, i32)> for Pt {
    fn from(p: (i32, i32)) -> Self {
        Self { y: p.0, x: p.1 }
    }
}

impl Pt {
    fn zero() -> Self {
        Pt { y: 0, x: 0 }
    }

    fn is_zero(&self) -> bool {
        self.y == 0 && self.x == 0
    }

    fn mul(&self, rhs: PUnit) -> Self {
        Self {
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }

    fn contain_rect(&self, left_up: Pt, right_down: Pt) -> bool {
        left_up.x <= self.x
            && self.x <= right_down.x
            && left_up.y <= self.y
            && self.y <= right_down.y
    }
}

const K_ADJACENTS_4 : [Pt; 4] = [Pt { y: 0, x: -1 }, Pt { y: -1, x: 0 }, Pt { y: 0, x: 1 }, Pt { y: 1, x: 0 }];

#[derive(Clone)]
struct Fi<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Clone> Fi<T> {
    fn new(height: usize, width: usize, default: T) -> Self {
        Self {
            height,
            width,
            data: vec![default; height * width],
        }
    }
    fn fill(&mut self, default: T) {
        self.data.fill(default);
    }
}

impl<T> Fi<T> {
    // 関数名わからん
    fn new_generate<F: FnMut(usize, usize) -> T>(height: usize, width: usize, mut con: F) -> Self {
        let mut data = Vec::<T>::new();
        data.reserve(height * width);
        for y in 0..height {
            for x in 0..width {
                data.push(con(y, x));
            }
        }
        Self {
            height,
            width,
            data,
        }
    }
    fn get<I: Into<(usize, usize)>>(&self, yx: I) -> Option<&T> {
        let yx = yx.into();
        if 0 <= yx.0 && yx.0 < self.height && 0 <= yx.1 && yx.1 < self.width {
            Some(&self[yx])
        } else {
            None
        }
    }
    fn get_mut<I: Into<(usize, usize)>>(&mut self, yx: I) -> Option<&mut T> {
        let yx = yx.into();
        if 0 <= yx.0 && yx.0 < self.height && 0 <= yx.1 && yx.1 < self.width {
            Some(&mut self[yx])
        } else {
            None
        }
    }
    fn geti<I: Into<(i32, i32)>>(&self, yx: I) -> Option<&T> {
        let yx = yx.into();
        if 0 <= yx.0 && yx.0 < self.height as i32 && 0 <= yx.1 && yx.1 < self.width as i32 {
            Some(&self[(yx.0 as usize, yx.1 as usize)])
        } else {
            None
        }
    }
    fn geti_mut<I: Into<(i32, i32)>>(&mut self, yx: I) -> Option<&mut T> {
        let yx = yx.into();
        if 0 <= yx.0 && yx.0 < self.height as i32 && 0 <= yx.1 && yx.1 < self.width as i32 {
            Some(&mut self[(yx.0 as usize, yx.1 as usize)])
        } else {
            None
        }
    }

    fn write_fmt<FN: FnMut(&mut fmt::Formatter, &T) -> fmt::Result>(
        &self,
        f: &mut fmt::Formatter,
        mut cb: FN,
    ) -> fmt::Result {
        if let Err(e) = write!(f, "[{}x{}]", self.width, self.height) {
            return Err(e);
        }
        for y in 0..self.height {
            if let Err(e) = writeln!(f) {
                return Err(e);
            }
            for x in 0..self.width {
                if let Err(e) = cb(f, &self[(y, x)]) {
                    return Err(e);
                }
            }
        }
        writeln!(f)
    }
    fn eprintln<FN: FnMut(&T)>(&self, mut cb: FN) {
        // FIXME: うまい書き方
        eprintln!("[{}x{}]", self.width, self.height);
        for y in 0..self.height {
            eprintln!();
            for x in 0..self.width {
                cb(&self[(y, x)]);
            }
        }
        eprintln!()
    }
}

impl<I: Into<(usize, usize)>, T> ops::Index<I> for Fi<T> {
    type Output = T;
    fn index(&self, yx: I) -> &Self::Output {
        let yx: (usize, usize) = yx.into();
        &self.data[self.width * yx.0 + yx.1]
    }
}

impl<I: Into<(usize, usize)>, T> ops::IndexMut<I> for Fi<T> {
    fn index_mut(&mut self, yx: I) -> &mut Self::Output {
        let yx: (usize, usize) = yx.into();
        &mut self.data[self.width * yx.0 + yx.1]
    }
}

impl<T: fmt::Display> fmt::Debug for Fi<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_fmt(f, |f, t| write!(f, "{} ", t))
    }
}

fn gen_2d_range(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |a| (0..width).map(move |b| (a, b)))
}
fn gen_2d_rangei(height: usize, width: usize) -> impl Iterator<Item = (i32, i32)> {
    (0..(height as i32)).flat_map(move |a| (0..(width as i32)).map(move |b| (a, b)))
}

fn gen_adjacentsi(center_yx: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    K_ADJACENTS_4
        .iter()
        .map(move |v| (center_yx.0 + v.0, center_yx.1 + v.1))
}

fn gen_adjacentsu(center_yx: (usize, usize)) -> impl Iterator<Item = (i32, i32)> {
    K_ADJACENTS_4
        .iter()
        .map(move |v| (center_yx.0 as i32 + v.0, center_yx.1 as i32 + v.1))
}

// %=END CODE
