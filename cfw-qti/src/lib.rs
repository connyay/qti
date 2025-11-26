use worker::*;

mod ai_handlers;
mod ai_html;
mod handlers;
mod html;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |req, ctx| async move {
            handlers::serve_html(req, ctx).await
        })
        .post_async("/generate", |req, ctx| async move {
            handlers::generate_qti(req, ctx).await
        })
        .get_async("/ai", |req, ctx| async move {
            ai_handlers::serve_ai_html(req, ctx).await
        })
        .post_async("/ai/generate", |req, ctx| async move {
            ai_handlers::generate_ai_quiz(req, ctx).await
        })
        .get_async("/health", |req, ctx| async move {
            handlers::health_check(req, ctx).await
        })
        .run(req, env)
        .await
}
