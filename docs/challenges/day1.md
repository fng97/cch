🎄 Day 1: Packet "exclusive-cube" not found
===========================================

_In the frosty expanses of the North Pole, Santa's advanced packet management system has encountered a glitch. This system, known for its robustness and magical speed, is responsible for sorting and dispatching all the Christmas gifts. However, a sudden aurora borealis storm has scattered bits of data across the snowy landscape, leaving Santa in dire need of a digital handyman to restore order._

⭐ Task 1: Cube the bits
-----------------------

Santa needs your programming expertise to recalibrate the packet IDs in his packet management system.

Implement a GET endpoint `/1/<num1>/<num2>` that takes 2 integers in the path, `num1` and `num2`, and returns the result of `(num1 XOR num2) POW 3`, where _XOR_ is the exclusive OR operation, and _POW_ is exponentiation.

### 🔔 Tips

Check your web framework's docs for clues how to match the URI path pattern:

* [Axum Routing](https://docs.rs/axum/latest/axum/#routing)
* [Axum Extractors](https://docs.rs/axum/latest/axum/extract/index.html)
* [Actix Web Extractors](https://actix.rs/docs/extractors)
* [Rocket Dynamic Paths](https://rocket.rs/v0.5/guide/requests/#dynamic-paths)

### 💠 Example

    curl http://localhost:8000/1/4/8
    
    1728

* * *

🎁 Task 2: The sled ID system (100 bonus points)
------------------------------------------------

After the packet IDs are calibrated and the packets are packed into sleds, Santa needs to calibrate the sled ID.

The formula is very similar: All packet IDs (integers) are _XOR_'ed with each other, and then the result is (again) raised to the power of 3. The catch is that there can be between 1 and 20 packets in a sled!

Adapt the endpoint from Task 1 so that it can also be used to calculate sled IDs.

### 💠 Examples

    curl http://localhost:8000/1/10
    
    1000
    

    curl http://localhost:8000/1/4/5/8/10
    
    27
