#[derive(Deserialize)]
pub struct User {
    data: UserData
}

#[derive(Deserialize)]
struct UserData {
    name: String,
    comment_karma: i64,
    link_karma: i64
}

impl User {
    pub fn name(&self) -> String {
        self.data.name.clone()
    }
    pub fn comment_karma(&self) -> i64 {
        self.data.comment_karma
    }
    pub fn link_karma(&self) -> i64 {
        self.data.link_karma
    }
}

#[derive(Deserialize)]
pub struct Post {
    data: PostData
}

#[derive(Deserialize)]
struct PostData {
    title: String,
    selftext: String,
    url: String,
    score: i64,
    num_comments: i64,
    subreddit: String,
    author: String,
    permalink: String,
}

impl Post {
    pub fn title(&self) -> String {
        self.data.title.clone()
    }

    pub fn url(&self) -> String {
        self.data.url.clone()
    }

    pub fn body(&self) -> String {
        self.data.selftext.clone()
    }

    pub fn score(&self) -> i64 {
        self.data.score
    }

    pub fn num_comments(&self) -> i64 {
        self.data.num_comments
    }

    pub fn subreddit(&self) -> String {
        self.data.subreddit.clone()
    }

    pub fn author(&self) -> String {
        self.data.author.clone()
    }

    pub fn permalink(&self) -> String {
        self.data.permalink.clone()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CommentTypes {
    Comment(Comment),
    More(CommentMore)
}

#[derive(Deserialize)]
pub struct Comment {
    data: CommentData
}

#[derive(Deserialize)]
struct CommentData {
    score: i64,
    author: String,
    permalink: String,
    body: String,
    replies: Listing<CommentTypes>
}

impl Comment {
    pub fn score(&self) -> i64 { self.data.score }
    pub fn author(&self) -> String { self.data.author.clone() }
    pub fn permalink(&self) -> String { self.data.permalink.clone() }
    pub fn body(&self) -> String { self.data.body.clone() }
    pub fn replies(&self) -> Vec<&Comment> {
        let mut comments : Vec<&Comment> = Vec::new();
        self.data.replies.data.children.iter().for_each(|c|
            match c {
                CommentTypes::Comment(c) => comments.push(c),
                _ => ()
            }
        );
        comments
    }
}

#[derive(Deserialize)]
pub struct CommentMore {
    data: CommentMoreData
}

#[derive(Deserialize)]
struct CommentMoreData {}

#[derive(Deserialize)]
pub struct CommentList (
    Listing<Post>,
    Listing<CommentTypes>
);

impl CommentList {
    pub fn post(&self) -> &Post {
        &self.0.data.children[0]
    }

    pub fn comments(&self) -> Vec<&Comment>{
        let mut comments : Vec<&Comment> = Vec::new();
        self.1.data.children.iter().for_each(|c|
            match c {
                CommentTypes::Comment(c) => comments.push(c),
                _ => ()
            }
        );
        comments
    }
}

#[derive(Deserialize)]
pub struct ListingData<T> {
    children: Vec<T>
}

#[derive(Deserialize)]
pub struct Listing<T> {
    kind: String,
    data: ListingData<T>,
}

impl<T> IntoIterator for Listing<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.children.into_iter()
    }
}
