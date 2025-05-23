// zadanie 1

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err("Niepoprawna cyfra".to_string()),
    }
}
fn zad1(){
    println!("{:?}", wartosc_cyfry('3'));
    println!("{:?}", wartosc_cyfry('9'));
    println!("{:?}", wartosc_cyfry('a'));
}

// zadanie 2

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    if a.is_empty() || b.is_empty(){
        return Err("Podano puste dane".to_string());
    }

    let mut result = String::new();
    let mut carry = 0;

    let mut iter_a = a.chars().rev();
    let mut iter_b = b.chars().rev();

    loop {
        let next_a = iter_a.next();
        let next_b = iter_b.next();

        if next_a.is_none() && next_b.is_none() && carry == 0 {
            break;
        }

        let digit_a = match next_a {
            Some(c) => wartosc_cyfry(c)?,
            None => 0,
        };

        let digit_b = match next_b {
            Some(c) => wartosc_cyfry(c)?,
            None => 0,
        };

        let sum = digit_a + digit_b + carry;
        result += &(sum % 10).to_string();
        carry = sum / 10;
    }

    Ok(result.chars().rev().collect())
}
fn zad2(){
    println!("{:?}", dodaj_pisemnie("123", "456"));
    println!("{:?}", dodaj_pisemnie("999", "1"));
    println!("{:?}", dodaj_pisemnie("12a", "34"));
    println!("{:?}", dodaj_pisemnie("12a", ""));
}

// zadanie 3

fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    match c {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err("Niepoprawny znak".to_string()),
    }
}
fn zad3(){
    println!("{:?}", wartosc_cyfry_rzymskiej('I'));
    println!("{:?}", wartosc_cyfry_rzymskiej('L'));
    println!("{:?}", wartosc_cyfry_rzymskiej('o'));
}

// zadanie 4

fn rzymskie(napis: &str) -> Result<u128, String> {
    if napis.is_empty(){
        return Err("Podano puste dane".to_string());
    }

    let mut result = 0;
    let mut prev = 0;

    for c in napis.chars(){
        let current = wartosc_cyfry_rzymskiej(c)?;

        if current > prev {
            result += current - 2 * prev;
        }else{
            result += current;
        }

        prev = current;
    }

    return Ok(result.into());
}
fn zad4(){
    println!("{:?}", rzymskie("XV"));
    println!("{:?}", rzymskie("XIV"));
    println!("{:?}", rzymskie("MCMXC"));
    println!("{:?}", rzymskie(""));
    println!("{:?}", rzymskie("MCZZXC"));
}

fn main() {
    // zad1();
    // zad2();
    // zad3();
    // zad4();
}
