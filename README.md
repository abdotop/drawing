# drawing

The purpose of this exercise is to create an image like the example below:

example

You will need to do the following:

- Copy the code in the usage to your `main.rs`.

- Create a module called `geometrical_shapes` in another file.

You'll define the logic for creating and working with shapes in `geometrical_shapes`. Create the following traits:

- `Drawable` which contains the methods `draw` and `color`.

- `Displayable` which contains the method `display`.

Define them according to the way they are called in the `main.rs` function.

In order to compile and run `main.rs`, you'll need to define some structures. You are free to implement all the shapes with whatever internal structure you see fit, but you must provide an associated function `new` for all the shapes, which will be described below:

- `Point`: a new point should be created from two `i32` values.

- `Line`: a new line should be created from references to two different points.

- `Triangle`: a new triangle should be created from references to three different points.

- `Rectangle`: a new rectangle should be created from references to two different points.

- `Circle`: a new circle should be created from a reference to a point representing the center, and an `i32` value representing the circle's radius.

You'll also need to create the associated function `random` for `Line`, `Point`, and `Circle`. You should derive their signatures from the usage.