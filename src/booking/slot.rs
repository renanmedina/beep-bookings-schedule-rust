use super::{Order, BlockingRule};
use chrono::{NaiveDateTime};

#[derive(Clone)]
pub struct Slot {
  pub slot_date: NaiveDateTime,
  pub order: Option<Order>,
  blocked: bool
}

impl Slot {
  pub fn new(date: String, hour: u32) -> Self {
    let slot_date: NaiveDateTime = match hour {
      ..= 9 => NaiveDateTime::parse_from_str(&format!("{} 0{}:00", date, hour), "%Y-%m-%d %H:%M"),
      _ => NaiveDateTime::parse_from_str(&format!("{} {}:00", date, hour), "%Y-%m-%d %H:%M")
    }.unwrap();

    Slot {
      slot_date,
      order: None,
      blocked: false
    }
  }

  pub fn is_blocked(&self) -> bool {
    return self.blocked;
  }

  pub fn blocked_by(&mut self, rule: BlockingRule) -> &Slot {
    self.blocked = true;
    return self;
  }

  pub fn hour(&self) -> String {
    return self.slot_date.format("%H:%M").to_string();
  }
}