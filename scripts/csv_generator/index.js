const generate = require('csv-generate');
const faker = require('faker');
const fs = require('fs');

const fileName = './fake_people.csv';

try {
  fs.unlinkSync(fileName);
} catch (e) {
  if (e.code && e.code !== 'ENOENT') {
    throw e;
  }
}

fs.closeSync(fs.openSync(fileName, 'w'));

const hashChars =
  '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

// create random alphanumeric string of length 40 (mimics a gooid)
function randomString() {
  let result = '';
  for (let i = 40; i > 0; --i) {
    result += hashChars[Math.floor(Math.random() * hashChars.length)];
  }

  return result;
}

function generateEmail() {
  return faker.internet.email();
}

function generateName() {
    return faker.name.findName();
}

const fileStream = fs.createWriteStream(fileName);

fileStream.write('id,email,name,is_parent\n');

generate({
  length: 1000,
  columns: [randomString, generateEmail, generateName, 'bool'],
}).pipe(fileStream);
