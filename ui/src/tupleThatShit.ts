//https://stackoverflow.com/questions/37270508/javascript-function-that-converts-array-to-array-of-2-tuples
const tupleIt = (arr: Array<number>): Array<Array<number>> =>
  arr.reduce((r, a, i) => {
    if (i % 3) {
      // @ts-ignore
      r[r.length - 1].push(a);
    } else {
      // @ts-ignore
      r.push([a]);
    }
    return r;
  }, []);

export default tupleIt;
