import { exploreDb, generateClient, dbName } from "../src/mgdb";
import { seed } from "../src/populate_v0";
import { MongoClient } from "mongodb";

describe("mgdb", function () {
  let client: MongoClient;

  beforeAll(async function () {
    client = generateClient();
    await client.connect();
  });

  afterAll(async function () {
    await client.close();
  });

  it("explore", async function () {
    const v = await exploreDb(client.db(dbName));
    // console.log(JSON.stringify(v, null, 2));
    // exploreSchema(v)

    expect(v).toBeInstanceOf(Array);
  });

  it("populate_v0_seed", async function () {
    await seed(client.db(dbName));

    expect(1).toEqual(1);
  });
});
