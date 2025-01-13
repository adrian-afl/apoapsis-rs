import * as path from "path";
import * as util from "util";
import * as fs from "fs/promises";
import * as child_process from "child_process";

async function resolveIncludes(
  fileName: string,
  contentsRaw: string,
  counter = 10
): Promise<string> {
  const contents = contentsRaw.replaceAll("\r\n", "\n").split("\n");
  if (!contents[0].startsWith("#version")) {
    // contents.unshift(`#line 1 ${counter.toString()}`);
    contents.unshift(`#line 1 "${fileName}"`);
  }

  const header = "#include ";
  const guard = "#pragma once";
  let encloseInGuard = false;
  for (let i = 0; i < contents.length; i++) {
    if (contents[i].startsWith(header)) {
      const include = contents[i].substring(header.length).replaceAll('"', "");
      const includeContents = await loadAndResolveShaderSource(
        include,
        counter + 10
      );
      contents[i] =
        includeContents +
        `\n#line ${(i + (encloseInGuard ? 3 : 2)).toString()} "${fileName}"`;
    } else if (contents[i].includes(guard)) {
      contents[i] = "";
      encloseInGuard = true;
    }
  }
  let result = contents.join("\n");
  if (encloseInGuard) {
    const guid = "X" + fileName.replaceAll(/[^A-z0-9]/g, "_");
    result =
      "#ifndef " + guid + "\n#define " + guid + "\n" + result + "\n#endif";
  }
  return result;
}

export async function loadAndResolveShaderSource(
  file: string,
  fileIdent = 0
): Promise<string> {
  const contents = await fs.readFile(path.join("src", file));
  return resolveIncludes(file, contents.toString("utf-8"), fileIdent);
}

async function scanInner(dir: string) {
  const result: { files: string[], dirs: string[] } = {files: [], dirs: []};
  const content = await fs.readdir(dir, {withFileTypes: true});
  for (const entry of content) {
    if (entry.isDirectory()) {
      result.dirs.push(path.join(dir, entry.name));
    } else if (entry.isFile()) {
      result.files.push(path.join(dir, entry.name));
    }
  }
  return result;
}

async function scanDir(dir: string) {
  const result = await scanInner(dir);
  do {
    const dirs = result.dirs;
    result.dirs = [];
    await Promise.all(
      dirs.map(async (a) => {
        const sub = await scanInner(a);
        result.files.push(...sub.files);
        result.dirs.push(...sub.dirs);
      })
    );
  } while (result.dirs.length !== 0);
  return result.files;
}

async function run() {
  await fs.rm("tmp-preparsed", {recursive: true, force: true});
  await fs.rm("compiled", {recursive: true, force: true});

  const list = await scanDir("src");
  const entrypoints = list.filter(x => x.endsWith(".vert") || x.endsWith(".frag") || x.endsWith(".comp") || x.endsWith(".geom"));
  for (const entrypoint of entrypoints) {
    let relativeName = path.relative(path.join(__dirname, "src"), entrypoint);
    let resolved = await loadAndResolveShaderSource(relativeName);

    let outDirTmp = path.dirname(path.join("tmp-preparsed", relativeName));
    let outDirCompiled = path.dirname(path.join("compiled", relativeName));

    await fs.mkdir(outDirTmp, {recursive: true});

    await fs.mkdir(outDirCompiled, {recursive: true});

    await fs.writeFile(path.join("tmp-preparsed", relativeName), resolved);

    const compilationResult = child_process.spawnSync("glslc", [`${path.join("tmp-preparsed", relativeName)}`,  "-o", `${path.join("compiled", relativeName)}.spv`]);
    if(compilationResult.status !== 0){
      console.log(util.styleText("red", compilationResult.stderr.toString("utf-8").trim()));
    }
  }
}

void run();