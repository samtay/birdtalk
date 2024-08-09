set dotenv-load

# watch server
watch-server platform='web':
  #!/usr/bin/env bash
  set -euxo pipefail
  cd app
  dx serve --platform {{platform}} --features {{platform}} --port 3000

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

  # TODO this should all just move to seed.rs. or \copy to temp table and then resolve conflicts in seed.rs

  # copy birds
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
      from program 'cat $SEED_DIR/packs.json | jq -r \".[] | {pack: .name, bird: .birds[]} | [.pack, .bird] | @csv\"'
      csv" || true
  # copy courses
  psql "$DATABASE_URL" -c \
    "\copy courses (name, description, free)
      from program 'cat $SEED_DIR/courses.json | jq -r \".[] | [.name, .description, .free] | @csv\"'
      csv" || true
  # copy course/pack relations
  psql "$DATABASE_URL" -c \
    "\copy course_pack (course, pack, index)
      from program 'cat $SEED_DIR/courses.json | jq -r \".[] | {course: .name, packs: .packs | to_entries[] | {index: .key, pack: .value}} | [.course, .packs.pack, .packs.index] | @csv\"'
      csv" || true

  cargo run -p birdtalk-data --bin seed

expo-android:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd native
  npx expo run:android
