import {
  exploreDb,
  insertExampleDocs,
  generateClient,
  dbName, exploreSchema,
} from "../src/mgdb";
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
    const v = await exploreDb();
    // console.log(JSON.stringify(v, null, 2));
    // exploreSchema(v)

    expect(v).toBeInstanceOf(Array);
  });

  it("insertExampleDocs", async function () {
    await insertExampleDocs(client.db(dbName));

    expect(1).toEqual(1);
  });
});
