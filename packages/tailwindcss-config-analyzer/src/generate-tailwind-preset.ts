// run this script: $ just gen-tw
// requires Bun installed globally: bun.sh

import path from "node:path";
import { readPackageUp } from "read-package-up";
import { introspectTailwindConfig } from "./introspect.js";
import { type SortConfig, sortConfigFromSpec } from "./sort-config.js";

const ROOT_PACKAGE_NAME = "@biomejs/monorepo";
const OUTPUT_PATH =
	"crates/biome_js_analyze/src/semantic_analyzers/nursery/use_sorted_classes/tailwind_preset.rs";

const EXCLUDED_LAYERS = ["defaults", "base"];
const LAYER_ORDER = ["components", "utilities"];

const LINE_LIMIT = 100;
const INDENT = "    ";
const FILE_HEADER = `//! DO NOT EDIT MANUALLY - this file is autogenerated from the default Tailwind CSS config.
//! To update, run \`pnpm gen:tailwind-preset\` from \`packages/@biomejs/tailwindcss-config-analyzer\`.

use super::sort_config::UtilityLayer;
`;

function generateLayer({ layer, classes }: SortConfig["utilities"][number]) {
	const header = `const ${layer.toUpperCase()}_LAYER_CLASSES: [&str; ${
		classes.length
	}] = [`;

	// try single line
	const singleLine = `${header}${classes.map((c) => `"${c}"`).join(", ")}];`;
	if (singleLine.length < LINE_LIMIT) return `${singleLine}\n`;

	// if single line is too long, do multi-line
	return `${header}\n${classes
		.map((c) => `${INDENT}"${c}"`)
		.join(",\n")},\n];\n`;
}

function generateLayerArray(layers: SortConfig["utilities"]) {
	let output = `pub const TAILWIND_LAYERS: [UtilityLayer; ${layers.length}] = [\n`;
	for (const { layer } of layers) {
		output += `${INDENT}UtilityLayer {\n`;
		output += `${INDENT}${INDENT}name: "${layer}",\n`;
		output += `${INDENT}${INDENT}classes: ${layer.toUpperCase()}_LAYER_CLASSES.as_slice(),\n`;
		output += `${INDENT}},\n`;
	}
	output += "];\n";
	return output;
}

function generateFile(sortConfig: SortConfig) {
	let output = FILE_HEADER;
	output += "\n";
	output += sortConfig.utilities.map(generateLayer).join("\n");
	output += "\n";
	output += generateLayerArray(sortConfig.utilities);
	return output;
}

async function findRoot() {
	let nextPath = import.meta.dir;
	let rootPath: string | false = false;
	while (!rootPath) {
		const pkg = await readPackageUp({ cwd: nextPath });
		if (!pkg) throw new Error("Could not find package.json");
		const {
			packageJson: { name },
			path: pkgPath,
		} = pkg;
		const dirPath = path.dirname(pkgPath);
		if (name === ROOT_PACKAGE_NAME) {
			rootPath = dirPath;
		} else {
			nextPath = path.resolve(dirPath, "..");
		}
	}
	return rootPath;
}

async function generateTailwindPreset() {
	const spec = introspectTailwindConfig(
		{},
		{ excludedLayers: EXCLUDED_LAYERS },
	);
	const sortConfig = sortConfigFromSpec(spec, {
		layerOrder: LAYER_ORDER,
	});
	const file = generateFile(sortConfig);
	const rootPath = await findRoot();
	const outputPath = path.join(rootPath, OUTPUT_PATH);
	await Bun.write(outputPath, file);
}

await generateTailwindPreset();
