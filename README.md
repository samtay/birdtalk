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

  - get like 10 legit birds with photos and audio
  - handle game finished (all birds learned)

- animations

  - pulse on error,
  - disable btn on error so user doesn't wrack up mistakes,
  - success animation: spin, green, modal with more info and nice things?

- settings

  - create a client-side settings context using local storage
  - implement game settings like confirm choice, autoplay, etc.
  - get autoplay working with toggle button

- remember: use ? none propagation to hide elements. also see above for error handling

- get fullstack platform working with [Evan's tip](https://discord.com/channels/899851952891002890/1241875862924824658/1241901511030734963)

# business

### Freemium

- Free app with 20 birds, 1 song/call each
- Additional 25-bird packs $5/each for life
- Or enchilada full North American 800+ bird pack for $100 for life
- $10/month for access to everything
- Note: for life options probably require downloading bird packs rather than streaming, to limit server costs.
- Naming: bird pack? bevy? flock?

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
