## CSV Generator Script
Generates a fake CSV that is 1000 lines long

structure is as follows
| id                | email                                                        | name | is_parent |
|-------------------|--------------------------------------------------------------|------|-----------|
| alphanumeric hash | [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt) valid email | text | boolean   |

## Running
make sure you have (Node)[https://nodejs.org/en/download/] installed

make sure you have installed the dependencies by running
```
npm install
```

to generate a csv run
```bash
npm run generate
```
