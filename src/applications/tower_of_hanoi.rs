use std::fmt::Display;

type Move = (u8, u8);

pub(crate) struct HanoiState {
    rods: [Vec<u8>; 3],
}

impl HanoiState {
    fn make_move(&mut self, from: u8, to: u8) {
        self.is_valid((from, to));
        self.make_move_unsafe((from, to));
    }

    fn make_move_unsafe(&mut self, to_make: Move) {
        let disc = self.rods[to_make.0 as usize].pop().unwrap();
        self.rods[to_make.1 as usize].push(disc);
    }

    fn is_valid(&self, maybe_move: Move) -> bool {
        // from and to should not be the same
        if maybe_move.0 == maybe_move.1 {
            return false;
        }

        // you cannot move from an empty rod
        if self.rods[maybe_move.0 as usize].is_empty() {
            return false;
        }

        // you are not allowed to move a larger disc
        // onto a smaller one
        if self.rods[maybe_move.0 as usize] > self.rods[maybe_move.1 as usize] {
            return false;
        }

        // valid move
        true
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
