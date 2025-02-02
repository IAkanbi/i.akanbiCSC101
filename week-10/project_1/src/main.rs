struct HP {
    name:String, quantity:f32, price:f32
}

impl HP{
    fn total_price(&self) -> f32{
        self.quantity * self.price
    }
}

struct IBM {
    name:String, quantity:f32, price:f32
}

impl IBM{
    fn total_price(&self) -> f32{
        self.quantity * self.price
    }
}

struct Toshiba {
    name:String, quantity:f32, price:f32
}

impl Toshiba{
    fn total_price(&self) -> f32{
        self.quantity * self.price
    }
}

struct Dell {
    name:String, quantity:f32, price:f32
}

impl Dell{
    fn total_price(&self) -> f32{
        self.quantity * self.price
    }
}


fn main() {
    let batch1 = HP{
        name:String::from("HP Laptop"), quantity:3.0, price:650_000.00
    };
    let batch2 = IBM{
        name:String::from("IBM Laptop"), quantity:3.0, price:755_000.00
    };
    let batch3 = Toshiba {
        name:String::from("Toshiba Laptop"), quantity:3.0, price:550_000.00
    };
    let batch4 = Dell {
        name:String::from("Dell Laptop"), quantity:3.0, price:850_000.00
    };

    let customer_total = batch1.total_price() + batch2.total_price() + batch3.total_price() + batch4.total_price();
    println!(" 3 {} ----- {} \n 3 {} ----- {} \n 3 {} ----- {} \n 3 {} ----- {} \n Your total is {}", batch1.name, batch1.total_price(), batch2.name,
        batch2.total_price(), batch3.name, batch3.total_price(), batch4.name, batch4.total_price(), customer_total);

}
