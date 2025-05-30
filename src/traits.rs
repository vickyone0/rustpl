

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub loction: String,
    pub author: String,
}


pub trait Summary {
    fn summarize(&self) -> String;

}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.loction)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}