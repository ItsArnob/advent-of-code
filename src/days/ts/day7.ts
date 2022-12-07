const input = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`

let lines = input.split("\n");

let pathHierarchy: string[] = [];
let folders: any = {};
for(let line of lines)  {
    let command = line.startsWith("$") ? line.replace("$ ", "").split(" ") : [];
    
    if(command.length) {
        if(command[0] == "cd" && command[1] == "..") {
            pathHierarchy.pop();         
            
        } else if(command[0] == "cd") {
            pathHierarchy.push(command[1]);
        } else if(command[0] == "ls") continue; 
    } else if(line.startsWith("dir")) continue;
    else {
        const file = line.split(" ");
        const size = Number(file[0]);

        for(let i = 0; i < pathHierarchy.length; i++) {
            const openFolderPath = pathHierarchy.slice(0, i + 1).join("/");
            if (folders[openFolderPath]) {

                folders[openFolderPath] += size;
            }
            else folders[openFolderPath] = size;
        }

    }
        

}
const sizeLimit = 100000;
let command_part1 = 0;
const spaceTaken = folders["/"];
const freeSpace = 70000000 - spaceTaken;
const needToFree = 30000000 - freeSpace;

// start off with a bigger number so we can compare it to the actual values and find the smaller ones.
let part2 = spaceTaken;
Object.entries(folders).forEach(([folder, props] : any) => {
    
    if(props <= sizeLimit) {
        // console.log(props.size)
        command_part1 += props; 
     
    }

    if(props > needToFree && props < part2) {
        part2 = props;
    }
})

console.log(command_part1)
console.log(part2)
