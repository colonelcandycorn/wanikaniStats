# Wanikani Stats

## Creator: Sarah Dylan

## Project Description

Wanikani is a website that is meant to aid in Japanese language
learning, specifically learning Kanji and related vocabulary. It is
the type of tool that requires dedicating some amount of time to every day
in order to make progress. Given the level of persistence this requires,
sometimes it is nice to just be able to look at some summary data to see how
far you've come. This is somewhat available on the website, but you kind of have
to do the math yourself. I wanted part of this just have a nice summary dashboard of
the progress you've made so far. Secondly, you probably want to get a sense of what percentage
of the time you are getting your reviews correct, meaning the number of times you successfully
remembered the item. This can help you gauge if you're trying to learn too many new items
each day or going too slow.

To accomplish my goals, I wanted to start by creating a website modeled after [wkstats](wkstats.com). This website will give you summary stats for Kanji, Vocabulary, and Radicals. I noticed Kana vocabulary is missing from the stats, and I am assuming this site was probably created prior to their addition. It also gives some general account information.

To create the website, my main tools were: reqwest, axum, and minijinja. Reqwest seems to be the main Rust crate used for making API requests. For this purpose, I found that it worked great. In conjunction with serde, I was easily able to set up a bunch of structs with names similar to the API response JSON fields and deserialization was handled pretty much for me. I created several utility functions to deal with the fact the Wanikani API uses paginated data in their responses. Axum is a web application framework that I used to handle all of the routing logic. Minijinja is a crate that lets me use Jinja templates. 

The website starts by serving a login form that accepts a token. If the token is valid it routes to /info where an ApiClient is created and used to obtain a CompleteUserInfo struct. This info struct is then used to create the context for the info template.

For testing, a lot of the actual logic of the project is handled by Rust type system. A lot of what could go wrong in this project is either not receiving data from user or API or not properly deserializing the data from API. Most of the work is done in ApiClient, but I was unsure how much it needed to be tested as much of the test I could think of writing would just be testing other crates I'm using. Like testing that I'm properly deserializing a response from the API is basically just proving that serde does what it says it does. I also spent a lot of time trying to set up mocking, but it was such a nightmare that I put a pause on it. There is some light testing for CompleteUserInfo because some math is used to generate the stats.

## Build Instructions

### Prerequisites
Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (with `cargo`)

### Clone the Repository
```sh
git clone https://github.com/colonelcandycorn/wanikaniStats.git
cd wanikaniStats
```

### Build the Project
Run the following command to build the project:
```sh
cargo build --release
```

### Run the Server
Start the web server with:
```sh
cargo run
```

## Example

![A screenshot of the login screen](<static/login_page_screenshot.png>)

When first navigating to the url, you will be directed to /login, which will accept an API token. After entering it, if it is correct, you will be redirected to the info page. This will give you a 'uuid' token which if you want to get rid you will currently have to manually reset the server or delete it from your browser

![A screenshot of the info page](<static/info_page_screenshot.png>)

## Work to be done and current shortcomings

* Testing might be lacking, but as this is a relatively static website
there isn't a ton of logic to test. (async testing is also hard)

* There is zero styling

* No logout button

* You don't really need an account so I think just storing the token is fine, but I think maybe I should use a moka cache as well for tokens and give them an expiry date.

* I'm currently getting information for every subject and the user's current performance on each subject, but not presenting it in a non-aggregated way. I'd like to create an infinite scroll of all the items the user has learned with various stats for each item. I'd also like a filter and search bar.

* I'd like to get more information to present about how long the user is taking to level up

* More information on the percentage completeness of the user's current level.

## What Worked and What Didn't

Overall, I was surprised at how intuitive reqwest and axum were. When interacting with just those libraries, I rarely felt like I was dealing with a 'difficult' language and wouldn't describe it as much different from creating a flask app. 

I don't, however, feel like I completely understood how all of libraries fit together. For example, I know I wanted an in memory cache and rate limiter. I think I used moka correctly, but it seemed a little overkill for this project. The rate limiter seemed to work, but I was unsure if I really needed to pass the limiter to the ApiClient struct.

I created mocks when testing frontend websites, and I guess I didn't appreciate the difficulty in accomplishing the same task in a statically typed language. Also, I realize I am not as confident in my understanding of Async rust. 


## Getting an API token

To obtain a Wanikani token, you need to first have an account. If you don't you can create one
at the [sign up page](https://www.wanikani.com/signup). Then you need to access your settings(click
on your account picture in top right) or navigate to this [personal access token](https://www.wanikani.com/settings/personal_access_tokens) link.
If you are concerned at all about giving out access to your account you can create a read only token and delete when you are done using 
the site.

