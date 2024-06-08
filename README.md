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

- `/game` for game mode
- `/lib` or `/train` for just learning and no quizzing.
- Additionally, perhaps game mode should include learning stuff between challenges. Anything you get quizzed on should have been presented at some point beforehand. Maybe this is configurable?
  - Choose your adventure!
    - [Learn] [Mixed] [Quiz]
    - [No pressure! Just learn the birds in this pack with no challenges.]
    - [Learn these birds as the challenges progress.]
    - [Think you know all these bird calls already? Jump right in!]
- Maybe you can create/share your own bird packs with premium subscription.
- Maybe catalog, filterable, with "bird cards" that each have a play/pause button. (def would want preload=metadata for that).
- `/ambience` with a playlist of birds.
  - per dan: maybe option for river background and random intervals between bird calls
- Option to show picture of the bird, but no points awarded in this mode.
- Show sonogram animation
- achievements, streaks, etc.
- can I do a "tour" with context state? and once tour is marked as skipped or finished, those elements go away?
- #46764e is a great color for text

#### settings ideas

- out of 2 vs out of 4 (less points for 2)
- possibly: show img on mystery bird instead of choices (less points)
- autoplay: auto / auto after first / off
- loop: on/off

- https://dioxuslabs.com/learn/0.5/reference/context#using-shared-state use context for settings

### mvp todo

MVP should probably just be a game demo using local storage, no user identity, database, etc.

- [ ] change the flip text (identified / streak is confusing here, it seems like overall streak)
  - three dots to fill green would be nice
- [ ] Display level/xp nicely (in header?)
- [ ] Awards? levels? badges?
- [ ] Run through a11y tool; looks like at least a bunch of labels are needed
- [ ] Simplify all the responsive designs, just assume sm > mobile, md > tablet.

#### Bonus

- [ ] Duolingo also has temporary text "2 in a row!"
- [ ] Exiting / navigating away should present the user with a confirm modal:
  - "Quit and you'll lose your current progress!"
- [ ] Settings
  - Create a client-side settings context using local storage
  - Implement game settings like confirm choice, autoplay, etc.
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

# business

### Freemium

- Free app with 20 birds, 1 song/call each
- Additional 30-bird packs $5/each for life
- $3/month for access to everything if paying by year
- Note: for life options probably require downloading bird packs rather than streaming, to limit server costs.
- Naming: bird pack? bevy? flock?

### Cost to run

- Maybe it makes more sense to drop fullstack. Instead, just build frontend (web, mobile) and initially render with placeholders that `use_server_future()` against `supabase {pg, auth}`.

- See linode https://www.linode.com/products/object-storage/
  - and digital ocean https://www.digitalocean.com/products/spaces
  - similar offerings, DO being .005 / GB more expensive on > 1TB transfer
  - Can I use these to just host the static site? I guess if I want "fullstack" I need to run server as well...
  - Too bad bc supabase would be free, could just hit them instead of running my own server.
- can i leverage supabase edge functions? they have to be typescript...

### Realistically

Just do $3/month for everything. Otherwise you just get demo (maybe you also get
bird pack of the day).

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
