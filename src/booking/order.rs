pub enum OrderType {
  VACCINES,
  EXAMS
}

#[derive(Clone)]
pub struct Product {
  pub id: u32,
  pub name: String
}

#[derive(Clone)]
pub struct OrderItem {
  pub quantity: u32,
  pub product: Product
}

#[derive(Clone)]
pub struct Order {
  pub id: u32,
  pub items: Vec<OrderItem>,
}

impl Order {
  pub fn order_type() -> OrderType {
    return OrderType::VACCINES;
  }
}