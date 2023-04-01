mod booking;
use booking::{TimeTable};

fn main() {
  let matrix: TimeTable = TimeTable::build_with(String::from("2023-03-10"), 10);
  println!("-------------------------------------------------------------------------------------------------------------------------------------");
  
  for booking in matrix.bookings {
    println!("Booking {}", booking.id);
    
    print!("{}", booking.slots_list.stringify());
    // for slot in booking.available_slots() {
    //   let blocked_string = match slot.blocked() {
    //     true => String::from("B"),
    //     false => String::from("F")
    //   };
    //   print!("[{} - {}]|", slot.hour(), blocked_string);
    // }
    println!("");
    println!("-------------------------------------------------------------------------------------------------------------------------------------");
    
  }
}


