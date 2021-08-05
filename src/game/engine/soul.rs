
/// soul power of an entity
/// (now, max)
pub struct SoulPower(pub u32, pub u32);

impl SoulPower {
  pub fn cost(&mut self, cost: u32) -> bool {
    if self.0 >= cost {
      self.0 -= cost;
      true
    } else {
      false
    }
  }

  pub fn gain(&mut self, gain: u32) {
    self.0 = self.1.min(self.0 + gain);
  }
}

