use super::board;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MoveNode {
    pub action: u8,
    value: i32,
}

// sort from minimum to maximum
pub struct MoveSort {
    moves: [MoveNode; board::Board::WIDTH as usize],
    pub size: usize,
}

impl MoveSort {
    pub fn new() -> MoveSort {
        let moves = [MoveNode {
            action: 0,
            value: 0,
        }; board::Board::WIDTH as usize];
        MoveSort {
            moves,
            size: 0,
        }
    }

    fn clear(&mut self) {
        self.size = 0;
    }

    pub fn insert(&mut self, action: u8, value: i32) {
        self.size += 1;
        // increase the size
        // shift things over by 1 until you find a move with a bigger value or you have shifted everything over
        // then insert at that location
        let mut current = self.size - 1;
        while current > 0 && self.moves[current - 1].value > value {
            self.moves[current] = self.moves[current - 1];
            current -= 1;
        }
        self.moves[current].action = action;
        self.moves[current].value = value;
    }

    pub fn get_next(&mut self) -> u8 {
        self.size -= 1;
        self.moves[self.size].action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_sorter() {
        let mut sorter = MoveSort::new();
        sorter.insert(0, 0);
        sorter.insert(5, 5);
        sorter.insert(1, 1);
        sorter.insert(4, 4);
        sorter.insert(2, 2);
        sorter.insert(3, 3);
        //let mut action = sorter.getNext();
        for i in (0..6).rev() {
            assert_eq!(sorter.get_next(), i);
        }
    }
}
