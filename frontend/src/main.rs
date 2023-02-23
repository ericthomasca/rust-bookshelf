use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct Post {
    id: String,
    title: String,
    body: String,
    published: bool
}

#[derive(Properties, PartialEq)]
struct PostListProps {
    posts: Vec<Post>,
    on_click: Callback<Post>
}

#[derive(Properties, PartialEq)]
struct PostDetailsProps {
    post: Post
}

#[function_component(PostList)]

#[function_component(App)]
fn app() -> Html {
    let posts = use_state(Vec::new);
    {
        let posts = posts;
        use_effect_with_deps(move |_| {
            let posts = posts;
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<Post> = Request::get("http://localhost:8088/api/posts")
                    .send()
                    .await
                    .expect("Data not received")
                    .json()
                    .await
                    .expect("Problem with data received");
                posts.set(fetched_posts);
            });
            || ()
        }, ());
    }

    html! {
        <>
            <h1>{"Posts"}</h1>
            <div>
                <h3>{"Published Posts"}</h3>
                
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
