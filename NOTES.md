# NOTES

## TODO

* Restructure
  * pull the lib out into a separate Github repo
  * break up the apps into separate projects
* Metapixel
* Undo

## Undo Design

The UI model is very simple, and I think that we can get a lot of benefit for just implementing Undo for each grid. Each grid will have its own Undo queue and UI.

The Undo stack doesn't need to persist.

I think that we can do this by adding an `Undoable` trait, then implementing a grid wrapper that implements both `GridTrait` and `Undoable`.

```
trait Undoable {
  fn checkpoint();
  fn undo();
  fn redo();
}
```

Both serialization and the ownership model present some issues. We want to serialize the backing grid, not the Undo grid. This means that we want the backing grid to exist in the Stored struct, but `UndoableGrid` will require a _mutable_ ref to the grid. This may necessitate using the `Rc<RefCell>` approach. 


## Future project structure

Having all of the various projects together is becoming less convenient (mostly due to build speed) with little or no specific benefit. Things are faster now that we have broken stuff into separate crates, but each app adds more and more delays.

I think we should pull all of the library code into a separate Github project, then set up a project for each app that uses the libraries.


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
