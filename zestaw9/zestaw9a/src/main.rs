// zadanie 1

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kolor {
    Trefl,      // 0, najnizszy
    Karo,
    Kier,
    Pik         // 3, najwyzszy
}

fn zad1(){
    let pik = Kolor::Pik;
    let kier = Kolor::Kier;
    let trefl = Kolor::Trefl;

    println!("{:?} > {:?} = {}", pik, kier, pik > kier);
    println!("{:?} < {:?} = {}", trefl, kier, trefl < kier);
    println!("{:?} > {:?} = {}", trefl, kier, trefl > kier);

}

// zadanie 2

enum Error {
    NoError,
    WrongFileFormat,
    FileDoesNotExist(String),
    TooBigFile {current: u32, max: u32}
}

impl Error{
    fn show_message(&self) {
        match self {
            Error::NoError => println!("No error"),
            Error::WrongFileFormat => println!("Wrong file format"),
            Error::FileDoesNotExist(name) => {
                println!("File '{}' doesn't exist", name);
            }
            Error::TooBigFile {current, max} => {
                println!("File is too big. Current size of file: {}, max size: {}", current, max);
            }
        }
    }
}

fn zad2(){
    let er1 = Error::NoError;
    let er2 = Error::FileDoesNotExist("dane.txt".to_string());
    let er3 = Error::TooBigFile {current: 5000, max: 4096};

    er1.show_message();
    er2.show_message();
    er3.show_message();
}

fn main() {
    // zad1();
    // zad2();
}
