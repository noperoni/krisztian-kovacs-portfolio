# Kovács Krisztián Géza - Portfolio

Personal portfolio website built with Rust, Leptos, and Axum.

**Live Site**: [https://kovacs.pilgrim.ovh](https://kovacs.pilgrim.ovh)

## Tech Stack

- **Frontend**: [Leptos](https://leptos.dev/) (Rust → WebAssembly)
- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust async web framework)
- **Database**: PostgreSQL 16
- **Styling**: SCSS with CSS custom properties
- **Blog**: Markdown with build-time compilation (pulldown-cmark + syntect)

## Features

### Three Visual Themes
- **Forge** - Industrial, bold Bebas Neue typography
- **Terminal** - Warm CRT vibes, JetBrains Mono, cozy hacker aesthetic
- **Bitart** - Press Start 2P font, RPG-inspired pixel art style

### Internationalization
- Full English and French translations
- Language persisted in localStorage

### Color Modes
- Light and dark mode for each theme
- Respects system preference, user-overridable

### Blog System
- Markdown files compiled at build time
- Syntax highlighting with theme-matched colors
- Tag filtering and pagination
- Bilingual posts (EN/FR in single file)

### Contact Form
- Server-side validation
- PostgreSQL storage
- Email notifications via SMTP
- Honeypot spam protection
- Rate limiting

### GitHub Integration
- Live repository stats from GitHub API
- Stale-while-revalidate caching

## Pages

- **About** - Professional introduction and skills
- **CV** - Interactive resume with print styles
- **Projects** - Filterable project showcase
- **Blog** - Technical articles

## Development

### Prerequisites
- Rust (stable)
- cargo-leptos
- PostgreSQL 16+

### Setup
```bash
# Install cargo-leptos
cargo install cargo-leptos

# Copy environment template
cp .env.example .env
# Edit .env with your database credentials

# Run migrations
sqlx database create
sqlx migrate run

# Start development server
cargo leptos watch
```

### Build for Production
```bash
cargo leptos build --release
```

## License

Copyright (c) 2025 Kovács Krisztián Géza. All Rights Reserved.

See [LICENSE](LICENSE) file for details.

## Contact

- **LinkedIn**: [Krisztián Géza Kovács](https://www.linkedin.com/in/kriszti%C3%A1n-g%C3%A9za-kov%C3%A1cs-2b72251a2/)
- **Email**: kovacs@pilgrim.ovh
