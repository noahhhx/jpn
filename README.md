# jpn
host our japan trip

## Run

```sh
devenv shell
cargo leptos watch   # http://127.0.0.1:3000
```

## Edit the itinerary

The content lives in `src/itinerary.rs`:

- `START`: the date of Day 0. Every other date follows from it.
- `DAYS`: one entry per day (city, title, and sections of text, lists, places, links, schedules and images).
- `INFO`: the general info page at `/info`.

Images go in `public/img/` and are referenced as `/img/<name>`. The block types are defined in `src/trip.rs`.

On a trip day (Japan time), `/` redirects to that day's page. `/days` always shows the full list.
