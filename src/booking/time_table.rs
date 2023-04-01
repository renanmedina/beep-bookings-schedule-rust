use super::{ScheduleBooking, Slot};

pub struct TimeTable {
  pub bookings: Vec<ScheduleBooking>
}

impl TimeTable {
  pub fn new(bookings: Vec<ScheduleBooking>) -> TimeTable {
    TimeTable {
      bookings: bookings
    }
  }

  pub fn build_with(date: String, mut bookings_count: u32) -> TimeTable {
    let mut bookings: Vec<ScheduleBooking> = Vec::new();
  
    while bookings_count > 0 {
      let mut hour: u32 = 7;
      let mut booking = ScheduleBooking::new(bookings_count, date.clone());
      while hour < 18 {
        let slot: Slot = Slot::new(date.clone(), hour);
        booking = booking.add_slot(slot);
        hour += 1;
      }
  
      bookings.push(booking);
      bookings_count -= 1;
    }
  
    return TimeTable::new(bookings);
  }
}