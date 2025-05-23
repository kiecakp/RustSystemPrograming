// zadanie 1

fn small_letters() -> Vec<char> {
    // let mut result = Vec::new();
    // for c in 'a'..='z' {
    //     result.push(c);
    // }
    // return result;

    let result: Vec<char> = ('a'..='z').collect();
    return result;
}

fn squares_10_numbers() -> Vec<u16> {
    // let mut result = Vec::new();
    // for i in 1..=10 {
    //     result.push((i as u16).pow(2));
    // }
    // return result;

    let result: Vec<u16> = (1..=10).map(|i| (i as u16).pow(2)).collect();
    return result;
}

fn powers_of_2() -> Vec<u16> {
    // let mut result = Vec::new();
    // for i in 1..=10 {
    //     result.push((2 as u16).pow(i));
    // }
    // return result;

    let result: Vec<u16> = (1..=10).map(|i| (2 as u16).pow(i)).collect();
    return result;
}

fn reciprocals_of_number() -> Vec<f32> {
    // let mut result = Vec::new();
    // for i in 1..=20{
    //     result.push(1.0 / (i as f32));
    // }
    // return result;

    let result: Vec<f32> = (1..=20).map(|i| 1.0 / (i as f32)).collect();
    return result;
}

fn div_by_3_not_by_4() -> Vec<u16> {
    // let mut result = Vec::new();
    // for i in 1..=100 {
    //     if i % 3 == 0 && i % 4 != 0 {
    //         result.push(i);
    //     }
    // }
    // return result;

    let result: Vec<u16> = (1..=100).filter(|i| i % 3 == 0 && i % 4 != 0).collect();
    return result;
}
fn zad1(){
    println!("{:?}", small_letters());
    println!("{:?}", squares_10_numbers());
    println!("{:?}", powers_of_2());
    println!("{:?}", reciprocals_of_number());
    println!("{:?}", div_by_3_not_by_4());
}

// zadanie 2

fn small_than_4_letters(arr: Vec<&str>) -> Vec<&str> {
    // let mut result = Vec::new();
    // for c in arr {
    //     if c.len() < 4 {
    //         result.push(c);
    //     }
    // }
    // return result;

    return arr.into_iter().filter(|c| c.len() < 4).collect();
}

fn without_a_A(arr: Vec<&str>) -> Vec<&str> {
    // let mut result = Vec::new();
    // for c in arr{
    //     if !c.contains('a') && !c.contains('A'){
    //         result.push(c);
    //     }
    // }
    // return result;

    return arr.into_iter().filter(|c| !c.contains('a') && !c.contains('A')).collect();
}

fn with_numbers(arr: Vec<&str>) -> Vec<&str> {
    // let mut result = Vec::new();
    // for c in arr{
    //     if c.chars().any(|i| i.is_ascii_digit()){
    //         result.push(c);
    //     }
    // }
    // return result;

    return arr.into_iter().filter(|s| s.chars().any(|i| i.is_ascii_digit())).collect();
}

fn reversed(arr: Vec<&str>) -> Vec<String> {
    // let mut result = Vec::new();
    // for c in arr{
    //     result.push(c.chars().rev().collect());
    // }
    // return result;

    return arr.into_iter().map(|c| c.chars().rev().collect()).collect();
}

fn doubbled_letter(arr: Vec<&str>) -> Vec<&str>{
    // let mut result = Vec::new();
    // for c in arr{
    //     let mut chars = c.chars();
    //     let mut prev = chars.next();

    //     for curr in chars {
    //         if prev == Some(curr){
    //             result.push(c);
    //             break;
    //         }
    //         prev = Some(curr);
    //     }
    // }
    // return result;

    return arr.into_iter().filter(|c| {c.chars().zip(c.chars().skip(1)).any(|(a, b)| a == b)}).collect();
}
fn zad2(){
    let arr: Vec<&str> = ["kot", "pizza", "mucha", "ala", "brutto", "Ale", "ble444", "mle32ko"].to_vec();

    println!("{:?}", small_than_4_letters(arr.clone()));
    println!("{:?}", without_a_A(arr.clone()));
    println!("{:?}", with_numbers(arr.clone()));
    println!("{:?}", reversed(arr.clone()));
    println!("{:?}", doubbled_letter(arr.clone()));
}

// zadanie 3

fn indeksy(arr: Vec<&str>, element: &str) -> Vec<u16> {
    // let mut result = Vec::new();
    // let mut i: u16 = 0;
    // for c in arr{
    //     if c == element{
    //         result.push(i);
    //     }
    //     i += 1;
    // }
    // return result;

    return arr.into_iter().enumerate().filter_map(|(i, c)| if c == element {Some(i as u16)} else {None}).collect();
}
fn zad3(){
    let arr:Vec<&str> = ["kot", "ala", "ela", "kot", "mleko", "kot"].to_vec();

    println!("{:?}", indeksy(arr, "kot"));
}

fn main() {
    // zad1();
    // zad2();
    // zad3();
}
