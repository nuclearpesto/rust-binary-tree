mod tree {
    use std::rc::Rc;
    #[derive(Debug)]
    pub struct BTree {
        root: Rc<Node>,
    }
    impl BTree {
        pub fn insert(tree: &BTree, key: &str) -> BTree {
            BTree {
                root: Node::insert(&Some(Rc::clone(&tree.root)), key).unwrap(),
            }
        }
        pub fn new(key: &str) -> BTree {
            BTree {
                root: Node::insert(&None, key).unwrap(),
            }
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
                    println!("{} is less than {}, recursing left", key, &tree.key);
                    left = Node::insert(&tree.left, key);
                    right = Node::clone_node(&tree.right);
                    newkey = tree.key.clone();
                }
                More => {
                    println!("{} is more than {}, recursing right", key, &tree.key);
                        left = Node::insert(&tree.left, key);
                    right = Node::clone_node(&tree.right);
                    newkey = tree.key.clone();
                }
                Equal => {
                    println!("{} is equal to {}, clonin node", key, &tree.key);
                    left = Node::clone_node(&tree.left);
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

    #[cfg(test)]
    mod test {

        use tree::BTree;
        #[test]
        fn new_btree_works() {
            let tree = BTree::new("hello");
            assert!(tree.root.left.is_none());
            assert!(tree.root.right.is_none());
            assert!(tree.root.key == "hello");
        }
        #[test]
        fn insert_one_works() {
            let tree = BTree::new("hello");
            let tree2 = BTree::insert(&tree, "goodbye");
            //first tree remains unaltered
            assert!(tree.root.left.is_none());
            assert!(tree.root.right.is_none());
            assert!(tree.root.key == "hello");

            //second tree root right has a new node
            assert!(tree2.root.left.is_some());
            assert!(tree2.root.right.is_none());
            assert_eq!(tree2.root.left.as_ref().unwrap().key, "goodbye");





        }

    }
}

fn main() {}
