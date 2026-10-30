// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0120
// CVE: N/A
// CWE: N/A
// Crate: json5
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/json5
// Extraction Method: repo_search
// GitHub Repo: callum-oakley/json5-rs
// Commit: 52146d2
// File: src/de.rs


{
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            match pair.as_rule() {
                Rule::null => visitor.visit_unit(),
                Rule::boolean => visitor.visit_bool(parse_bool(&pair)),
                Rule::string | Rule::identifier => visitor.visit_string(parse_string(pair)?),
                Rule::number => {
                    if is_int(pair.as_str()) {
                        visitor.visit_i64(parse_integer(&pair)?)
                    } else {
                        visitor.visit_f64(parse_number(&pair)?)
                    }
                 }
                Rule::array => visitor.visit_seq(Seq::new(pair)),
                Rule::object => visitor.visit_map(Map::new(pair)),
                _ => unreachable!(),
             }
         })();
         error::set_location(&mut res, &span);
         res
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_enum(Enum { pair })
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_i8(parse_number(&pair)? as i8)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_i16(parse_number(&pair)? as i16)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_i32(parse_number(&pair)? as i32)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_i64(parse_number(&pair)? as i64)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_i128(parse_number(&pair)? as i128)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_u8(parse_number(&pair)? as u8)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_u16(parse_number(&pair)? as u16)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_u32(parse_number(&pair)? as u32)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_u64(parse_number(&pair)? as u64)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_u128(parse_number(&pair)? as u128)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_f32(parse_number(&pair)? as f32)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            visitor.visit_f64(parse_number(&pair)?)
        })();
         error::set_location(&mut res, &span);
         res
     }
     {
         let pair = self.pair.take().unwrap();
         let span = pair.as_span();
        let mut res = (move || {
            match pair.as_rule() {
                Rule::null => visitor.visit_none(),
                _ => visitor.visit_some(&mut Deserializer::from_pair(pair)),
            }
         })();
         error::set_location(&mut res, &span);
         res
         V: de::Visitor<'de>,
     {
         let span = self.pair.as_ref().unwrap().as_span();
        let mut res = (move || {
            visitor.visit_newtype_struct(self)
        })();
         error::set_location(&mut res, &span);
         res
     }
     }
 }
 
fn parse_string(pair: Pair<'_, Rule>) -> Result<String> {
    let span = pair.as_span();
    let mut res = (move || {
    pair.into_inner()
        .map(|component| match component.as_rule() {
            Rule::char_literal => Ok(String::from(component.as_str())),
            Rule::char_escape_sequence => Ok(parse_char_escape_sequence(&component)),
            Rule::nul_escape_sequence => Ok(String::from("\u{0000}")),
            Rule::hex_escape_sequence | Rule::unicode_escape_sequence => {
                 let hex_escape = parse_hex(component.as_str())?;
                 match char::from_u32(hex_escape) {
                    Some(s) => Ok(s.to_string()),
                    None => Err(de::Error::custom("error parsing hex prefix")),
                 }
             }
             _ => unreachable!(),
        })
        .collect()
    })();
     error::set_location(&mut res, &span);
     res
 }
 
fn parse_char_escape_sequence(pair: &Pair<'_, Rule>) -> String {
    String::from(match pair.as_str() {
         "b" => "\u{0008}",
         "f" => "\u{000C}",
         "n" => "\n",
         "r" => "\r",
         "t" => "\t",
         "v" => "\u{000B}",
         c => c,
    })
 }
 
 fn parse_number(pair: &Pair<'_, Rule>) -> Result<f64> {
         V: de::DeserializeSeed<'de>,
     {
         let span = self.pair.as_span();
        let mut res = (move || {
            match self.pair.as_rule() {
                Rule::string => {
                    let tag = seed.deserialize(&mut Deserializer::from_pair(self.pair))?;
                    Ok((tag, Variant { pair: None }))
                }
                Rule::object => {
                    let mut pairs = self.pair.into_inner();

                    if let Some(tag_pair) = pairs.next() {
                        let tag = seed.deserialize(&mut Deserializer::from_pair(tag_pair))?;
                        Ok((tag, Variant { pair: pairs.next() }))
                    } else {
                        Err(de::Error::custom("expected a nonempty object"))
                    }
                 }
                _ => Err(de::Error::custom("expected a string or an object")),
             }
         })();
         error::set_location(&mut res, &span);
         res