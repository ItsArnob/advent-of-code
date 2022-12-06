let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
let lastFourOrFoureenChars: string[] = [];


for (let i = 0; i < data.length; i++) {
    let compareTo = 4; // set to 14 for part 2 or 4 for part 1
    
    if (lastFourOrFoureenChars.length != compareTo) {
        lastFourOrFoureenChars.push(data[i]);
    } else {
        let duplicateExists = dupeChecker1000(lastFourOrFoureenChars);
        if (duplicateExists) {
            lastFourOrFoureenChars.shift();
            lastFourOrFoureenChars.push(data[i]);
        }
        else {
            console.log(i);
            break;
        }
    }
}
function dupeChecker1000(arr) {
    return new Set(arr).size !== arr.length
}