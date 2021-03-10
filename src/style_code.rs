use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq)]
pub struct StyleCode {
  pub bas: u32,
  pub mov: u32,
  pub din: u32,
  pub com: u32,
  pub sapd: u32,
  pub gcc: u32,
  pub dif: u32,
  pub sog: u32,
  pub pen: u32,
}

static ALPHABET_26: &str = "abcdefghijklmnopqrstuvwxyz";
static ALPHABET_23: &str = "abcdefghjklmnpqrstuvwxy";
static PENALTIES_CODE: &str = "0123456789abcdefghjkl";

impl StyleCode {
  pub const MAX_PENALTIES: u32 = 20;

  fn encode26(value: u32) -> &'static str {
    let i = value as usize;
    &ALPHABET_26[i..i + 1]
  }

  fn decode26(value: &str) -> Option<u32> {
    ALPHABET_26
      .find(value)
      .map(|i: usize| i.try_into().unwrap())
  }

  fn encode23(value: u32) -> &'static str {
    let i = value as usize;
    &ALPHABET_23[i..i + 1]
  }

  fn decode23(value: &str) -> Option<u32> {
    ALPHABET_23
      .find(value)
      .map(|i: usize| i.try_into().unwrap())
  }

  fn encode_penalties(value: u32) -> &'static str {
    let i = value as usize;
    &PENALTIES_CODE[i..i + 1]
  }

  fn decode_penalties(value: &str) -> Option<u32> {
    PENALTIES_CODE
      .find(value)
      .map(|i: usize| i.try_into().unwrap())
  }

  pub fn decode(code: &str) -> Option<StyleCode> {
    lazy_static! {
      static ref DECODE_REGEX : Regex = Regex::new(r"^(?P<first>[a-zA-Z])(?P<points>[0-9]{1,2})(?:(?P<second>[a-hj-np-zA-HJ-NP-Z])(?:(?P<third>[a-zA-Z])?(?:(?P<sog>[0-3])(?P<pen>[0-9a-l])?)?)?)?$").unwrap();
    }

    let code_lowercase = code.to_ascii_lowercase();
    let m = match DECODE_REGEX.captures(&code_lowercase) {
      Option::Some(v) => v,
      None => return None,
    };

    let mut out = StyleCode {
      bas: 0,
      mov: 0,
      din: 0,
      com: 0,
      sapd: 0,
      gcc: 0,
      dif: 0,
      sog: 0,
      pen: 0,
    };

    let points: u32 = m.name("points").unwrap().as_str().parse().unwrap();

    out.sog = m
      .name("sog")
      .map_or_else(|| 0, |s: Match| s.as_str().parse().unwrap());

    if let Some(val) = m.name("pen") {
      if let Some(pen) = StyleCode::decode_penalties(val.as_str()) {
        out.pen = pen;
      } else {
        return None;
      }
    }
    let first_str = m.name("first").unwrap().as_str();
    let second_group = m.name("second");
    let third_group = m.name("third");

    match third_group {
      None => {
        match second_group {
          Some(_) if second_group.unwrap().as_str() != "z" => {
            // 2-letters code
            let mut remainder;
            let second_letter = second_group.unwrap().as_str();
            if let Some(val1) = StyleCode::decode26(first_str) {
              if let Some(val2) = StyleCode::decode23(second_letter) {
                remainder = val1 * 23 + val2;
              } else {
                return None;
              }
            } else {
              return None;
            }

            out.mov = remainder / (4 * 4 * 2 * 2 * 2);
            remainder = remainder % (4 * 4 * 2 * 2 * 2);
            out.din = remainder / (4 * 2 * 2 * 2);
            remainder = remainder % (4 * 2 * 2 * 2);
            out.gcc = remainder / (2 * 2 * 2);
            remainder = remainder % (2 * 2 * 2);
            out.com = remainder / (2 * 2);
            remainder = remainder % (2 * 2);
            out.sapd = remainder / 2;
            out.dif = remainder % 2;
          }
          None | Some(_) => {
            // 1-letter code
            let mut remainder;
            if let Some(val) = StyleCode::decode26(first_str) {
              remainder = val;
            } else {
              return None;
            }
            out.mov = remainder / (3 * 3);
            remainder = remainder % (3 * 3);
            out.din = remainder / 3;
            out.gcc = remainder % 3;
          }
        }
      }
      Some(_) => {
        // 3-letters code
        let mut remainder;
        if let Some(val1) = StyleCode::decode26(first_str) {
          if let Some(val2) = StyleCode::decode23(second_group.unwrap().as_str()) {
            if let Some(val3) = StyleCode::decode26(third_group.unwrap().as_str()) {
              remainder = val1 * 26 * 23 + val2 * 26 + val3;
            } else {
              return None;
            }
          } else {
            return None;
          }
        } else {
          return None;
        }
        out.mov = remainder / (4 * 4 * 4 * 4 * 4);
        remainder = remainder % (4 * 4 * 4 * 4 * 4);
        out.din = remainder / (4 * 4 * 4 * 4);
        remainder = remainder % (4 * 4 * 4 * 4);
        out.gcc = remainder / (4 * 4 * 4);
        remainder = remainder % (4 * 4 * 4);
        out.com = remainder / (4 * 4);
        remainder = remainder % (4 * 4);
        out.sapd = remainder / 4;
        out.dif = remainder % 4;
      }
    }

    let total_other_scores = out.mov + out.din + out.com + out.sapd + out.gcc + out.dif;
    out.bas = if points >= total_other_scores {
      points - total_other_scores
    } else {
      return None;
    };

    if out.valid() {
      Option::from(out)
    } else {
      Option::None
    }
  }

  #[allow(dead_code)]
  pub fn encode(&self) -> String {
    assert!(self.valid());

    // Compute total number of style points.
    let points = self.bas + self.mov + self.din + self.com + self.sapd + self.gcc + self.dif;

    // Create string representation
    let mut out = String::with_capacity(8);

    let one_letter;
    if (self.com + self.sapd + self.dif == 0)
      && (self.mov + self.din + self.gcc < 6)
      && self.mov < 3
      && self.din < 3
      && self.gcc < 3
    {
      // 1-letter format
      one_letter = true;
      out.push_str(StyleCode::encode26(
        (self.mov * 3 + self.din) * 3 + self.gcc,
      ));
      out.push_str(&points.to_string());
    } else if self.com < 2 && self.sapd < 2 && self.dif < 2 {
      // 2-letters format
      one_letter = false;
      let value = ((((self.mov * 4 + self.din) * 4 + self.gcc) * 2 + self.com) * 2 + self.sapd) * 2
        + self.dif;
      out.push_str(StyleCode::encode26(value / 23));
      out.push_str(&points.to_string());
      out.push_str(StyleCode::encode23(value % 23));
    } else {
      // 3-letters format
      one_letter = false;
      let value = ((((self.mov * 4 + self.din) * 4 + self.gcc) * 4 + self.com) * 4 + self.sapd) * 4
        + self.dif;
      out.push_str(StyleCode::encode26(value / (23 * 26)));
      out.push_str(&points.to_string());
      let remainder = value % (23 * 26);
      out.push_str(StyleCode::encode23(remainder / 26));
      out.push_str(StyleCode::encode26(remainder % 26));
    }

    if self.sog + self.pen != 0 {
      if one_letter {
        out.push('z');
      }

      out.push_str(&self.sog.to_string());
      if self.pen != 0 {
        let penalties = std::cmp::min(self.pen, StyleCode::MAX_PENALTIES);
        out.push_str(StyleCode::encode_penalties(penalties));
      }
    }

    out
  }
  pub fn valid(&self) -> bool {
    self.bas <= 3
      && self.mov <= 3
      && self.din <= 3
      && self.com <= 3
      && self.sapd <= 3
      && self.gcc <= 3
      && self.dif <= 3
      && self.sog <= 3
      && self.pen <= StyleCode::MAX_PENALTIES
  }

  pub fn score(&self) -> f32 {
    let points = self.bas + self.mov + self.din + self.com + self.sapd + self.gcc + self.dif;
    let score = 55 + 2 * points + 1 * self.sog - 5 * self.pen;
    score as f32 / 10.0
  }

  pub fn pretty_print(&self) -> String {
    format!(
      "Score: {}
BAS  : {}
MOV  : {}
DIN  : {}
COM  : {}
SAPD : {}
GCC  : {}
DIF  : {}
SOG  : {}
PEN  : {}
",
      self.score(),
      self.bas,
      self.mov,
      self.din,
      self.com,
      self.sapd,
      self.gcc,
      self.dif,
      self.sog,
      self.pen
    )
  }

  pub fn raw_print(&self) -> String {
    format!(
      "{}
{}
{}
{}
{}
{}
{}
{}
{}
{}
",
      self.score(),
      self.bas,
      self.mov,
      self.din,
      self.com,
      self.sapd,
      self.gcc,
      self.dif,
      self.sog,
      self.pen
    )
  }
}
