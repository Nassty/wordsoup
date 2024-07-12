use crate::board_display::BoardDisplay;
use yew::prelude::*;
use yewdux::prelude::*;

use engine::{board::Board, word_puzzle::WordPuzzle};

#[derive(Debug, Store, Clone, Default, PartialEq)]
pub struct State {
    pub selected: Vec<(isize, isize)>,
    pub correct: Vec<(isize, isize)>,
    pub board: Board,
    pub solved: bool,
    pub words: Vec<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<State>();
    let kb = dispatch.reduce_mut_callback::<_, _, ()>(move |state| {
        let words = crate::words::words(3);
        state.words.clone_from(&words);
        let mut board = WordPuzzle::new(words, 10, 10).unwrap().search().unwrap();
        let k = board.clone();
        let bd = k
            .data()
            .iter()
            .enumerate()
            .filter(|(_, &c)| c != '-')
            .map(|(i, _)| k.at(i as isize))
            .collect::<Vec<_>>();
        board.replace().unwrap();
        state.correct.clone_from(&bd);
        state.board.clone_from(&board);
        web_sys::console::log_1(&"Hello".into());
    });
    use_state(move || kb.emit(()));

    let solved = dispatch.reduce_mut_callback(move |state| {
        if state.solved {
            web_sys::window().unwrap().location().reload().unwrap();
        }
        state.solved = !state.solved;
    });
    let label = if state.solved { "Hide" } else { "Show" };
    let ws = state
        .words
        .iter()
        .map(|w| {
            html! {
                <li>{w}</li>
            }
        })
        .collect::<Html>();
    html! {
        <main>
            <ul>
                {ws}
            </ul>
            <BoardDisplay />
             <button onclick={solved}>{ label }</button>
        </main>
    }
}
