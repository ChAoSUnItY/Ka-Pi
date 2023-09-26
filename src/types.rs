pub fn compute_method_descriptor_sizes(descriptor: &str, is_static: bool) -> (u8, u8) {
  let mut arg_size = if is_static { 1 } else { 0 };
  let mut chars = descriptor.chars().peekable();

  chars.next(); // Skips '('

  loop {
    let Some(mut char) = chars.next() else {
      panic!("Incomplete method descriptor `{descriptor}` while computing sizes");
    };

    if char == ')' {
      break;
    }

    match char {
      'J' | 'D' => arg_size += 2,
      _ => {
        if char == '[' {
          while let Some(_) = chars.next_if_eq(&'[') {}

          let Some(starting_char) = chars.next() else {
            panic!("Incomplete method descriptor `{descriptor}` while computing sizes");
          };

          char = starting_char;
        }
        
        if char == 'L' {
          while let Some(_) = chars.next_if(|&c| c != ';') {}
      
          chars.next(); // Skips ';'
        }

        arg_size += 1;
      }
    }
  }

  let return_size = match chars.next() {
    Some(char) => match char {
      'V' => 0,
      'J' | 'D' => 2,
      _ => 1,
    },
    None => {
      panic!("Incomplete method descriptor `{descriptor}` while computing sizes");
    }
  };

  (arg_size, return_size)
}

#[cfg(test)]
mod test {
    use crate::types::compute_method_descriptor_sizes;

  #[test]
  fn test_computing_method_descriptor_size() {
      assert_eq!(compute_method_descriptor_sizes("(JDJ)V", false), (6, 0));
      assert_eq!(compute_method_descriptor_sizes("([[J[[I)V", false), (2, 0));
      assert_eq!(compute_method_descriptor_sizes("([[Ljava/lang/String;I)V", false), (2, 0));
      assert_eq!(compute_method_descriptor_sizes("(Ljava/lang/String;Ljava/lang/Class;)V", false), (2, 0));
      assert_eq!(compute_method_descriptor_sizes("()V", true), (1, 0));
      assert_eq!(compute_method_descriptor_sizes("(I)V", true), (2, 0));
      assert_eq!(compute_method_descriptor_sizes("()Z", true), (1, 1));
      assert_eq!(compute_method_descriptor_sizes("(J)Z", true), (3, 1));
  }
}
