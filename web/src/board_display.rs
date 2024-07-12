use crate::app::State;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(BoardDisplay)]
pub fn board_display() -> Html {
    let (state, _) = use_store::<State>();
    let mut elements = vec![];
    for i in 0..state.board.row() {
        let mut row = vec![];
        for j in 0..state.board.cols() {
            let val = state.board.index(i, j).unwrap();
            row.push(html! {<Cell solved={state.solved} label={val.to_string()} coords={(i, j)}/>});
        }
        elements.push(html! {<div class="row">{row}</div>});
    }

    html! {<>
     <div class="table">{elements}</div>
    </>}
}

#[derive(Properties, Clone, PartialEq)]
struct CellProps {
    label: String,
    coords: (isize, isize),
    solved: bool,
}

#[function_component(Cell)]
fn row(props: &CellProps) -> Html {
    let (state, dispatch) = use_store::<State>();
    let selected = state.selected.contains(&props.coords);
    let correct = state.correct.contains(&props.coords);
    let solved = props.solved;
    let cls = match (solved, selected, correct) {
        (true, true, true) => "cell right",
        (true, true, false) => "cell wrong",
        (true, false, true) => "cell miss",
        (false, true, _) => "cell clicked",
        (_, false, _) => "cell",
    };
    let onclick = |coords| {
        dispatch.reduce_mut_callback(move |state| {
            if state.solved {
                return;
            }
            if state.selected.contains(&coords) {
                state.selected.retain(|&c| c != coords);
            } else {
                state.selected.push(coords);
            }
        })
    };
    html! {
        <button onclick={onclick(props.coords)} class={classes!(cls)}>
            {props.label.clone()}
        </button>
    }
}
