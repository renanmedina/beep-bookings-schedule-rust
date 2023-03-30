use super::Order;
use chrono::{NaiveDateTime};

pub struct Slot {
  pub slot_date: NaiveDateTime,
  pub order: Option<Order>
}

impl Slot {
  pub fn blocked(&self) -> bool {
    return self.order.is_some();
  }

  pub fn hour(&self) -> String {
    return self.slot_date.format("%H:%M").to_string();
  }
}