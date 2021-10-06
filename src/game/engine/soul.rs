
/// soul power of an entity
/// (now, max)
pub struct SoulPower {
  pub now: u32,
  pub max: u32,
}

impl SoulPower {
  pub fn cost(&mut self, cost: u32) -> bool {
    if self.now >= cost {
      self.now -= cost;
      true
    } else {
      false
    }
  }

  pub fn obtain(&mut self, soul: u32) {
    self.now = self.max.min(self.now + soul);
  }
}

