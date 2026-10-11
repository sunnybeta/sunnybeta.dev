//! Site content. Everything the templates render lives here as data — edit
//! this file to change the site; the templates never hardcode copy, names or
//! links.

use std::sync::LazyLock;

// Identity -----------------------------------------------------------

pub struct Person {
    /// Sunny Béta
    pub name: &'static str,
    /// Sunny
    pub first_name: &'static str,
    /// sunnybeta
    pub handle: &'static str,
    /// sunnybeta.dev
    pub domain: &'static str,
    /// sunnybeta@protonmail.com
    pub email: &'static str,
    /// Dubai, UAE
    pub location: &'static str,
    /// Dubai
    pub location_short: &'static str,
    /// AI Engineer & Freelancer
    pub role: &'static str,
    /// short line under the name on the home page
    pub headline: &'static str,
    /// two or three sentence introduction
    pub intro: &'static str,
    /// e.g. "Available for new projects"
    pub availability: &'static str,
    /// e.g. "7+"
    pub years_coding: &'static str,
    pub github_url: &'static str,
    pub linkedin_url: &'static str,
    pub instagram_url: &'static str,
}

/// Services offered, used on the retro page and the pro home page.
pub struct Service {
    pub title: &'static str,
    pub body: &'static str,
    pub tag: &'static str,
    /// short label used as a retro glyph
    pub icon: &'static str,
}

pub struct Fact {
    pub value: &'static str,
    pub label: &'static str,
}

// Pricing ------------------------------------------------------------

pub struct Tier {
    pub name: &'static str,
    pub tag: &'static str,
    pub price: &'static str,
    pub period: &'static str,
    pub summary: &'static str,
    pub features: Vec<&'static str>,
    pub popular: bool,
    pub cta: &'static str,
}

pub struct AddOn {
    pub name: &'static str,
    pub price: &'static str,
    pub summary: &'static str,
}

// Content blocks -----------------------------------------------------

pub struct Project {
    pub name: &'static str,
    pub client: &'static str,
    pub year: &'static str,
    pub kind: &'static str,
    pub summary: &'static str,
    pub tags: Vec<&'static str>,
    pub url: &'static str,
    pub featured: bool,
}

pub struct Testimonial {
    pub quote: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub company: &'static str,
}

pub struct Post {
    pub title: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub url: &'static str,
    pub tags: Vec<&'static str>,
}

pub struct Role {
    pub org: &'static str,
    pub title: &'static str,
    pub period: &'static str,
    pub summary: &'static str,
    pub tags: Vec<&'static str>,
}

pub struct SkillGroup {
    pub name: &'static str,
    pub items: Vec<&'static str>,
}

// Retro site ----------------------------------------------------------

pub struct RetroStatus {
    pub status: Vec<&'static str>,
    pub listening: Vec<&'static str>,
    pub mood: Vec<&'static str>,
    pub build: &'static str,
    pub guestbook: &'static str,
    pub webring: Vec<&'static str>,
    pub badges: Vec<&'static str>,
    pub visitor_id: &'static str,

    /* navbar random details */
    pub last_updated: Vec<&'static str>,
    pub quest: Vec<&'static str>,
    pub built_with: Vec<&'static str>,
    pub fortune: Vec<&'static str>,

    /* random decorations */
    pub word_art: Vec<&'static str>,
    pub captions: Vec<&'static str>,
    pub awards: Vec<&'static str>,
    pub stickers: Vec<&'static str>,
    pub ticker: Vec<&'static str>,

    /* sidebar widgets */
    pub now_playing: Vec<&'static str>,
    pub guestbook_entries: Vec<&'static str>,
}

/// Site bundles everything handed to the templates.
pub struct Site {
    pub person: Person,
    pub year: &'static str,
    pub nav: Vec<NavItem>,
    pub retro_nav: Vec<NavItem>,
    pub facts: Vec<Fact>,
    pub services: Vec<Service>,
    pub skills: Vec<SkillGroup>,
    pub tiers: Vec<Tier>,
    pub add_ons: Vec<AddOn>,
    pub projects: Vec<Project>,
    pub testimonials: Vec<Testimonial>,
    pub posts: Vec<Post>,
    pub roles: Vec<Role>,
    pub retro: RetroStatus,
}

