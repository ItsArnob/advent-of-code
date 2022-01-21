console.time("It took")

const fs = require("fs");

const inputData = fs.readFileSync("./data/day-4.txt").toString().split("\n");
const nums = inputData[0].split(",");

inputData.splice(0, 1);
const boards = [];
let currentBoardIndex = 0;
let inputDataLength = inputData.length;
let winningNumber = 0;
let won = 0;

for (let i = 0; i < inputDataLength; i++) {
    if (!boards[currentBoardIndex]) {
        boards.push({
            boardIndex: currentBoardIndex, rowsNcolumns: []
        });
    } else {
        let e = [];
        inputData[0].split(/ +/).forEach(a => a.trim() && e.push(Number.parseInt(a)));
        boards[currentBoardIndex].rowsNcolumns.push(e); // just add the rows for now
    };
    inputData.splice(0, 1)
    if (!inputData[0]?.trim()) {

        const rows = boards[currentBoardIndex].rowsNcolumns;
        const columns = [];

        for (let avar = 0; columns.length < rows.length; avar++) {
            let column = [];
            for (let rowIndex = 0; rowIndex < rows.length; rowIndex++) {
                column.push(rows[0 + rowIndex][0 + avar]);
            }
            columns.push(column);
        }
        boards[currentBoardIndex].rowsNcolumns.push(...columns);

        currentBoardIndex++;
    }
}

labelsAreInteresting:
for (num of nums) {

    for (let boardIndex = 0; boardIndex < boards.length; boardIndex++) {
        const rowNcolumn = boards[boardIndex].rowsNcolumns;
        for (let rowNcolumnIndex = 0; rowNcolumnIndex < rowNcolumn.length; rowNcolumnIndex++) {
            let indexOf = rowNcolumn[rowNcolumnIndex].indexOf(Number.parseInt(num));
            if (indexOf != -1) {
                boards[boardIndex].rowsNcolumns[rowNcolumnIndex].splice(indexOf, 1);
            }
        }
        for (iDontKnowWhatToNameThis of rowNcolumn) {
            if (!iDontKnowWhatToNameThis.length) {
                
                won++;

                boards[boardIndex].rowsNcolumns.splice(0, 5);
                
                let sumOfWinningBoard = 0;
                for (let item of boards[boardIndex].rowsNcolumns) {

                    item.forEach(number => sumOfWinningBoard += number);

                };
                winningNumber = Number.parseInt(num);
                if(won == boards.length) {
                    console.log("part two:", winningNumber , sumOfWinningBoard)
                    
                }
                if(won == 1) {
               

                console.log("Part one:", sumOfWinningBoard * winningNumber);
                /*
                | It was at this point when i realized that nesting the columns and rows in the same array was a
                | bad idea and Im too tired to refactor all of this. So Im just gonna use the first 5 arrays
                | from rowsNcolumns.
                */
                /*
                boards[boardIndex].rowsNcolumns.forEach(item => {
                   if(!item) return;
                   item.forEach(number => sumOfWinningBoard += Number.parseInt(item));
                });
                */
                
                

                
                } //break labelsAreInteresting;
            };
        };

    };
};

console.timeEnd("It took");