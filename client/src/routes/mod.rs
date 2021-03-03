pub mod about;
pub mod blog;
pub mod home;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to="/about"]
    About,
    #[to="/blog/{id}"]
    BlogPage(String),
    #[to="/blog"]
    Blog,
    #[to="/"]
    Home
}