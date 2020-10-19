const assert = require('assert');
const sort = require('./bitonic-sorter');

const input = [10, 20, 11, 20, 4, 330, 21, 110];

const output = sort(input, true);

assert.deepEqual(output, [4, 10, 11, 20, 20, 21, 110, 330]);
