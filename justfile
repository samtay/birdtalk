# watch server
watch-server:
  dx serve

# watch tailwind
watch-tailwind:
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

# deploy linode
deploy-linode:
  rsync --compress --recursive --verbose --human-readable --progress --rsh ssh --exclude target --exclude .dioxus --exclude dist ~/code/birdtalk linode:
  ssh linode ~/.cargo/bin/just --justfile ~/birdtalk/justfile _deploy-linode-server-cmds >/dev/null 2>&1 &

_deploy-linode-server-cmds:
  touch server.log server.log.old
  cat server.log >> server.log.old
  pkill -xf 'dx serve --release --port 3000' || true
  pkill -xf '/home/sam/birdtalk/dist/birdtalk' || true
  PATH=~/.cargo/bin:$PATH dx serve --release --port 3000 > server.log
