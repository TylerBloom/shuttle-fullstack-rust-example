use axum::Router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new();

    #[cfg(not(debug_assertions))]
    let router = assets::inject_ui(router);

    Ok(router.into())
}

#[cfg(not(debug_assertions))]
mod assets {
    use axum::{
        body::{Body, Bytes},
        http::header,
        response::{Html, Response},
        routing::get,
        Router,
    };

    /* Bundle the frontend assets into the binary */
    const INDEX_HTML: &str = include_str!("../../assets/index.html");
    const APP_WASM: &[u8] = include_bytes!("../../assets/template-frontend_bg.wasm");
    const APP_JS: &str = include_str!("../../assets/template-frontend.js");

    pub fn inject_ui(router: Router<()>) -> Router<()> {
        router
            .route("/", get(landing))
            .route("/template-frontend_bg.wasm", get(get_wasm))
            .route("/template-frontend.js", get(get_js))
            .fallback(landing)
    }

    /* Methods to get frontend assets */
    pub async fn landing() -> Html<&'static str> {
        Html(INDEX_HTML)
    }

    pub async fn get_wasm() -> Response<Body> {
        let bytes = Bytes::from_static(APP_WASM);
        let body: Body = bytes.into();

        Response::builder()
            .header(header::CONTENT_TYPE, "application/wasm")
            .body(body)
            .unwrap()
    }

    pub async fn get_js() -> Response<Body> {
        let bytes = Bytes::from_static(APP_JS.as_bytes());
        let body: Body = bytes.into();

        Response::builder()
            .header(header::CONTENT_TYPE, "application/javascript;charset=utf-8")
            .body(body)
            .unwrap()
    }
}
