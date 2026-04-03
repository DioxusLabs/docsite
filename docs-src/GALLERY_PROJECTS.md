# Dioxus Gallery Projects

A collection of example projects to build and feature in a gallery page. Each project demonstrates specific Dioxus features and real-world patterns.

---

## Project Guidelines

**For each project:**
- Keep it focused. One app, a few key features, done well.
- Include a README with setup instructions
- Deploy somewhere accessible (Vercel, Fly.io, GitHub Pages, etc.)
- Use the same styling approach (Tailwind) for visual consistency
- Include comments explaining non-obvious patterns

**Naming convention:** `dioxus-example-{name}`

---

## Tier 1: Core Showcases

These demonstrate fundamental Dioxus patterns every developer needs.

### 1. Todo App
**What it demonstrates:** Signals, lists, local storage, basic CRUD

**Features:**
- Add/edit/delete todos
- Mark complete, filter by status
- Persist to localStorage
- Keyboard shortcuts (Enter to add, Escape to cancel)
- Drag to reorder

**Tech:** Web, Tailwind, localStorage API

**Complexity:** Beginner

---

### 2. HackerNews Clone
**What it demonstrates:** Data fetching, routing, pagination, SSR

**Features:**
- Front page with stories
- Nested comments (recursive components)
- Multiple pages (top, new, ask, show, jobs)
- User profiles
- Time-ago formatting

**Tech:** Fullstack, SSR, external API integration

**Complexity:** Intermediate

---

### 3. Markdown Editor
**What it demonstrates:** Controlled inputs, real-time preview, keyboard handling

**Features:**
- Split pane (editor / preview)
- Syntax highlighting in editor
- Live markdown rendering
- Export to HTML/PDF
- Auto-save to localStorage
- Keyboard shortcuts for formatting

**Tech:** Web, markdown-rs or pulldown-cmark, highlight.js

**Complexity:** Intermediate

---

### 4. File Explorer
**What it demonstrates:** Desktop APIs, tree structures, context menus

**Features:**
- Browse local filesystem
- Tree view with expand/collapse
- File preview (text, images)
- Right-click context menu
- Keyboard navigation
- Breadcrumb path bar

**Tech:** Desktop, native file dialogs, system APIs

**Complexity:** Intermediate

---

### 5. Chat Application
**What it demonstrates:** WebSockets, real-time updates, authentication

**Features:**
- User registration/login
- Real-time messages
- Multiple chat rooms
- Typing indicators
- Message history with infinite scroll
- Online/offline status

**Tech:** Fullstack, WebSockets, database (SQLite or Postgres)

**Complexity:** Advanced

---

## Tier 2: Platform Showcases

These highlight platform-specific capabilities.

### 6. System Monitor (Desktop)
**What it demonstrates:** Desktop-specific APIs, real-time data, system tray

**Features:**
- CPU/memory/disk usage graphs
- Process list with kill option
- System tray with quick stats
- Notifications when thresholds exceeded
- Minimize to tray
- Start on boot option

**Tech:** Desktop, sysinfo crate, native notifications

**Complexity:** Intermediate

---

### 7. Menubar Notes (Desktop)
**What it demonstrates:** Menubar app pattern, global shortcuts

**Features:**
- Lives in system tray/menubar
- Click to open quick note
- Global hotkey to capture
- Markdown support
- Search notes
- Sync to file system

**Tech:** Desktop, global shortcuts, menubar mode

**Complexity:** Intermediate

---

### 8. QR Scanner (Mobile)
**What it demonstrates:** Camera access, mobile permissions, native feel

**Features:**
- Camera viewfinder
- QR code detection
- History of scans
- Share scanned content
- Flashlight toggle
- Haptic feedback on scan

**Tech:** Mobile (iOS/Android), camera APIs

**Complexity:** Intermediate

---

### 9. Expense Tracker (Mobile)
**What it demonstrates:** Mobile-first design, charts, local database

**Features:**
- Add expenses with categories
- Monthly/weekly views
- Pie chart breakdown
- Budget setting and alerts
- Export to CSV
- Photo receipt attachment

