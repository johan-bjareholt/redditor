extern crate redditor;

use redditor::Client;
use redditor::types::*;

#[test]
fn basic_get_user() {
    let mut client = Client::new();
    let user = client.get_user("spez").unwrap();
    println!("{}", user.name());
    println!("{}", user.comment_karma());
    println!("{}", user.link_karma());
}

#[test]
fn basic_get_subreddit() {
    let mut client = Client::new();
    let posts = client.get_subreddit_posts("all");
    for post in posts.into_iter() {
        println!("r/{}, u/{}", post.subreddit(), post.author());
        println!("{}: {}", post.score(), post.title());
        println!("");
    }
}

fn print_comment(comment: &Comment, depth: usize) {
    println!("{: <1$}{2}, u/{3}", "", depth*4, comment.score(), comment.author());
    println!("{: <1$}{2}", "", depth*4, comment.body());
    for child in comment.replies() {
        print_comment(child, depth+1);
    }
}

#[test]
fn basic_get_comments() {
    let mut client = Client::new();
    let posts = client.get_comments("/r/pics/comments/9vrdc9/my_amazing_grandmother_turns_100_on_tuesday_she/").unwrap();
    println!("{}: ", posts.post().title());
    for comment in posts.comments() {
        print_comment(comment, 0);
    }
}
