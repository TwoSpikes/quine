fn main() {
    let src = String::from("fn main() {
    let src = String::from(\"?\");
    let src_n = src.len();
    for c in src.chars() {
        if c == 63 as char {
            for c in src.chars() {
                match c {
                    '\\n' => {
                        print!(\"\\n\");
                    },
                    '\"' => {
                        print!(\"\\\\\\\"\");
                    },
                    '\\\\' => {
                        print!(\"\\\\\\\\\");
                    },
                    _ => {
                        print!(\"{}\", c);
                    }
                }
            }
        } else {
            print!(\"{}\", c);
        }
    }
}
");
    let src_n = src.len();
    for c in src.chars() {
        if c == 63 as char {
            for c in src.chars() {
                match c {
                    '\n' => {
                        print!("\n");
                    },
                    '"' => {
                        print!("\\\"");
                    },
                    '\\' => {
                        print!("\\\\");
                    },
                    _ => {
                        print!("{}", c);
                    }
                }
            }
        } else {
            print!("{}", c);
        }
    }
}
