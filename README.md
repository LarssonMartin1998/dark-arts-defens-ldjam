# Ludum Dare 55 [entry](https://ldjam.com/events/ludum-dare/55/dark-arts-defense (Linux, and Windows binaries available))

Please judge this for what it is, my first time using Bevy, and the first "real" time using Rust outside of examples and exercises. The code base is filled with shortcuts, and workarounds, and every commit comes with new learnings about Bevy and Rust, and the final few commits were minutes before deadline...

The gameplay code is completely data oriented and multithreaded, I had to implement these systems from scratch:

* Flipping through a Texture Atlas for animated sprites
* Animation States, as well as having animations that we guarantee to play through before changing to another.
* Behavior Tree, and a state machine to switch behaviors
* The following AI behaviors:
  * Idle
  * MoveTowarsOrigo
  * Chase
  * Flee
  * Attack
* Support for restarting the game loop without exiting the application and starting it up again.(
