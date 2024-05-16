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

# game

### roadmap

- Eventually, `/game` for game and `/lib` or `/train` for just learning and no quizzing.
- Option to show picture of the bird, but no points awarded in this mode.

#### settings

- out of 2 vs out of 4 (less points for 2)
- possibly: show img on mystery bird instead of choices (less points)
- autoplay: auto / auto after first / off
- loop: on/off

- https://dioxuslabs.com/learn/0.5/reference/context#using-shared-state use context for settings

### error handling

- use https://dioxuslabs.com/learn/0.5/cookbook/error_handling#throwing-errors
  to have an error msg section up top. or context?

### mvp
