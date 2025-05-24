// zadanie 1

struct RandGen {
    seed: i64
}

impl RandGen {
    fn new(x: i64) -> Self {
        RandGen {seed: x}
    }

    fn next_u32(&mut self) -> u64 {
        const A: u64 = 1103515245;
        const C: u64 = 12345;
        const M: u64 = u64::MAX;

        let seed_u64 = self.seed as u64;
        let new_seed = (A.wrapping_mul(seed_u64).wrapping_add(C)) % M;
        self.seed = new_seed as i64;
        new_seed
    }

    fn gen_range(&mut self, min: i16, max: i16) -> u64{
        let range = (max - min + 1) as u64;
        min as u64 + (self.next_u32() % range)
    }
}

fn zad1(){
    let mut generator1 = RandGen::new(123);
    let a = generator1.gen_range(3, 15);
    let b = generator1.gen_range(3, 15);
    let c = generator1.gen_range(3, 15);

    let mut generator2 = RandGen::new(123);
    let a2 = generator2.gen_range(3, 15);
    let b2 = generator2.gen_range(3, 15);
    let c2 = generator2.gen_range(3, 15);

    println!("{}", a == a2);
    println!("{}", b == b2);
    println!("{}", c == c2);

    println!("{}", a >= 3);
    println!("{}", b >= 3);
    println!("{}", c >= 3);

    println!("{}", a <= 15);
    println!("{}", b <= 15);
    println!("{}", c <= 15);
}

// zadanie 2

struct Urna {
    arr: Vec<char>,
    generator: RandGen
}

impl Urna {
    fn new(random: RandGen) -> Self{
        Urna {
            arr: Vec::new(),
            generator: random
        }
    }

    fn losuj_z_us(&mut self) -> Option<char>{
        if self.arr.is_empty(){
            return None;
        } else{
            let id = self.generator.gen_range(0, (self.arr.len() as u64).try_into().unwrap());
            Some(self.arr.remove(id as usize))
        }
    }

    fn losuj_bez_us(&mut self) -> Option<char> {
        if self.arr.is_empty(){
            return None;
        } else{
            let id = self.generator.gen_range(0, (self.arr.len() as u64).try_into().unwrap());
            self.arr.get(id as usize).copied()
        }
    }

    fn doloz(&mut self, element: char){
        self.arr.push(element);
    }

    fn rozmiar(&self) -> usize{
        self.arr.len()
    }

}

fn zad2(){
    let mut urna = Urna::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}

fn main() {
    // zad1();
    // zad2();
}
