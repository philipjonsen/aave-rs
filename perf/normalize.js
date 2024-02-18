const {
  normalize,
  calculateHealthFactorFromBalances,
} = require("@aave/protocol-js");
const aave = require("../index");
const Benchmark = require("benchmark");

// suite settings
const suite = new Benchmark.Suite();
const PRECISION = 2;
const results = [];

// test settings
const size = 10000;
const set = [];

for (let i = 0; i < size; i++) {
  set.push((Math.random() * 1_000_000_000_000_000_000_000).toString());
}

suite
  /*.add(`aave-js ${size} normalize`, () => {
    set.forEach((num) => normalize(num, 18));
  })
  .add(`aave-rs ${size} normalize`, () => {
    set.forEach((num) => aave.normalize(num, 18));
  })*/
  .add(`aave-js ${size} hf`, () => {
    set.forEach((num) =>
      calculateHealthFactorFromBalances(num, num, num).toString(),
    );
  })
  .add(`aave-rs ${size} hf`, () => {
    set.forEach((num) => aave.calculateHealthFactorFromBalances(num, num, num));
  })
  .on("cycle", (event) =>
    results.push({
      name: event.target.name,
      hz: event.target.hz,
      "margin of error": `±${Number(event.target.stats.rme).toFixed(2)}%`,
      "runs sampled": event.target.stats.sample.length,
    }),
  )
  .on("complete", function () {
    const lowestHz = results.slice().sort((a, b) => a.hz - b.hz)[0].hz;

    console.table(
      results
        .sort((a, b) => b.hz - a.hz)
        .map((result) => ({
          ...result,
          hz: Math.round(result.hz).toLocaleString(),
          numTimesFaster:
            Math.round((10 ** PRECISION * result.hz) / lowestHz) /
            10 ** PRECISION,
        }))
        .reduce((acc, { name, ...cur }) => ({ ...acc, [name]: cur }), {}),
    );
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })

  .run({ async: false });
