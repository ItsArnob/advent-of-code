console.time("It took");

const fs = require("fs");
const binaryData = fs.readFileSync("./data/day-3.txt").toString().split("\n");
let oxygenRateInBits = binaryData;
let co2RateInBits = binaryData;
let gammaRateInBits = "";
let epsilonRateInBits = "";

function getNumFromBinary(binary) {
    
    const onesNzeros= binary.split("");
    let power = onesNzeros.length;
    let result =  0;
    
    onesNzeros.forEach(oneOrZero => {
        power--;
        result += oneOrZero * Math.pow(2, power);
        
    });
    return result;
   
};

function getCommonBitFromNthPos(n, binaries = binaryData) {
    
    let nthBits = [];
    let zerosOccurances = 0;
    let onesOccurances = 0;
    
    for (let i = 0; i < binaries.length; i++) {
        nthBits.push(binaries[i].charAt(n));
    };
    
    nthBits.forEach(bit => {
        if(bit === "0") zerosOccurances++;
        else onesOccurances++;
    });
    
    return { 
        mostCommonBit: String(Number(onesOccurances > zerosOccurances)),
        p2MostCommonBit: onesOccurances == zerosOccurances
                         ? "1"
                         : String(Number(onesOccurances > zerosOccurances)),
        p2LeastCommonBit: onesOccurances == zerosOccurances
                         ? "0"
                         : String(Number(onesOccurances < zerosOccurances)),
        zerosOccurances, 
        onesOccurances
    };
    
};

for(let i = 0; i < binaryData[0].length; i++) {
    gammaRateInBits += getCommonBitFromNthPos(i).mostCommonBit;
};

for(let i = 0; true; i++) {
    
    if(oxygenRateInBits.length > 1) {
    oxygenRateInBits = oxygenRateInBits.filter(item => item.charAt(i) == getCommonBitFromNthPos(i, oxygenRateInBits).p2MostCommonBit)
    };
    
    if(co2RateInBits.length > 1) {
    co2RateInBits = co2RateInBits.filter(item => item.charAt(i) == getCommonBitFromNthPos(i, co2RateInBits).p2LeastCommonBit)
    };
    if(oxygenRateInBits.length == 1 && co2RateInBits.length == 1) break;
    if(oxygenRateInBits.length == 0 || co2RateInBits.length == 0) {
        console.log("This shouldn't have happened! make sure the day 3 data is correct.");
        return;
    };
};

gammaRateInBits.split("").forEach(bit => epsilonRateInBits += bit == "1" ? "0" : "1");


console.log(`Part one: ${getNumFromBinary(gammaRateInBits) * getNumFromBinary(epsilonRateInBits)}`);
console.log(`Part two: ${getNumFromBinary(oxygenRateInBits[0]) * getNumFromBinary(co2RateInBits[0])}`);

console.timeEnd("It took");