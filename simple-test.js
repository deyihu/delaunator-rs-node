const { delaunator } = require('./index')

// console.assert(plus100(0) === 100, 'Simple test failed')

// console.info('Simple test passed')

const points = [0, 0, 1, 0, 1, 1, 0, 1];
const result = delaunator(points)
console.log(result)
