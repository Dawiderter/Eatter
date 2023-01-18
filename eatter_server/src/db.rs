use std::{sync::Arc};

use tokio::sync::RwLock;

use crate::{posts::{Post, Comment}};

#[derive(Debug, Clone)]
pub struct Database {
    post_map: Arc<RwLock<Vec<Post>>>,
}

impl Database {
    pub async fn create_post(&self, author: String, content: String) -> usize{
        let mut map = self.post_map.write().await;
        let len = map.len();
        map.push(Post { post_id: len, author, content, comments: Vec::new() });
        len
    }
    pub async fn create_comment(&self, post_id: usize, author: String, content: String) -> Option<usize> {
        let mut lock = self.post_map.write().await;
        let post = lock.get_mut(post_id);
        if let Some(post) = post {
            let len = post.comments.len();
            post.comments.push(Comment { author, content });
            Some(len)
        } else {
            None
        }
    }

    pub async fn get_post(&self, key: usize) -> Option<Post> {
        self.post_map.read().await.get(key).map(Post::clone)
    }
    pub async fn posts(&self) -> impl Iterator<Item = Post> {
        self.post_map.read().await.clone().into_iter()
    }
    pub async fn size(&self) -> usize {
        self.post_map.read().await.len()
    }
    pub async fn new() -> Self {
        Self {
            post_map: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
