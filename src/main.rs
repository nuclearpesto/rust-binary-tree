mod Tree {

    pub struct Tree {
        pub left: Option<Box<Tree>>,
        pub right: Option<Box<Tree>>,
        pub key: String,
    }

    impl Tree {
        fn insert(tree: &Option<Box<Tree>>, key: &str) -> (Tree) {
            match tree {
                None => Tree {
                    left: None,
                    right: None,
                    key: key.to_string(),
                },
                Some(tree) => {
                    let Void = match key.cmp(&tree.key) {
                        Less => {insert(&tree.left, key)}
                        More => {insert(&tree.right, key)}
                        Equal => {}
                    };
                    Tree {
                        left: None,
                        right: None,
                        key: "fo".to_string(),
                    }
                }
            }
        }
    }
}
fn main() {
    let tree = Tree::Tree {
        left: None,
        right: None,
        key: "ab".to_string(),
    };
}
