const aave = require("../index");
const {
  normalize,
  getLinearBalance,
  calculateLinearInterest,
  calculateHealthFactorFromBalances,
} = require("@aave/protocol-js");

describe("bignumber", () => {
  it("should normalize", () => {
    const num = (Math.random() * 1_000_000_000_000_000_000_000).toString();
    const rs = aave.normalize(num, 18);
    const js = normalize(num, 18);
    expect(rs).toBe(js);
  });

  it("should calculate correct hf", () => {
    const num = (Math.random() * 1_000_000_000_000_000_000_000).toString();
    const rs = aave.calculateHealthFactorFromBalances(num, num, num);
    const js = calculateHealthFactorFromBalances(num, num, num).toString();
    expect(rs).toBe(js);
  });

  it("should calculate linear interest", () => {
    const rs = aave.calculateLinearInterest(
      "23845771767403410038567586",
      1621966873,
      1621966928
    );
    const js = calculateLinearInterest(
      "23845771767403410038567586",
      1621966873,
      1621966928
    ).toString();
    expect(rs).toBe(js);
  });

  it("should calculate linear balance", () => {
    const rs = aave.getLinearBalance(
      "394131189472991302310507111",
      "1004199807746484496834346378",
      "23845771767403410038567586",
      1621966873,
      1621966928
    );
    const js = getLinearBalance(
      "394131189472991302310507111",
      "1004199807746484496834346378",
      "23845771767403410038567586",
      1621966873,
      1621966928
    ).toString();
    expect(rs).toBe(js);
  });
});
