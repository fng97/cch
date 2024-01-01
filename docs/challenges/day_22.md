Shuttle's Christmas Code Hunt
=============================

[Info](https://www.shuttle.rs/cch) • [Scoreboard](https://www.shuttle.rs/cch#scoreboard)

* * *

[< Back to main page](/cch)

🎄 Day 22: Dawn of the day before the day before the final day
==============================================================

_When Christmas Eve rolls around, it's game time. Santa, decked out in his jolly red suit, strides over to his fully-stacked sleigh. His eyes twinkle as he checks his list one last time on his high-tech sleigh dashboard. The backend has been working like a charm after the recent upgrades._

_With a crack of his whip and a hearty "Ho ho ho," off they go into the snowy night. The reindeers take off like an orange space rocket, disappearing into the starry sky, on a journey to deliver joy to the world!_

❤️ A Big THANK YOU
-------------------

We at Shuttle want to give a big Thank You for participating in this Code Hunt - our first one ever, and hopefully not the last. We are amazed at the response and the amount of interest and signups.

Even though we had a rough time on December 1 and had to scope down the event, we love the enthusiasm that we have seen from participants. We learned a lot and will continue working on making the Shuttle platform better in many ways!

These challenges will stay up for your enjoyment. They turned out to be a pretty good tutorial for Rust web development and Shuttle, so feel free to challenge your friends to solve them.

_We still want to reward everyone who completed all the challenges - stay tuned for announcements regarding deployment and online validation of your solutions!_

⭐ Task 1: Leave no gift behind
-------------------------------

During a last minute database migration in the gift order database, Santa noticed that a small de-sync happened. One gift order slipped through the cracks and only ended up in one of the database replicas. Since it's already Dec 22nd, Santa tells you we need to recover the lost record immediately. No child must be left without a gift this Christmas!

When Santa started extracting all gift order IDs from the database replicas, something got jumbled up and caused them to print in a random order. Great... now we have two long lists of random numbers with just one number differing between them. Santa knows you are good at coding, so he concatenates the two files, scrambles the order again and lets you find the integer without a pair.

Make a POST endpoint `/22/integers` that takes in a text body with one positive `u64` integer on each line. All integers appear twice, except for one. Find it and respond with a string consisting of that number of Present emojis (🎁).

_Food for thought: How memory efficient can you make your solution? Is it possible to make the integer filtering non-allocating?_

### 💠 Example

    curl -X POST http://localhost:8000/22/integers \
      -H 'Content-Type: text/plain' \
      -d '888
    77
    888
    22
    77
    '
    
    🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁

🎁 Task 2: The Shuttle Rocket (600 bonus points)
------------------------------------------------

When Santa speeds across Earth to deliver presents, he looks up to the skies and sees the Shuttle Rocket taking off to the stars. The crew on the rocket, in their quest to visit the stars at the edge of the CCH23 galaxy, has discovered that some magical portals have opened near every star. The portals allow instant bidirectional travel between stars. This saves the crew a lot of flight time to the outer edge, but now they need to figure out which portals to take in order to get to the destination.

The input is sent as text in a POST request to `/22/rocket` in this format:

* The first line has a number _N_ (`2 <= N <= 100`), the number of stars in the galaxy.
* On the following _N_ lines are the 3D coordinates of each star in the galaxy as three `i32`s.
* Then follows a line with the number _K_ (`1 <= K <= 100`), the number of portals in the galaxy.
* On the following _K_ lines are the stars that each portal connects as two star indices.

The crew wants to travel from star _0_ to star _N-1_ on the path that goes through the least amount of portals, since going through portals make them feel dizzy.

After the path with the least portals has been found, the crew wants to know:

* How many portals did they have to go through?
* How long would the path they took have been if no portals existed? (as an `f32` rounded to 3 decimals)

Remember to not get stuck in an infinite portal loop!

### 💠 Example

    curl -X POST http://localhost:8000/22/rocket \
      -H 'Content-Type: text/plain' \
      -d '5
    0 1 0
    -2 2 3
    3 -3 -5
    1 1 5
    4 3 5
    4
    0 1
    2 4
    3 4
    1 2
    '
    
    3 26.123

Explanation:

There are 5 stars and 4 portals. We can get from star 0 to star 4 by going through these portals:

* portal 0 from star 0 to star 1
* portal 3 from star 1 to star 2
* portal 1 from star 2 to star 4

The path is 0 -> 1 -> 2 -> 4. 3 portals were used. The length of this path without taking any portals would have been `distance(star 0, star 1) + distance(star 1, star 2) + distance(star 2, star 4)` where `distance()` is the distance between two stars.

🎁 Bonus challenge (Completed)
------------------------------

_The mischievous elves have been hard at work hiding words! Santa has noticed that 4 words from the manuscript for his grand Christmas Speech have been erased. The speech will be held on Christmas Eve (before the big trip) in front of everyone at the North Pole, so it would be a great embarrassment to not have the words right._

The **4 secret words** have been hidden in various places across the CCH23 realm. Search for them **everywhere** in the published CCH23 pages and resources.

* The answers are single english words.
* The elves are no amateurs - some words are hidden far outside of direct sight!

**All words have been found.**

* Word 1: Found! ✅
  * Hint 1: Santa's night vision goggles received a software update
  * Hint 2: The update is called "V11.0.0.PX module 8: Enhanced bull vision"
  * Hint 3: Red, module 8
* Word 2: Found! ✅
  * Hint 1: Santa dropped a bag of toys on the floor
  * Hint 2: They landed in an interesting way
  * Hint 3: They landed in a line in a specific order
* Word 3: Found! ✅
* Word 4: Found! ✅

* * *

Author: [jonaro00](https://github.com/jonaro00)

* * *

📗 Validate challenge
---------------------

You can now run our test cases against your locally running project with the [official validator](https://crates.io/crates/cch23-validator)!
