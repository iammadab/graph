use std::fmt::Display;

pub(crate) struct HanoiState {
    rods: [[u8; 3]; 3],
}

impl Display for HanoiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut label = String::new();
        for rod in self.rods {
            for disc in rod {
                label.push_str(&disc.to_string());
            }
        }
        write!(f, "{}", label)
    }
}

#[cfg(test)]
mod test {
    use super::HanoiState;
    use std::fmt::Display;

    #[test]
    fn test_state_display() {
        let a = HanoiState {
            rods: [[3, 2, 1], [0, 0, 0], [0, 0, 0]],
        };
        assert_eq!(a.to_string(), "321000000");
    }
}
