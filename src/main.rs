use std::{convert::Infallible, str::FromStr, vec};

use futures::{Future, StreamExt, TryStreamExt};
use k8s_openapi::{api::core::v1::Pod, List};
use kube::{
    api::{Api, ListParams, PostParams, ResourceExt},
    Client,
};
use warp::{http::StatusCode, Filter, Rejection};

async fn get_pods() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let client = Client::try_default().await.ok().unwrap();
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let pods: Api<Pod> = Api::all(client);
    let ol_pods = pods.list(&ListParams::default()).await.ok().unwrap();
    let mut vec_names = Vec::new();
    for p in ol_pods {
        vec_names.push(p.name_any());
    }
    if vec_names.len() > 0 {
        Ok(Box::new(vec_names.join("\n")))
    } else {
        Ok(Box::new(StatusCode::BAD_REQUEST))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ready = warp::path!("ready").map(|| StatusCode::OK);
    let live = warp::path!("live").map(|| StatusCode::OK);
    let pods = warp::path!("pods").and_then(get_pods);
    let api = warp::get().and(ready.or(live).or(pods));
    warp::serve(api).run(([0, 0, 0, 0], 8080)).await;
    // Infer the runtime environment and try to create a Kubernetes Client

    Ok(())
}
