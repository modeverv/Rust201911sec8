trait Coodinates {
    fn to_cartesian(self) -> CartesianCood;
    fn from_cartesian(cart: CartesianCood) -> Self;
}
struct CartesianCood {
    x: f64,
    y: f64,
}
impl Coodinates for CartesianCood {
    fn to_cartesian(self) -> CartesianCood {
        self
    }
    fn from_cartesian(cart: CartesianCood) -> Self {
        cart
    }
}
struct PolarCoord {
    r: f64,
    theta: f64,
}
impl Coodinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCood {
        CartesianCood {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartesianCood) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coodinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCood {
        CartesianCood {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCood) -> Self {
        (cart.x, cart.y)
    }
}

struct Matrix([[f64; 2]; 2]);

trait LinearTransform: Coodinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized,
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;
        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][1] * y;
        Self::from_cartesian(cart)
    }
    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }
}

impl LinearTransform for CartesianCood {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;
        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }
}

impl LinearTransform for PolarCoord {
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

fn print_point<P: Coodinates>(point: P) {
    let p = point.to_cartesian();
    println!("x = {}, y = {}", p.x, p.y);
}

fn main() {
    println!("Hello, world!");
    let point = (1.0, 1.0);
    let c = point.to_cartesian();
    println!("x = {} y = {}", c.x, c.y);
    let p = PolarCoord::from_cartesian(c);
    println!("r = {} θ = {}", p.r, p.theta);
    print_point((3.2, 3.3));
    {
        let p = (1.0, 0.0).to_cartesian();
        print_point(p.rotate(std::f64::consts::PI));
    }
}
