use chrono::{NaiveDateTime};
use chrono::format::ParseError;

fn main() {
  let matrix: Vec<Booking> = build_schedule_matrix(10, String::from("2023-03-10"));
  println!("-------------------------------------------------------------------------------------------------------------------------------------");
  for booking in matrix {
    // println!("Booking {} -> {}: Slots count: {}", booking.id, booking.date, booking.slots_count());
    for slot in booking.available_slots() {
      let blocked_string = match slot.blocked() {
        true => String::from("B"),
        false => String::from("F")
      };
      print!("[{} - {}]|", slot.hour(), blocked_string);
    }
    println!("");
    println!("-------------------------------------------------------------------------------------------------------------------------------------");
    
  }
}

enum OrderType {
  VACCINES,
  EXAMS
}

struct Product {
  id: u32,
  name: String
}

struct OrderItem {
  quantity: u32,
  product: Product
}

struct Order {
  id: u32,
  items: Vec<OrderItem>,
}

impl Order {
  fn order_type() -> OrderType {
    return OrderType::VACCINES;
  }
}

struct Slot {
  slot_date: NaiveDateTime,
  order: Option<Order>
}

impl Slot {
  fn blocked(&self) -> bool {
    return self.order.is_some();
  }

  fn hour(&self) -> String {
    return self.slot_date.format("%H:%M").to_string();
  }
}

struct Booking {
  id: u32,
  date: String,
  slots: Vec<Slot>,
}

impl Booking {
  fn slots_count(&self) -> usize {
    return Vec::len(&self.slots);
  }

  fn available_slots(self) -> Vec<Slot> {
    return self.slots;
  }
}

fn build_schedule_matrix(mut bookings_count: u32, date: String) -> Vec<Booking> {
  let mut bookings: Vec<Booking> = Vec::new();

  while bookings_count > 0 {
    let mut slots: Vec<Slot> = Vec::new();
    let mut hour: u32 = 7;
    while hour < 18 {
      let slot_date: NaiveDateTime = match hour {
        ..= 9 => NaiveDateTime::parse_from_str(&format!("{} 0{}:00", date, hour), "%Y-%m-%d %H:%M"),
        _ => NaiveDateTime::parse_from_str(&format!("{} {}:00", date, hour), "%Y-%m-%d %H:%M")
      }.unwrap();
      
      let slot: Slot = Slot { slot_date, order: None };
      slots.push(slot);
      hour += 1;
    }

    let booking: Booking = Booking { id: bookings_count, date: date.clone(), slots: slots };
    bookings.push(booking);

    bookings_count -= 1;
  }

  return bookings;
}
