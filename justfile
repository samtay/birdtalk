set dotenv-load

# watch server
initial-setup:
  #!/usr/bin/env bash
  cd app
  npm install

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

# build SSG site
build-ssg:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd app
  dx clean
  dx build --profile release --platform static-generation --features static-generation
  cd ..
  ./dist/birdtalk
  cp -r ./static/* ./dist/public
  rm -rf ./static
  cd dist/public
  sed -i 's/<html>/<html lang="en">/' index.html
  sed -i 's/<html>/<html lang="en">/' birds/index.html
  sed -i 's/<html>/<html lang="en">/' play/index.html

# serve SSG site
serve-ssg:
  miniserve --spa --index index.html --port 3000 ./dist/public

# build and serve SSG site
build-and-serve-ssg:
  just build-ssg
  just serve-ssg

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

  cargo run -p birdtalk-data --bin seed

expo-android:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd native
  npx expo run:android
