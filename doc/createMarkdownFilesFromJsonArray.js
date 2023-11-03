const fs = require('fs').promises;

async function readFileContent(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    return content;
  } catch (error) {
    throw error;
  }
}

function createFile(path, content) {
  fs.writeFile(path, content, 'utf8', (err) => {
    if (err) {
      console.error('Error creating the file:', err);
      return;
    }
    console.log('File created and content written successfully.');
  });
}

function parseJSON(obj, depth) {
  let content = '';
  for (const prop in obj) {
    const value = obj[prop];
    if (typeof value === 'object') {
      console.log(`Property: ${prop} has nested properties:`);
      content += `${'#'.repeat(depth + 1)} ${prop}\n\n`;
      content += parseJSON(value, depth + 1); 
    } else {
      console.log(`Property: ${prop}, Value: ${value}`);
      content += `${prop}: ${value}\n`;
    }
  }
  return content;
}

async function createMarkdownFilesFromJsonArray(path) {
  const fileContent = await readFileContent(path);
  const jsonContent = JSON.parse(fileContent);
  depth = 1;
  for (const key in jsonContent) {
    let content = '';
    const value = jsonContent[key];
    if (typeof value === 'object') {
      console.log(`Property: ${key} has nested properties:`);
      content += `# ${key}\n\n`;
      content += parseJSON(value, depth); 
    } else {
      content += `${key}: ${value}\n`;
    }
    createFile(`./${key}.md`, content);
  }
}

createMarkdownFilesFromJsonArray("./tmp.json");