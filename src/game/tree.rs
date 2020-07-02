use crate::board::Board;
use crate::game::play_random_game;
use crate::game::result::GameResult;
use crate::moves::Move;
use rand::rngs::ThreadRng;

pub struct Node {
    // board is after half_move
    board: Board,
    half_move: Option<Move>,
    score: i32,
    games_played_count: i32,
}
pub struct Tree {
    node: Box<Node>,
    children: Vec<Box<Tree>>,
    can_have_children: bool,
}

impl Tree {
    pub fn new(board: &mut Board) -> Self {
        let result = Tree {
            node: Box::new(Node {
                board: board.clone(),
                half_move: None,
                score: 0,
                games_played_count: 0,
            }),
            children: Vec::new(),
            can_have_children: true,
        };

        result
    }

    fn make_children(&mut self) {
        let moves = self.node.board.generate_moves();
        for &half_move in moves.iter() {
            let mut board_clone = self.node.board.clone();
            board_clone.make_move(half_move);
            let king_attackers_locations =
                board_clone.get_attackers_of_king_square_locations(board_clone.state.side);
            let can_have_children = board_clone
                .get_game_state(&king_attackers_locations)
                .is_in_progress();
            let tree_children = Vec::new();
            let tree = Tree {
                node: Box::new(Node {
                    board: board_clone,
                    half_move: Some(half_move),
                    score: 0,
                    games_played_count: 0,
                }),
                children: tree_children,
                can_have_children: can_have_children,
            };
            self.children.push(Box::new(tree));
        }
    }

    pub fn get_value(&self, parent: &Self) -> f64 {
        let node = &self.node;
        if node.games_played_count == 0 {
            return f64::INFINITY;
        }
        let node_parent = &parent.node;
        let score = node.score as f64 / 2f64;
        let games_played_count = node.games_played_count as f64;
        let parent_games_played_count = node_parent.games_played_count as f64;
        let a = score / games_played_count;
        let b = (parent_games_played_count.ln() / games_played_count).sqrt();
        a + b
    }

    fn get_selected_child(&mut self) -> &mut Box<Tree> {
        let mut best_value = -1f64;
        let mut best_index = 0;
        let mut has_found = false;
        self.children.iter().enumerate().for_each(|(index, child)| {
            let value = child.get_value(&self);
            if value > best_value {
                has_found = true;
                best_value = value;
                best_index = index;
            }
        });

        self.children.get_mut(best_index).unwrap()
    }

    fn get_selected_leaf(&mut self) -> &mut Box<Tree> {
        let mut leaf = self.get_selected_child();
        while !leaf.children.is_empty() {
            leaf = leaf.get_selected_child();
        }
        leaf
    }

    pub fn get_best_move(&self) -> Option<Move> {
        if self.children.is_empty() {
            None
        } else {
            let mut best_score = -1;
            let mut best_move = 0;
            for leave in self.children.iter() {
                let node = &leave.node;
                if node.games_played_count > best_score {
                    best_score = node.games_played_count;
                    best_move = node.half_move.unwrap();
                }
            }
            Some(best_move)
        }
    }

    pub fn update(&mut self, result: GameResult) {
        self.node.games_played_count += 1;
        self.node.score += result.get_points(!self.node.board.state.side);
        if !self.children.is_empty() {
            let mut tree = self.get_selected_child();
            while !tree.children.is_empty() {
                tree.node.games_played_count += 1;
                tree.node.score += result.get_points(!tree.node.board.state.side);
                tree = tree.get_selected_child();
            }
        }
    }

    pub fn get_subtree(&mut self, half_move: Move) -> Option<&mut Box<Tree>> {
        self.children
            .iter_mut()
            .filter(|child| child.node.half_move.unwrap() == half_move)
            .next()
    }

    pub fn make_round(&mut self, rng: &mut ThreadRng) {
        let child = if self.children.is_empty() {
            self.make_children();
            self.children.get_mut(0).unwrap()
        } else {
            let leaf = self.get_selected_leaf();
            if leaf.can_have_children {
                leaf.make_children();
                leaf.children.get_mut(0).unwrap()
            } else {
                leaf
            }
        };

        let mut board = child.node.board.clone();
        let result = play_random_game(&mut board, rng);
        child.node.score += result.get_points(!child.node.board.state.side);
        child.node.games_played_count += 1;

        {
            self.update(result);
        }
    }
}
