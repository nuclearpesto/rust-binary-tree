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
                None =>  Some(Rc::new(Tree {
                    left: None,
                    right: None,
                    key: key.to_string()
                })),
                Some(&tree) => {
                    let Void = match key.cmp(tree.key) {
                        Less => Tree{
                            left: Tree.insert(&tree.left, key),
                            key: tree.key.clone(),
                            right: Rc::clone(&tree.right)
                        },
                        More => Tree{
                            left: Some(Rc::clone(tree.left.unwrap())),
                            key: tree.key.clone(),
                            right: insert(&tree.right, key)
                        },
                        Equal => {}
                    };
                }
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
