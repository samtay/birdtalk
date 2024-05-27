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

### error handling

- use https://dioxuslabs.com/learn/0.5/cookbook/error_handling#throwing-errors
  to have an error msg section up top. or context?

### mvp todo

MVP should probably just be a game demo using local storage, no user identity, database, etc.

- game loop

  - handle game finished (all birds learned)

- animations

  - success animation:
    - Change card flip to be a modal. Animate cards in/out from corners of
      screen between challenges. As card flies out, modal comes in with next
      button and stats. Modal should slide from bottom on mobile.
  - See https://codepen.io/designcouch/pen/obvKxm for modal animations
  - Keep in mind, many of this could possibly be purely tailwind css driven:
    - https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-sibling-state
    - by using peer classes to determine when modal pops up, etc.
    - or even set class on parent when correct choice has been made, then do
      - group-[correct-choice]:animate-out
      - or e.g. if initial opacity 0 and then peer:success:animate-modal-in
  - Another idea: drawing outline of bird? Like the logo?
    - https://css-tricks.com/almanac/properties/s/stroke-dashoffset/
    - TODO: export logo bird as SVG! with transparency!
  - see https://www.joshwcomeau.com/animation/css-transitions/ for tips on
    finishing touches and leveraging GPU for smooth transitions
    and apply https://tailwindcss.com/docs/will-change for any perf problems

- settings

  - create a client-side settings context using local storage
  - implement game settings like confirm choice, autoplay, etc.

- remember: use ? none propagation to hide elements. also see above for error handling

- fix header

  - get the commit from the saved branch. export the header with the nice font
    from canva, resized to fit the word and with transparent background.
  - then export the birdtalk icon as SVG
    - or, export both the talk bubble and bird separately
  - then animate the bird portion of the svg with css! (could be .gif still but smaller footprint)

- add 'space' or other handler to toggle audio

  - see docsite/src/shortcuts.rs
  - and
    https://github.com/DioxusLabs/dioxus/blob/e2002d6ea42f5844a3832ab7f038620ecf977a1c/packages/desktop/src/hooks.rs#L72
    for desktop

- make svg component for audio toggle

- maybe add gradients https://tailwindcss.com/docs/gradient-color-stops nice
  look

- if html necessary:
  depending on what you wanna do you can also maybe (depends on your use case) put something like this in your input.css
  ```css
  html,
  body {
    @apply h-full overflow-hidden;
  }
  ```

# business

### Freemium

- Free app with 20 birds, 1 song/call each
- Additional 25-bird packs $5/each for life
- Or enchilada full North American 800+ bird pack for $100 for life
- $10/month for access to everything
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
