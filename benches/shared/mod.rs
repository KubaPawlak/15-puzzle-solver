use solver::board::OwnedBoard;

pub fn create_sample_board() -> OwnedBoard {
    let board_str = r#"3 3
2 4 0
1 6 3
7 5 8
"#;

    board_str.parse().unwrap()
}
