mod booking;
use booking::{ScheduleBooking, Slot};
use chrono::{NaiveDateTime};

fn main() {
  let matrix: Vec<ScheduleBooking> = build_schedule_matrix(10, String::from("2023-03-10"));
  println!("-------------------------------------------------------------------------------------------------------------------------------------");
  for booking in matrix {
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

fn build_schedule_matrix(mut bookings_count: u32, date: String) -> Vec<ScheduleBooking> {
  let mut bookings: Vec<ScheduleBooking> = Vec::new();

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

    let booking: ScheduleBooking = ScheduleBooking { id: bookings_count, date: date.clone(), slots: slots };
    bookings.push(booking);

    bookings_count -= 1;
  }

  return bookings;
}