**Tech:** Mobile, SQLite, charts library

**Complexity:** Intermediate

---

### 10. PWA Weather App (Web)
**What it demonstrates:** PWA, offline support, geolocation

**Features:**
- Current weather + forecast
- Location detection
- City search
- Offline cached data
- Install as PWA
- Push notifications for alerts

**Tech:** Web (PWA), service workers, weather API

**Complexity:** Intermediate

---

## Tier 3: Integration Showcases

These demonstrate third-party integrations developers commonly need.

### 11. Stripe Checkout
**What it demonstrates:** Payment integration, server functions, webhooks

**Features:**
- Product listing
- Add to cart
- Stripe Checkout flow
- Order confirmation
- Webhook handling for fulfillment
- Order history

**Tech:** Fullstack, Stripe API, webhooks

**Complexity:** Advanced

---

### 12. OAuth Login Demo
**What it demonstrates:** OAuth2 flows, session management, protected routes

**Features:**
- Login with GitHub
- Login with Google
- Login with Discord
- Protected dashboard page
- Session persistence
- Logout everywhere

**Tech:** Fullstack, OAuth2, cookies/sessions

**Complexity:** Intermediate

---

### 13. Maps & Location
**What it demonstrates:** Map embedding, geolocation, markers

**Features:**
- Interactive map (Mapbox or Leaflet)
- Current location
- Search places
- Add custom markers
- Draw routes
- Save favorite locations

**Tech:** Web, Mapbox GL JS or Leaflet

**Complexity:** Intermediate

---

### 14. S3 File Uploader
**What it demonstrates:** File uploads, presigned URLs, progress tracking

**Features:**
- Drag and drop upload
- Multiple file selection
- Upload progress bar
- Image preview
- Delete files
- Generate shareable links

**Tech:** Fullstack, AWS S3 or compatible (R2, Minio)

**Complexity:** Intermediate

---

### 15. Supabase CRUD
**What it demonstrates:** Supabase integration, real-time subscriptions

**Features:**
- Auth with Supabase
- CRUD operations on a table
- Real-time updates when data changes
- Row-level security demo
- File storage

**Tech:** Fullstack, Supabase client

**Complexity:** Intermediate

---

### 16. Firebase App
**What it demonstrates:** Firebase integration, alternative to Supabase

**Features:**
- Firebase Auth
- Firestore CRUD
- Real-time listeners
- Cloud Functions trigger
- Analytics events

**Tech:** Web, Firebase JS SDK via interop

**Complexity:** Intermediate

---

## Tier 4: Feature Showcases

These highlight specific Dioxus features in isolation.

### 17. Form Playground
**What it demonstrates:** Form patterns, validation, complex inputs

**Features:**
- Text, number, email, password fields
- Validation (inline and on-submit)
- Multi-step wizard form
- Dependent fields (show X if Y selected)
- Date picker
- File input with preview
- Form reset and dirty tracking

**Tech:** Web, form validation patterns

**Complexity:** Intermediate

---

### 18. Animation Gallery
**What it demonstrates:** CSS transitions, animations, gestures

**Features:**
- Page transitions
- List item enter/exit animations
- Drag gestures
- Spring physics
- Scroll-triggered animations
- Loading skeletons
- Micro-interactions

**Tech:** Web, CSS animations, maybe motion library

**Complexity:** Intermediate

---

### 19. Virtualized List
**What it demonstrates:** Performance with large datasets

**Features:**
- 100,000+ item list
- Smooth scrolling
- Variable height items
- Jump to index
- Sticky headers
- Performance metrics display

**Tech:** Web, virtual scrolling implementation

**Complexity:** Advanced

---

### 20. Theme Switcher
**What it demonstrates:** Theming, CSS variables, persistence

**Features:**
- Light/dark/system modes
- Custom color themes
- Live preview
- Persist preference
- Respect system preference
- Smooth transitions

**Tech:** Web, CSS custom properties

**Complexity:** Beginner

