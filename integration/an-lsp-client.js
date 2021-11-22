async function main() {
    const { Lsp, initLog } = await import('@influxdata/flux-lsp-node');
    initLog();
    const server = new Lsp();

    server.onMessage((msg) => {
        console.log(msg);
    });


    const request = 'Content-Length: 84\r\n\r\n{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "capabilities": {}}}';
    await server.send(request);

    await server.run();
    console.log("server returned?");
}

console.log("Starting up");
main().then(() => {
    console.log("done");
}).catch((err) => {
    console.error(err);
});
