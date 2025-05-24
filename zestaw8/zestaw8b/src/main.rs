// zadanie 1

#[derive(Debug, PartialEq)]
enum Unit {
    Piece,
    Liters,
    Kilograms
}

#[derive(Debug, PartialEq)]
enum Place{
    Freezer,
    Fridge,
    Normal
}

#[derive(Debug, PartialEq)]
struct Product {
    name: String,
    unit: Unit,
    weight: f32,
    place: Place
}

impl Product{
    fn new(name: String, unit: Unit, mut weight: f32, place: Place) -> Result<Self, String> {
        if weight <= 0.0 {
            return Err("Weight should be positive".to_string());
        }

        if let Unit::Kilograms = unit {
            weight = 1.0;
        }

        Ok(Product {name, unit, weight, place})
    }
}

fn zad1(){
    let prod1 = Product::new("mleko".to_string(), Unit::Liters, 1.5, Place::Fridge);
    let prod2 = Product::new("jablka".to_string(), Unit::Kilograms, 3.4, Place::Normal);
    let prod3 = Product::new("ser".to_string(), Unit::Kilograms, -3.4, Place::Fridge);
    println!("{:?}", prod1);
    println!("{:?}", prod2);
    println!("{:?}", prod3);
}

// zadanie 2

#[derive(Debug, PartialEq)]
struct OrderItem {
    product: Product,
    quantity: u32
}

#[derive(Debug, PartialEq)]
struct Order {
    products: Vec<OrderItem>
}

impl Order {
    fn new() -> Self {
        Order {products: Vec::new()}
    }

    fn sum_weight(&self) -> f32 {
        if self.products.is_empty(){
            return 0.0;
        }

        let mut result = 0.0;
        for i in 0..self.products.len(){
            result += (self.products[i].quantity as f32) * self.products[i].product.weight;
        }

        return result;
    }   

    fn sum_weight_special(&self, place: Place) -> f32 {
        if self.products.is_empty(){
            return 0.0;
        }

        let mut result = 0.0;
        for i in 0..self.products.len(){
            if self.products[i].product.place == place {
                result += (self.products[i].quantity as f32) * self.products[i].product.weight;
            }
        }

        return result;
    }

    fn add_product(&mut self, quantity: u32, name: String, unit: Unit, weight: f32, place: Place) -> Result<(), String> {
        if weight <= 0.0 {
            return Err("Weight should be positive".to_string());
        }

        if let Some(item) = self.products.iter_mut().find(|item| item.product.name == name){
            item.quantity += quantity;
        } else{
            let prod = Product::new(name, unit, weight, place)?;
            self.products.push(OrderItem {product: prod, quantity: quantity});
        }

        Ok(())
    }

}

fn zad2(){
    let mut order = Order::new();
    println!("{}", order.sum_weight());

    order.add_product(1, "mleko".to_string(), Unit::Liters, 1.5, Place::Fridge);
    order.add_product(1, "jablka".to_string(), Unit::Kilograms, 3.4, Place::Normal);
    println!("{}", order.sum_weight());

    order.add_product(4, "mleko".to_string(), Unit::Liters, 1.5, Place::Fridge);
    println!("{}", order.sum_weight());
    println!("{}", order.sum_weight_special(Place::Fridge));
    println!("{}", order.sum_weight_special(Place::Normal));

    order.add_product(1, "ser".to_string(), Unit::Kilograms, -3.4, Place::Fridge);
    println!("{}", order.sum_weight());
}

fn main() {
    // zad1();
    // zad2();
}
