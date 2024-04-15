import * as fs from 'fs';

let filePath = process.argv[2];

if (!filePath) {
    console.log('Please provide a file path');
    process.exit(1);
}

fs.readFileSync(filePath, 'utf-8')
  .toString()
  .split('\n')
  .forEach((line) => console.log(line));



