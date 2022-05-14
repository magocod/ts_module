import { GlobalConfig } from "@jest/types/build/Config";
import { ProjectConfigTsJest } from "ts-jest";

export default async function teardown(
  globalConfig: GlobalConfig,
  projectConfig: ProjectConfigTsJest
) {
  // console.log(globalConfig.testPathPattern);
  // console.log(projectConfig.cache);

  // await globalThis.__MONGOD__.stop();
  console.log("teardown from ts");
}

// module.exports = async function (globalConfig, projectConfig) {
//   // console.log(globalConfig.testPathPattern);
//   // console.log(projectConfig.cache);
//
//   // await globalThis.__MONGOD__.stop();
//   console.log("teardown from js");
// };
