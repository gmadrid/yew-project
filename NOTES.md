# NOTES

## Dist files

Ideally, we want to pack up the distribution so that

1. It can be unzipped in place and work with a simple http server.
2. The path to the webasm/js files is absolute so that the serving page can be moved.
3. The `pkg` directory is versioned so that it can't be skewed by server caching.

So,

```
  dist
    - index.html
    - /pkg-{VERSION}
      - main.js (and/or other js files)
      - code.webasm
```

## Feature list

* Unified UI
  * Add some UI to the Nav bar so that all of the apps are on the same page.
  * Share grids across the various apps.
    * This probably means changing everything to a Color grid.
    * With two-pattern, how do you choose a grid?
* Tile maker
* Allow changing colors to use custom colors.
* Unify the grid display so that you don't DRY.
