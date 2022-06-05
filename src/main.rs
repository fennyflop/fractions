extern crate num;

#[derive(Debug)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    fn simplify(&mut self) {
        let gcd: u64 = num::integer::gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }
    // return self
    fn add(&mut self, second: &mut Fraction) {
        self.simplify();
        second.simplify();

        let lcm = num::integer::lcm(self.denominator, second.denominator);
        let first_multiplier = lcm / self.denominator;
        let second_multiplier = lcm / second.denominator;
        
        self.numerator = (self.numerator*first_multiplier) + (second.numerator*second_multiplier);
        self.denominator = lcm;

        self.simplify();
    }
}

fn main() {
    let mut half = Fraction {
        numerator: 4,
        denominator: 8,
    };
    
    let mut forth = Fraction {
        numerator: 5,
        denominator: 8,
    };

    half.add(&mut forth);
    print!("{:?}", half);
}