/// NavItem is a single header/anchor link.
pub struct NavItem {
    pub label: &'static str,
    pub path: &'static str,
}

/* =============================================================================
SITE CONTENT
============================================================================= */

static SITE: LazyLock<Site> = LazyLock::new(|| {
    let person = Person {
        name: "Sunny Béta",
        first_name: "Sunny",
        handle: "sunnybeta",
        domain: "sunnybeta.dev",
        email: "sunnybeta@protonmail.com",
        location: "Dubai, UAE",
        location_short: "Dubai",
        role: "AI Engineer & Freelancer",
        headline: "I build websites, apps and AI tooling that people actually enjoy using.",
        intro: "Placeholder intro — two or three sentences about who you are, what you build and who you build it for. Replace this in src/content.rs.",
        availability: "Available for new projects",
        years_coding: "7+",
        github_url: "https://github.com/sunnybeta",
        linkedin_url: "https://www.linkedin.com/in/sunnybeta",
        instagram_url: "https://www.instagram.com/sunnybeta",
    };

    let facts = vec![
        Fact {
            value: person.years_coding,
            label: "Years coding",
        },
        Fact {
            value: "20+",
            label: "Projects shipped",
        },
        Fact {
            value: "3",
            label: "Years freelancing",
        },
        Fact {
            value: "24h",
            label: "Reply time",
        },
    ];

    let services = vec![
        Service {
            title: "Websites",
            body: "Fast, hand-built websites with a CMS your team can actually use. No page-builder bloat.",
            tag: "WEB",
            icon: "@",
        },
        Service {
            title: "Apps",
            body: "Dashboards, portals and SaaS products — auth, data model, admin panel, tests.",
            tag: "APP",
            icon: "</>",
        },
        Service {
            title: "AI & automation",
            body: "Retrieval, agents and LLM features wired into your stack with evals and cost control.",
            tag: "AI",
            icon: "*ai*",
        },
        Service {
            title: "Hosting & domains",
            body: "Domain registration, hosting and deploys set up so nothing breaks at 3am.",
            tag: "OPS",
            icon: "###",
        },
    ];

    let skills = vec![
        SkillGroup {
            name: "Frontend",
            items: vec![
                "HTML",
                "CSS",
                "JavaScript",
                "TypeScript",
                "React",
                "Next.js",
                "Vite",
            ],
        },
        SkillGroup {
            name: "Backend",
            items: vec!["Java", "Spring", "Node.js", "Express", "Python", "Rust"],
        },
        SkillGroup {
            name: "Data & infra",
            items: vec!["PostgreSQL", "MariaDB", "KeyDB", "Docker", "MinIO", "Git"],
        },
        SkillGroup {
            name: "AI",
            items: vec!["LLM APIs", "RAG", "Evals", "Prompting", "Agents"],
        },
    ];

    let tiers = vec![
        Tier {
            name: "One-page Website",
            tag: "STARTER",
            price: "75",
            period: "USD / project",
            summary:
                "A clean single page for a personal project, small business or a quick launch.",
            features: vec![
                "Single page, fully responsive",
                "Basic SEO setup",
                "Contact form",
                "Content editable via Markdown",
                "Two weeks delivery",
            ],
            popular: false,
            cta: "Request this package",
        },
        Tier {
            name: "Business Website",
            tag: "POPULAR",
            price: "230",
            period: "USD / project",
            summary:
                "A multi-page site for businesses that need to present real services and proof.",
            features: vec![
                "Everything from One-page",
                "Up to 10 subpages",
                "Blog or news section",
                "Editable content via Markdown",
                "SEO optimisation",
            ],
            popular: true,
            cta: "Request this package",
        },
        Tier {
            name: "Professional Website",
            tag: "SCALE",
            price: "1 200",
            period: "USD / project",
            summary: "A complete site with its own CMS for companies and larger projects.",
            features: vec![
                "Everything from Business",
                "Full CMS system",
                "Unlimited subpages",
                "Professional SEO",
                "Training included",
            ],
            popular: false,
            cta: "Request this package",
        },
        Tier {
            name: "Web App",
            tag: "SYSTEMS",
            price: "2 600",
            period: "USD / project",
            summary: "A real product: authentication, database, admin panel and integrations.",
            features: vec![
                "Login and role system",
                "Database and API design",
                "Admin panel included",
                "Third-party integrations",
                "Tests on critical paths",
            ],
            popular: false,
            cta: "Request this package",
        },
        Tier {
            name: "AI Build",
            tag: "NEW",
            price: "1 600",
            period: "USD / project",
            summary: "Retrieval, agents or LLM features added to a product you already have.",
            features: vec![
                "Retrieval over your documents",
                "Streaming UI with citations",
                "Prompt and eval harness",
                "Cost and latency monitoring",
                "Handed over with docs",
            ],
            popular: false,
            cta: "Request this package",
        },
        Tier {
            name: "End to End",
            tag: "FULL",
            price: "Custom",
            period: "scoped together",
            summary: "Idea to launch: design, build, infrastructure, deployment and handover.",
            features: vec![
                "Discovery and scoping",
                "Design system and UI",
                "Frontend, backend and infra",
                "CI/CD and monitoring",
                "Documentation and handover",
            ],
            popular: false,
            cta: "Let's scope it",
        },
    ];

    let add_ons = vec![
        AddOn {
            name: "Hosting and domain setup",
            price: "from 75 USD",
            summary: "Domain registration and reliable hosting arranged for your site. Hosting and domain costs are not included.",
        },
        AddOn {
            name: "Website management training",
            price: "from 45 USD",
            summary: "A walkthrough so you can edit your own content without breaking the layout.",
        },
        AddOn {
            name: "Maintenance — 2 weeks",
            price: "Free",
            summary: "After launch I stay available for two weeks to fix anything that surfaces.",
        },
        AddOn {
            name: "Ongoing maintenance",
            price: "45 USD / month",
            summary: "Regular checks, updates and hosting management.",
        },
    ];

    let projects = vec![
        Project {
            name: "Baileys krmivo",
            client: "Baileys Horse Feeds",
            year: "2026",
            kind: "Website",
            summary: "E-commerce website for an exclusive horse feed importer, built on Next.js and Spring Boot.",
            tags: vec!["Next.js", "Spring Boot", "TypeScript", "MinIO", "Docker"],
            url: "",
            featured: true,
        },
        Project {
            name: "Axiom English Academy",
            client: "Axiom",
            year: "2025",
            kind: "Web app",
            summary: "Full-stack web app with a custom CMS, student accounts and payments.",
            tags: vec!["Next.js", "Spring Boot", "TypeScript", "KeyDB", "PostgreSQL"],
            url: "",
            featured: true,
        },
        Project {
            name: "Pgames.cz",
            client: "Pgames",
            year: "2024",
            kind: "Website",
            summary: "Website and information panel for a gaming server, including a player ticket system.",
            tags: vec!["Next.js", "React", "TypeScript", "Java", "Spring Boot"],
            url: "",
            featured: true,
        },
        Project {
            name: "2D Craft",
            client: "Personal",
            year: "2024",
            kind: "Game",
            summary: "A Minecraft-like 2D game that runs entirely in the terminal, written in Java.",
            tags: vec!["Java", "Terminal"],
            url: "",
            featured: false,
        },
        Project {
            name: "chronos",
            client: "Personal",
            year: "2023",
            kind: "Game",
            summary: "A simple multiplayer CLI game written in Java.",
            tags: vec!["Java", "Networking"],
            url: "",
            featured: false,
        },
        Project {
            name: "Placeholder project",
            client: "—",
            year: "2026",
            kind: "AI",
            summary: "Placeholder description — replace with a real project summary in src/content.rs.",
            tags: vec!["Python", "LLM API"],
            url: "",
            featured: false,
        },
    ];

    let testimonials = vec![
        Testimonial {
            quote: "Placeholder testimonial. Replace this quote with what the client actually said — specific beats generic every time.",
            name: "Client Name",
            role: "Owner",
            company: "Company s.r.o.",
        },
        Testimonial {
            quote: "Placeholder testimonial. A second project, a second perspective, ideally a different kind of client from the first.",
            name: "Client Name",
            role: "Founder",
            company: "Startup",
        },
        Testimonial {
            quote: "Placeholder testimonial. Mentioning how the project actually ran — communication, timeline, what surprised them.",
            name: "Client Name",
            role: "Manager",
            company: "Agency",
        },
    ];

    let posts = vec![
        Post {
            title: "Placeholder post title",
            date: "2026",
            summary: "Placeholder summary — one or two sentences about what the post argues.",
            url: "#",
            tags: vec!["placeholder"],
        },
        Post {
            title: "Placeholder post title",
            date: "2025",
            summary: "Placeholder summary — one or two sentences about what the post argues.",
            url: "#",
            tags: vec!["placeholder"],
        },
        Post {
            title: "Placeholder post title",
            date: "2025",
            summary: "Placeholder summary — one or two sentences about what the post argues.",
            url: "#",
            tags: vec!["placeholder"],
        },
        Post {
            title: "Placeholder post title",
            date: "2024",
            summary: "Placeholder summary — one or two sentences about what the post argues.",
            url: "#",
            tags: vec!["placeholder"],
        },
        Post {
            title: "Placeholder post title",
            date: "2024",
            summary: "Placeholder summary — one or two sentences about what the post argues.",
            url: "#",
            tags: vec!["placeholder"],
        },
    ];

    let roles = vec![
        Role {
            org: "NetDirect s.r.o.",
            title: "Intern",
            period: "2026",
            summary: "Placeholder — what you worked on, what you owned, what changed because you were there. Mention concrete outcomes where you can.",
            tags: vec!["Internship"],
        },
        Role {
            org: "Pgames.cz",
            title: "AI Engineer & Freelancer",
            period: "2025",
            summary: "Placeholder — the website, the information panel and the player ticket system built for this server.",
            tags: vec!["Next.js", "React", "Java"],
        },
        Role {
            org: "QPlay.cz",
            title: "AI Engineer & Freelancer",
            period: "2023 — 2024",
            summary: "Placeholder — the official website for the information panel, plus internal tooling and Discord OAuth.",
            tags: vec!["OAuth", "Internal tools"],
        },
        Role {
            org: "Freelance",
            title: "Self-taught beginnings",
            period: "2020 — 2023",
            summary: "Placeholder — where you learned to scope, quote and ship on the date you promised.",
            tags: vec!["Web", "Automation"],
        },
    ];

    let retro = RetroStatus {
        status: vec![
            "compiling the next big thing",
            "debugging something that worked yesterday",
            "writing docs nobody asked for",
            "refactoring a regex at 2am",
            "available for work",
            "staring at a blank commit",
            "shipping it anyway",
        ],
        listening: vec![
            "lo-fi beats to debug to",
            "the sound of a deploy",
            "albums I will forget by Friday",
            "ancient jungle drum and bass",
            "nothing (peace and quiet)",
        ],
        mood: vec![
            "caffeinated",
            "curious",
            "sleep-deprived but functional",
            "over-cautious",
            "quietly confident",
            "perpetually mid-redesign",
        ],
        build: "last build: today, probably",
        guestbook: "GUESTBOOK MODULE DISABLED — this page is static and nothing you type here goes anywhere. Use email instead.",
        webring: vec![
            "sunnybeta.dev",
            "github.com/sunnybeta",
            "example-neon-site.tld",
            "example-void-site.tld",
        ],
        badges: vec![
            "BEST VIEWED IN NETSCAPE NAVIGATOR",
            "800 x 600",
            "OPTIMISED FOR 56K MODEM",
            "UNDER CONSTRUCTION",
        ],
        visitor_id: "SUNNY",

        /* navbar random details */
        last_updated: vec![
            "03/14/1998 11:42:07",
            "yesterday, 23:58",
            "sometime last week",
            "just now, probably",
            "1997-08-21 04:11",
            "after the last redesign",
            "TUESDAY, IN THE EVENING",
        ],
        quest: vec![
            "fix the classpath",
            "rewrite it in Rust",
            "find the missing semicolon",
            "deploy before lunch",
            "stop redesigning the navbar",
            "answer 14 unread emails",
            "learn what a monad is",
            "make the floppy drive sing",
        ],
        built_with: vec![
            "Notepad ++ a 56k modem",
            "a text editor and spite",
            "HTML 3.2 and hope",
            "tables. just tables.",
            "a Pentium II and a dream",
            "frames, unfortunately",
        ],
        fortune: vec![
            "a deploy will fail at 5pm",
            "you will find the bug in the last place you look",
            "someone will email you about Comic Sans",
            "the floppy disk was in the drive all along",
            "your CSS will work tomorrow",
            "a client will ask for one small change",
        ],

        /* random decorations */
        word_art: vec![
            "SUNNY'S HOME PAGE",
            "WELCOME TO MY SITE",
            "BEST VIEWED AT 800x600",
            "SIGN MY GUESTBOOK!",
            "NEW AND IMPROVED",
            "MY LITTLE CORNER OF THE WEB",
        ],
        captions: vec![
            "definitely not a stock image",
            "rendered at great expense",
            "found on a floppy in 1999",
            "hand-crafted, 1 pixel at a time",
            "do not right-click, it is copyrighted",
            "best viewed in fullscreen",
            "this took me all night",
        ],
        awards: vec![
            "COOL SITE OF THE DAY",
            "KILLER WEBPAGE AWARD",
            "5 STAR SITE",
            "EDITOR'S CHOICE",
            "BEST HOMEPAGE 1998",
            "WEBMASTER APPROVED",
        ],
        stickers: vec![
            "NEW!",
            "100% PURE HTML",
            "NO COOKIES HERE",
            "HAND CODED",
            "ZERO FRAMEWORKS",
            "GIF FREE ZONE",
        ],
        ticker: vec![
            "UPDATED WEEKLY",
            "PLEASE SIGN MY GUESTBOOK",
            "LAST SEEN ON THE INFORMATION SUPERHIGHWAY",
            "THIS PAGE IS ALWAYS UNDER CONSTRUCTION",
            "BROWSER SAFE",
            "THANKS FOR VISITING",
            "TELL YOUR FRIENDS",
        ],

        /* sidebar widgets */
        now_playing: vec![
            "coastal highway — neon nights",
            "the modem handshake",
            "midnight build queue",
            "sunday morning, slowly",
            "rain on a CRT monitor",
            "the quiet one before deploy",
            "dubai skyline, 3am",
            "waking up the database",
        ],
        guestbook_entries: vec![
            "FIRST!! nice page!!",
            "found this via webring, hi",
            "the dice widget made my day",
            "signed from a very slow modem",
            "best vibes on the information superhighway",
            "add me to your webring?",
            "still checking if the hit counter works",
        ],
    };

    Site {
        person,
        year: "2026",
        nav: vec![
            NavItem {
                label: "Home",
                path: "/pro",
            },
            NavItem {
                label: "Experience",
                path: "/experience",
            },
            NavItem {
                label: "Projects",
                path: "/projects",
            },
            NavItem {
                label: "Pricing",
                path: "/pricing",
            },
            NavItem {
                label: "Testimonials",
                path: "/testimonials",
            },
            NavItem {
                label: "Blogs",
                path: "/blogs",
            },
            NavItem {
                label: "Contact Me",
                path: "/contact",
            },
        ],
        retro_nav: vec![
            NavItem {
                label: "About",
                path: "#about",
            },
            NavItem {
                label: "Experience",
                path: "#experience",
            },
            NavItem {
                label: "Projects",
                path: "#projects",
            },
            NavItem {
                label: "Pricing",
                path: "#pricing",
            },
            NavItem {
                label: "Testimonials",
                path: "#testimonials",
            },
            NavItem {
                label: "Blogs",
                path: "#blogs",
            },
            NavItem {
                label: "Contact",
                path: "#contact",
            },
        ],
        facts,
        services,
        skills,
        tiers,
        add_ons,
        projects,
        testimonials,
        posts,
        roles,
        retro,
    }
});

/// The shared site data, built once and reused by every request.
pub fn site() -> &'static Site {
    &SITE
}
