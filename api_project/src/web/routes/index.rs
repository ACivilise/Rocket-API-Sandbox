

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello World")
    )
)]
#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}