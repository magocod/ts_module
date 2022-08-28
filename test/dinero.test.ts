import { USD } from "@dinero.js/currencies";
import { allocate, Dinero, dinero, toUnit, add } from "dinero.js";

function createUSD(amount: number): Dinero<number> {
  return dinero({ amount, currency: USD });
}

const addMany = (addends: Dinero<number>[]) => addends.reduce(add);

describe("dinero", () => {
  const cart = [
    {
      name: "Mass Effect: Legendary Edition",
      platform: "Xbox One",
      price: 69.99,
    },
    {
      name: "The Legend of Zelda: Breath of the Wild",
      platform: "Nintendo Switch",
      price: 51.91,
    },
  ];

  const purchase = {
    title: "Microsoft Xbox Series S",
    price: 369.99,
  };

  const countPayment = 2;
  const payment = purchase.price / countPayment;
  const lossRoundPayment = 185;

  const toDivide = 100.53;

  describe("inaccurate results", function () {
    it("js, sum", function () {
      const total = cart.reduce((acc, { price }) => acc + price, 0);
      // console.log(total);

      expect(total).toBe(121.89999999999999);
    });

    it("dinero.js, sum", function () {
      const dinArr = cart.map((v) => {
        // console.log(Math.round(v.price * 100))
        // return v.price * Math.pow(10, 2)
        return createUSD(Math.round(v.price * Math.pow(10, 2)))
      });
      // console.log(dinArr)
      const total = addMany(dinArr);
      // console.log(toUnit(total));

      expect(toUnit(total)).toBe(121.9);
    });

    it("js, divide 100.53", function () {
      const r = 100.53 / 2;

      // console.log(r, r.toFixed(2))
      expect(r).toBe(50.265);
      expect(r.toFixed(2)).toBe("50.27");
    });

    it("dinero.js, divide 100.53", function () {
      const price = dinero({ amount: toDivide * 100, currency: USD });
      const [d1, d2] = allocate(price, [1, 1]);
      // console.log(toUnit(d1), toUnit(d2)); // 50.27 50.26
      expect(toUnit(d1)).toBe(50.27);
      expect(toUnit(d2)).toBe(50.26);
    });
  });

  describe("purchase in two payments", function () {
    it("js", function () {
      const payments = new Array(countPayment).fill(null).map(() => payment);
      const roundPayment =
        Math.round((purchase.price / countPayment) * 100) / 100;

      // console.log("payment", payment);
      // console.log("payments", payments);
      // console.log("roundPayment", roundPayment);

      expect(payments).toEqual([payment, payment]);
      expect(roundPayment).toEqual(lossRoundPayment);
    });

    it("dinero.js", function () {
      const dinPurchase = createUSD(purchase.price * 100);

      const [d1, d2] = allocate(dinPurchase, [1, 1]);
      const roundPayment =
        Math.round((purchase.price / countPayment) * 100) / 100;

      // console.log("payment", payment);
      // console.log(toUnit(d1), toUnit(d2));
      // console.log("roundPayment", roundPayment);

      // expect(payments).toEqual([payment, payment]);
      expect(toUnit(d1)).toEqual(roundPayment);
      expect(toUnit(d2)).toEqual(184.99);
    });
  });
});
