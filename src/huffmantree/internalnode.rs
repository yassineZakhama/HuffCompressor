use super::HuffBaseNode;

struct HuffInternalNode {
    weight: i32,
    left: Box<dyn HuffBaseNode>,
    right: Box<dyn HuffBaseNode>,
}

impl HuffInternalNode {
    fn new(weight: i32, left: Box<dyn HuffBaseNode>, right: Box<dyn HuffBaseNode>) -> Self {
        Self {
            weight,
            left,
            right,
        }
    }

    fn left(&self) -> &Box<dyn HuffBaseNode> {
        &self.left
    }

    fn right(&self) -> &Box<dyn HuffBaseNode> {
        &self.right
    }
}

impl HuffBaseNode for HuffInternalNode {
    fn is_leaf(&self) -> bool {
        false
    }

    fn weight(&self) -> i32 {
        self.weight
    }
}
