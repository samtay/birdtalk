# birdtalk

### dependencies

- [rust](https://rustup.rs/)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [supabase cli](https://github.com/supabase/cli/) (and a free account)
- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- dx: `cargo install dioxus-cli` (or `cargo binstall` if you have it)

### development

#### Initial setup

- Run `just initial-setup`.
- Log into supabase [locally](https://supabase.com/docs/reference/cli/supabase-login).
- Run `supabase start`.
- `cp .env.example .env` and fill in the anon key found in `supabase status`.

#### Start up

- `just supabase-up`: Start up local supabase with some seed data
- `just watch-server`: Start dioxus app running at `localhost:3000` with hot reload
