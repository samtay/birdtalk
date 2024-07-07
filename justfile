set dotenv-load

# watch all (server, tailwind)
watch:
  #!/usr/bin/env -S parallel --shebang --ungroup --jobs {{num_cpus()}} --retry-failed
  just watch-tailwind
  just watch-server

# watch server
watch-server platform='fullstack':
  #!/usr/bin/env bash
  set -euxo pipefail
  cd app
  dx serve --platform {{platform}} --features {{platform}}

# watch tailwind
watch-tailwind:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd app
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

# deploy linode
deploy-linode:
  rsync --compress --recursive --verbose --human-readable --progress --rsh ssh --exclude target --exclude app/.dioxus --exclude app/dist ~/code/birdtalk linode:
  ssh linode ~/.cargo/bin/just --justfile ~/birdtalk/justfile _deploy-linode-server-cmds >/dev/null 2>&1 &

_deploy-linode-server-cmds:
  #!/usr/bin/env bash
  cd app
  touch server.log server.log.old
  cat server.log >> server.log.old
  pkill -xf 'dx serve --platform fullstack --features fullstack --release --port 3000' || true
  pkill -xf '/home/sam/birdtalk/app/dist/birdtalk' || true
  PATH=~/.cargo/bin:$PATH dx serve --release --port 3000 > server.log

# bring supabase up locally 
supabase-up:
  supabase start
  supabase db reset
  just supabase-seed

# seed supabase with bird data and media
supabase-seed env='local':
  #!/usr/bin/env bash
  set -euxo pipefail

  if [[ '{{env}}' != "local" ]]; then
    . '.env.{{env}}'
    # using pw env variable seems to hang supabase, so just pass it for now
    supabase link --project-ref "$SUPABASE_PROJECT_ID" --password "$SUPABASE_DB_PASSWORD"
  fi

  # copy birds
  # TODO this should all just move to seed.rs. or \copy to temp table and then resolve conflicts in seed.rs
  psql "$DATABASE_URL" -c \
    "\copy birds (scientific_name, common_name)
      from program 'cat $SEED_DIR/birds.json | jq -r \".[] | [.scientific_name, .common_name] | @csv\"'
      csv" || true
  # copy packs
  psql "$DATABASE_URL" -c \
    "\copy packs (name, description, free)
      from program 'cat $SEED_DIR/packs.json | jq -r \".[] | [.name, .description, .free] | @csv\"'
      csv" || true
  # copy bird/pack relations
  psql "$DATABASE_URL" -c \
    "\copy bird_pack (pack, bird)
      from program 'cat data/seed/packs.json | jq -r \".[] | {pack: .name, bird: .birds[]} | [.pack, .bird] | @csv\"'
      csv" || true
  cargo run -p birdtalk-data --bin seed
