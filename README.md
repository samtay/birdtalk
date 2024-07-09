# birdtalk

### dependencies

- [rust](https://rustup.rs/)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [supabase cli](https://github.com/supabase/cli/) (and a free account)

### development

#### Initial setup

- Log into supabase [locally](https://supabase.com/docs/reference/cli/supabase-login)
- `supabase start`
- `cp .env.example .env` and fill in the anon key found in `supabase status`.

#### Start up

- `just supabase-up`: Start up local supabase with some seed data
- `just watch-tailwind`: Start tailwind watcher
- `just watch-server`: Start dioxus app running at `localhost:8080` with hot reload
