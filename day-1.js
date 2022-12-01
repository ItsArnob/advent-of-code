const fs = require("fs");
const nums = fs.readFileSync("./data/day-1.txt").toString().split("\n");
let incremented = 0;
let threeMeasurementWindowIncrements = 0;

for (let i = 0; i < nums.length; i++) {
    
    if(i && Number(nums[i]) > Number(nums[i -1])) incremented++;
    
    if(nums[i + 2]) {
        const prevWindow = Number(nums[i]) + Number(nums[i + 1]) + Number(nums[i + 2]);
        const nextWindow = Number(nums[i + 1]) + Number(nums[i + 2]) + Number(nums[i + 3]);
        
        if(prevWindow < nextWindow) threeMeasurementWindowIncrements++;
    };
};

console.log(`Part one: It incremented ${incremented} times.`);
console.log(`Part two: It incremented ${threeMeasurementWindowIncrements} times.`);