use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::body::HttpBody;

async fn get_body(mut body: Body) -> String {
    let bytes = body.data().await.unwrap().unwrap();
    return String::from_utf8(bytes.to_vec()).unwrap();
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (parts, body) = req.into_parts();

    println!("{} {} {:#?}", parts.method, parts.uri, parts.version);
    for (name, value) in parts.headers {
        match name {
            Some(hname) => println!("{}: {}", hname, value.to_str().unwrap()),
            None => {}
        }
    }

    let bb = get_body(body).await;
    println!("\n{}", bb);
    println!("----------------------------------------------------------------");

    Ok(Response::default())
}

pub async fn run_proxy() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}