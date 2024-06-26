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

#[ic_cdk::update]
fn delete_post(post_id: usize) {
    POSTS.with(|posts| {
        posts.borrow_mut().remove(post_id)
    });
}

#[ic_cdk::update]
fn edit_post(post_id: usize, new_post: String) {
    POSTS.with(|posts| {
        let mut binding = posts.borrow_mut();
        let post = binding.get_mut(post_id);
        let old_post = post.unwrap();
        *old_post = new_post;
    });
} 