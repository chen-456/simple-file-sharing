import { TinyEmitter } from 'tiny-emitter';

export class FileUploader {
  private readonly BLOCK_SIZE = (1 << 20);

  private file: File;
  private uuid: string;
  private socket: WebSocket | undefined;
  // private fileReader = new FileReader();
  private eventEmitter = new TinyEmitter();

  private current = 0;
  private running = false;
  private error: string | undefined;

  constructor(file: File, uuid: string) {
    this.file = file;
    this.uuid = uuid;

    // this.fileReader.onload = () => this.startNextBlock();
    // this.fileReader.onerror = (ev) => {
    //   console.error('Failed to read file:', ev);
    //   this.error = 'failed to read file';
    //   this.socket?.close();
    //   this.eventEmitter.emit('progress');
    // };
  }

  start(): void {
    this.running = true;
    this.socket = new WebSocket(import.meta.env.VITE_WS_BASE_URL + '/api/uploads/' + this.uuid);
    this.socket.onopen = () => {
      this.startNextBlock();
    };
    this.socket.onmessage = (ev) => {
      if (typeof ev.data === 'string') {
        const resp = JSON.parse(ev.data);
        if (this.running) {
          if (resp.cur_pos !== this.current) {
            console.warn('File position mismatch: local', this.current, 'remote', resp.cur_pos);
          }
          this.startNextBlock();
        } else {
          // Upload done
          this.error = resp.err || undefined;
          this.eventEmitter.emit('progress');
          this.socket?.close();
        }
      } else {
        console.warn('Uploader socket received data of unknown type:', ev);
      }
    };
    this.socket.onerror = (ev) => {
      console.error('Upload socket error:', ev);
      this.socket?.close();
    };
    this.socket.onclose = (ev) => {
      if (!this.running) return;
      console.error('Upload socket closed by server:', ev);
      this.error = 'socket closed by server';
      this.running = false;
      this.eventEmitter.emit('progress');
    };
  }

  private startNextBlock() {
    if (this.current === this.file.size) {
      // Finished
      this.running = false;
      this.socket?.send(JSON.stringify({
        'cmd': 'Finish',
      }));
    } else {
      const block_end = Math.min(this.current + this.BLOCK_SIZE, this.file.size);
      this.socket!.send(this.file.slice(this.current, block_end));
      this.current = block_end;
    }
  }

  onProgress(callback: Function): void {
    this.eventEmitter.on('progress', callback);
  }

  isRunning(): boolean { return this.running; }
  getPercentage(): number { return this.current / this.file.size; }
  getError(): string | undefined { return this.error; }
}
