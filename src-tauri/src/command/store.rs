use crate::store::{InternalStore, SyncedStore};
use tauri::State;

#[tauri::command]
pub fn retrieve_store(state: State<InternalStore>) -> SyncedStore {
    println!("retrieve_state called");
    SyncedStore::from(state)
}
