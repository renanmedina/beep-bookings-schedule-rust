use super::Slot;

pub struct ScheduleBooking {
  pub id: u32,
  pub date: String,
  pub slots: Vec<Slot>,
}

impl ScheduleBooking {
  pub fn slots_count(&self) -> usize {
    return Vec::len(&self.slots);
  }

  pub fn available_slots(self) -> Vec<Slot> {
    return self.slots;
  }
}