use std::env;
use regex::Regex;

fn parse_data(param: &str) -> (&str, &str) {
   let mut rd = ("", "");
   while let Some(idx) = param.find("}") {
      let g = (r"\w+", &param[idx + 1..]);
      rd = g;
      break;
   }
   return rd;
}

fn parse(param: &str) -> String {
   let mut unprocessed = param;
   let mut regex = String::from("");
   while let Some(idx) = unprocessed.find("{") {
      let (prefix, rmn) = unprocessed.split_at(idx);
      regex.push_str(prefix);
      let (reg, rmn) = parse_data(rmn);
      unprocessed = rmn;
      regex.push_str(reg);
   }
   regex.push_str(unprocessed);
   regex
}


fn main() {
}    



#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn match_route() {
      let route = "/user/get/{id}/name/{username}/get".to_string();
      let url = "/user/get/111/name/sankar/get";
      let regex = parse(&route);
      let re = Regex::new(&regex).unwrap();
      let a = re.is_match(url);
      assert_eq!(a, true);
   }

   #[test]
   fn test_date() {
      let re = Regex::new(r"^\((\d{4}-\d{2}-\d{2})\)$").unwrap();
      assert!(re.is_match("(2014-01-01)"));
   }
}
