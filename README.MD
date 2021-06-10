##Jordan Malubay

###Attack of the Blobs (Atob)

Atob is a binary containing a rogue-like game where the player must destroy the blobs on screen and collect the crystals they drop.  Once the play has collected all the crystals they win the game.

To run the game all the user needs to do is execute cargo run from the source folder to build the executable.

```bash
cargo run
```
![Screenshot](/Screen.jpg)

The player can move their character with the W,A,S,D keys, and attack the blobs with the arrow keys.  The safe rooms will randomly spawn around the map every game.  The player can only exit the rooms through the doors, the single lines on the center of each wall. When the player is inside the room the blobs cannot see or attack.  Once the player is outside and in range of the blobs they will move towards the player and attack when they are adjacent.  


###Implementation Notes
Throughout the quarter I have been working on this while going through the [Roguelike Tutorial](https://bfnightly.bracketproductions.com/chapter_0.html) by Herbert Wolverson, to help get a better understanding of Rust and put together my project.  A lot of my original ideas were not possible for the systems from this.  I was not able to fully automate the enemies since the RunState system was waiting for user input to advance.  I tried to make the world run when the user was not inputing but it cause the blobs to immediately destroy the user or the game would lag and become unplayable after a minute.

I really liked working with the Component system from the SPECS library to control what attributes entities have.  It makes it easier to add new features and functions with out entirely changing how each entity is made.  Instead new attributes are added at creation as components.  This makes accessing their attributes a but more work as each function has to be given the data of each component instead of just the entity itself.

If I had more time I would like to finish the game and polish up the functionality.  Right now the player cannot shoot at the blobs as I had intended. I could not figure out a way to draw and animate the lasers on the turn based system the tutorial uses. Since the animation is halted while waiting for the player's next action. I was not able to fully implement the winning conditions so the the only thing that happens is the player runs around and dies with crashes the game since there is no test.  I bit off a bit more than I could chew and did not have enough time to do all this myself. 

[LICENSE](/LICENSE.txt)
