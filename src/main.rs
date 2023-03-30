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
  fn orderType() -> String {
    return String::from("vaccines");
  }
}

struct Slot {
  slot_date: String,
  hour: String,
  order: Option<Order>
}

impl Slot {
  fn blocked(&self) -> bool {
    return self.order.is_some();
  }

  // fn hour() -> String {
  //   return String::from("09:00");
  // }
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
      print!("[{} - {}]|", slot.hour, blocked_string);
    }
    println!("");
    println!("-------------------------------------------------------------------------------------------------------------------------------------");
    
  }
}

fn build_schedule_matrix(mut bookings_count: u32, date: String) -> Vec<Booking> {
  let mut bookings: Vec<Booking> = Vec::new();

  while bookings_count > 0 {
    let mut slots: Vec<Slot> = Vec::new();
    let mut hour: u32 = 7;
    while hour < 18 {
      let time: String = match hour {
        ..= 9 => format!("0{}:00", hour),
        _ => format!("{}:00", hour.to_string())
      };
      
      let slot_date: String = format!("{} {}", date, time);
      let slot: Slot = Slot { slot_date, order: None, hour: time };
      slots.push(slot);
      hour += 1;
    }

    let booking: Booking = Booking { id: bookings_count, date: date.clone(), slots: slots };
    bookings.push(booking);

    bookings_count -= 1;
  }

  return bookings;
}
