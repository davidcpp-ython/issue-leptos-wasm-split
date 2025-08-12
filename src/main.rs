#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::State,
        http::Request,
        response::IntoResponse,
        routing::get,
        Router,
    };
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_lazy_routes_issue::App;

    async fn leptos_routes_handler(
        State(_leptos_options): State<LeptosOptions>,
        request: Request<AxumBody>,
    ) -> axum::response::Response {
        let handler = leptos_axum::render_app_async_with_context(
            move || {},
            move || view! { <App/> },
        );

        handler(request).await.into_response()
    }

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}