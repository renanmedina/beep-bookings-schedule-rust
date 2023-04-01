use super::{Slot, BlockingRule};

#[derive(Clone)]
pub struct ScheduleBooking {
  rules: Vec<BlockingRule>,
  pub id: u32,
  pub date: String,
  pub slots_list: SlotNode
}

impl ScheduleBooking {
  pub fn new(id: u32, date: String) -> Self {  
    return ScheduleBooking {
      rules: build_rules(),
      id, 
      date, 
      slots_list: SlotNode::Nil 
    };
  }

  pub fn add_slot(mut self, mut slot: Slot) -> ScheduleBooking {
    slot = self.apply_blocks(slot);
    match self.slots_list {
      SlotNode::Nil => {
        self.slots_list = SlotNode::Node(slot, Box::new(SlotNode::Nil));
      },
      SlotNode::Node(_, _) => {
        self.slots_list = self.slots_list.append(slot);
      }
    }

    return self;
  }

  fn apply_blocks(&self, mut slot: Slot) -> Slot {
    for rule in self.rules.clone() {
      slot = self.apply_block(slot, rule);
    }

    return slot;
  }

  pub fn apply_block(&self, mut slot: Slot, rule: BlockingRule) -> Slot {
    match rule {
      BlockingRule::ProgressiveSlotLockEnabledByProduct => {
        if (slot.hour() == "10:00") {
          slot.blocked_by(rule);
        }
      }
      BlockingRule::AppointmentAtSameDay => {
        // println!("Applying AppointmentAtSameDay");
      }
      BlockingRule::AppointmentMaxEstimatedTime => {}
      BlockingRule::AppointmentVaccinesCountAndPreviousSlot => {}
      BlockingRule::AppointmentVaccinesCountAndNextSlot => {}
      BlockingRule::AppointmentChildrenCountPreviousSlot => {}
      BlockingRule::AppointmentChildrenCountNextSlot => {}
      BlockingRule::AppointmentAdultsCountPreviousSlot => {}
      BlockingRule::AppointmentAdultsCountNextSlot => {}
      BlockingRule::AppointmentAdultsAndChildrenCountPreviousSlot => {}
      BlockingRule::AppointmentAdultsAndChildrenCountNextSlot => {}
      BlockingRule::AppointmentHasProductRestrictions => {}
      BlockingRule::AppointmentHasGripeProductRestrictions => {}
    }

    return slot;
  }
}

fn build_rules() -> Vec::<BlockingRule> {
  let mut rules: Vec<BlockingRule> = Vec::with_capacity(10);
  rules.push(BlockingRule::ProgressiveSlotLockEnabledByProduct);
  rules.push(BlockingRule::AppointmentAtSameDay);
  rules.push(BlockingRule::AppointmentMaxEstimatedTime);
  rules.push(BlockingRule::AppointmentVaccinesCountAndPreviousSlot);
  rules.push(BlockingRule::AppointmentVaccinesCountAndNextSlot);
  rules.push(BlockingRule::AppointmentChildrenCountPreviousSlot);
  rules.push(BlockingRule::AppointmentChildrenCountNextSlot);
  rules.push(BlockingRule::AppointmentAdultsCountPreviousSlot);
  rules.push(BlockingRule::AppointmentAdultsCountNextSlot);
  rules.push(BlockingRule::AppointmentAdultsAndChildrenCountPreviousSlot);
  rules.push(BlockingRule::AppointmentAdultsAndChildrenCountNextSlot);
  rules.push(BlockingRule::AppointmentHasProductRestrictions);
  rules.push(BlockingRule::AppointmentHasGripeProductRestrictions);
  return rules;
}

#[derive(Clone)]
pub enum SlotNode {
  Node(Slot, Box<SlotNode>),
  Nil
}

impl SlotNode {
  fn back(&mut self) -> &mut SlotNode {
    let mut node = self;

    loop {
      match {node} {
        &mut SlotNode::Node(_, ref mut next) => node = next,
        other => return other,
      }
    }
  }

  fn append_ref(&mut self, slot: Slot) {        
    *self.back() = SlotNode::Node(slot, Box::new(SlotNode::Nil));
  }

  fn append(mut self, slot: Slot) -> Self {
    self.append_ref(slot);
    return self
  }

  pub fn stringify(&self) -> String {
    match self {
      SlotNode::Node(slot, ref next) => {
        match slot.is_blocked() {
          true => {
            format!("{} - B, {}", slot.hour(), next.stringify())
          }
          false => {
            format!("{} - F, {}", slot.hour(), next.stringify())
          }
        }
      },
      SlotNode::Nil => {
        format!("Nil")
      },
    }
}
}