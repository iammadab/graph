use std::fmt::Display;

pub(crate) struct HanoiState {
    rods: [Vec<u8>; 3],
}

impl HanoiState {
    fn make_move(&mut self, from: u8, to: u8) {
        // index 0 represents the value at the bottom
    }
}

impl Display for HanoiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut label = String::new();
        for rod in &self.rods {
            label.push('_');
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
            rods: [vec![3, 2, 1], vec![], vec![]],
        };
        assert_eq!(a.to_string(), "_321__");
    }
}

// now that I can display this what is next?
// probably to implement node on the state
// but to do that I will then need to implement neighbors
// so maybe first is to generate all possible next states from a given state
// to do this I need to apply a move
