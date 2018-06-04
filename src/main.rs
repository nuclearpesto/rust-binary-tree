mod tree {
    use std::rc::Rc;

    pub struct Tree {
        pub left: Option<Rc<Tree>>,
        pub right: Option<Rc<Tree>>,
        pub key: String,
    }

    impl Tree {
        fn insert(tree: &Option<Rc<Tree>>, key: &str) -> (Option<Rc<Tree>>) {
            match tree {
                None => new_leaf(),
                Some(&tree) => tree_by_key(tree);
                }
            }
        }

        fn append_node(node: Option<Rc<Tree>>) -> (Option<Rc<Tree>>){
            match node{
                None => None,
                Some(rc) => Some(Rc::clone(rc))

            }
        }
        fn new_leaf(key: &str) -> (Option<Rc<Tree>>){
            Some(Rc::new(Tree {
                left: None,
                right: None,
                key: key.to_string()
            }))
        }

        fn tree_by_key(&tree) ->( Option<Rc<Tree>>){
            match key.cmp(tree.key) {
                Less => Some(Rc::new(Tree{
                    left: Tree.insert(&tree.left, key),
                    key: tree.key.clone(),
                    right: append_node(&tree.right)
                })),
                More => Some(Rc::new(Tree{
                    left: append_node(&tree.left),
                    key: tree.key.clone(),
                    right: insert(&tree.right)
                })),
                Equal => {
                    Rc::clone(:)
                }

    }


    }
}
fn main() {
    let tree = tree::Tree {
        left: None,
        right: None,
        key: "ab".to_string(),
    };
}
