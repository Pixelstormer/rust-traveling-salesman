#[derive(Clone, Copy, Default)]
struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
    
    pub fn distance(self, other: Self) -> f32 {
        self.distance_sq(other).sqrt()
    }
    
    pub fn distance_sq(self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x) + (y * y)
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait F32NextUp {
    fn next_upper(self) -> Self;
}

impl F32NextUp for f32 {
    fn next_upper(self) -> Self {
        // We must use strictly integer arithmetic to prevent denormals from
        // flushing to zero after an arithmetic operation on some platforms.
        const TINY_BITS: u32 = 0x1; // Smallest positive f32.
        const CLEAR_SIGN_MASK: u32 = 0x7fff_ffff;

        let bits = self.to_bits();
        if self.is_nan() || bits == Self::INFINITY.to_bits() {
            return self;
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            TINY_BITS
        } else if bits == abs {
            bits + 1
        } else {
            bits - 1
        };
        Self::from_bits(next_bits)
    }
}

fn main() {
    //let count = 4;
    //run(count);
    
    let positions = vec![
        Vec2::new(-1.0, -1.0),
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(1.0, 0.5),
        Vec2::new(0.5, 0.5),
        Vec2::new(-1.0, -0.5),
        Vec2::new(-0.5, -0.5),
        Vec2::new(-0.5, -1.0),
        Vec2::new(-0.5, 0.5),
        Vec2::new(1.0, -0.5),
        Vec2::new(-1.0, 0.5),
    ];
    
    run_innermost(&positions);
}

fn run(count: usize) {
    let mut positions = vec![Vec2::default(); count];
    run_inner(count, &mut positions);
}

fn run_inner(count: usize, positions: &mut Vec<Vec2>) {
    match count {
        0 => run_innermost(&positions),
        _ => {
            let mut x = -1.0;
            while x <= 1.0 {
                let mut y = -1.0;
                while y <= 1.0 {
                    let position = Vec2::new(x, y);
                    positions[count - 1] = position;
                    run_inner(count - 1, positions);
                
                    y = y.next_upper();
                }
                x = x.next_upper();
            }
        },
    }
}

fn run_innermost(positions: &[Vec2]) {
    let mut smallest_distance = f32::INFINITY;
    let mut smallest_route = Vec::with_capacity(positions.len());
    //println!("Solving for {positions:?}");
    
    let permutations = positions.to_vec();
    permute(permutations, |permutation| {
        let mut total_distance = 0.0;
        for ab in permutation.windows(2) {
            let a = ab[0];
            let b = ab[1];
            total_distance += a.distance(b);
        }
        total_distance += permutation.first().unwrap().distance(*permutation.last().unwrap());
        if total_distance < smallest_distance {
            let diff = smallest_distance - total_distance;
            println!("Found new smallest distance (Improved by {diff}: {smallest_distance} -> {total_distance})");
            smallest_distance = total_distance;
            smallest_route = permutation.to_vec();
        }
    });

    //std::hint::black_box((smallest_distance, smallest_route));
    
    println!("Best route for {positions:?} is {smallest_route:?} with distance {smallest_distance}");
}

fn permute<T>(mut a: Vec<T>, mut inspect: impl FnMut(&[T])) {
    permute_inner(a.len(), &mut a, &mut inspect);
}

fn permute_inner<T>(k: usize, a: &mut Vec<T>, inspect: &mut impl FnMut(&[T])) {
    if k == 1 {
        inspect(&a);
    } else {
        for i in 0..k {
            permute_inner(k - 1, a, inspect);
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
        }
    }
}

