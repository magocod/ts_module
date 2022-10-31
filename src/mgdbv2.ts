import { Collection, Db, ObjectId } from "mongodb";
import {
  FinderRelationships,
  exploreObjectId,
  MongoCollection,
  InferredSchema,
  ObjectSchema,
} from "./mgdb";
import moment from "moment";
import { promises as fsPromise } from "fs";

export class Explorer {
  private mongoCollections: MongoCollection[] = [];
  private collections: Collection[] = [];

  constructor(
    private readonly db: Db,
    private finderRelationships: FinderRelationships = exploreObjectId
  ) {}

  private getObjectSchema<T = InferredSchema>(doc: ObjectSchema<T>) {
    const schema: ObjectSchema = {};
    for (const key in doc) {
      schema[key] = this.getType(doc[key], key);
    }

    return schema as ObjectSchema<T>;
  }

  private getType<T = unknown>(ob: unknown, key: string) {
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
              return this.finderRelationships(key, this.collections);
            }

            if (moment(e as string, "YYYY-MM-DD").isValid()) {
              return "Date";
            }

            return this.getObjectSchema<T>(e as ObjectSchema<T>);
          }

          return "unknown";
        });
      }

      if (ObjectId.isValid(ob as string)) {
        return this.finderRelationships(key, this.collections);
      }

      if (moment(ob as string, "YYYY-MM-DD").isValid()) {
        return "Date";
      }

      return this.getObjectSchema<T>(ob as ObjectSchema<T>);
    }

    return "unknown";
  }

  async explore() {
    this.collections = await this.db.collections();
    const mongoCollections: MongoCollection[] = [];

    for (const collection of this.collections) {
      const findResult = await collection.find({}).limit(1).toArray();
      // console.log("Found documents =>", findResult);

      const col: MongoCollection = {
        name: collection.collectionName,
        schema: {},
      };

      for (const doc of findResult) {
        col.schema = this.getObjectSchema(doc);
      }

      mongoCollections.push(col);
    }

    this.mongoCollections = mongoCollections;
    return mongoCollections;
  }

  saveFile(): Promise<void> {
    return fsPromise.writeFile(
      `./tmp/${this.db.databaseName}_ts_v2.json`,
      JSON.stringify(this.mongoCollections, null, 2)
    );
  }

  cache() {
    return {
      mongoCollections: this.mongoCollections,
      collections: this.collections
    }
  }
}
