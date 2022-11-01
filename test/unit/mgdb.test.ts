import { exploreDb, MongoTypes } from "../../src/mgdb";
import { Explorer } from "../../src/mgdbv2";
import {
  seed,
  generateClient,
  DbNames,
  CollectionNames,
} from "../../src/populate_v0";
import { MongoClient } from "mongodb";
import pluralize from "pluralize";

describe("mgdb", function () {
  let client: MongoClient;
  const collectionNames = Object.values(CollectionNames);

  beforeAll(async function () {
    client = generateClient();
    await client.connect();
  });

  afterAll(async function () {
    await client.close();
  });

  describe("v1", function () {
    it("explore dbv0", async function () {
      await seed(client.db(DbNames.dbv0));
      const v = await exploreDb(client.db(DbNames.dbv0));
      // console.log(JSON.stringify(v, null, 2));
      // exploreSchema(v)

      expect(v).toBeInstanceOf(Array);
      expect(
        v.every((col) => {
          return collectionNames.includes(col.name as CollectionNames);
        })
      ).toEqual(true);
    });

    it("explore dbv2", async function () {
      await seed(client.db(DbNames.dbv2));
      const v = await exploreDb(client.db(DbNames.dbv2), (key, collections) => {
        if (key === "_id") {
          return MongoTypes.ObjectId;
        }

        const parts = key.replace(/([a-z])([A-Z])/g, "$1 $2").split(" ");

        // console.log("key", key, parts);
        const k = parts.shift();

        if (k === undefined) {
          return MongoTypes.ObjectId;
        }

        const col = collections.find((c) => {
          return c.collectionName === pluralize.plural(k);
        });

        if (col === undefined) {
          return MongoTypes.ObjectId;
        }

        return `${MongoTypes.ObjectId}:${col.collectionName}`;
      });
      // console.log(JSON.stringify(v, null, 2));
      // exploreSchema(v)

      expect(v).toBeInstanceOf(Array);
      expect(
        v.every((col) => {
          return collectionNames.includes(col.name as CollectionNames);
        })
      ).toEqual(true);
    });

    it("explore dbv3", async function () {
      await seed(client.db(DbNames.dbv3));
      const v = await exploreDb(client.db(DbNames.dbv3));
      // console.log(JSON.stringify(v, null, 2));
      // exploreSchema(v)

      expect(v).toBeInstanceOf(Array);
      expect(
        v.every((col) => {
          return collectionNames.includes(col.name as CollectionNames);
        })
      ).toEqual(true);
    });
  });

  describe("v2", function () {
    it("explore dbv0", async function () {
      await seed(client.db(DbNames.dbv0));
      const explorer = new Explorer(client.db(DbNames.dbv0));
      const v = await explorer.explore();
      const cache = explorer.cache();
      // console.log(JSON.stringify(v, null, 2));
      // await explorer.saveFile();

      expect(v).toBeInstanceOf(Array);
      expect(
        v.every((col) => {
          return collectionNames.includes(col.name as CollectionNames);
        })
      ).toEqual(true);
      expect(cache.collections.length).toEqual(collectionNames.length);
      expect(cache.mongoCollections.length).toEqual(collectionNames.length);
    });
  });

  it("populate_v0_seed", async function () {
    await seed(client.db(DbNames.dbv0));

    expect(1).toEqual(1);
  });
});
