mod internalnode;
mod leafnode;

trait HuffBaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> i32;
}