---

### 21. Keyboard Shortcuts App
**What it demonstrates:** Keyboard handling, command palette

**Features:**
- Cmd+K command palette
- Configurable shortcuts
- Shortcut hints on hover
- Conflict detection
- Export/import config
- Cheat sheet modal

**Tech:** Web/Desktop, keyboard event handling

**Complexity:** Intermediate

---

### 22. Drag and Drop Kanban
**What it demonstrates:** Drag and drop, complex state

**Features:**
- Multiple columns
- Drag cards between columns
- Reorder within column
- Add/edit/delete cards
- Column management
- Persist state

**Tech:** Web, drag and drop APIs or library

**Complexity:** Intermediate

---

### 23. Data Table
**What it demonstrates:** Complex component, sorting, filtering, pagination

**Features:**
- Column sorting (multi-column)
- Text filtering
- Column resizing
- Row selection (single/multi)
- Pagination
- Export to CSV
- Column visibility toggle

**Tech:** Web, table implementation patterns

**Complexity:** Advanced

---

### 24. Rich Text Editor
**What it demonstrates:** ContentEditable, complex input handling

**Features:**
- Bold, italic, underline
- Headings
- Lists (ordered/unordered)
- Links
- Images
- Undo/redo
- HTML output

**Tech:** Web, contenteditable or Tiptap-style approach

**Complexity:** Advanced

---

## Tier 5: Full Applications

Complete apps that combine multiple patterns.

### 25. Blog with CMS
**What it demonstrates:** SSR, markdown, admin panel, SEO

**Features:**
- Public blog pages
- Admin dashboard for posts
- Markdown editor for content
- Image uploads
- Tags and categories
- SEO meta tags
- RSS feed

**Tech:** Fullstack, SSR, database, markdown

**Complexity:** Advanced

---

### 26. Project Management
**What it demonstrates:** Complex state, real-time, roles

**Features:**
- Projects with tasks
- Assign users to tasks
- Due dates and priorities
- Kanban and list views
- Comments on tasks
- Activity feed
- Role-based permissions

**Tech:** Fullstack, WebSockets, database

**Complexity:** Advanced

---

### 27. E-commerce Store
**What it demonstrates:** Full shopping experience

**Features:**
- Product catalog
- Category filtering
- Search
- Product detail pages
- Shopping cart
- Checkout flow (Stripe)
- Order confirmation
- Order history

**Tech:** Fullstack, Stripe, database, SSR

**Complexity:** Advanced

---

### 28. Social Media Feed
**What it demonstrates:** Infinite scroll, real-time, user content

**Features:**
- Post creation (text, images)
- Feed with infinite scroll
- Like and comment
- Follow users
- Notifications
- User profiles
- Image upload

**Tech:** Fullstack, real-time, file storage

**Complexity:** Advanced

---

### 29. Dashboard Template
**What it demonstrates:** Layout patterns, charts, responsive design

**Features:**
- Sidebar navigation
- Top navbar with user menu
- Multiple chart types (line, bar, pie)
- Stats cards
- Recent activity table
- Responsive (collapses on mobile)
- Multiple pages

**Tech:** Web, charting library, Tailwind

**Complexity:** Intermediate

---

### 30. CLI + GUI Companion
**What it demonstrates:** Desktop + CLI sharing code

**Features:**
- Rust CLI tool for some task
- Desktop GUI for same task
- Shared core logic
- CLI can open GUI
- GUI can run CLI commands
- Configuration shared

**Tech:** Desktop, CLI (clap), shared library

**Complexity:** Advanced

---

## Tier 6: Educational Showcases

These teach specific concepts clearly.

### 31. State Patterns Demo
**What it demonstrates:** Different state management approaches

**Features:**
- Same app built 4 ways:
  - Local signals
  - Context
  - Global signals
  - Stores
- Side-by-side comparison
- Explanation of trade-offs

**Tech:** Web, educational

**Complexity:** Beginner

---

### 32. Reactivity Visualizer
**What it demonstrates:** How signals and effects work

