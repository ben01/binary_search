use std::io;

fn binary_search(search_term: i64, sorted_array: &Vec<i64>) -> isize {
  let mut lower: isize = -1;
  let mut upper: isize = sorted_array.len() as isize;
  loop {
    if lower + 1 == upper {
      return -1;
    }

    let middle = lower + ((upper - lower) / 2);
    if sorted_array[middle as usize] == search_term {
      return middle;
    } else if sorted_array[middle as usize] < search_term {
      lower = middle;
    } else {
      upper = middle;
    }
  }
}

fn main() -> io::Result<()> {
  let mut buff: String = String::new();

  io::stdin().read_line(&mut buff)?;
  let mut sorted_array_tokens = buff.split(" ");
  sorted_array_tokens.next();

  let mut sorted_array: Vec<i64> = Vec::new();
  for token in sorted_array_tokens {
    sorted_array.push(token.trim().parse::<i64>().unwrap());
  }

  buff = "".to_string();
  io::stdin().read_line(&mut buff)?;
  let mut search_term_tokens = buff.split(" ");
  search_term_tokens.next();

  let mut search_terms: Vec<i64> = Vec::new();
  for token in search_term_tokens {
    search_terms.push(token.trim().parse::<i64>().unwrap());
  }

  for search_term in search_terms {
    print!("{} ", binary_search(search_term, &sorted_array));
  }
  println!();

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let test_array: Vec<i64> = vec![1, 5, 8, 12, 13];
    assert_eq!(binary_search(8, &test_array), 2);
    assert_eq!(binary_search(1, &test_array), 0);
    assert_eq!(binary_search(23, &test_array), -1);
    assert_eq!(binary_search(1, &test_array), 0);
    assert_eq!(binary_search(11, &test_array), -1);
  }

  #[test]
  fn test_2() {
    let test_array: Vec<i64> = vec![1, 2, 3, 4, 5];
    assert_eq!(binary_search(1, &test_array), 0);
    assert_eq!(binary_search(2, &test_array), 1);
    assert_eq!(binary_search(3, &test_array), 2);
    assert_eq!(binary_search(4, &test_array), 3);
    assert_eq!(binary_search(5, &test_array), 4);
  }

  #[test]
  fn test_empty_array() {
    let test_array: Vec<i64> = Vec::new();
    assert_eq!(binary_search(25, &test_array), -1);
  }
}
