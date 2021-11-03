// Convert an ident to a wasm text format instruction
//
macro_rules! ident_to_instruction {
    ($name: ident) => {{
        let name = stringify!($name);
        let mut underscored = String::new();

        for (i, s) in name.chars().enumerate() {
            if i > 0 && s >= 'A' && s <= 'Z' {
                underscored.push('_');
            }
            underscored.push(s);
        }
        underscored.replacen("_", ".", 1).to_lowercase()
    }};
}

pub(crate) use ident_to_instruction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_test_instruction() {
        assert_eq!(
            ident_to_instruction!(I32TestInstruction),
            "i32.test_instruction"
        );
    }

    #[test]
    fn test_global_test_instruction() {
        assert_eq!(
            ident_to_instruction!(GlobalTestInstruction),
            "global.test_instruction"
        );
    }
}
