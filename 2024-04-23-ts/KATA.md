# Kata: Mars Rover

_This is a Markdown version of the [instructions at KataLog](https://kata-log.rocks/mars-rover-kata)_.

> ⚠ This is an **incremental kata** — No peeping ahead!

## Instructions

You're part of the team that explores Mars by sending remotey controlled vehicles to the surface of the planet.

Develop an API that translates the commands sent from Earth to instructions that are understood by the rover.

### Requirements

- [x] You are given the initial **starting point** `(x, y)` of a rover and the **direction** `(N, S, E, W)` it is facing.

- [ ] The rover receives a character sequence of **commands**.

- [ ] Implement the commands that **move** the rover **forward/backward** `(f, b)`.

- [ ] Implement the commands that **turn** the rover **left/right** `(l, r)`.

- [ ] Implement **wrapping** at edges. But be careful, planets are spheres.

- [ ] Implement **obstacle detection** before each move to a new terrain cell. If a given sequence of commands encounters an obstacle, the rover moves up to the last possible point, aborts the sequence and **reports** the obstacle.

### Rules

- Hardcore test-driven development.

- No red phases while refactoring.

- Be careful about edge cases and exceptions. We cannot afford to lose a Mars rover just because the developers overlooked a null pointer.
