use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        add_subscription_handler, create_note_handler, create_user_handler, delete_note_handler, delete_user_handler, edit_note_handler, edit_user_handler, get_note_handler, get_user_handler, health_checker_handler, list_subscription_handler, list_user_handler, note_list_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthchecker", get(health_checker_handler))
        .route("/notes/", post(create_note_handler))
        .route("/notes", get(note_list_handler))
        .route(
            "/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .route("/user", get(list_user_handler)
                .post(create_user_handler)
        )
        .route(
            "/user/:id",
            get(get_user_handler)
                .patch(edit_user_handler)
                .delete(delete_user_handler),
        )
        .route(
            "/subscription",
            get(list_subscription_handler)
                .post(add_subscription_handler)
        )

        .with_state(app_state)
}
