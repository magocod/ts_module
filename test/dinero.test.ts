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

  const count = 2;
  const payment = purchase.price / count;
  const lossRoundPayment = 185;

  describe("inaccurate results", function () {
    it("with js", function () {
      const total = cart.reduce((acc, { price }) => acc + price, 0);

      expect(total).toBe(121.89999999999999);
    });
  });

  describe("purchase in two payments", function () {
    it("with js", function () {
      const payments = new Array(count).fill(null).map(() => payment);
      const roundPayment = Math.round((purchase.price / count) * 100) / 100;

      // console.log("payment", payment);
      // console.log("payments", payments);
      // console.log("roundPayment", roundPayment);

      expect(payments).toEqual([payment, payment]);
      expect(roundPayment).toEqual(lossRoundPayment);
    });
  });
});
