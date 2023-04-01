use super::{Slot, ScheduleBooking};

#[derive(Clone)]
pub enum BlockingRule {
  ProgressiveSlotLockEnabledByProduct,
  AppointmentAtSameDay,
  AppointmentMaxEstimatedTime,
  AppointmentVaccinesCountAndPreviousSlot,
  AppointmentVaccinesCountAndNextSlot,
  AppointmentChildrenCountPreviousSlot,
  AppointmentChildrenCountNextSlot,
  AppointmentAdultsCountPreviousSlot,
  AppointmentAdultsCountNextSlot,
  AppointmentAdultsAndChildrenCountPreviousSlot,
  AppointmentAdultsAndChildrenCountNextSlot,
  AppointmentHasProductRestrictions,
  AppointmentHasGripeProductRestrictions 
}