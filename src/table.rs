use super::board::Board;

#[derive(Clone, Copy)]
struct TableNode {
    node: u64,
}

impl TableNode {
    const HASH_MASK: u64 = (1_u64 << 49) - 1;

    fn get_value(&self) -> i8 {
        (self.node & ((1_u64 << 8) - 1)) as i8
    }

    fn get_hash(&self) -> u64 {
        self.node >> 8
    }

    fn new(key: u64, value: i8) -> TableNode {
        TableNode {
            node: (key << 8) | (value as u64),
        }
    }
}

pub struct Table {
    contents: Vec<TableNode>,
    add_count: i32,
    get_count: i32,
}

impl Table {
    const CAPACITY: u64 = 8388593 * 2;

    pub fn new() -> Table {
        let empty = TableNode {
            node: TableNode::HASH_MASK,
        };
        Table {
            contents: vec![empty; Table::CAPACITY as usize],
            add_count: 0,
            get_count: 0,
        }
    }

    pub fn add(&mut self, position: &Board, score: i32) {
        self.add_count += 1;
        let index = (position.hash() % Table::CAPACITY) as usize;
        self.contents[index] = TableNode::new(position.hash(), score as i8);
    }

    pub fn get(&mut self, position: &Board) -> Option<i32> {
        self.get_count += 1;
        let index = (position.hash() % Table::CAPACITY) as usize;
        let node = self.contents[index];
        if node.get_hash() == position.hash() {
            Some(node.get_value() as i32)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        let empty = TableNode { node: 0 };
        for i in 0..Table::CAPACITY {
            self.contents[i as usize] = empty;
        }
    }

    pub fn results(&self) {
        println!(
            "There have been {} adds and {} gets",
            self.add_count, self.get_count
        );
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn table() {
        let board = Board::construct("162636");
        let mut table = Table::new();
        table.add(&board, 20);
        assert_eq!(table.get(&board), Some(20));
        table.add(&board, 10);
        assert_eq!(table.get(&board), Some(10));

        table.results();
    }
}
