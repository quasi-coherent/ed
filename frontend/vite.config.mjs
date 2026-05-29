import { defineConfig } from "vite";
import path from "node:path";
import fs from "node:fs";

// Melange emits ES modules into _build/default/src/app/src/. Its runtime
// dependencies (melange, melange.js, melange.dom, reason-react, ...) are
// emitted as separate "packages" into _build/default/src/app/node_modules/.
// Vite needs explicit aliases for each of these, since they're not in the
// project's npm-managed node_modules. CSS module files live in the *source*
// src/ dir (not the build output), so we alias relative .module.css imports
// to the source dir.
const srcDir = path.resolve(process.cwd(), "src");
const melangeOut = path.resolve(process.cwd(), "_build/default/src/app/src");
const melangeRuntime = path.resolve(
  process.cwd(),
  "_build/default/src/app/node_modules",
);

const melangePackageAliases = fs.existsSync(melangeRuntime)
  ? Object.fromEntries(
      fs.readdirSync(melangeRuntime).map((name) => [
        name,
        path.join(melangeRuntime, name),
      ]),
    )
  : {};

// Plugin that rewrites relative ./Foo.module.css imports inside the
// melange output to point at the corresponding file in src/.
const cssFromSource = {
  name: "melange-css-from-source",
  resolveId(source, importer) {
    if (
      source.endsWith(".module.css") &&
      importer &&
      importer.startsWith(melangeOut)
    ) {
      // Preserve subdirectory layout: src/components/Foo.js importing
      // ./Foo.module.css should resolve to src/components/Foo.module.css.
      const importerDir = path.dirname(importer);
      const relDir = path.relative(melangeOut, importerDir);
      return path.join(srcDir, relDir, path.basename(source));
    }
    return null;
  },
};

export default defineConfig({
  root: ".",
  plugins: [cssFromSource],
  resolve: {
    alias: {
      "/app": melangeOut,
      ...melangePackageAliases,
    },
  },
  server: {
    fs: {
      allow: [".."],
    },
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
});
