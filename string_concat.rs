fn main() {
  println!("{}", foo());
  println!("{}", swap_chars("doodley"));
  println!("{}", swap_chars("Steven"));
}

fn swap_chars(s: &str) -> String {
  // == Version that capitalizes first and lowercases last

  // The first term takes a slice and calls to_uppercase to get a String, the second term is an &str (ref to slice), the third term is like the first, but needs an "&" just to make rust's typechecker happy (seems like rust could easily add a version of Add for String that takes str, somehow. Or maybe it'd have to be syntax sugar)
  (s[s.len()-1..]).to_uppercase() + &s[1..s.len() - 1] + &(s[0..1]).to_lowercase()

  // == Versions that simply swap first and last

  // The first term needs to be a String because of the Add trait being defined on String for &str
//  (&s[s.len()-1..]).to_owned() + &s[1..s.len() - 1] + &s[0..1]

  // Better: the first term doesn't need to be a reference slice because we're calling a method on it to get a String)
//  (s[s.len()-1..]).to_owned() + &s[1..s.len() - 1] + &s[0..1]

  // Stylistically better? Starting with String::new() on the left makes the allocation we're doing very clear and makes all the following terms visually similar
//  String::new() + &s[s.len()-1..] + &s[1..s.len() - 1] + &s[0..1]
}

fn foo() -> String {
  let s: String = String::from("hello");
  return (&s[1..]).to_owned() + " orld";
}

