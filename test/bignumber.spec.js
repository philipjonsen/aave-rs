const aave = require("../index");
const { normalize } = require("@aave/protocol-js");

const size = 1000;
const set = [];

for (let i = 0; i < size; i++) {
  set.push((Math.random() * 1_000_000_000_000_000_000_000).toString());
}

describe("bignumber", () => {
  it("should sync", () => {
    expect(aave.sync(10)).toBe(110);
  });
  it("should normalize", () => {
    set.forEach((num) => {
      const rs = aave.normalize(num, 18);
      const js = normalize(num, 18);
      expect(rs).toBe(js);
    });
  });
});
