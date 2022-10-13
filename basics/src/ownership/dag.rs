use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {

    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v|v.clone())
    }

}

#[derive(Debug)]
struct MutNode {
    id: usize,
    downstream: Option<Rc<RefCell<MutNode>>>,
}

impl MutNode {

    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<MutNode>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<MutNode>>> {
        self.downstream.as_ref().map(|v | v.clone())
    }

}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use super::*;

    #[test]
    pub fn test_node() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);

        node3.update_downstream(Rc::new(node4));

        node1.update_downstream(Rc::new(node3));

        node2.update_downstream(node1.get_downstream().unwrap());

        println!("node1: {:?}, node2: {:?}", node1, node2)
    }

    #[test]
    pub fn test_node_update() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);

        node3.update_downstream(Rc::new(node4));

        node1.update_downstream(Rc::new(node3));

        node2.update_downstream(node1.get_downstream().unwrap());

        println!("node1: {:?}, node2: {:?}", node1, node2);

        let node5 = Node::new(5);
        let node3 = node1.get_downstream().unwrap();
        /*
        cannot borrow data in an `Rc` as mutable [E0596] cannot borrow as mutable
        Help: trait `DerefMut` is required to modify through a dereference,
        but it is not implemented for `std::rc::Rc<dag::Node>`
         */
        //node3.update_downstream(Rc::new(node5));
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }

    #[test]
    pub fn test_refcell() {
        let data = RefCell::new(1);
        {
            // 获取 RefCell 内部数据的可变借用
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }

    #[test]
    pub fn test_refcell_scope() {
        let data = RefCell::new(1);
        // 获取 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
        println!("data: {:?}", data.borrow()); // already mutably borrowed: BorrowError
    }

    #[test]
    pub fn test_node_update_refcell() {
        let mut node1 = MutNode::new(1);
        let mut node2 = MutNode::new(2);
        let mut node3 = MutNode::new(3);
        let node4 = MutNode::new(4);

        node3.update_downstream(Rc::new(RefCell::new(node4)));

        node1.update_downstream(Rc::new(RefCell::new(node3)));

        node2.update_downstream(node1.get_downstream().unwrap());

        println!("node1: {:?}, node2: {:?}", node1, node2);

        let node5 = MutNode::new(5);
        let node3 = node1.get_downstream().unwrap();

        // 获取可变引用，来修改downstream
        node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }

}
