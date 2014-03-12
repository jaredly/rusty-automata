

pub struct Rules {
  danger: u8,
  crowd: u8,
  alone: u8,
  food: u8,
  support: u8,
  min_grow: u8,
  gang: bool
}

pub enum RuleKey {
  Danger,
  Crowd,
  Alone,
  Support,
  Food
}

pub fn ruleIt(rules: &mut Rules, key: RuleKey, val: u8) {
  match key {
    Danger => rules.danger = val,
    Support => rules.support = val,
    Crowd => rules.crowd = val,
    Alone => rules.alone = val,
    Food => rules.food = val
  }
}

