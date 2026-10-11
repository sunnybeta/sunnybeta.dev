//! Page types bound to the Askama templates in `templates/`.

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::content::Site;

/// Declare one page template. Every page receives the same context:
/// the shared [`Site`] plus per-page metadata.
macro_rules! page_template {
    ($name:ident, $path:literal) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name {
            /// Everything the templates render.
            pub site: &'static Site,
            /// `<title>` and og:title text.
            pub title: &'static str,
            /// Meta description.
            pub description: &'static str,
            /// Current nav path, for highlighting.
            pub active: &'static str,
        }
    };
}

page_template!(RetroPage, "layouts/retro.html");
page_template!(ProIndexPage, "layouts/pro_index.html");
page_template!(ProPricingPage, "layouts/pro_pricing.html");
page_template!(ProTestimonialsPage, "layouts/pro_testimonials.html");
page_template!(ProBlogsPage, "layouts/pro_blogs.html");
page_template!(ProProjectsPage, "layouts/pro_projects.html");
page_template!(ProExperiencePage, "layouts/pro_experience.html");
page_template!(ProContactPage, "layouts/pro_contact.html");

/// Wrapper that renders an Askama template into an Axum response.
pub struct HtmlTemplate<T>(pub T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("template error: {err}"),
            )
                .into_response(),
        }
    }
}
