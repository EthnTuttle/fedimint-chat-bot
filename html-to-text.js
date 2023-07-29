import fs from 'fs/promises';
import path from 'path';
import { convert as htmlToText } from 'html-to-text';

const inputDir = "/Users/ethantuttle/code/fedimint-chat-bot/data/fedimint-docs/doc"

const options = {
  wordwrap: 130
};

const convertHTMLtoText = async (dir) => {
  const files = await fs.readdir(dir);
  for (const file of files) {
    const filePath = path.join(dir, file);
    const stat = await fs.lstat(filePath);
    if (stat.isDirectory()) {
      await convertHTMLtoText(filePath);
    } else if (path.extname(filePath) === '.html') {
      const html = await fs.readFile(filePath, 'utf8');
      const text = htmlToText(html, options);
      await fs.writeFile(filePath.replace('.html', '.txt'), text, 'utf8');
    }
  }
}

convertHTMLtoText(inputDir)
  .catch((error) => {
    console.error(`Failed to convert HTML files to text: ${error}`);
  });
