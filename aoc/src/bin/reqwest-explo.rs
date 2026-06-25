use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Shared types

#[derive(Debug, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Debug, Serialize)]
struct NewPost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

fn main() {
    step1_simple_get();
    step2_status_and_headers();
    step3_query_params();
    step4_post_json();
    step5_timeout_and_errors();
    step6_reusable_client();

    // Part 2 — Async
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        step7_simple_get_async().await;
        step8_concurrent_requests().await;
    });
}

//Part 1 — Blocking Request

/// 1. Simple GET : read the body as plain text.
fn step1_simple_get() {
    println!("\n 1. Simple GET");

    let body = reqwest::blocking::get(format!("{BASE_URL}/posts/1"))
        .expect("request failed")
        .text()
        .expect("failed to read body");

    println!("{body}");
}

/// 2. GET + inspect status code & response headers.
fn step2_status_and_headers() {
    println!("\n 2. Status & Headers");

    let response = reqwest::blocking::get(format!("{BASE_URL}/posts/1")).expect("request failed");

    println!("Status : {}", response.status());
    println!(
        "Content-Type : {:?}",
        response.headers().get("content-type")
    );
}

/// 3. GET with query parameters
fn step3_query_params() {
    println!("\n 3. Query Params");

    let response =
        reqwest::blocking::get(format!("{BASE_URL}/posts?userId=1")).expect("request failed");

    let posts: Vec<Post> = response.json().expect("failed to deserialize");
    println!("Posts for userId=1 : {} results", posts.len());
    for post in &posts {
        println!("  [{}] {}", post.id, post.title);
    }
}

/// 4. POST with a JSON body + deserialize the JSON response.
fn step4_post_json() {
    println!("\n 4. POST JSON");

    let new_post = NewPost {
        title: "Hello reqwest".to_string(),
        body: "Exploring the reqwest crate".to_string(),
        user_id: 1,
    };

    let created: Post = reqwest::blocking::Client::new()
        .post(format!("{BASE_URL}/posts"))
        .json(&new_post)
        .send()
        .expect("request failed")
        .json()
        .expect("failed to deserialize");

    println!(
        "Created post with id={} title='{}'",
        created.id, created.title
    );
}

/// 5. Timeout + error handling — what happens when things go wrong.
fn step5_timeout_and_errors() {
    println!("\n5. Timeout & Errors");

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(1))
        .build()
        .expect("failed to build client");

    match client.get(format!("{BASE_URL}/posts/1")).send() {
        Ok(resp) => println!("Got response: {}", resp.status()),
        Err(e) if e.is_timeout() => println!("Timed out (expected): {e}"),
        Err(e) => println!("Other error: {e}"),
    }

    // error_for_status() turns a 4xx/5xx into an Err
    let resp = reqwest::blocking::get(format!("{BASE_URL}/posts/99999")).expect("request failed");

    println!("\nStatus for missing resource : {}", resp.status());
    match resp.error_for_status() {
        Ok(_) => println!("2xx — success"),
        Err(e) => println!("error_for_status() caught : {e}"),
    }
}

/// 6. Reusable Client: build once, share across calls
fn step6_reusable_client() {
    println!("\n 6. Reusable Client");
    let client = Client::new();

    for id in 1..=3 {
        let post: Post = client
            .get(format!("{BASE_URL}/posts/{id}"))
            .send()
            .expect("request failed")
            .json()
            .expect("failed to deserialize");

        println!("Post {id}: {}", post.title);
    }
}

// Part 2 — Async

/// 7. Same simple GET but async — notice the only differences are:
///    async fn, .await, and using reqwest::get instead of reqwest::blocking::get.
async fn step7_simple_get_async() {
    println!("\n 7. Simple GET (async)");

    let body = reqwest::get(format!("{BASE_URL}/posts/1"))
        .await
        .expect("request failed")
        .text()
        .await
        .expect("failed to read body");

    println!("{body}");
}

/// 8. Concurrent requests with tokio::join!.
async fn step8_concurrent_requests() {
    println!("\n 8. Concurrent Requests");

    let client = reqwest::Client::new();

    let (r1, r2, r3) = tokio::join!(
        client.get(format!("{BASE_URL}/posts/1")).send(),
        client.get(format!("{BASE_URL}/posts/2")).send(),
        client.get(format!("{BASE_URL}/posts/3")).send(),
    );

    for (i, result) in [r1, r2, r3].into_iter().enumerate() {
        let post: Post = result
            .expect("request failed")
            .json()
            .await
            .expect("failed to deserialize");
        println!("Post {}: {}", i + 1, post.title);
    }
}

/// 9. Concurrent requests with tokio::spwan.
async fn step9_concurrent_requests() {
    println!("\n 8. Concurrent Requests");

    let client = reqwest::Client::new();

    let handle1 = tokio::spawn(client.get(format!("{BASE_URL}/posts/1")).send());
    let handle2 = tokio::spawn(client.get(format!("{BASE_URL}/posts/2")).send());
    let handle3 = tokio::spawn(client.get(format!("{BASE_URL}/posts/3")).send());

    for (i, result) in [
        handle1.await.unwrap(),
        handle2.await.unwrap(),
        handle3.await.unwrap(),
    ]
    .into_iter()
    .enumerate()
    {
        let post: Post = result
            .expect("request failed")
            .json()
            .await
            .expect("failed to deserialize");
        println!("Post {}: {}", i + 1, post.title);
    }
}
