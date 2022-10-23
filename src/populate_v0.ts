import { Db, InsertOneResult, MongoClient, ObjectId } from "mongodb";
import { faker } from "@faker-js/faker";

// Connection URL
const url =
  "mongodb://127.0.0.1:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false";

export function generateClient(): MongoClient {
  return new MongoClient(url);
}

export enum DbNames {
  dbv0 = "dbv0",
  dbv1 = "dbv1",
  dbv2 = "dbv2",
  dbv3 = "dbv3",
}

export enum CollectionNames {
  Countries = "countries",
  Chapters = "chapters",
  Books = "books",
  Users = "users",
  Tokens = "tokens",
  Profiles = "profiles",
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

export async function seed(db: Db) {
  const countryCol = db.collection<Country>(CollectionNames.Countries);
  const resultCountry: InsertOneResult = await countryCol.insertOne({
    name: faker.address.country(),
    code: faker.datatype.number({ min: -100, max: 100 }),
  });

  const chapterCol = db.collection<Chapter>(CollectionNames.Chapters);
  const resultChapter = await chapterCol.insertOne({
    title: faker.animal.insect(),
    isActive: faker.datatype.boolean(),
  });

  const bookCol = db.collection<Book>(CollectionNames.Books);
  const bookData = {
    title: faker.animal.cat(),
    author: faker.name.fullName(),
    preview: {
      content: faker.animal.fish(),
    },
    chapters: [resultChapter.insertedId],
  };
  const resultBook = await bookCol.insertOne(bookData);

  const userCol = db.collection<User>(CollectionNames.Users);
  const resultUser = await userCol.insertOne({
    name: faker.name.fullName(),
    email: faker.internet.email(),
    password: faker.internet.password(),
    roles: faker.helpers.arrayElements([0, 1, 2, 3]),
    date: new Date(),
  });

  const tokenCol = db.collection<Token>(CollectionNames.Tokens);
  await tokenCol.insertOne({
    token: faker.datatype.uuid(),
    user: resultUser.insertedId,
    date: new Date(),
  });

  const profileCol = db.collection<Profile>(CollectionNames.Profiles);
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
