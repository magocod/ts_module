const shell = require("shelljs");

function backTop() {
  shell.cd("../../");
  shell.echo(shell.pwd());
}

/**
 *
 * @param {string} packageName
 */
function compileWasm(packageName) {
  shell.cd("crates/" + packageName);
  // shell.echo("build package");
  // shell.echo(shell.pwd());
  shell.exec("wasm-pack build --target nodejs");
}

function compileNeon(packageName) {
  shell.cd("crates/" + packageName);
  // shell.echo("build package");
  // shell.echo(shell.pwd());
  shell.exec("npm run build");
}

function compileNapi(packageName) {
  shell.cd("crates/" + packageName);
  // shell.echo("build package");
  // shell.echo(shell.pwd());
  shell.exec("napi build --platform --release");
}

// shell.echo("build packages");

compileWasm("practice");

backTop();

compileNeon("cpu-count");

backTop();

compileNapi("calculator-napi");

backTop();

compileNapi("awesome");
