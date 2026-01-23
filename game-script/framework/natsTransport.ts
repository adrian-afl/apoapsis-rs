import { connect, NatsConnection } from "nats";
import {
  GameApiIncomingMessage,
  GameApiOutgoingMessage,
} from "./BaseGameApi.js";
import * as net from "node:net";
import { StringDecoder } from "node:string_decoder";

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
      const c = net.createConnection({ host, port: parseInt(port) }, () => {
        console.log("connected to server!");
        resolve(c);
      });
    });

    this.connection = client;

    let bigBuffer: number[] = [];
    console.log("XD");
    client.on("data", (data) => {
      console.log("RECS", data.toString("utf-8"));
      bigBuffer.push(...data);
      while (true) {
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
        console.log(msgSlice);
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

  send(message: GameApiOutgoingMessage & { replyTo: string }): void {
    if (!this.connection) {
      throw new Error("not connected to nats");
    }
    console.log(
      "WRITE",
      `${message.name}\n${message.replyTo}\n${
        typeof message.payload === "object"
          ? JSON.stringify(message.payload)
          : message.payload.toString()
      }\0`,
    );
    this.connection.write(
      `${message.name}\n${message.replyTo}\n${
        typeof message.payload === "object"
          ? JSON.stringify(message.payload)
          : message.payload.toString()
      }\0`,
    );
    // this.connection.end();
  }
}

export class NATSTransport implements RemoteApiTransport {
  private readonly host: string;
  private onReceive: ((message: GameApiIncomingMessage) => void) | null;
  private connection: NatsConnection | null;

  public constructor(host: string) {
    this.host = host;
    this.onReceive = null;
    this.connection = null;
  }

  async connect(): Promise<void> {
    this.connection = await connect({ servers: this.host, noEcho: true });
    this.connection.subscribe(">", {
      callback: (_, x) => {
        const name = x.subject;
        const payload = JSON.parse(Buffer.from(x.data).toString("utf-8"));
        const success = x.headers?.get("status") === "ok";
        // console.log(name, success ? "ok" : "fail", payload);
        this.onReceive?.({ name, payload, success });
      },
    });
    console.log(`connected to ${this.connection.getServer()}`);

    process.on("exit", async () => {
      await this.connection?.drain();
      await this.connection?.close();
    });
  }

  setOnReceive(
    onReceive: ((message: GameApiIncomingMessage) => void) | null,
  ): void {
    this.onReceive = onReceive;
  }

  async close(): Promise<void> {
    await this.connection?.drain();
    await this.connection?.closed();
  }

  send(message: GameApiOutgoingMessage & { replyTo: string }): void {
    if (!this.connection) {
      throw new Error("not connected to nats");
    }
    this.connection.publish(
      message.name,
      typeof message.payload === "object"
        ? JSON.stringify(message.payload)
        : message.payload.toString(),
      {
        reply: message.replyTo,
      },
    );
  }
}
