describe('LSP Server', () => {
    let server;

    beforeAll(async () => {
        const { initLog } = await import('@influxdata/flux-lsp-node');
        initLog();
    });

    beforeEach(async () => {
        const { Lsp } = await import('@influxdata/flux-lsp-node');
        server = new Lsp();
    });

    it('echoes whatever is sent to it', async () => {
        const callback = jest.fn((message) => {
            //console.log(message);
        });

        const request = 'Content-Length: 84\r\n\r\n{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "capabilities": {}}}';
        server.onMessage(callback);
        await server.send(request);

        await server.run();
        expect(callback).toHaveBeenCalled();
    });

    /*
    it('doesn\'t echo when not running', async () => {
        const callback = jest.fn((message) => {
            console.log(message);
        });

        const request = 'Content-Length: 84\r\n\r\n{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "capabilities": {}}}';
        server.onMessage(callback);
        server.send(request);

        expect(callback).not.toHaveBeenCalled();
    });
    */
});
