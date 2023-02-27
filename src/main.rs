pub enum BinaryTree {
    Nil,
    Node {
        val:i32,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
}
impl BinaryTree {
    pub fn replace(&mut self, to:Self){
        *self = to;
    }
    pub fn remove(&mut self) {
        self.replace(BinaryTree::Nil);
    }

}

macro_rules! bin_tree {
    ( val: $val:expr, left: $left:expr, right: $right:expr $(,)? ) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new($right),
        }
    };

    ( val: $val:expr, right: $right:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: $right,
        }
    };

    ( val: $val:expr, left: $left:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: $left,
            right: bin_tree!(),
        }
    };

    ( val: $val:expr $(,)? ) => {
        bin_tree!(val: $val, left: bin_tree!(), right: bin_tree!(),)
    };

    () => {
        BinaryTree::Nil
    };
}

fn main() {
    let root = bin_tree!{
        val:5,
        left:bin_tree!{
            val:4,
        left:bin_tree! {val: 11},
    },  right:bin_tree! {val: 8},};


}

fn find_13_by_DFS(cur_node: BinaryTree) -> bool {
    if cur_node.Node.val == 13 { return true; }

    // 左の子ノードから再帰的に 13 を探索
    if cur_node.left != Nil {
        if find_13_by_DFS(cur_node.left) { return true; }  // left 以下に13があった場合
    }

    // 右の子ノードから再帰的に 13 を探索
    if cur_node.right != Nil {
        if find_13_by_DFS(cur_node.right) { return true; }  // right 以下に13があった場合
    }

    false  // cur_node 以下には 13 が見つからなかった
}
fn find_13_by_BFS(root: BinaryTree) -> bool {
    let mut queue = VecDeque::<BinaryTree>::new();  // ノードを入れるキューを用意
    queue.push_back(root);  // ルートをpushしてループ開始

    while let Some(cur_node) = queue.pop_front() {  // キューが空の場合は pop_front() は None を返すので、ループから抜ける
        if cur_node.Node.val == 13 { return true; }

        if cur_node.left != Nil { queue.push_back(cur_node.left); }  // 左の子ノードがあればキューにpush
        if cur_node.right != Nil { queue.push_back(cur_node.right); }
    }

    false  // 13はどこにもなかった
}
