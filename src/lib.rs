use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Element, window, Request, RequestInit, Response, MouseEvent};
use std::sync::Arc;

pub struct PostSummary {
    id: String,
    title: String,
}

/// Get a web_sys reference to a DOM element by ID.
/// Returns None if there was no reference found
fn get_element_by_id(element_id: &str) -> Option<Element> {
    window()?.document()?.get_element_by_id(element_id) 
}

/// Get a vec of post keys and titles
fn get_post_summaries() -> Vec<PostSummary> {
    vec![
        PostSummary { 
            id: String::from("hello"),
            title: String::from("Hello, World"),
         },
        PostSummary {
            id: String::from("two"),
            title: String::from("Post Two"),
        }
    ]
}

/// Get the text of a markdown post given its blog id.
/// Uses fetch to fetch `/markdown/{post_id}.md` and return
/// the text result, which will later be processed
async fn get_one_post(post_id: &str) -> Option<String> {
    
    // Create the fetch request Options
    let mut opts = RequestInit::new();
    opts.method("GET");
    
    // Create the fetch request URL
    let url = format!("/markdown/{post_id}.md");
    
    // Create the request
    let request = Request::new_with_str_and_init(&url, &opts).ok()?;
    
    // Get the promise from the fetch request
    let response_promise = window()?.fetch_with_request(&request);
    
    // Await the promise as a future
    let response = JsFuture::from(response_promise).await.ok()?;
    
    // Convert the response in to a response object
    let response: Response = response.dyn_into().ok()?;

    // Get the promise with the text content
    let text = response.text().ok()?;
    
    // Await the promise as a future
    let text = JsFuture::from(text).await.ok()?;
    
    // Convert the JsValue to a rust string
    text.as_string()
}

fn create_post_list_element(post: &PostSummary, post_element_id: &str) -> Option<Element> {
    // Create element
    let element = window()?.document()?.create_element("li").ok()?;
    
    // Set the text to the post title
    element.set_text_content(Some(&post.title.clone()));
    
    let post_element_id_clone = Arc::new(String::from(post_element_id));
    let post_id_clone = Arc::new(post.id.clone());
    
    let closure = Closure::<dyn Fn()>::new(move || {
        let post_1 = post_element_id_clone.clone();
        let post_2 = post_id_clone.clone();
        spawn_local(async move { 
            render_post(&post_1, &post_2).await; 
        });
    });

    // Add an event listener
    element.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).ok()?;
    
    closure.forget();
    
    // Return the element
    Some(element)
}

/// Render app
#[wasm_bindgen]
pub async fn render(list_element_id: &str, post_element_id: &str, initial_post: &str) -> Result<(), JsValue> {

    // Render the list
    render_posts_list(list_element_id, post_element_id)?;
    
    // Render the initial post
    render_post(post_element_id, initial_post).await?;
    
    Ok(())

}

/// Render a list of all posts to the element with ID `element_id`
fn render_posts_list(list_element_id: &str, post_element_id: &str) -> Result<(), JsValue> {

    // Get the element to render to
    let element = get_element_by_id(list_element_id)
        .expect("No matching element");
    
    // Get the list of posts
    let posts: Vec<Element> = get_post_summaries()
        .iter()
        .filter_map(|post| create_post_list_element(post, post_element_id))
        .collect();
    
    for post in posts {
        element.append_child(&post)?;
    }
    
    Ok(())
}

/// Render a specific post with ID `post_id` 
/// to the element with ID `element_id`
async fn render_post(element_id: &str, post_id: &str)  -> Result<(), JsValue> {

    // Get the element to render to    
    let element = get_element_by_id(element_id)
        .expect("No matching element");

    // Get the post
    let post = get_one_post(post_id).await
        .expect("Could not retrieve that post");
    
    // Write the blog post to the element
    element.set_text_content(Some(&post));
    
    // Return without throwing an error
    Ok(())
    
}