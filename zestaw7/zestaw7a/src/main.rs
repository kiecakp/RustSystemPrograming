#[derive(PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Rgb{
        Rgb {r, g, b}
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Rgb> {
        if (r >= 0.0 && r <= 100.0) && (g >= 0.0 && g <= 100.0) && 
            (b >= 0.0 && b <= 100.0) {
            Some(Rgb {
                r: (r * 2.55) as u8,
                g: (g * 2.55) as u8,
                b: (b * 2.55) as u8
            })
        } else{
            None
        }
    }

    fn gray(p: f32) -> Option<Rgb> {
        if p >= 0.0 && p <= 100.0 {
            Some(Rgb{
                r: (p * 2.55) as u8,
                g: (p * 2.55) as u8,
                b: (p * 2.55) as u8
            })
        } else{
            None
        }
    }

    fn white() -> Rgb{
        Rgb::from_3u8(255, 255, 255)
    }

    fn black() -> Rgb{
        Rgb::from_3u8(0, 0, 0)
    }

    fn invert(&mut self){
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    fn intensity(&self) -> f32{
        let sum = (self.r as f32) + (self.g as f32) + (self.b as f32);
        return sum / (3.0 * 255.0);
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8){
        (self.r, self.g, self.b)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8){
        (255 - self.r, 255 - self.g, 255 - self.b)
    }
}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}
