// Convert an ident to a wasm text format instruction
//
macro_rules! ident_to_instruction {
    ($name: ident) => {{
        let lower = stringify!($name);
        let (l, r) = lower.split_at(3);
        let mut underscored = String::new();
        for (i, s) in r.chars().enumerate() {
            if i > 0 && s >= 'A' && s <= 'Z' {
                underscored.push('_');
            }
            underscored.push(s);
        }
        format!("{}.{}", l.to_lowercase(), underscored.to_lowercase())
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
}
