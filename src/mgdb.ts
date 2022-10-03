import { promises as fsPromise } from "fs";
import { MongoClient, Db, ObjectId, InsertOneResult } from "mongodb";
import { faker } from "@faker-js/faker";
import moment from "moment";

// Connection URL
const url =
  "mongodb://127.0.0.1:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false";

export const dbName = "actix";

export function generateClient(): MongoClient {
  return new MongoClient(url);
}

export const client = generateClient();

export interface ObjectSchema<T = unknown> {
  [key: string]: T;
}

export type InferredSchema<T = unknown> = string | ObjectSchema<T>;

export interface MongoCollection<T = InferredSchema> {
  name: string;
  schema: ObjectSchema<T>;
}

function getType<T = unknown>(ob: unknown) {
  const obType = typeof ob;
  console.log(ob, obType);

  if (["string", "number", "boolean"].includes(obType)) {
    return obType;
  }

  if (obType === "object") {
    if (Array.isArray(ob)) {
      return ob.map((e) => {
        const typeE = typeof e;
        console.log("map", e, typeE)
        if (["string", "number", "boolean"].includes(typeof typeE)) {
          return typeE;
        }

        if (typeE === "object") {
          if (ObjectId.isValid(e as string)) {
            return "ObjectId";
          }

          if (moment(e as string, "YYYY-MM-DD").isValid()) {
            return "Date";
          }
        }

        return "unknown";
      });
    }

    if (ObjectId.isValid(ob as string)) {
      return "ObjectId";
    }

    if (moment(ob as string, "YYYY-MM-DD").isValid()) {
      return "Date";
    }

    return getObjectSchema<T>(ob as ObjectSchema<T>);
  }

  return "unknown";
}

function getObjectSchema<T = InferredSchema>(doc: ObjectSchema) {
  const schema: ObjectSchema = {};
  for (const key in doc) {
    schema[key] = getType(doc[key]);
  }

  return schema as ObjectSchema<T>;
}

export async function explore() {
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
        col.schema = getObjectSchema(doc);
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
  let insertOneResult: InsertOneResult = await countryCol.insertOne({
    name: faker.address.country(),
    code: faker.datatype.number({ min: -100, max: 100 }),
  });
  const country = await countryCol.findOne({ _id: insertOneResult.insertedId });
  if (country === null) {
    throw new Error("countryCol.findOne");
  }

  const bookCol = db.collection<Book>("books");
  insertOneResult = await bookCol.insertOne({
    title: faker.animal.cat(),
    author: faker.name.fullName(),
    preview: {
      content: faker.animal.fish(),
    },
  });
  const book = await bookCol.findOne({ _id: insertOneResult.insertedId });
  if (book === null) {
    throw new Error("bookCol.findOne");
  }

  const userCol = db.collection<User>("users");
  insertOneResult = await userCol.insertOne({
    name: faker.name.fullName(),
    email: faker.internet.email(),
    password: faker.internet.password(),
    roles: faker.helpers.arrayElements([0, 1, 2, 3]),
    date: new Date(),
  });
  const user = await userCol.findOne({ _id: insertOneResult.insertedId });
  if (user === null) {
    throw new Error("userCol.findOne");
  }

  const tokenCol = db.collection<Token>("tokens");
  await tokenCol.insertOne({
    token: faker.datatype.uuid(),
    user: user._id,
    date: new Date(),
  });

  const publicationCol = db.collection<Profile>("profiles");
  insertOneResult = await publicationCol.insertOne({
    user: user._id,
    country: country._id,
    books: [book._id],
    publications: [
      {
        date: new Date(),
        bookData: {
          title: book.title,
          author: book.author,
          preview: book.preview,
        },
        book: book._id,
        tags: faker.helpers.arrayElements(["a", "b", "c", "d"]),
      },
    ],
  });
  await publicationCol.findOne({
    _id: insertOneResult.insertedId,
  });
}
