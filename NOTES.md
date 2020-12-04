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
