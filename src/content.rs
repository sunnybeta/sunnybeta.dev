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
    /// https://x.com/sunnybeta_
    pub x_url: &'static str,
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
    /// Path of the article page, e.g. "/blogs/rewrite-it-in-rust".
    pub url: &'static str,
    pub tags: Vec<&'static str>,
    /// Paragraphs of the article, in order.
    pub body: Vec<&'static str>,
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
        headline: "I build websites, apps and AI tooling that startups and business utilise to solve their problems.",
        intro: "Placeholder intro — two or three sentences about who you are, what you build and who you build it for. Replace this in src/content.rs.",
        availability: "Available for new projects",
        years_coding: "7+",
        github_url: "https://github.com/sunnybeta",
        linkedin_url: "https://www.linkedin.com/in/sunnybeta",
        instagram_url: "https://www.instagram.com/sunnybeta_",
        x_url: "https://x.com/sunnybeta_",
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
            items: vec![
                "Go", "Rust", "Java", "Spring", "Node.js", "Express", "Python", "Rust",
            ],
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
            name: "Karo",
            client: "Liminal Minds",
            year: "2026",
            kind: "AI",
            summary: "An AI powered productivity tool for manaing thoughts and tasks with agentic features of executing possible tasks in cloud.",
            tags: vec!["Rust", "Agentic SaaS", "NextJS"],
            url: "",
            featured: false,
        },
    ];

    let testimonials = vec![
        Testimonial {
            quote: "We needed a website that looked professional and loaded fast, and that’s exactly what we got. Shailesh understood our brand from the first call, suggested layout improvements we hadn’t thought of, and delivered ahead of schedule. Since launch, our enquiries through the site have noticeably increased. Communication was clear throughout, and revisions were handled quickly and without fuss.",
            name: "Nitesh",
            role: "CEO",
            company: "Rasva",
        },
        Testimonial {
            quote: "We came in with a rough idea of using AI to automate [task, e.g., customer support / document processing], and Sunny turned it into a working solution. He explained the technical trade-offs in plain language, helped us pick the right approach for our budget, and built something that’s now saving our team hours every week. What stood out was their focus on accuracy and real-world testing, not just a flashy demo.",
            name: "Vinod",
            role: "Founder",
            company: "Wellnesys",
        },
        Testimonial {
            quote: "Sunny handled everything, from database design and backend APIs to the frontend and deployment. Having one person own the entire stack made the project far smoother than our past experiences with multiple vendors. The code is clean and well documented, and when we needed new features after launch, they were easy to add. Reliable, proactive, and genuinely invested in the product’s success.",
            name: "Tyson Black",
            role: "Founder",
            company: "Filed",
        },
        Testimonial {
            quote: "Our app went from concept to the Play Store and App Store in 2 weeks. Shailesh paid close attention to the user experience, kept us updated with regular builds we could test on our own phones, and resolved feedback quickly. Users regularly comment on how smooth and easy the app is to use. We’d happily work with them again on our next release.",
            name: "Aditya Sharma",
            role: "CEO",
            company: "BTBiz",
        },
    ];

    let posts = vec![
        Post {
            title: "OOP Was a 30-Year Detour, and We're Still Paying for It",
            date: "2026-09-04",
            summary: "Inheritance hierarchies, getters and setters everywhere, and design patterns that exist only to work around the paradigm itself. A look at why data-oriented and functional approaches produce simpler, more maintainable code, and what we lost by treating OOP as the default.",
            url: "/blogs/oop-was-a-30-year-detour",
            tags: vec!["oop", "programming-paradigms", "functional-programming", "software-design", "opinion"],
            body: vec![
                "Object-oriented programming was sold as the way software would finally scale: model the world as objects, send messages between them, and the complexity would keep itself contained. Thirty years on, the result is ceremony. The most reliable systems I have worked on share one trait, and it is not more objects.",
                "Start with inheritance, the headline feature and the one that ages worst. A class hierarchy is a promise about the future you cannot keep: the moment someone needs a variant that does not fit the tree, the choice is between breaking the abstraction for every client or bolting on another layer to work around it. Deep hierarchies turn small changes into surveys of everything that might override them.",
                "Then there is the getter and setter ritual. A class whose every field is reachable through accessors is a struct with more steps, and pretending otherwise does not buy encapsulation. Real encapsulation is a type whose invalid states are unrepresentable, not private fields wrapped in public syntax.",
                "The design-pattern catalogue is the clearest evidence of the detour. Read it honestly: most of those patterns exist to overcome limits the paradigm itself created. Visitor, Factory, Abstract Factory and Command collapse into a function, a closure or an enum in languages with first-class functions and algebraic data types. What counted as sophistication was often the cost of missing language features, fossilised into convention.",
                "Data-oriented design reaches the same conclusion from the other direction. The CPU does not care about your abstractions; it cares about cache lines. Plain arrays of data routinely beat graphs of pointers by an order of magnitude, which is why the people writing game engines and databases quietly abandoned the object model years before the essays did.",
                "Functional programming presses the same point from correctness. Immutable values, functions that map inputs to outputs and types that say what they mean are simply easier to reason about than a web of mutable objects signalling each other in an order nobody wrote down. Races and most logic bugs are shared mutable state; remove it and the bugs have nowhere to live.",
                "None of this makes objects useless. A boundary module behind a clear interface is exactly the right tool, and I build them. The mistake was promoting that boundary to the default shape of every line of code, then grading entire university curricula on UML diagrams.",
                "We are still paying for the detour in ceremony: classes to hold functions, interfaces with a single implementation, frameworks whose main job is wiring objects to other objects. The correction is boring and good: plain data, explicit state, pure functions where you can, and objects at the few real boundaries where they earn their keep.",
            ],
        },
        Post {
            title: "Rewrite It in Rust: Why I Stopped Apologizing for Saying It",
            date: "2026-09-11",
            summary: "The meme became a punchline, but the argument still holds. Memory safety without a garbage collector, a compiler that catches bugs before production, and tooling that just works make Rust a serious choice for CLIs, backends, and even web frontends.",
            url: "/blogs/rewrite-it-in-rust",
            tags: vec!["rust", "memory-safety", "performance", "systems-programming", "opinion"],
            body: vec![
                "Say it in any engineering forum and the replies arrive like a reflex: it is a meme, it is dogma, it is a phase everyone grows out of. I have shipped real projects in Rust and watched what the rewrites did to those codebases. I stopped apologising for the recommendation because the results kept making me right.",
                "The first reason is memory safety without a garbage collector. The vulnerability classes that dominate CVE databases — buffer overflows, use-after-free, data races — are prevented by construction in safe Rust. The compiler is not asking you to be more careful; it refuses to compile carelessness. That is a step change in what a team can promise its users, not a marginal improvement in discipline.",
                "Second, the compiler is the best design reviewer on the team. Exhaustive matches, sum types for errors and the borrow checker push whole categories of mistake to build time, and refactors stop being terrifying because the machine lists every affected call site. Fearless concurrency is a marketing phrase, but fearless refactoring is a daily, measurable experience.",
                "Third, the tooling stopped being the excuse. Cargo handles build, test, docs, benchmarks and publishing with one command and a lockfile, which is more than most ecosystems claim for any single one of those. rust-analyzer and clippy make the inner loop fast enough that new joiners are productive in weeks, not quarters.",
                "This does not mean rewrite everything. The honest version of the argument is narrower: greenfield services and CLIs, components with security obligations, hot paths where performance is a requirement rather than a hope. A shell script is still a shell script, and a rewrite that trashes momentum to chase purity is the same dogmatism the meme mocks.",
                "Say the costs out loud, because they are real. The borrow checker is hostile for about a month, compile times on large crates are noticeable, and a team new to the language slows down before it speeds up. Budget for that phase. The migrations I have seen fail failed the same way: nobody planned for the learning curve, the timeline panicked, and a working codebase was abandoned half-rewritten.",
                "But the cost of not learning it has gone up too. Rust is in the Linux kernel, in browsers, in cloud infrastructure and in more job postings every quarter. The WebAssembly story is good on the frontend when it fits. The meme survived because, annoyingly often, rewriting it in Rust was the correct engineering call — and the parts of the industry that kept laughing have started to notice.",
            ],
        },
        Post {
            title: "Stop Vibe-Coding: The Hard Rules of Building with AI",
            date: "2026-09-18",
            summary: "AI assistants can make you dramatically faster or quietly fill your codebase with code nobody understands. A practical set of dos and don'ts for using AI in real projects, covering code review, testing, prompt discipline, and knowing when to switch it off.",
            url: "/blogs/stop-vibe-coding",
            tags: vec!["ai", "llm", "developer-productivity", "best-practices", "code-quality"],
            body: vec![
                "There is a way of working with AI assistants that feels wonderful and is quietly dangerous: describe the feature, accept the code, describe the next one. The demos are spectacular, which is exactly the trap. On anything that has to survive contact with real users, vibe-coding sends the bill later, with interest.",
                "The failure mode is not the one people brace for. Vibe-coded programs usually compile, usually look plausible, and usually pass the happy path on the first try. The damage hides where nobody demos: error paths, concurrency, security boundaries, and the quiet assumption that the API behaves the way the training data remembers rather than the way the version you run actually documents.",
                "Rule one: do not ship code you cannot explain. If you cannot say why a change is correct, you have not reviewed it — you have witnessed it. The speed argument collapses at the first incident, because the person debugging at the worst possible hour is you.",
                "Rule two: tests decide, not vibes. A suite that defines what correct means is what keeps the assistant useful; it is the difference between a wrong guess that turns red and one that ships. Make the model write the feature, then make it write the adversarial tests for its own code, including the cases it did not think of. The second task is where these tools genuinely earn their keep.",
                "Rule three: review diffs structurally. Anything touching authentication, payments, personal data or infrastructure gets a human reading every line, every time, no exceptions. Skimming a security diff is how the subtle ones get in: the endpoint that forgets an authorisation check, the query that selects one column too many, the secret that lands in a log line.",
                "Rule four: keep the context small and the goal explicit. Pasting half the codebase into one prompt gets you a solution optimised for nothing in particular. State the outcome, the constraints and the failure modes; split the work into small, well-scoped tasks with a definition of done; and do not let one session sprawl across unrelated parts of the system.",
                "Rule five: measure the real throughput. Sometimes the assistant is a two-times multiplier; sometimes it is minus ten percent in a confident tone. Track how long features take to done-and-tested, not to first draft, and be honest about which classes of task the tool is losing you money on.",
                "The goal was never to reject the tool. AI accelerates intention, and if the intention is fuzzy, the acceleration multiplies the fuzz. Put the engineering in the loop — tests, boundaries, review — and the same acceleration is the biggest leverage most developers have been handed in years.",
            ],
        },
        Post {
            title: "Your Staging Environment Is Lying to You. Test in Production.",
            date: "2026-09-25",
            summary: "Staging never matches real traffic, real data, or real users. How feature flags, canary releases, observability, and fast rollbacks let you test safely where it actually matters, and why that beats maintaining an expensive environment that gives false confidence.",
            url: "/blogs/test-in-production",
            tags: vec!["testing", "devops", "feature-flags", "observability", "opinion"],
            body: vec![
                "Staging never matches production. Not the traffic shape, not the data volume, not the third parties at 3am, and above all not the users, who will always find the tenth button you did not test. We keep the environments anyway, read their green dashboards, and ship with a confidence reality does not confirm.",
                "This is not sloppiness, it is economics. A faithful clone of production costs more every quarter: the data grows, the integrations multiply, the configuration drifts, and every month the gap between what you tested and what you run widens anyway. You are paying to maintain a lie that gets more expensive and less true at the same time.",
                "The alternative is not recklessness; it is engineering production so that being wrong is survivable, then testing where reality lives. The goal is not zero bugs — nobody has that — it is bugs with a small blast radius and a fast time-to-detect.",
                "Feature flags come first. Every meaningful change ships dark, off by default and targetable to a fraction of users. Turning a feature on becomes a config change, not a deploy, and the rollback is a toggle a support engineer can flip. Decoupling release from deploy removes most of the fear the staging environment was built to manage.",
                "Canaries make the blast radius numerical. Route one percent of traffic to the new version, watch error rate and latency against the stable baseline for an hour, and let an automated rollback act on the numbers without waiting for a human to feel brave. Gradual rollouts replaced my favourite failure: the big-bang deploy where the only test that matters runs on everyone at once.",
                "Observability is what makes any of this honest. Structured logs, traces that follow one request across service boundaries, metrics tied to user-visible behaviour, and dashboards you have actually used before an incident. If you cannot see the feature working, you are not testing in production — you are hoping there.",
                "None of this retires the test suite. Unit and integration tests still catch the cheap classes of bug in seconds, and they should run on every commit. The claim is narrower: the bugs that survive a good suite are precisely the ones that exist only under real load, real data and real people, and no synthetic environment has reliably reproduced them.",
                "There is a cultural prerequisite, and it is the hard part. Teams do not test in production because some tool was missing; they do it because leadership accepts that incidents are information and on-call is humane. Blameless postmortems, error budgets, rollbacks that are boring. The mechanics are standard SRE practice; the breakthrough is treating production as the lab and funding it like one.",
            ],
        },
        Post {
            title: "You Don't Need Kubernetes. You Need a $5 VPS.",
            date: "2026-10-02",
            summary: "Most apps serve a few thousand users, not millions, yet teams adopt Kubernetes on day one and spend more time on YAML than features. Why a single well-configured server with a reverse proxy, systemd, and good backups takes you much further than you think, and the signs that you've genuinely outgrown it.",
            url: "/blogs/you-dont-need-kubernetes",
            tags: vec!["kubernetes", "devops", "infrastructure", "simplicity", "opinion"],
            body: vec![
                "Kubernetes is a great answer to a question most products eventually ask and almost none start with: how do you coordinate thousands of heterogeneous workloads for teams shipping many times a day. If that is your situation, run it and enjoy it. The rest of us adopted it because the architecture diagrams implied scale, and now we maintain a distributed system whose main product is YAML.",
                "Do the arithmetic on your real load before reaching for an orchestrator. A four-core VPS with 8GB of RAM comfortably fronts a busy web app, its database and background jobs, with headroom to spare. Most products would celebrate reaching the traffic that saturates that box, and when one does, the upgrade path is usually a second server behind a load balancer: a Saturday of work, not an infrastructure programme.",
                "The boring stack is powerful precisely because it is boring. One Linux box, Caddy or nginx terminating TLS and reverse-proxying, systemd units running the application, timers for scheduled jobs, restic for backups that have actually been test-restored. Every layer is a mature tool with fifteen years of documentation, and every failure has a known first move.",
                "Compare failure modes honestly. When a VPS dies at 3am you restart it, restore the backup, and are serving again before sunrise. When a cluster quietly wedges at 3am you are diffing ConfigMaps against CRD versions in a runbook nobody has updated since the team that wrote it left. Boring systems fail legibly; complex ones fail cryptically.",
                "The stronger argument is focus, not money. Every hour spent on storage classes, ingress annotations and node affinity is an hour not spent on the product. Operating Kubernetes is also a hiring constraint: you need specialists for your platform, and they are expensive, while a well-tended Linux server is administrable by anyone with general experience.",
                "Outgrowing one box does not mean outgrowing the simple stack. The ladder usually looks like this: one VPS, one bigger VPS, two behind a load balancer, then a managed database — because the first thing you genuinely outgrow is running your own primary database, not the application runtime.",
                "By the time a team truly needs distributed scheduling — hundreds of services, many deploys a day, strict tenant isolation — Kubernetes is the correct answer, and migrating to it is straightforward: there is a working product, real traffic patterns, and metrics defining what must not degrade. Adopting it on day one inverts this, taxing every feature with complexity to prepare for loads that may never arrive.",
                "The judgement is not that Kubernetes is bad; it is that infrastructure should match the problem, and you should buy the simplest thing that keeps the pager quiet. For most products a modest, well-tended VPS keeps it quiet for years. Start there, and renew complexity deliberately, like any other subscription.",
            ],
        },
    ];

    let roles = vec![
        Role {
            org: "YogiFi",
            title: "A.I. Lead",
            period: "2024 - Present",
            summary: "Building end-to-end AI pipelines from raw data and processing to training, evaluations and deployment",
            tags: vec!["AI","LLM","Langchain","Unsloth","Image Processing", "GenAI"],
        },
        Role {
            org: "Lolly",
            title: "Backend Engineer & DevOps",
            period: "2022-2023",
            summary: "Built APIs and managed legacy systems",
            tags: vec!["Python","FastAPI","Flask","AWS"],
        },
        Role {
            org: "Aptive Globe",
            title: "Data Scientist",
            period: "2021 — 2022",
            summary: "Data cleaning, preprocessing, analysis and building python frameworks for automation",
            tags: vec!["Python", "Data Wrangling", "Feature Engineering"],
        },
        Role {
            org: "Flux Auto",
            title: "Machine Learning Engineer - Self-taught Beginnings",
            period: "2018",
            summary: "Here began the journey from MLP to CNNs and from vectors to TensorFlow",
            tags: vec!["AI", "CNN", "Object Detection", "Vision"],
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
                path: "/",
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

/// Look up a published post by the slug at the end of its URL, e.g.
/// `rewrite-it-in-rust` from `/blogs/rewrite-it-in-rust`.
pub fn post_by_slug(slug: &str) -> Option<&'static Post> {
    site()
        .posts
        .iter()
        .find(|post| post.url.rsplit('/').next() == Some(slug))
}
