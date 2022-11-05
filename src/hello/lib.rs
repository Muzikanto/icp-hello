use std::cell::RefCell;
use ic_cdk::{
    api::{
        self,
        call::{self, RejectionCode},
    },
    storage,
};

thread_local! {
    static MESSAGE: RefCell<String> = RefCell::new("".to_string());
}

#[ic_cdk_macros::query]
fn get() -> String {
    MESSAGE.with(|message| format!("{}, {}", api::caller().to_string(), (*message.borrow()).clone()))
}

/// Set the value of the counter.
#[ic_cdk_macros::update]
fn set(n: String) -> String {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    MESSAGE.with(|message| *message.borrow_mut() = n.clone());

    n.clone()
}
