//! HTTP routes: every page is rendered from an Askama template at request
//! time. Hand-written assets (`static/css/`, `static/img/`, `static/files/`
//! and `static/js/`) are served straight off the repo root — nothing is
//! copied anywhere.

use std::path::{Path, PathBuf};

use axum::{
    extract::Path as AxumPath,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

use crate::content::{post_by_slug, site};
use crate::templates::{
    BlogPostPage, HtmlTemplate, ProBlogsPage, ProContactPage, ProExperiencePage, ProIndexPage,
    ProPricingPage, ProProjectsPage, ProTestimonialsPage, RetroPage,
};

/// Locate the `static/` directory. Prefer one beside the working directory (a
/// deployed bundle), otherwise fall back to the crate directory so the server
/// works no matter where it is launched from. `STATIC_DIR` overrides both.
fn static_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("STATIC_DIR") {
        return PathBuf::from(dir);
    }
    let local = Path::new("static");
    if local.is_dir() {
        return local.to_path_buf();
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("static")
}

/// Build the application router.
pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/retro", get(retro))
        .route("/pricing", get(pricing))
        .route("/testimonials", get(testimonials))
        .route("/blogs", get(blogs))
        .route("/blogs/{slug}", get(blog_post))
        .route("/projects", get(projects))
        .route("/experience", get(experience))
        .route("/contact", get(contact))
        .nest_service("/static", ServeDir::new(static_dir()))
        .fallback(not_found)
}

async fn retro() -> HtmlTemplate<RetroPage> {
    HtmlTemplate(RetroPage {
        site: site(),
        title: "Sunny Béta",
        description: "Independent AI engineer and freelancer.",
        active: "",
    })
}

async fn index() -> HtmlTemplate<ProIndexPage> {
    HtmlTemplate(ProIndexPage {
        site: site(),
        title: "Home",
        description: "AI Engineer & Freelancer — Dubai, UAE",
        active: "/",
    })
}

async fn pricing() -> HtmlTemplate<ProPricingPage> {
    HtmlTemplate(ProPricingPage {
        site: site(),
        title: "Pricing",
        description: "Fixed prices for websites, apps and AI builds.",
        active: "/pricing",
    })
}

async fn testimonials() -> HtmlTemplate<ProTestimonialsPage> {
    HtmlTemplate(ProTestimonialsPage {
        site: site(),
        title: "Testimonials",
        description: "What clients say about working with me.",
        active: "/testimonials",
    })
}

async fn blogs() -> HtmlTemplate<ProBlogsPage> {
    HtmlTemplate(ProBlogsPage {
        site: site(),
        title: "Blogs",
        description: "Notes on engineering, AI and freelancing.",
        active: "/blogs",
    })
}

/// One published article, looked up by the slug at the end of the URL.
async fn blog_post(AxumPath(slug): AxumPath<String>) -> Response {
    match post_by_slug(&slug) {
        Some(post) => HtmlTemplate(BlogPostPage {
            site: site(),
            post,
            title: post.title,
            description: post.summary,
            active: "/blogs",
        })
        .into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

async fn projects() -> HtmlTemplate<ProProjectsPage> {
    HtmlTemplate(ProProjectsPage {
        site: site(),
        title: "Projects",
        description: "Selected work — websites, apps and games.",
        active: "/projects",
    })
}

async fn experience() -> HtmlTemplate<ProExperiencePage> {
    HtmlTemplate(ProExperiencePage {
        site: site(),
        title: "Experience",
        description: "Where I have worked and what I built.",
        active: "/experience",
    })
}

async fn contact() -> HtmlTemplate<ProContactPage> {
    HtmlTemplate(ProContactPage {
        site: site(),
        title: "Contact Me",
        description: "Email and PGP key for encrypted communication.",
        active: "/contact",
    })
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}
