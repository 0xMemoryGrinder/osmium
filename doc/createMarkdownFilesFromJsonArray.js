import { promises as fs } from 'fs';

let filename = '';

async function readFileContent(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    return content;
  } catch (error) {
    throw error;
  }
}

async function saveToFile(path, content) {
  try {
    await fs.writeFile(path, content, 'utf8');
  } catch (err) {
    console.error('Error creating the file:', err);
  }
}

function id(value) {
  filename = value;
}

function code(value) {
  const blockCodeDelimiter = "```";
  const langage = 'solidity';
  return `${blockCodeDelimiter}${langage}\n${value}\n\n${blockCodeDelimiter}\n`;
}

const balise = ["id", "code"];
const functions = [id, code];

function parseJSON(obj, depth) {
  let content = '';
  for (const prop in obj) {
    const value = obj[prop];

    if (typeof value === 'object') {
      content += `${'#'.repeat(depth + 1)} ${prop}\n\n`;
      content += parseJSON(value, depth + 1);
      continue;
    }
    let isText = true;
    for (const elem in balise) {
      if (prop === balise[elem]) {
        isText = false;
        content += functions[elem](value);
      }
    }
    if (isText) {
      content += `${prop}: ${value}\n\n`;
    }
  }
  return content;
}

async function createMarkdownFilesFromJsonArray(path) {
  const fileContent = await readFileContent(path);
  const jsonContent = JSON.parse(fileContent);
  let depth = 1;
  for (const key in jsonContent) {
    let content = '';
    const value = jsonContent[key];
    const body = parseJSON(value, depth);
    content += `# ${filename}\n\n` + body;
    saveToFile(`./${filename}.md`, content);
  }
}

async function main() {
  const args = process.argv.slice(2);
  let filepath = "./docTree.json"

  if (args.length !== 0) {
    filepath = args[0];
  }

  try {
    await fs.access(filepath, fs.constants.F_OK);
    createMarkdownFilesFromJsonArray(filepath);
  } catch (err) {
    console.error('Le fichier n\'existe pas.', err);
  }
}

main();