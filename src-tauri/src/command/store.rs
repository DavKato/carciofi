use crate::store::{InternalState, SyncedState};
use tauri::State;

#[tauri::command]
pub fn retrieve_state(state: State<InternalState>) -> SyncedState {
    println!("retrieve_state called");
    SyncedState::from(&state)
}
