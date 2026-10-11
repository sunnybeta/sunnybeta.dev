package main

import (
	"flag"
	"fmt"
	"html/template"
	"io/fs"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

/* =============================================================================
   CONTENT
   Everything the templates render lives here as variables. Edit this file to
   change the site; the templates never hardcode copy, names or links.
   ============================================================================= */

// Identity -------------------------------------------------------

type Person struct {
	Name          string // Sunny Béta
	FirstName     string // Sunny
	Handle        string // sunnybeta
	Domain        string // sunnybeta.dev
	Email         string // email@sunnybeta.dev
	Location      string // Dubai, UAE
	LocationShort string // Dubai
	Role          string // AI Engineer & Freelancer
	Headline      string // short line under the name on the home page
	Intro         string // two or three sentence introduction
	Availability  string // e.g. "Available for new projects"
	YearsCoding   string // e.g. "7+"
	GithubURL     string
	LinkedinURL   string
	InstagramURL  string
}

// Services offered, used on the retro page and the pro home page.

type Service struct {
	Title string
	Body  string
	Tag   string
	Icon  string // short label used as a retro glyph
}

type Fact struct {
	Value string
	Label string
}

// Pricing --------------------------------------------------------

type Tier struct {
	Name     string
	Tag      string
	Price    string
	Period   string
	Summary  string
	Features []string
	Popular  bool
	CTA      string
}

type AddOn struct {
	Name    string
	Price   string
	Summary string
}

// Content blocks --------------------------------------------------

type Project struct {
	Name     string
	Client   string
	Year     string
	Kind     string
	Summary  string
	Tags     []string
	URL      string
	Featured bool
}

type Testimonial struct {
	Quote   string
	Name    string
	Role    string
	Company string
}

type Post struct {
	Title   string
	Date    string
	Summary string
	URL     string
	Tags    []string
}

type Role struct {
	Org     string
	Title   string
	Period  string
	Summary string
	Tags    []string
}

type SkillGroup struct {
	Name  string
	Items []string
}

// Retro site ------------------------------------------------------

type RetroStatus struct {
	Status    []string
	Listening []string
	Mood      []string
	Build     string
	Guestbook string
	Webring   []string
	Badges    []string
	VisitorID string

	/* navbar random details */
	LastUpdated []string
	Quest       []string
	BuiltWith   []string
	Fortune     []string

	/* random decorations */
	WordArt  []string
	Captions []string
	Awards   []string
	Stickers []string
	Ticker   []string
}

// Site bundles everything handed to the templates.

type Site struct {
	Person       Person
	Year         string
	Nav          []NavItem
	RetroNav     []NavItem
	Facts        []Fact
	Services     []Service
	Skills       []SkillGroup
	Tiers        []Tier
	AddOns       []AddOn
	Projects     []Project
	Testimonials []Testimonial
	Posts        []Post
	Roles        []Role
	Retro        RetroStatus
}

// NavItem is a single header/anchor link.

type NavItem struct {
	Label string
	Path  string
}

// Page is the data set every template receives.

type Page struct {
	Site        Site
	Title       string
	Description string
	Active      string // current nav path, for highlighting
}

/* =============================================================================
   SITE CONTENT
   ============================================================================= */

var person = Person{
	Name:          "Sunny Béta",
	FirstName:     "Sunny",
	Handle:        "sunnybeta",
	Domain:        "sunnybeta.dev",
	Email:         "email@sunnybeta.dev",
	Location:      "Dubai, UAE",
	LocationShort: "Dubai",
	Role:          "AI Engineer & Freelancer",
	Headline:      "I build websites, apps and AI tooling that people actually enjoy using.",
	Intro:         "Placeholder intro — two or three sentences about who you are, what you build and who you build it for. Replace this in main.go.",
	Availability:  "Available for new projects",
	YearsCoding:   "7+",
	GithubURL:     "https://github.com/sunnybeta",
	LinkedinURL:   "https://www.linkedin.com/in/sunnybeta",
	InstagramURL:  "https://www.instagram.com/sunnybeta",
}

var year = "2026"

var nav = []NavItem{
	{Label: "Home", Path: "/pro"},
	{Label: "Experience", Path: "/experience"},
	{Label: "Projects", Path: "/projects"},
	{Label: "Pricing", Path: "/pricing"},
	{Label: "Testimonials", Path: "/testimonials"},
	{Label: "Blogs", Path: "/blogs"},
	{Label: "Contact Me", Path: "/contact"},
}

var retroNav = []NavItem{
	{Label: "About", Path: "#about"},
	{Label: "Experience", Path: "#experience"},
	{Label: "Projects", Path: "#projects"},
	{Label: "Pricing", Path: "#pricing"},
	{Label: "Testimonials", Path: "#testimonials"},
	{Label: "Blogs", Path: "#blogs"},
	{Label: "Contact", Path: "#contact"},
}

var facts = []Fact{
	{Value: person.YearsCoding, Label: "Years coding"},
	{Value: "20+", Label: "Projects shipped"},
	{Value: "3", Label: "Years freelancing"},
	{Value: "24h", Label: "Reply time"},
}

var services = []Service{
	{
		Title: "Websites",
		Body:  "Fast, hand-built websites with a CMS your team can actually use. No page-builder bloat.",
		Tag:   "WEB",
		Icon:  "@",
	},
	{
		Title: "Apps",
		Body:  "Dashboards, portals and SaaS products — auth, data model, admin panel, tests.",
		Tag:   "APP",
		Icon:  "</>",
	},
	{
		Title: "AI & automation",
		Body:  "Retrieval, agents and LLM features wired into your stack with evals and cost control.",
		Tag:   "AI",
		Icon:  "*ai*",
	},
	{
		Title: "Hosting & domains",
		Body:  "Domain registration, hosting and deploys set up so nothing breaks at 3am.",
		Tag:   "OPS",
		Icon:  "###",
	},
}

var skills = []SkillGroup{
	{Name: "Frontend", Items: []string{"HTML", "CSS", "JavaScript", "TypeScript", "React", "Next.js", "Vite"}},
	{Name: "Backend", Items: []string{"Java", "Spring", "Node.js", "Express", "Python", "Rust"}},
	{Name: "Data & infra", Items: []string{"PostgreSQL", "MariaDB", "KeyDB", "Docker", "MinIO", "Git"}},
	{Name: "AI", Items: []string{"LLM APIs", "RAG", "Evals", "Prompting", "Agents"}},
}

var tiers = []Tier{
	{
		Name:    "One-page Website",
		Tag:     "STARTER",
		Price:   "65",
		Period:  "EUR / project",
		Summary: "A clean single page for a personal project, small business or a quick launch.",
		Features: []string{
			"Single page, fully responsive",
			"Basic SEO setup",
			"Contact form",
			"Content editable via Markdown",
			"Two weeks delivery",
		},
		CTA: "Request this package",
	},
	{
		Name:    "Business Website",
		Tag:     "POPULAR",
		Price:   "210",
		Period:  "EUR / project",
		Summary: "A multi-page site for businesses that need to present real services and proof.",
		Features: []string{
			"Everything from One-page",
			"Up to 10 subpages",
			"Blog or news section",
			"Editable content via Markdown",
			"SEO optimisation",
		},
		Popular: true,
		CTA:     "Request this package",
	},
	{
		Name:    "Professional Website",
		Tag:     "SCALE",
		Price:   "1 100",
		Period:  "EUR / project",
		Summary: "A complete site with its own CMS for companies and larger projects.",
		Features: []string{
			"Everything from Business",
			"Full CMS system",
			"Unlimited subpages",
			"Professional SEO",
			"Training included",
		},
		CTA: "Request this package",
	},
	{
		Name:    "Web App",
		Tag:     "SYSTEMS",
		Price:   "2 400",
		Period:  "EUR / project",
		Summary: "A real product: authentication, database, admin panel and integrations.",
		Features: []string{
			"Login and role system",
			"Database and API design",
			"Admin panel included",
			"Third-party integrations",
			"Tests on critical paths",
		},
		CTA: "Request this package",
	},
	{
		Name:    "AI Build",
		Tag:     "NEW",
		Price:   "1 450",
		Period:  "EUR / project",
		Summary: "Retrieval, agents or LLM features added to a product you already have.",
		Features: []string{
			"Retrieval over your documents",
			"Streaming UI with citations",
			"Prompt and eval harness",
			"Cost and latency monitoring",
			"Handed over with docs",
		},
		CTA: "Request this package",
	},
	{
		Name:    "End to End",
		Tag:     "FULL",
		Price:   "Custom",
		Period:  "scoped together",
		Summary: "Idea to launch: design, build, infrastructure, deployment and handover.",
		Features: []string{
			"Discovery and scoping",
			"Design system and UI",
			"Frontend, backend and infra",
			"CI/CD and monitoring",
			"Documentation and handover",
		},
		CTA: "Let's scope it",
	},
}

var addOns = []AddOn{
	{Name: "Hosting and domain setup", Price: "from 65 EUR", Summary: "Domain registration and reliable hosting arranged for your site. Hosting and domain costs are not included."},
	{Name: "Website management training", Price: "from 40 EUR", Summary: "A walkthrough so you can edit your own content without breaking the layout."},
	{Name: "Maintenance — 2 weeks", Price: "Free", Summary: "After launch I stay available for two weeks to fix anything that surfaces."},
	{Name: "Ongoing maintenance", Price: "40 EUR / month", Summary: "Regular checks, updates and hosting management."},
}

var projects = []Project{
	{Name: "Baileys krmivo", Client: "Baileys Horse Feeds", Year: "2026", Kind: "Website", Summary: "E-commerce website for an exclusive horse feed importer, built on Next.js and Spring Boot.", Tags: []string{"Next.js", "Spring Boot", "TypeScript", "MinIO", "Docker"}, Featured: true},
	{Name: "Axiom English Academy", Client: "Axiom", Year: "2025", Kind: "Web app", Summary: "Full-stack web app with a custom CMS, student accounts and payments.", Tags: []string{"Next.js", "Spring Boot", "TypeScript", "KeyDB", "PostgreSQL"}, Featured: true},
	{Name: "Pgames.cz", Client: "Pgames", Year: "2024", Kind: "Website", Summary: "Website and information panel for a gaming server, including a player ticket system.", Tags: []string{"Next.js", "React", "TypeScript", "Java", "Spring Boot"}, Featured: true},
	{Name: "2D Craft", Client: "Personal", Year: "2024", Kind: "Game", Summary: "A Minecraft-like 2D game that runs entirely in the terminal, written in Java.", Tags: []string{"Java", "Terminal"}, Featured: false},
	{Name: "chronos", Client: "Personal", Year: "2023", Kind: "Game", Summary: "A simple multiplayer CLI game written in Java.", Tags: []string{"Java", "Networking"}, Featured: false},
	{Name: "Placeholder project", Client: "—", Year: "2026", Kind: "AI", Summary: "Placeholder description — replace with a real project summary in main.go.", Tags: []string{"Python", "LLM API"}, Featured: false},
}

var testimonials = []Testimonial{
	{
		Quote:   "Placeholder testimonial. Replace this quote with what the client actually said — specific beats generic every time.",
		Name:    "Client Name",
		Role:    "Owner",
		Company: "Company s.r.o.",
	},
	{
		Quote:   "Placeholder testimonial. A second project, a second perspective, ideally a different kind of client from the first.",
		Name:    "Client Name",
		Role:    "Founder",
		Company: "Startup",
	},
	{
		Quote:   "Placeholder testimonial. Mentioning how the project actually ran — communication, timeline, what surprised them.",
		Name:    "Client Name",
		Role:    "Manager",
		Company: "Agency",
	},
}

var posts = []Post{
	{Title: "Placeholder post title", Date: "2026", Summary: "Placeholder summary — one or two sentences about what the post argues.", URL: "#", Tags: []string{"placeholder"}},
	{Title: "Placeholder post title", Date: "2025", Summary: "Placeholder summary — one or two sentences about what the post argues.", URL: "#", Tags: []string{"placeholder"}},
	{Title: "Placeholder post title", Date: "2025", Summary: "Placeholder summary — one or two sentences about what the post argues.", URL: "#", Tags: []string{"placeholder"}},
	{Title: "Placeholder post title", Date: "2024", Summary: "Placeholder summary — one or two sentences about what the post argues.", URL: "#", Tags: []string{"placeholder"}},
	{Title: "Placeholder post title", Date: "2024", Summary: "Placeholder summary — one or two sentences about what the post argues.", URL: "#", Tags: []string{"placeholder"}},
}

var roles = []Role{
	{
		Org:     "NetDirect s.r.o.",
		Title:   "Intern",
		Period:  "2026",
		Summary: "Placeholder — what you worked on, what you owned, what changed because you were there. Mention concrete outcomes where you can.",
		Tags:    []string{"Internship"},
	},
	{
		Org:     "Pgames.cz",
		Title:   "AI Engineer & Freelancer",
		Period:  "2025",
		Summary: "Placeholder — the website, the information panel and the player ticket system built for this server.",
		Tags:    []string{"Next.js", "React", "Java"},
	},
	{
		Org:     "QPlay.cz",
		Title:   "AI Engineer & Freelancer",
		Period:  "2023 — 2024",
		Summary: "Placeholder — the official website for the information panel, plus internal tooling and Discord OAuth.",
		Tags:    []string{"OAuth", "Internal tools"},
	},
	{
		Org:     "Freelance",
		Title:   "Self-taught beginnings",
		Period:  "2020 — 2023",
		Summary: "Placeholder — where you learned to scope, quote and ship on the date you promised.",
		Tags:    []string{"Web", "Automation"},
	},
}

var retro = RetroStatus{
	Status: []string{
		"compiling the next big thing",
		"debugging something that worked yesterday",
		"writing docs nobody asked for",
		"refactoring a regex at 2am",
		"available for work",
		"staring at a blank commit",
		"shipping it anyway",
	},
	Listening: []string{
		"lo-fi beats to debug to",
		"the sound of a deploy",
		"albums I will forget by Friday",
		"ancient jungle drum and bass",
		"nothing (peace and quiet)",
	},
	Mood: []string{
		"caffeinated",
		"curious",
		"sleep-deprived but functional",
		"over-cautious",
		"quietly confident",
		"perpetually mid-redesign",
	},
	Build:     "last build: today, probably",
	Guestbook: "GUESTBOOK MODULE DISABLED — this page is static and nothing you type here goes anywhere. Use email instead.",
	Webring: []string{
		"sunnybeta.dev",
		"github.com/sunnybeta",
		"example-neon-site.tld",
		"example-void-site.tld",
	},
	Badges: []string{
		"BEST VIEWED IN NETSCAPE NAVIGATOR",
		"800 x 600",
		"OPTIMISED FOR 56K MODEM",
		"UNDER CONSTRUCTION",
	},
	VisitorID: "SUNNY",

	/* navbar random details -------------------------------------- */

	LastUpdated: []string{
		"03/14/1998 11:42:07",
		"yesterday, 23:58",
		"sometime last week",
		"just now, probably",
		"1997-08-21 04:11",
		"after the last redesign",
		"TUESDAY, IN THE EVENING",
	},
	Quest: []string{
		"fix the classpath",
		"rewrite it in Rust",
		"find the missing semicolon",
		"deploy before lunch",
		"stop redesigning the navbar",
		"answer 14 unread emails",
		"learn what a monad is",
		"make the floppy drive sing",
	},
	BuiltWith: []string{
		"Notepad ++ a 56k modem",
		"a text editor and spite",
		"HTML 3.2 and hope",
		"tables. just tables.",
		"a Pentium II and a dream",
		"frames, unfortunately",
	},
	Fortune: []string{
		"a deploy will fail at 5pm",
		"you will find the bug in the last place you look",
		"someone will email you about Comic Sans",
		"the floppy disk was in the drive all along",
		"your CSS will work tomorrow",
		"a client will ask for one small change",
	},

	/* random decorations ----------------------------------------- */

	WordArt: []string{
		"SUNNY'S HOME PAGE",
		"WELCOME TO MY SITE",
		"BEST VIEWED AT 800x600",
		"SIGN MY GUESTBOOK!",
		"NEW AND IMPROVED",
		"MY LITTLE CORNER OF THE WEB",
	},
	Captions: []string{
		"definitely not a stock image",
		"rendered at great expense",
		"found on a floppy in 1999",
		"hand-crafted, 1 pixel at a time",
		"do not right-click, it is copyrighted",
		"best viewed in fullscreen",
		"this took me all night",
	},
	Awards: []string{
		"COOL SITE OF THE DAY",
		"KILLER WEBPAGE AWARD",
		"5 STAR SITE",
		"EDITOR'S CHOICE",
		"BEST HOMEPAGE 1998",
		"WEBMASTER APPROVED",
	},
	Stickers: []string{
		"NEW!",
		"100% PURE HTML",
		"NO COOKIES HERE",
		"HAND CODED",
		"ZERO FRAMEWORKS",
		"GIF FREE ZONE",
	},
	Ticker: []string{
		"UPDATED WEEKLY",
		"PLEASE SIGN MY GUESTBOOK",
		"LAST SEEN ON THE INFORMATION SUPERHIGHWAY",
		"THIS PAGE IS ALWAYS UNDER CONSTRUCTION",
		"BROWSER SAFE",
		"THANKS FOR VISITING",
		"TELL YOUR FRIENDS",
	},
}

/* =============================================================================
   GENERATOR
   ============================================================================= */

// page couples a template name with the file it is written to.

/* outDir is the deploy root: everything a static host needs is written or
   copied into it, so it can be uploaded on its own. */

const outDir = "public"

type page struct {
	Template string // template name
	Output   string // output file path
	Title    string
	Desc     string
	Active   string
}

var pages = []page{
	{
		Template: "retro",
		Output:   outDir + "/index.html",
		Title:    "Sunny Béta",
		Desc:     "Independent AI engineer and freelancer.",
	},
	{
		Template: "pro_index",
		Output:   outDir + "/pro.html",
		Title:    "Home",
		Desc:     person.Role + " — " + person.Location,
		Active:   "/pro",
	},
	{
		Template: "pro_pricing",
		Output:   outDir + "/pricing.html",
		Title:    "Pricing",
		Desc:     "Fixed prices for websites, apps and AI builds.",
		Active:   "/pricing",
	},
	{
		Template: "pro_testimonials",
		Output:   outDir + "/testimonials.html",
		Title:    "Testimonials",
		Desc:     "What clients say about working with me.",
		Active:   "/testimonials",
	},
	{
		Template: "pro_blogs",
		Output:   outDir + "/blogs.html",
		Title:    "Blogs",
		Desc:     "Notes on engineering, AI and freelancing.",
		Active:   "/blogs",
	},
	{
		Template: "pro_projects",
		Output:   outDir + "/projects.html",
		Title:    "Projects",
		Desc:     "Selected work — websites, apps and games.",
		Active:   "/projects",
	},
	{
		Template: "pro_experience",
		Output:   outDir + "/experience.html",
		Title:    "Experience",
		Desc:     "Where I have worked and what I built.",
		Active:   "/experience",
	},
	{
		Template: "pro_contact",
		Output:   outDir + "/contact.html",
		Title:    "Contact Me",
		Desc:     "Email and PGP key for encrypted communication.",
		Active:   "/contact",
	},
}

/* copyFile copies a single file, creating parent directories as needed. */

func copyFile(src, dst string) error {
	data, err := os.ReadFile(src)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
		return err
	}
	return os.WriteFile(dst, data, 0o644)
}

