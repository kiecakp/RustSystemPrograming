use std::collections::HashMap;

// zadanie 1

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }

    let mut change: HashMap<char, &str> = HashMap::new();
    change.insert('0', "000");
    change.insert('1', "001");
    change.insert('2', "010");
    change.insert('3', "011");
    change.insert('4', "100");
    change.insert('5', "101");
    change.insert('6', "110");
    change.insert('7', "111");

    let mut result = String::new();
    for c in z.chars(){
        if let Some(&binary) = change.get(&c) {
            result += binary;
        } else{
            return None;
        }
    }
    return Some(result);
}
fn zad1(){
    println!("{:?}", zamien_syst8_na_syst2("65"));
    println!("{:?}", zamien_syst8_na_syst2("6588"));
}

// zadanie 2

fn wartosc_syst2(z: &str) -> Option<u8> {
    if z.len() > 8 {
        return None;
    } else{
        let mut i: u8 = 0;
        let mut result: u8 = 0;

        for c in z.chars().rev() {
            if c == '0'{
                i += 1;
                continue;
            } else if c == '1'{
                result += 2_u8.pow(i.into());
                i += 1;
            } else{
                return None;
            }
        }
        return Some(result);
    }
}
fn zad2(){
    println!("{:?}", wartosc_syst2("1111"));
    println!("{:?}", wartosc_syst2("1001"));
    println!("{:?}", wartosc_syst2("113Z1"));
}

// zadanie 3

fn wartosc_syst8_ze_znakiem(z: &str) -> Option<u8> {
    let syst2: String = zamien_syst8_na_syst2(z)?;
    let result: u8 = wartosc_syst2(&syst2)?;
    return Some(result);
}
fn wartosc_syst8_bez_znaku(z: &str) -> Option<u8> {
    if let Some(syst2) = zamien_syst8_na_syst2(z){
        if let Some(result) = wartosc_syst2(&syst2){
            return Some(result);
        }
    }
    return None;
}
fn zad3(){
    println!("{:?}", wartosc_syst8_ze_znakiem("45"));
    println!("{:?}", wartosc_syst8_ze_znakiem("127"));
    println!("{:?}", wartosc_syst8_ze_znakiem("113Z1"));

    println!("{:?}", wartosc_syst8_bez_znaku("45"));
    println!("{:?}", wartosc_syst8_bez_znaku("127"));
    println!("{:?}", wartosc_syst8_bez_znaku("113Z1"));
}

fn main() {
    // zad1();
    // zad2();
    // zad3();
}
