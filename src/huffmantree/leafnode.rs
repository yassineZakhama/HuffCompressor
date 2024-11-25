use super::HuffBaseNode;

struct HuffLeafNode {
    element: char,
    weight: i32,
}

impl HuffLeafNode {
    fn new(element: char, weight: i32) -> Self {
        Self { element, weight }
    }
}

impl HuffBaseNode for HuffLeafNode {
    fn is_leaf(&self) -> bool {
        true
    }

    fn weight(&self) -> i32 {
        self.weight
    }
}
