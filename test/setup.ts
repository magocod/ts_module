import { GlobalConfig } from "@jest/types/build/Config";
import { ProjectConfigTsJest } from "ts-jest";

export default async function setup(
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  globalConfig: GlobalConfig,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  projectConfig: ProjectConfigTsJest
) {
  // console.log(globalConfig);
  // console.log(projectConfig.cache);

  // Set reference to mongod in order to close the server during teardown.
  // globalThis.__MONGOD__ = mongod;
  console.log("setup from ts");
}

// js
// module.exports = async function (globalConfig, projectConfig) {
//   // console.log(globalConfig);
//   // console.log(projectConfig.cache);
//
//   // Set reference to mongod in order to close the server during teardown.
//   // globalThis.__MONGOD__ = mongod;
//   console.log("setup from js");
// };