**Features:**
- Visual graph of signal dependencies
- Highlight what re-renders
- Step through updates
- Show subscription tracking
- Explain with annotations

**Tech:** Web, educational

**Complexity:** Intermediate

---

### 33. Component Lifecycle Demo
**What it demonstrates:** Mount, update, cleanup

**Features:**
- Visual timeline of events
- Show hook execution order
- Demonstrate cleanup
- Show effect timing
- Interactive controls

**Tech:** Web, educational

**Complexity:** Beginner

---

### 34. Router Playground
**What it demonstrates:** All router features

**Features:**
- Nested routes
- Dynamic segments
- Query parameters
- Redirects
- Guards (simulated)
- History manipulation
- URL display

**Tech:** Web, router features

**Complexity:** Beginner

---

### 35. Server Functions Playground
**What it demonstrates:** Server function patterns

**Features:**
- GET vs POST vs PUT
- Path parameters
- Query parameters
- Request/response headers
- Error handling
- Loading states
- Extractors demo

**Tech:** Fullstack, server functions

**Complexity:** Intermediate

---

## Quick Reference by Feature

| Feature | Projects |
|---------|----------|
| **Signals/State** | Todo, State Patterns Demo, Reactivity Visualizer |
| **Routing** | HackerNews, Blog, Router Playground |
| **Forms** | Form Playground, Auth Demo, E-commerce |
| **Data Fetching** | HackerNews, Weather PWA, Supabase CRUD |
| **SSR** | HackerNews, Blog, E-commerce |
| **WebSockets** | Chat App, Project Management |
| **Desktop APIs** | File Explorer, System Monitor, Menubar Notes |
| **Mobile APIs** | QR Scanner, Expense Tracker |
| **Authentication** | Chat App, OAuth Demo, Blog CMS |
| **File Upload** | S3 Uploader, Blog CMS, Social Feed |
| **Payments** | Stripe Checkout, E-commerce |
| **Maps** | Maps & Location |
| **Charts** | System Monitor, Expense Tracker, Dashboard |
| **Animation** | Animation Gallery, Kanban |
| **Performance** | Virtualized List, Data Table |
| **Theming** | Theme Switcher, Dashboard |
| **Offline/PWA** | Weather PWA, Todo App |

---

## Suggested Build Order

**Phase 1: Foundations (5 projects)**
1. Todo App (simplest complete app)
2. Theme Switcher (common need)
3. Form Playground (everyone needs forms)
4. State Patterns Demo (educational)
5. Router Playground (routing basics)

**Phase 2: Integrations (5 projects)**
6. OAuth Demo (auth is universal need)
7. Supabase CRUD (popular backend)
8. S3 Uploader (file handling)
9. HackerNews Clone (data fetching patterns)
10. Markdown Editor (real utility)

**Phase 3: Platform (5 projects)**
11. File Explorer (desktop showcase)
12. System Monitor (desktop APIs)
13. Weather PWA (web PWA)
14. QR Scanner (mobile showcase)
15. Dashboard Template (layout patterns)

**Phase 4: Advanced (5 projects)**
16. Chat App (real-time)
17. Stripe Checkout (payments)
18. Blog CMS (full app)
19. Data Table (complex component)
20. Kanban Board (drag and drop)

---

## Deployment Targets

| Project Type | Deploy To |
|--------------|-----------|
| Web only | Vercel, Netlify, GitHub Pages |
| Fullstack | Fly.io, Railway, Shuttle |
| Desktop | GitHub Releases (binaries) |
| Mobile | TestFlight (iOS), APK download (Android) |

---

## Template Structure

Each project should follow:

```
dioxus-example-{name}/
├── README.md           # Setup, features, what it demonstrates
├── Cargo.toml
├── Dioxus.toml
├── src/
│   ├── main.rs
│   └── ...
├── assets/
│   └── ...
├── screenshots/        # For gallery page
│   ├── desktop.png
│   └── mobile.png
└── .github/
    └── workflows/
        └── deploy.yml  # Auto-deploy on push
```
