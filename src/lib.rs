extern crate restson;
#[macro_use]
extern crate serde_derive;


pub mod types;

use restson::{RestClient,RestPath};
use self::types::*;


impl RestPath<&str> for User {
    fn get_path(username: &str) -> Result<String,restson::Error> { Ok(format!("user/{}/about.json", username)) }
}

impl RestPath<&str> for Listing<Post> {
    fn get_path(subreddit: &str) -> Result<String,restson::Error> { Ok(format!("r/{}.json", subreddit)) }
}

impl RestPath<&str> for CommentList {
    fn get_path(permalink: &str) -> Result<String,restson::Error> { Ok(format!("{}.json", permalink)) }
}

pub struct Client {
    client: RestClient,
}

// TODO: Better error handling
#[derive(Debug)]
pub enum Error {
    NotFound(),
    UnknownError()
}

impl Client {
    pub fn new() -> Client {
        let client = RestClient::new("https://www.reddit.com/").unwrap();
        Client {
            client: client
        }
    }

    pub fn get_subreddit_posts(&mut self, subreddit: &str) -> Listing<Post> {
        self.client.get(subreddit).unwrap()
    }

    pub fn get_comments(&mut self, permalink: &str) -> Result<CommentList, Error> {
        match self.client.get(permalink) {
            Ok(cl) => Ok(cl),
            Err(_) =>  Err(Error::UnknownError())
        }
    }

    pub fn get_user(&mut self, user: &str) -> Result<User, Error> {
        match self.client.get(user) {
            Ok(user) => Ok(user),
            Err(e) => Err(Error::UnknownError())
        }
    }
}
