import { resolve } from "node:path";
import {
  cleanBuildOutputDir,
  writeAssets,
  writeRootConfig,
  writeWorkerConfig,
} from "@cloudflare/build-output-utils";
import config from "../cloudflare.config.ts";

const root = resolve(import.meta.dir, "..");
await Bun.write(
  resolve(root, "dist/_headers"),
  "/*\n  Access-Control-Allow-Origin: *\n  Cache-Control: public, max-age=300\n  X-Content-Type-Options: nosniff\n",
);
await cleanBuildOutputDir(root);
await writeRootConfig(root, { accountId: config.accountId }, { isPreview: false, mode: undefined });
await writeWorkerConfig({ root, config: config.worker });
await writeAssets({ root, sourceDirectory: resolve(root, "dist") });
