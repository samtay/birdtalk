# birdtalk

### dependencies

- [rust](https://rustup.rs/)
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

# resources

1. [Supabase?]()

- Can also self host on e.g.
  [digital](https://docs.digitalocean.com/developer-center/hosting-supabase-on-digitalocean/)
  [ocean](https://www.youtube.com/watch?v=dDhy6pk282U) ([another
  resource](https://medium.com/@kelvinpompey.me/self-hosting-supabase-on-ubuntu-and-digital-ocean-9bff8a819250)).

100. Possibly useful in the future https://docs.rs/anim/latest/anim/

### xeno-canto

Cornell said fuck me, so XC it is. There's a lot of recordings, will need to
optimize a [search](https://xeno-canto.org/help/search).

- length (maybe 6-12s)
- license:

Can even box a location and filter by a box of lat/long coordinates, however it
might make more sense to leverage Merlin or eBird to list out birds of a given
region, and then query for them specifically.

Will likely need an `xc-indexer` tool that hits their API repeatedly and builds
out a database. It will need to sleep `1s` between requests to avoid the
throttle.

For MVP, can probably just their hosted audio for now, but eventually will want to index this and store it myself, so that it is easily cropped.

### deployment

See
[https://discord.com/channels/899851952891002890/1236011566910935060/1236015208175243274](this discord post) for distributing server assets/binary.

### national audubon society collection

- Waiting to hear back from [Lang](https://musicofnature.com/audubon-bird-songs/)

# game

### roadmap

- Maybe you can create/share your own bird packs with premium subscription.
- Maybe catalog, filterable, with "bird cards" that each have a play/pause button. (def would want preload=metadata for that).
- `/ambience` with a playlist of birds.
  - per dan: maybe option for river background and random intervals between bird calls
- Option to show picture of the bird, but no points awarded in this mode.
- Show sonogram animation
- achievements, streaks, etc.
- can I do a "tour" with context state? and once tour is marked as skipped or finished, those elements go away?
- #46764e is a great color for text
- for separate server/frontend deployment, see https://discord.com/channels/899851952891002890/1251248438482440302 and e.g. https://github.com/DioxusLabs/dioxus/blob/487570d89751b34bbfd5e9b5ff1e0fd3780bf332/packages/fullstack/examples/axum-desktop/Cargo.toml#L20-L28
- probably want mostly CSR and hopefully avoid backend hosting. pre-rendered SSG for speed?

#### settings ideas

- game:
  - out of 2 vs out of 4 (less points for 2)
  - possibly: show img on mystery bird instead of choices (less points)
  - autoplay: auto / auto after first / off
  - loop: on/off
  - confirm choice: y/n
- listen:
  - how many times each bird is played default 1,
  - auto skip after \_, etc.

### todo

- [ ] Revise justfile to use environment variables
- [ ] Ensure .env, github secrets, and justfile are consistent!
- [ ] Call supabase-seed from staging CI

- [ ] Edit `staging.yaml` to build staging and deploy to static branch
- [ ] Push `develop` to push staging
- [ ] Update `production.yaml` to match staging once working.
- [ ] Need outer "courses" to order packs.
- [ ] Make `app` the root package of the workspace? Then don't need to specify `-p` and probably works better with `dx`
- [ ] Buy birdtalk.xyz
- Potentially one of these backends: https://aromatic-guava-53f.notion.site/Deploying-To-The-Cloud-6f6ce065f1aa4b43a6e4dca66d0c8f7e
- [ ] Host the parsed data on supabase (data + storage)
  - Get started with local CLI, staging env, etc.
- [ ] Auth via supabase, client side! (try copying t5 logic, make lib)
  - https://supabase.com/docs/guides/cli/managing-config
- [ ] Run through a11y tool; looks like at least a bunch of labels are needed
- [ ] Simplify all the responsive designs, just assume sm > mobile, md > tablet.
- [ ] Make a landing (web-only) www page with links to Login (send to app) + app stores
  - This should be an SSG! with any necessary splash screen
- [ ] Collections / packs
- [ ] Awards? levels? badges?
- [ ] Get in touch with Lang Elliot!
- [ ] Decide pack/course structure
  - Maybe you choose the course by category:
    - Category: Region
      - Course: East coast, west coast, etc.
        - Pack: EC common 1, EC common 2, etc.
    - Category: Habitat
      - Course: Coasts and shorelines, Urban and Suburban Habitats,
        - Pack: Coastal 1, Coastal 2, etc.
    - Category: Commonality
      - Course: Common, Uncommon, Rare, etc.
        - Pack: ..
    - (these could be user defined as well)
- [ ] Settings: game, listen, other. (local storage only?)
- [ ] Subscription/payments (stripe - wrapper around pg via supabase): https://supabase.github.io/wrappers/
- [ ] Will need actual emails: https://supabase.com/docs/guides/auth/auth-smtp
- [ ] Go through db stuff: https://supabase.com/docs/guides/database/tables?queryGroups=database-method&database-method=sql&queryGroups=language&language=js#bulk-data-loading

#### bonus

- [ ] Duolingo also has temporary text "2 in a row!"
- [ ] Exiting / navigating away should present the user with a confirm modal:
  - "Quit and you'll lose your current progress!"
- Take a cue from Duolingo: modal instead of flip on mobile
  - Keep cards on screen, but highlight correct with green (and some subtle
    animation, maybe star(s) appear and then {fade,rotate, translate})
  - slide modal up on mobile
  - This will fix the sizing issue on the flip card, as the button will no
    longer be on the card. Maybe have room to describe the sound just heard
    (mating call, song, etc.)

### later

- storage

  - what is stored where? I have access to local storage (not on mobile currently)
  - what about external DB?
  - don't want to lock into situation where user needs internet connection to use the app..
  - so try to stick most in local storage, and attempt to sync up when I can?
    - this has the con that mobile gets pushed out... can offer mobile sooner with db backing only?
    - maybe I can swap out storage implementation with db backing...
  - maybe for now we have our own central storage API:
    - and it just uses dioxus sdk for web
    - and an enum of some sort representing connection status
    - birdpacks can be accessed in offline mode
    - progress will be persisted with local storage, with syncs attempted periodically
      - merge should be mostly simple and additive (learned birds for example)
      - however identified count will have to take into account delta since updated datetime, etc.

- shortcuts

  - add 'p/a' or other handler to toggle audio
  - see docsite/src/shortcuts.rs
  - and
    https://github.com/DioxusLabs/dioxus/blob/e2002d6ea42f5844a3832ab7f038620ecf977a1c/packages/desktop/src/hooks.rs#L72
    for desktop

- error handling

  - use https://dioxuslabs.com/learn/0.5/cookbook/error_handling#throwing-errors to have an error msg section up top. or context?

- performance

  - see https://www.joshwcomeau.com/animation/css-transitions/ for tips on
    finishing touches and leveraging GPU for smooth transitions
    and apply https://tailwindcss.com/docs/will-change for any perf problems

- fix header

  - export bird / text from canva separately

- header: the current large version should only exist on a landing page (that we don't have)
- use that space for something else - XP, etc.

- can replace hacky tailwind landscape with `use_window_size`
  - Math.min(window.screen.width, window.screen.height) < 768
- or `@media only screen and (max-height: 575.98px) and (orientation: landscape)`
- also, just disallow landscape mode on mobile!

- try moving modal to top level so that the background can swap while blurred
- can apply "inert" to the main content to disable focusing within there whenever modal's open
  - unfortunately there doesn't seem to be a way to escape inert-ness on children
- requires opening a modal by storing component/element in a signal of some sort

  - also would allow a smoother transition while background blurred, although
    this could also be done by just putting the static modal backdrop in the root.

- Naming: bird pack? bevy? flock?

- Allow users to contribute birds by submitting PRs with json? Script to import
  these via supabase?

- Set up staging sb project: https://supabase.com/docs/guides/cli/managing-environments?queryGroups=environment&environment=staging

- For tuning later https://supabase.com/docs/reference/cli/supabase-inspect-db-vacuum-stats

- Tests? https://supabase.com/docs/guides/cli/github-action/testing

- Can play multiple calls at once for a hard mode

- Display level/ xp somewhere!?

# business

### Freemium

- Free app with 20 birds, 1 song/call each
- Maybe "Daily Dabble" is free?
  - Maybe pack of the day increases in difficulty Mon - Fri
- $1/month?
- $10/year?
- $30/life with offline mode

- Maybe _everything_ is free to start?

### Cost to run

- Maybe it makes more sense to drop fullstack. Instead, just build frontend (web, mobile) and initially render with placeholders that `use_server_future()` against `supabase {pg, auth}`.

- See linode https://www.linode.com/products/object-storage/
  - and digital ocean https://www.digitalocean.com/products/spaces
  - similar offerings, DO being .005 / GB more expensive on > 1TB transfer
  - Can I use these to just host the static site? I guess if I want "fullstack" I need to run server as well...
  - Too bad bc supabase would be free, could just hit them instead of running my own server.
- can i leverage supabase edge functions? they have to be typescript...

# template readme

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Fullstack app:

```bash
dx serve --platform fullstack
```
