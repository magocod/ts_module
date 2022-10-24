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

// shell.echo("build packages");

compileWasm("practice")
