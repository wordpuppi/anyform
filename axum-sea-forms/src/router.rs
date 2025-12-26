//! FormsRouter for easy Axum integration.

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;

use crate::handlers;

/// A pre-configured router for form routes.
///
/// # Example
///
/// ```rust,ignore
/// use axum::Router;
/// use axum_sea_forms::FormsRouter;
///
/// let app = Router::new()
///     .merge(FormsRouter::new(db.clone()));
///
/// // Routes available:
/// // GET  /forms/{slug}         - Render form HTML
/// // GET  /forms/{slug}/json    - Get form schema JSON
/// // POST /forms/{slug}         - Submit form (JSON response)
/// // POST /forms/{slug}/submit  - Submit form (redirect)
/// // GET  /forms/{slug}/success - Success page
/// ```
pub struct FormsRouter;

impl FormsRouter {
    /// Creates a new forms router with default routes.
    #[must_use]
    pub fn new(db: DatabaseConnection) -> Router {
        Self::builder().database(db).build()
    }

    /// Creates a builder for customizing the router.
    #[must_use]
    pub fn builder() -> FormsRouterBuilder {
        FormsRouterBuilder::default()
    }
}

/// Builder for customizing the FormsRouter.
#[derive(Default)]
pub struct FormsRouterBuilder {
    db: Option<DatabaseConnection>,
    enable_html: bool,
    enable_json: bool,
    enable_submit: bool,
    enable_success: bool,
    #[cfg(feature = "admin")]
    enable_admin: bool,
}

impl FormsRouterBuilder {
    /// Sets the database connection.
    #[must_use]
    pub fn database(mut self, db: DatabaseConnection) -> Self {
        self.db = Some(db);
        self
    }

    /// Enables HTML rendering routes (default: true when building).
    #[must_use]
    pub fn enable_html(mut self, enable: bool) -> Self {
        self.enable_html = enable;
        self
    }

    /// Enables JSON schema routes (default: true when building).
    #[must_use]
    pub fn enable_json(mut self, enable: bool) -> Self {
        self.enable_json = enable;
        self
    }

    /// Enables form submission routes (default: true when building).
    #[must_use]
    pub fn enable_submit(mut self, enable: bool) -> Self {
        self.enable_submit = enable;
        self
    }

    /// Enables success page route (default: true when building).
    #[must_use]
    pub fn enable_success(mut self, enable: bool) -> Self {
        self.enable_success = enable;
        self
    }

    /// Enables admin routes (default: false).
    #[cfg(feature = "admin")]
    #[must_use]
    pub fn enable_admin(mut self, enable: bool) -> Self {
        self.enable_admin = enable;
        self
    }

    /// Builds the router.
    ///
    /// # Panics
    ///
    /// Panics if no database connection was provided.
    #[must_use]
    pub fn build(self) -> Router {
        // Check explicitly set flags before consuming db
        let any_set = self.any_explicitly_set();
        let enable_html = self.enable_html || !any_set;
        let enable_json = self.enable_json || !any_set;
        let enable_submit = self.enable_submit || !any_set;
        let enable_success = self.enable_success || !any_set;

        let db = self
            .db
            .expect("Database connection is required. Call .database(db) before .build()");

        let mut router = Router::new();

        if enable_html {
            router = router.route("/forms/{slug}", get(handlers::get_form_html));
        }

        if enable_json {
            router = router.route("/forms/{slug}/json", get(handlers::get_form_json));
        }

        if enable_submit {
            router = router
                .route("/forms/{slug}", post(handlers::submit_form))
                .route("/forms/{slug}/submit", post(handlers::submit_form_redirect));
        }

        if enable_success {
            router = router.route("/forms/{slug}/success", get(handlers::form_success));
        }

        #[cfg(feature = "admin")]
        if self.enable_admin {
            router = router
                .route("/admin/forms", get(handlers::list_forms))
                .route("/admin/forms", post(handlers::create_form))
                .route("/admin/forms/sync", post(handlers::sync_forms))
                .route("/admin/forms/{id}", get(handlers::get_form_by_id))
                .route("/admin/forms/{id}", put(handlers::update_form))
                .route("/admin/forms/{id}", delete(handlers::delete_form))
                .route(
                    "/admin/forms/{id}/submissions",
                    get(handlers::list_submissions),
                )
                .route(
                    "/admin/forms/{form_id}/submissions/{sub_id}",
                    get(handlers::get_submission),
                )
                .route(
                    "/admin/forms/{form_id}/submissions/{sub_id}",
                    delete(handlers::delete_submission),
                );
        }

        router.with_state(db)
    }

    fn any_explicitly_set(&self) -> bool {
        self.enable_html || self.enable_json || self.enable_submit || self.enable_success
    }
}
