class ControlSocket {
    ws: WebSocket | undefined;
    ready: boolean = false;
    requests: Map<Number, [Function, Function]> = new Map();
    currentReq: number = 0;
    currentResp: number = 0;

    connect(): Promise<void> {
        if (this.ready)
            return Promise.resolve();
        return new Promise((resolve, reject) => {
            console.log('Connecting to control socket');
            this.ws = new WebSocket(import.meta.env.VITE_WS_BASE_URL + '/control');
            this.ws.onopen = () => {
                this.ready = true;
                resolve();
            };
            this.ws.onmessage = (ev) => {
                this.onMessage(ev.data);
            };
            this.ws.onerror = (ev) => {
                console.error('WebSocket error:', ev);
                if (!this.ready) {
                    reject();
                }
            };
            this.ws.onclose = (ev) => {
                console.error('WebSocket closed:', ev);
                // TODO: terminate all requests
                if (!this.ready) {
                    reject();
                } else {
                    this.ready = false;
                }
            };
        });
    }

    execute(request: any): Promise<any> {
        const ws = this.ws;
        if (ws === undefined) {
            return Promise.reject('WebSocket not connected');
        }

        const reqId = this.currentReq++;

        return new Promise((resolve, reject) => {
            this.requests.set(reqId, [resolve, reject]);
            // request.id = reqId;
            ws.send(JSON.stringify(request));
        });
    }

    onMessage(msg: any) {
        if (typeof msg === 'string') {
            const json = JSON.parse(msg) as Response;
            // const callbacks = this.requests.get(json.id);
            // if (typeof callbacks === 'undefined') {
            //     console.error('Missing WebSocket callback of ID', json.id);
            //     return;
            // }
            // this.requests.delete(json.id);
            const reqId = this.currentResp++;
            const callbacks = this.requests.get(reqId);
            if (typeof callbacks === 'undefined') {
                console.error('Should not happen: missing handler for request ID', reqId);
                return;
            }
            this.requests.delete(reqId);
            if (json.err === null) {
                callbacks[0](json);
            } else {
                // Error occurred
                console.error('Control socket error:', json.err);
                callbacks[1](json.err);
            }
        } else {
            console.warn('WebSocket received data with unknown type:', msg);
        }
    }
}

interface Response {
    err: string | null;
}

const globalInstance = new ControlSocket();

async function ensureConnection(): Promise<ControlSocket> {
    await globalInstance.connect();
    return globalInstance;
}

export { ControlSocket, ensureConnection };
