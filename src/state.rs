use tower_cookies::Key;

use crate::posts::Post;

#[derive(Clone)]
pub struct GlobalState {
    pub master_key: Key,
    pub posts: Vec<Post>
}