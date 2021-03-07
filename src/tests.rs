#[cfg(test)]
mod tests {

  use crate::style_code::StyleCode;

  #[test]
  fn roundtrip() {
    let mut scores = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let max = [3, 3, 3, 3, 3, 3, 3, 3, StyleCode::MAX_PENALTIES];

    let mut counter = 0;
    loop {
      let code = StyleCode {
        bas: scores[0],
        mov: scores[1],
        din: scores[2],
        com: scores[3],
        sapd: scores[4],
        gcc: scores[5],
        dif: scores[6],
        sog: scores[7],
        pen: scores[8],
      };

      let code_str = code.encode();

      let decoded = StyleCode::decode(&code_str);
      if !decoded.is_some() {
        panic!("Style score could not be decoded: {}", code_str);
      }

      assert_eq!(code, decoded.unwrap());
      counter += 1;
      let mut i = 0;
      loop {
        scores[i] += 1;
        if scores[i] > max[i] {
          scores[i] = 0;
          i += 1;
          if i == 9 {
            println!("Roundtrip test: {} codes tested.", counter);
            assert_eq!(
              counter,
              4 * 4 * 4 * 4 * 4 * 4 * 4 * 4 * (StyleCode::MAX_PENALTIES + 1)
            );
            return;
          }
        } else {
          break;
        }
      }
    }
  }
}
