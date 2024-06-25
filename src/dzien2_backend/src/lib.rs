use std::cell::RefCell;

thread_local! {
    static POSTS: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: String) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn add_post(post: String) {
    POSTS.with(|posts| {
        posts.borrow_mut().push(post)
    });
}


#[ic_cdk::query]
fn read_posts() -> Vec<String> {
    POSTS.with(|posts| {
        posts.borrow().clone()
    })
}