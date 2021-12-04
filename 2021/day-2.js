const fs = require("fs");
const directions = fs.readFileSync("./data/day-2.txt").toString().split("\n");

/* 
| p1* = variables for part one of the puzzle
| p2* = variables for part two of the puzzle
*/
let p1DepthPos = 0;
let p1p2HorizontalPos = 0;

let p2PrevAimPos = 0;
let p2PrevDepthPos = 0;
let p2FinalDepthPos = 0;

for (let i = 0; i < directions.length; i++) {
    let [direction, unit] = directions[i].split(" ");
    unit = Number(unit);
    
    switch(direction) {
        case "forward":
            p1p2HorizontalPos += unit;
            if(p2PrevAimPos) {
                p2FinalDepthPos += unit * p2PrevAimPos;
            } else {
                p2PrevDepthPos = unit;
            };
        break;
        case "up": 
            p1DepthPos -= unit;
            p2PrevAimPos -= unit;
        break;
        case "down":
            p1DepthPos += unit;
            p2PrevAimPos += unit;
        break;
    }
    
}
console.log("Part one: ");
console.log(`Final horizontal position multiplied by the final depth position: ${p1p2HorizontalPos * p1DepthPos}`);

console.log("Part two:");
console.log(`Final horizontal position multiplied by the final depth position: ${p1p2HorizontalPos * p2FinalDepthPos}`)
