mod tree {
    use std::rc::Rc;
    #[derive(Debug)]
    struct BTree {
        root: Node
    }
    impl Btree{
        fn insert(key : &str){

        }
    }

    #[derive(Debug)]
    struct Node {
         left: Option<Rc<Node>>,
         right: Option<Rc<Node>>,
         key: String,
    }

    impl Node {
        pub fn insert(tree: &Option<Rc<Node>>, key: &str) -> (Option<Rc<Node>>) {
            match tree {
                None => Node::new_leaf(key),
                Some(tree) => Node::tree_by_key(tree, key),
            }
        }

        fn clone_node(node: &Option<Rc<Node>>) -> (Option<Rc<Node>>) {
            match node {
                None => None,
                Some(rc) => Some(Rc::clone(rc)),
            }
        }
        fn new_leaf(key: &str) -> (Option<Rc<Node>>) {
            Some(Rc::new(Node {
                left: None,
                right: None,
                key: key.to_string(),
            }))
        }

        fn tree_by_key(tree: &Rc<Node>, key: &str) -> (Option<Rc<Node>>) {
            let left: Option<Rc<Node>>;
            let right: Option<Rc<Node>>;
            let newkey: String;

            match key.cmp(&tree.key) {
                Less => {
                    left = Node::insert(&tree.left, key);
                    right = Node::clone_node(&tree.right);
                    newkey = tree.key.clone();
                }
                More => {
                    left = Node::insert(&tree.left, key);
                    right = Node::clone_node(&tree.right);
                    newkey = tree.key.clone();
                }
                Equal => {
                    left = Node::insert(&tree.left, key);
                    right = Node::clone_node(&tree.right);
                    newkey = tree.key.clone();
                }
            };
            Some(Rc::new(Node {
                left,
                right,
                key: newkey,
            }))
        }
    }
}

fn main() {
    let tree = tree::Node {
        left: None,
        right: None,
        key: "ab".to_string(),
    };
}

#[cfg(test)]
mod test{

    use tree::Node;
    #[test]
    fn add_one_node(){
        let tree = Node {
            left: None,
            right: None,
            key: "ab".to_string(),
        };

       let newtree = Node::insert(&tree,"cb");
    }




}
