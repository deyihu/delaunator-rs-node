import b from 'benny'
import Delaunator from 'delaunator';
import { delaunator } from '../index'

const POINTS_COUNT = 100 * 1 * 2;
function randomCoords() {
  const coords = [];
  while (coords.length < POINTS_COUNT) {
    const x = Math.random() * 10000, y = Math.random() * 10000;
    coords.push(x, y);
  }
  return coords;
}


function delaunatorJS(coords: Array<number>) {
  new Delaunator(coords);
}

const coords = randomCoords();

async function run() {
  await b.suite(
    `delaunator points.length=${POINTS_COUNT / 2}`,

    b.add('Native', () => {
      delaunator(coords)
    }),

    b.add('JavaScript', () => {
      delaunatorJS(coords)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
