import type { Config } from "@jest/types";

// Sync object
const config: Config.InitialOptions = {
  verbose: true,
  preset: "ts-jest",
  testEnvironment: "node",
  globalSetup: "./test/setup.ts",
  globalTeardown: "./test/teardown.ts",
  testMatch: ["<rootDir>/test/unit/**/*.test.ts"],
  collectCoverageFrom: ["src/**/*.{ts,jxs}"],
};
export default config;

// js
// module.exports = {
//   preset: "ts-jest",
//   testEnvironment: "node",
//   globalSetup: "./test/setup.js",
//   globalTeardown: "./test/teardown.js",
// };
