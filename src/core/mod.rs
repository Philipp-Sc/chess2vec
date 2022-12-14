

use chess::{Board, ChessMove, Square};
use std::str::FromStr;
use std::result::{Result};

mod history;
use history::history_move::{Move};

mod move_info;
mod material;
mod mobility;
mod expansion_factor;
mod package_density;
mod pawn_structure;



/*
 * Known Bugs: if game_over is not checked -> RuntimeError: unreachable.
 *
 */

pub fn get_features(game_history: &str) -> Vec<u8> {
    // game_history = JSON.stringify(chess.history({ verbose: true }))
    // -> [{ color: 'w', from: 'e2', to: 'e4', flags: 'b', piece: 'p', san: 'e4' },
    //     { color: 'b', from: 'e7', to: 'e5', flags: 'b', piece: 'p', san: 'e5' },
    //     { color: 'w', from: 'f2', to: 'f4', flags: 'b', piece: 'p', san: 'f4' },
    //     { color: 'b', from: 'e5', to: 'f4', flags: 'c', piece: 'p', captured: 'p', san: 'exf4' }]

    //let data = r#"[{"color":"w","from":"e2","to":"e4","flags":"b","piece":"p","san":"e4"},{"color":"b","from":"e7","to":"e5","flags":"b","piece":"p","san":"e5"},{"color":"w","from":"f2","to":"f4","flags":"b","piece":"p","san":"f4"},{"color":"b","from":"e5","to":"f4","flags":"c","piece":"p","captured":"p","san":"exf4"},{"color":"w","from":"h2","to":"h4","flags":"b","piece":"p","san":"h4"}]"#;

    let history: Result<Vec<Move>,serde_json::Error> = serde_json::from_str(game_history);
    let res: Vec<u8>  = match history {
        Ok(history) => get_features_from_history(history), //format!("{:?}", get_features_from_history(history)),
        Err(history) => log_error_and_return_error_value(history.to_string().as_str())
    };
    res
}

fn log_error_and_return_error_value(_error: &str) -> Vec<u8>{
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &error.into());

    let error_value: Vec<u8> = Vec::new();
    error_value
}


pub fn get_keys() -> String {

    let mut a = move_info::get_keys();
    let mut b = material::get_keys();
    let mut c = mobility::get_keys();
    let mut d = expansion_factor::get_keys();
    let mut e = package_density::get_keys();
    let mut f = pawn_structure::get_keys();

    let mut res: Vec<String> = Vec::new();

    res.push("halfmove".to_string());

    res.append(&mut a);
    res.append(&mut b);
    res.append(&mut c);
    res.append(&mut d);
    res.append(&mut e);
    res.append(&mut f);
    res.join("\n")
}

fn get_features_from_history(history: Vec<Move>) -> Vec<u8> {

    let features: Result<Vec<u8>,chess::Error> = get_features_from_history_intern(history);

    let res: Vec<u8>  = match features {
        Ok(features) => features,
        Err(features) => log_error_and_return_error_value(features.to_string().as_str())
    };
    res
}

/*
fn move_to_new_chess_move(val: &Move, board: Board) -> Result<ChessMove,chess::Error> {
    let san = if val.flags=="e" {format!("{} {}",val.san,"e.p.")} else { val.san.to_string() };
    let m = ChessMove::from_san(&board, &san);
    let r: Result<ChessMove,chess::Error> = match m {
        Ok(m) => Ok(m),
        Err(_m) => move_rank_and_file_and_promotion_to_chess_move((*val.from).to_string(), (*val.to).to_string(), val.promotion.as_ref())
    };
    r
}*/

fn move_rank_and_file_and_promotion_to_chess_move(move_from: String, move_to: String, promotion: Option<&String>) -> Result<ChessMove,chess::Error> {

    let mut piece_promotion: Option<chess::Piece> = None;

    if promotion != None {
        let p = promotion.unwrap();

        for piece in chess::PROMOTION_PIECES {
            /*if &piece.to_string(chess::Color::White)==p {
                piece_promotion = Some(piece);
            }*/
            if &piece.to_string(chess::Color::Black)==p { // always lower case (chessjs)
                piece_promotion = Some(piece);
            }
        }
    }

    Ok(ChessMove::new(Square::from_str(move_from.as_str())?, Square::from_str(move_to.as_str())?, piece_promotion))
}

fn get_features_from_history_intern(history: Vec<Move>) -> Result<Vec<u8>,chess::Error> {
    let history_iter = history.iter();

    let mut board = Board::default();

    for val in history_iter {
        board = board.make_move_new(move_rank_and_file_and_promotion_to_chess_move((*val.from).to_string(), (*val.to).to_string(), val.promotion.as_ref())?);
    }

    let halfmove = history.len() as u8;


    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"1".into());
    let mut a = move_info::get_feature_vector(history);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"2".into());
    let mut b = material::get_feature_vector(board);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"3".into());
    let mut c = mobility::get_feature_vector(board);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"4".into());
    let mut d = expansion_factor::get_feature_vector(board);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"5".into());
    let mut e = package_density::get_feature_vector(board);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"6".into());
    let mut f = pawn_structure::get_feature_vector(board);
    //web_sys::console::log_2(&"WASM ERROR: ".into(), &"7".into());

    let mut res: Vec<u8> = Vec::new();
    res.push(halfmove);
    res.append(&mut a);
    res.append(&mut b);
    res.append(&mut c);
    res.append(&mut d);
    res.append(&mut e);
    res.append(&mut f);
    Ok(res)
}