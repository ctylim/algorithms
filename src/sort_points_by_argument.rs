use std::cmp::Ordering;

struct Point {
    x: i64,
    y: i64,
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if orthant(self) != orthant(other) {
            orthant(self).cmp(&orthant(other))
        } else {
            (other.x * self.y).cmp(&(self.x * other.y))
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn orthant(p: &Point) -> i32 {
    if p.y > 0 && p.x >= 0 {
        4
    } else if p.y >= 0 && p.x < 0 {
        5
    } else if p.y < 0 && p.x < 0 {
        1
    } else if p.y < 0 && p.x >= 0 {
        2
    } else {
        3
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut io = IO::new(r.lock(), w.lock());
    let n: usize = io.read();
    let mut v: Vec<Point> = Vec::with_capacity(n);
    for i in 0..n {
        let mut p = Point { x: 0, y: 0 };
        p.x = io.read();
        p.y = io.read();
        v.push(p);
    }
    v.sort();
    for i in 0..n {
        io.writeln(format!("{} {}", v[i].x, v[i].y));
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write(&mut self, s: String) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn writeln(&mut self, s: String) {
        use std::io::Write;
        self.1.write(format!("{}\n", s).as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
