Shuttle's Christmas Code Hunt
=============================

[Info](https://www.shuttle.rs/cch) • [Scoreboard](https://www.shuttle.rs/cch#scoreboard)

* * *

[< Back to main page](/cch)

🎄 Day 8: PokéPhysics
=====================

_In the heart of the North Pole, Santa's workshop buzzes with a new type of magic. A portal has opened, and Pokémon from another world have tumbled into the snow-dusted realm of elves and reindeer. Santa, ever the innovator, sees an opportunity: why not enlist these charming creatures in his annual gift-giving campaign?_

_But before the sleigh bells ring and the Pokémon can join the flight, Santa needs to understand their unique properties and figure out how many can fit into his sleigh, given their weight._

⭐ Task 1: IT'S PIKACHU
-----------------------

Your quest is to add a GET endpoint `/8/weight/<pokedex_number>` that, given a pokédex number, responds with the corresponding Pokémon's weight in kilograms as a number in plain text.

### 🔔 Tips

* [PokéAPI](https://pokeapi.co/)
* [reqwest](https://docs.rs/reqwest/latest/reqwest/)

### 💠 Example

    curl http://localhost:8000/8/weight/25
    
    6

* * *

🎁 Task 2: That's gonna leave a dent (160 bonus points)
-------------------------------------------------------

Once the Pokémon's weight is retrieved, Santa needs you to calculate the momentum it would have at the time of impact with the floor if dropped from a 10-meter high chimney _(so that he knows if he needs to climb down or if he can just drop it)_.

Keep in mind that the gravity of Earth that Santa has in his physics book was measured close to the North Pole. This could explain why his calculations are a bit off sometimes, but he still wants you to use it.

The momentum, measured in Newton-seconds, signifies the force the Pokémon would exert upon meeting the floor beneath the 10-meter high chimney.

The GET endpoint `/8/drop/<pokedex_number>` shall respond with a plain text floating point number.

_Use gravitational acceleration `g = 9.825 m/s²`. Ignore air resistance._

### 💠 Example

    curl http://localhost:8000/8/drop/25
    
    84.10707461325713

_Validation has a fault tolerance of 0.001_

* * *

📗 Validate challenge
---------------------

You can now run our test cases against your locally running project with the [official validator](https://crates.io/crates/cch23-validator)!