/* copyTree copies a directory recursively. */

func copyTree(src, dst string) error {
	return filepath.WalkDir(src, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(src, path)
		if err != nil {
			return err
		}
		target := filepath.Join(dst, rel)
		if d.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		return copyFile(path, target)
	})
}

/* assetPaths are the hand-written files that belong in the deploy root but are
   not generated from templates. style.css and retro.css stay at the repo root
   as the editable source and are copied in on every build. */

var assetPaths = []string{
	"style.css",
	"retro.css",
	"pgp-key.asc",
}

func generate() error {
	funcs := template.FuncMap{
		"join": strings.Join,
	}

	tmpl, err := template.New("").Funcs(funcs).ParseGlob("./templates/**/*.html")
	if err != nil {
		return err
	}

	for _, p := range pages {
		data := Page{
			Site: Site{
				Person:       person,
				Year:         year,
				Nav:          nav,
				RetroNav:     retroNav,
				Facts:        facts,
				Services:     services,
				Skills:       skills,
				Tiers:        tiers,
				AddOns:       addOns,
				Projects:     projects,
				Testimonials: testimonials,
				Posts:        posts,
				Roles:        roles,
				Retro:        retro,
			},
			Title:       p.Title,
			Description: p.Desc,
			Active:      p.Active,
		}

		if err := os.MkdirAll(filepath.Dir(p.Output), 0o755); err != nil {
			return err
		}
		f, err := os.Create(p.Output)
		if err != nil {
			return err
		}
		if err := tmpl.ExecuteTemplate(f, p.Template, data); err != nil {
			f.Close()
			return fmt.Errorf("%s: %w", p.Output, err)
		}
		f.Close()
		fmt.Println("generated", p.Output)
	}

	for _, a := range assetPaths {
		dst := filepath.Join(outDir, a)
		if err := copyFile(a, dst); err != nil {
			return err
		}
		fmt.Println("copied   ", dst)
	}

	if err := copyTree("static", filepath.Join(outDir, "static")); err != nil {
		return err
	}
	fmt.Println("copied    static/")

	return nil
}

// serve runs the dev server with the same routes the static files support.

func serve() error {
	if err := generate(); err != nil {
		return err
	}

	mux := http.NewServeMux()

	/* extensionless routes for the generated pages */
	for _, p := range pages {
		if p.Active == "" {
			continue
		}
		file := p.Output
		mux.HandleFunc("GET "+p.Active, func(w http.ResponseWriter, r *http.Request) {
			http.ServeFile(w, r, file)
		})
	}
	mux.HandleFunc("GET /retro", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, filepath.Join(outDir, "index.html"))
	})

	/* public/ is the web root: everything else is served straight from it */
	mux.Handle("/", http.FileServer(http.Dir(outDir)))

	fmt.Println("serving on http://localhost:8080")
	return http.ListenAndServe(":8080", mux)
}

func main() {
	run := flag.Bool("serve", false, "start the dev server after generating")
	flag.Parse()

	var err error
	if *run {
		err = serve()
	} else {
		err = generate()
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
}
