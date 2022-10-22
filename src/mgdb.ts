import { promises as fsPromise } from "fs";
import {
  MongoClient,
  Db,
  ObjectId,
  InsertOneResult,
  Collection,
} from "mongodb";
import { faker } from "@faker-js/faker";
import moment from "moment";
import pluralize from "pluralize";

// Connection URL
const url =
  "mongodb://127.0.0.1:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false";

export const dbName = "dbv0";

export function generateClient(): MongoClient {
  return new MongoClient(url);
}

export const client = generateClient();

export enum MongoTypes {
  ObjectId = "ObjectId",
}

export interface ObjectSchema<T = unknown> {
  [key: string]: T;
}

export type InferredSchema<T = unknown> = string | ObjectSchema<T>;

export interface MongoCollection<T = InferredSchema> {
  name: string;
  schema: ObjectSchema<T>;
}

function getType<T = unknown>(
  ob: unknown,
  key: string,
  collections: Collection[]
) {
  const obType = typeof ob;

  if (["string", "number", "boolean"].includes(obType)) {
    return obType;
  }

  if (obType === "object") {
    if (Array.isArray(ob)) {
      return ob.slice(0, 1).map((e) => {
        const typeE = typeof e;

        if (["string", "number", "boolean"].includes(typeE)) {
          return typeE;
        }

        if (typeE === "object") {
          if (ObjectId.isValid(e as string)) {
            return exploreObjectId(key, collections);
          }

          if (moment(e as string, "YYYY-MM-DD").isValid()) {
            return "Date";
          }

          return getObjectSchema<T>(e as ObjectSchema<T>, collections);
        }

        return "unknown";
      });
    }

    if (ObjectId.isValid(ob as string)) {
      return exploreObjectId(key, collections);
    }

    if (moment(ob as string, "YYYY-MM-DD").isValid()) {
      return "Date";
    }

    return getObjectSchema<T>(ob as ObjectSchema<T>, collections);
  }

  return "unknown";
}

function exploreObjectId(key: string, collections: Collection[]) {
  if (key === "_id") {
    return MongoTypes.ObjectId;
  }

  if (pluralize.isSingular(key)) {
    const col = collections.find((c) => {
      return c.collectionName === pluralize.plural(key);
    });

    if (col === undefined) {
      return MongoTypes.ObjectId;
    }

    return `${MongoTypes.ObjectId}:${col.collectionName}`;
  }

  if (pluralize.isPlural(key)) {
    const col = collections.find((c) => {
      return c.collectionName === key;
    });

    if (col === undefined) {
      return MongoTypes.ObjectId;
    }

    return `${MongoTypes.ObjectId}:${col.collectionName}`;
  }

  return MongoTypes.ObjectId;
}

function getObjectSchema<T = InferredSchema>(
  doc: ObjectSchema<T>,
  collections: Collection[]
) {
  const schema: ObjectSchema = {};
  for (const key in doc) {
    schema[key] = getType(doc[key], key, collections);
  }

  return schema as ObjectSchema<T>;
}

export async function exploreDb() {
  const mongoCollections: MongoCollection[] = [];

  try {
    await client.connect();
    const db = client.db(dbName);
    const collections = await db.collections();

    for (const collection of collections) {
      const findResult = await collection.find({}).limit(1).toArray();
      // console.log("Found documents =>", findResult);

      const col: MongoCollection = {
        name: collection.collectionName,
        schema: {},
      };

      for (const doc of findResult) {
        col.schema = getObjectSchema(doc, collections);
      }

      mongoCollections.push(col);
    }
  } catch (e) {
    console.log(e);
  } finally {
    await client.close();
  }

  // other
  await fsPromise.writeFile(
    "./tmp/ts.json",
    JSON.stringify(mongoCollections, null, 2)
  );

  return mongoCollections;
}

export function exploreSchema(collections: MongoCollection[]) {
  for (const col of collections) {
    showKeys(col.schema);
  }
}

export function showKeys(schema: ObjectSchema | string | unknown[]): void {
  console.log("showKeys", schema);

  if (typeof schema === "string") {
    console.log(schema);
    return;
  }

  if (Array.isArray(schema)) {
    for (const v of schema) {
      showKeys(v as ObjectSchema | string);
    }
    return;
  }

  for (const key in schema) {
    const value = schema[key];
    console.log(key);

    if (value === null || value === undefined) {
      continue;
    }

    if (typeof value === "object") {
      if (Array.isArray(value)) {
        for (const sub of value) {
          if (ObjectId.isValid(sub) || moment(sub, "YYYY-MM-DD").isValid()) {
            continue;
          }

          showKeys(sub);
        }
      }

      if (
        ObjectId.isValid(value as ObjectId) ||
        moment(value as Date, "YYYY-MM-DD").isValid()
      ) {
        continue;
      }

      showKeys(value as ObjectSchema);
    }
  }
}

export interface Country {
  name: string;
  code: number;
}

export interface Book {
  title: string;
  author: string;
  preview: {
    content: string;
  };
  chapters: ObjectId[];
}

export interface Chapter {
  title: string;
  isActive: boolean;
}

export interface User {
  name: string;
  email: string;
  password: string;
  roles: number[];
  date: Date;
}

export interface Token {
  token: string;
  user: ObjectId;
  date: Date;
}

export interface Publication {
  date: Date;
  bookData: Book;
  book: ObjectId;
  tags: string[];
}

export interface Profile {
  user: ObjectId;
  country: ObjectId;
  books: ObjectId[];
  publications: Publication[];
}

export async function insertExampleDocs(db: Db) {
  const countryCol = db.collection<Country>("countries");
  const resultCountry: InsertOneResult = await countryCol.insertOne({
    name: faker.address.country(),
    code: faker.datatype.number({ min: -100, max: 100 }),
  });

  const chapterCol = db.collection<Chapter>("chapters");
  const resultChapter = await chapterCol.insertOne({
    title: faker.animal.insect(),
    isActive: faker.datatype.boolean(),
  });

  const bookCol = db.collection<Book>("books");
  const bookData = {
    title: faker.animal.cat(),
    author: faker.name.fullName(),
    preview: {
      content: faker.animal.fish(),
    },
    chapters: [resultChapter.insertedId],
  };
  const resultBook = await bookCol.insertOne(bookData);

  const userCol = db.collection<User>("users");
  const resultUser = await userCol.insertOne({
    name: faker.name.fullName(),
    email: faker.internet.email(),
    password: faker.internet.password(),
    roles: faker.helpers.arrayElements([0, 1, 2, 3]),
    date: new Date(),
  });

  const tokenCol = db.collection<Token>("tokens");
  await tokenCol.insertOne({
    token: faker.datatype.uuid(),
    user: resultUser.insertedId,
    date: new Date(),
  });

  const profileCol = db.collection<Profile>("profiles");
  await profileCol.insertOne({
    user: resultUser.insertedId,
    country: resultCountry.insertedId,
    books: [resultBook.insertedId],
    publications: [
      {
        date: new Date(),
        bookData,
        book: resultBook.insertedId,
        tags: faker.helpers.arrayElements(["a", "b", "c", "d"]),
      },
    ],
  });
}
