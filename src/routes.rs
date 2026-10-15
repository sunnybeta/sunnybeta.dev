//! HTTP routes: every page is rendered from an Askama template at request
//! time. Hand-written assets (`static/css/`, `static/img/`, `static/files/`
//! and `static/js/`) are served straight off the repo root — nothing is
//! copied anywhere.

use std::path::{Path, PathBuf};

use axum::{http::StatusCode, routing::get, Router};
use tower_http::services::ServeDir;

use crate::content::site;
use crate::templates::{
    HtmlTemplate, ProBlogsPage, ProContactPage, ProExperiencePage, ProIndexPage, ProPricingPage,
    ProProjectsPage, ProTestimonialsPage, RetroPage,
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
        .route("/", get(pro_index))
        .route("/pro", get(pro_index))
        .route("/retro", get(retro))
        .route("/pricing", get(pro_pricing))
        .route("/testimonials", get(pro_testimonials))
        .route("/blogs", get(pro_blogs))
        .route("/projects", get(pro_projects))
        .route("/experience", get(pro_experience))
        .route("/contact", get(pro_contact))
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

async fn pro_index() -> HtmlTemplate<ProIndexPage> {
    HtmlTemplate(ProIndexPage {
        site: site(),
        title: "Home",
        description: "AI Engineer & Freelancer — Dubai, UAE",
        active: "/",
    })
}

async fn pro_pricing() -> HtmlTemplate<ProPricingPage> {
    HtmlTemplate(ProPricingPage {
        site: site(),
        title: "Pricing",
        description: "Fixed prices for websites, apps and AI builds.",
        active: "/pricing",
    })
}

async fn pro_testimonials() -> HtmlTemplate<ProTestimonialsPage> {
    HtmlTemplate(ProTestimonialsPage {
        site: site(),
        title: "Testimonials",
        description: "What clients say about working with me.",
        active: "/testimonials",
    })
}

async fn pro_blogs() -> HtmlTemplate<ProBlogsPage> {
    HtmlTemplate(ProBlogsPage {
        site: site(),
        title: "Blogs",
        description: "Notes on engineering, AI and freelancing.",
        active: "/blogs",
    })
}

async fn pro_projects() -> HtmlTemplate<ProProjectsPage> {
    HtmlTemplate(ProProjectsPage {
        site: site(),
        title: "Projects",
        description: "Selected work — websites, apps and games.",
        active: "/projects",
    })
}

async fn pro_experience() -> HtmlTemplate<ProExperiencePage> {
    HtmlTemplate(ProExperiencePage {
        site: site(),
        title: "Experience",
        description: "Where I have worked and what I built.",
        active: "/experience",
    })
}

async fn pro_contact() -> HtmlTemplate<ProContactPage> {
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
