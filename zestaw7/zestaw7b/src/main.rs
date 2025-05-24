#[derive(Debug)]
struct Matrix {
    wys: usize,
    szer: usize,
    arr: Vec<Vec<f64>>,
}

impl Matrix{
    fn new(wysokosc: usize, szerokosc: usize, wypelniacz: f64) -> Self{
        Matrix {
            wys: wysokosc,
            szer: szerokosc,
            arr: vec![vec![wypelniacz; szerokosc]; wysokosc]
        }
    }

    fn zerowa(wysokosc: usize, szerokosc: usize) -> Self {
        Matrix {
            wys: wysokosc,
            szer: szerokosc,
            arr: vec![vec![0.0; szerokosc]; wysokosc]
        }
    }

    fn jednostkowa(wysokosc: usize) -> Self {
        let mut data = vec![vec![0.0; wysokosc]; wysokosc];
        for i in 0..wysokosc{
            data[i][i] = 1.0;
        }

        Matrix {
            wys: wysokosc,
            szer: wysokosc,
            arr: data
        }
    }

    fn element(&self, id_wiersza: usize, id_kolumny: usize) -> f64 {
        return self.arr[id_wiersza][id_kolumny];
    }

    fn zmien_element(&mut self, id_wiersza: usize, id_kolumny: usize, wartosc: f64){
        self.arr[id_wiersza][id_kolumny] = wartosc;
    }

    fn suma(macierz1: &Matrix, macierz2: &Matrix) -> Option<Matrix> {
        if macierz1.wys != macierz2.wys || macierz1.szer != macierz2.szer {
            return None;
        } else{
            let mut result = Matrix::zerowa(macierz1.wys, macierz1.szer);

            for i in 0..macierz1.wys{
                for j in 0..macierz1.szer{
                    result.arr[i][j] = macierz1.arr[i][j] + macierz2.arr[i][j];
                }
            }

            return Some(result);
        }
    }

    fn wyswietl(&self){
        for i in 0..self.wys{
            for j in 0..self.szer{
                print!("{} ", self.arr[i][j]);
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let mut matrix1 = Matrix::new(5, 3, 6.1);
    let matrix_zero = Matrix::zerowa(3, 6);
    let matrix_jeden = Matrix::jednostkowa(4);
    matrix1.wyswietl();
    matrix_zero.wyswietl();
    matrix_jeden.wyswietl();

    println!("{}", matrix1.element(1, 2) == 6.1);
    println!("{}", matrix_zero.element(2, 1) == 0.0);
    println!("{}", matrix_jeden.element(2, 2) == 1.0);
    println!("{}", matrix_jeden.element(1, 2) == 0.0);

    matrix1.zmien_element(1, 2, 1.3);
    println!("{}", matrix1.element(2, 2) == 6.1);
    println!("{}", matrix1.element(1, 2) == 1.3);

    let matrix2 = Matrix::new(3, 4, 2.0);
    let matrix3 = Matrix::new(3, 4, 5.8);
    let matrix4 = Matrix::new(6, 2, 5.8);
    let result1 = Matrix::suma(&matrix2, &matrix3).unwrap();
    result1.wyswietl();
    println!("{:?}", Matrix::suma(&matrix2, &matrix4));
}
