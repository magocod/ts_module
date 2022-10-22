import { promises as fsPromise } from "fs";
import { MongoClient, Db, ObjectId, Collection } from "mongodb";
import moment from "moment";
import pluralize from "pluralize";

// Connection URL
const url =
  "mongodb://127.0.0.1:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false";

export const dbName = "dbv0";

export function generateClient(): MongoClient {
  return new MongoClient(url);
}

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

type FinderRelationships = (
  key: string,
  collections: Collection[]
) => MongoTypes.ObjectId | string;

function getType<T = unknown>(
  ob: unknown,
  key: string,
  collections: Collection[],
  finderRelationships: FinderRelationships
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
            return finderRelationships(key, collections);
          }

          if (moment(e as string, "YYYY-MM-DD").isValid()) {
            return "Date";
          }

          return getObjectSchema<T>(
            e as ObjectSchema<T>,
            collections,
            finderRelationships
          );
        }

        return "unknown";
      });
    }

    if (ObjectId.isValid(ob as string)) {
      return finderRelationships(key, collections);
    }

    if (moment(ob as string, "YYYY-MM-DD").isValid()) {
      return "Date";
    }

    return getObjectSchema<T>(
      ob as ObjectSchema<T>,
      collections,
      finderRelationships
    );
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
  collections: Collection[],
  finderRelationships: FinderRelationships
) {
  const schema: ObjectSchema = {};
  for (const key in doc) {
    schema[key] = getType(doc[key], key, collections, finderRelationships);
  }

  return schema as ObjectSchema<T>;
}

export async function exploreDb(
  db: Db,
  finderRelationships: FinderRelationships = exploreObjectId,
  saveFile = true
) {
  const mongoCollections: MongoCollection[] = [];
  const collections = await db.collections();

  for (const collection of collections) {
    const findResult = await collection.find({}).limit(1).toArray();
    // console.log("Found documents =>", findResult);

    const col: MongoCollection = {
      name: collection.collectionName,
      schema: {},
    };

    for (const doc of findResult) {
      col.schema = getObjectSchema(doc, collections, finderRelationships);
    }

    mongoCollections.push(col);
  }

  // other
  if (saveFile) {
    await fsPromise.writeFile(
      "./tmp/ts.json",
      JSON.stringify(mongoCollections, null, 2)
    );
  }

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
