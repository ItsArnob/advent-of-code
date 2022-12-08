let str = `30373
25512
65332
33549
35390`;

let data = str.split("\n");
let visibleTrees = 0;
let score = 0;

data.forEach((line,lineIndex) => {
    
    
    let numbers = line.trim().split("").map(str => Number.parseInt(str));
    if (lineIndex == 0) { 
         visibleTrees += numbers.length;
        return;
    };
    if(lineIndex + 1 == data.length) {
         visibleTrees += numbers.length;
        return;
    }
    numbers.forEach((number, index) => {
        if(index == 0) {
             visibleTrees++;
            return;
        };
        if(index + 1 == numbers.length) {
             visibleTrees++;
            return;
        }
      
        let numbersLeft = numbers.slice(0, index);
        let numbersRight = numbers.slice(index + 1, numbers.length);
        let numbersBottom = data.slice(lineIndex + 1, data.length).map(line => {
            let line2 = line.split("");
            return Number.parseInt(line2[index])
        });
        let numbersTop = data.slice(0, lineIndex).map(line => {
            let line2 = line.split("");
            return Number.parseInt(line2[index])
        });
        let isVisibleBottom = numbersBottom.filter(val => val >= number);
        let isVisibleTop = numbersTop.filter(val => val >= number);
        let isVisibleLeft = numbersLeft.filter(val => val >= number);
        let isVisibleRight = numbersRight.filter(val => val >= number);

        if (isVisibleBottom.length == 0 || isVisibleTop.length == 0 || isVisibleLeft.length == 0 || isVisibleRight.length == 0) {
            visibleTrees++;
        }
        const topViewingDistance = treeViewDistance(number, numbersTop.reverse());
        const BottomView = treeViewDistance(number, numbersBottom);
        const le = treeViewDistance(number, numbersLeft.reverse())
        const ri = treeViewDistance(number, numbersRight)
        const scn =  topViewingDistance * BottomView * le * ri
        // console.log(numbersRight)
        //  console.log(BottomView)
        // console.log(number)
        //  console.log()
        if(scn > score) score = scn
    })
})

function treeViewDistance(thisSpecificTree: number, treesInDirection: number[]) {
    let lastTree = 0;
    let viewingDistance = 0;

    for (let tree of treesInDirection) {

        if (tree >= lastTree || tree < thisSpecificTree) {

            viewingDistance++;
        }
        if(tree >= thisSpecificTree) break;
        lastTree = tree;
    }
    return viewingDistance;
}
console.log(visibleTrees)
console.log(score)