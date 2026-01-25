import {
  GameApiIncomingMessage,
  GameApiOutgoingMessage,
} from "./BaseGameApi.js";
import * as net from "node:net";
import { clearTimeout, promises } from "node:timers";

export interface RemoteApiTransport {
  connect(): Promise<void>;

  setOnReceive(
    onReceive: ((message: GameApiIncomingMessage) => void) | null,
  ): void;

  close(): Promise<void>;

  send(message: GameApiOutgoingMessage & { replyTo: string }): void;
}
export class TCPTransport implements RemoteApiTransport {
  private readonly host: string;
  private onReceive: ((message: GameApiIncomingMessage) => void) | null;
  private connection: net.Socket | null;

  public constructor(host: string) {
    this.host = host;
    this.onReceive = null;
    this.connection = null;
  }

  async connect(): Promise<void> {
    const [host, port] = this.host.split(":");
    const client = await new Promise<net.Socket>((resolve) => {
      const c = net.createConnection(
        {
          host,
          port: parseInt(port),
          keepAlive: true,
          allowHalfOpen: true,
          noDelay: true,
        },
        () => {
          console.log("connected to server!");
          resolve(c);
        },
      );
    });

    this.connection = client;

    let bigBuffer: number[] = [];
    // console.log("XD");
    client.on("data", (data) => {
      // console.log("RECS", data.toString("utf-8"));
      bigBuffer.push(...data);
      while (bigBuffer.length > 0) {
        const indexOf0 = bigBuffer.indexOf(0x00);
        if (indexOf0 === -1) {
          // not found
          break;
        }
        if (indexOf0 == 0) {
          bigBuffer.shift();
          continue;
        }
        const msgSlice = Buffer.from(bigBuffer.slice(0, indexOf0)).toString(
          "utf-8",
        );
        bigBuffer = bigBuffer.slice(indexOf0);
        // console.log(msgSlice);
        let msgParts = msgSlice.split("\n");
        this.onReceive({
          name: msgParts[0],
          payload: JSON.parse(msgParts[1]),
          success: msgParts[2] === "ok",
        });
      }
    });
    client.on("end", () => {
      console.log("disconnected from server");
      this.connection = null;
    });

    process.on("exit", async () => {
      this.connection?.resetAndDestroy();
    });
  }

  setOnReceive(
    onReceive: ((message: GameApiIncomingMessage) => void) | null,
  ): void {
    this.onReceive = onReceive;
  }

  async close(): Promise<void> {
    this.connection?.resetAndDestroy();
  }

  private outQueue: string[] = [];
  private loopActive: boolean = false;

  async send(
    message: GameApiOutgoingMessage & { replyTo: string },
  ): Promise<void> {
    if (!this.connection) {
      throw new Error("not connected to server");
    }
    // console.log(
    //   "WRITE",
    //   `${message.name}\n${message.replyTo}\n${
    //     typeof message.payload === "object"
    //       ? JSON.stringify(message.payload)
    //       : message.payload.toString()
    //   }\0`,
    // );
    this.outQueue.push(
      `${message.name}\n${message.replyTo}\n${
        typeof message.payload === "object"
          ? JSON.stringify(message.payload)
          : message.payload.toString()
      }\0`,
    );
    if (this.loopActive) {
      return;
    }
    this.loopActive = true;
    void this.flushQueue();
    // this.connection.end();
  }

  private async flushQueue() {
    while (true) {
      if (this.outQueue.length === 0) {
        break;
      }
      console.log(this.outQueue.length);
      let bigmsg = this.outQueue.join("");
      this.outQueue = [];
      await new Promise<void>(async (resolve, reject) => {
        while (!this.connection.writable) {
          await promises.setTimeout(1);
        }
        let abort = setTimeout(() => {
          console.log("DED");
          reject(new Error(`Timed out waiting for write to complete`));
        }, 5000);
        this.connection.write(bigmsg, (e) => {
          clearTimeout(abort);
          if (e) {
            reject(e);
          } else {
            resolve();
          }
        });
      });
    }
    this.loopActive = false;
  }
}
