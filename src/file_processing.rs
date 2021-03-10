use csv;

use crate::style_code::StyleCode;

struct CsvProcessor {
  has_headers: bool,
  column_index: Option<usize>,
}

pub fn process_file(
  infile: &str,
  outfile: &str,
  delimiter: char,
  has_headers: bool,
  column_index: Option<usize>,
) -> Result<(), (String, i32)> {
  let mut processor = CsvProcessor {
    has_headers,
    column_index,
  };

  if infile == outfile {
    return Err((
      format!(
        "Reading and writing to the same file is not supported: {}",
        infile
      ),
      1001,
    ));
  }

  let mut reader = match csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(delimiter as u8)
    .from_path(infile)
  {
    Ok(r) => r,
    Err(_) => return Err((format!("Error opening csv file: {}", infile), 1002)),
  };

  let mut writer = match csv::WriterBuilder::new()
    .has_headers(false)
    .delimiter(delimiter as u8)
    .from_path(outfile)
  {
    Ok(r) => r,
    Err(_) => return Err((format!("Error opening csv file: {}", outfile), 1003)),
  };
  for (line, result) in reader.records().enumerate() {
    let line_process_result = match result {
      Ok(val) => processor.process_record(&val),
      Err(_) => {
        return Err((
          format!("Error reading line {} from csv file: {}", line, infile),
          1004,
        ))
      }
    };

    let line_write_result = match line_process_result {
      Ok(processed_line) => writer.write_record(&processed_line),
      Err((msg, err_code)) => {
        return Err((
          format!(
            "Error processing line {} from csv file: {}\n  {}",
            line, infile, msg
          ),
          err_code,
        ))
      }
    };

    if line_write_result.is_err() {
      return Err((
        format!(
          "Error writing processed line {} to csv file: {}",
          line, outfile
        ),
        1005,
      ));
    }
  }

  Ok(())
}

impl CsvProcessor {
  fn process_record(
    &mut self,
    record: &csv::StringRecord,
  ) -> Result<csv::StringRecord, (String, i32)> {
    assert!(!record.is_empty());

    if self.column_index.is_none() {
      self.column_index = Option::from(record.len() - 1);
    }

    let mut out = record.clone();

    if self.has_headers {
      out.push_field("Score");
      out.push_field("BAS");
      out.push_field("MOV");
      out.push_field("DIN");
      out.push_field("COM");
      out.push_field("SAPD");
      out.push_field("GCC");
      out.push_field("DIF");
      out.push_field("SOG");
      out.push_field("PEN");

      self.has_headers = false;
      return Ok(out);
    }

    let ix = self.column_index.unwrap();

    let code = &record[ix];

    match StyleCode::decode(code.trim()) {
      Some(decoded) => {
        out.push_field(&decoded.score().to_string());
        out.push_field(&decoded.bas.to_string());
        out.push_field(&decoded.mov.to_string());
        out.push_field(&decoded.din.to_string());
        out.push_field(&decoded.com.to_string());
        out.push_field(&decoded.sapd.to_string());
        out.push_field(&decoded.gcc.to_string());
        out.push_field(&decoded.dif.to_string());
        out.push_field(&decoded.sog.to_string());
        out.push_field(&decoded.pen.to_string());
      }
      None => {
        for _ in 0..10 {
          out.push_field("<invalid code>");
        }
      }
    }

    return Ok(out);
  }
}
