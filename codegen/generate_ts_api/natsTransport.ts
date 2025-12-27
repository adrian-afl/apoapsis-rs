import { connect, NatsConnection } from "nats";
import { GameApiIncomingMessage, GameApiOutgoingMessage } from "./GameApi.js";

export class NATSTransport {
  private readonly host: string;
  private onReceive: ((message: GameApiIncomingMessage) => void) | null;
  private connection: NatsConnection | null;

  public constructor(host: string) {
    this.host = host;
    this.onReceive = null;
    this.connection = null;
  }

  public async connect(): Promise<void> {
    this.connection = await connect({ servers: this.host, noEcho: true });
    this.connection.subscribe("*", {
      callback: (_, x) => {
        console.log(
          "X",
          x.subject,
          x.headers,
          Buffer.from(x.data).toString("utf-8"),
        );
        const name = x.subject;
        const payload = JSON.parse(Buffer.from(x.data).toString("utf-8"));
        const success = x.headers?.get("status") === "ok";
        console.log({ x: { name, payload, success } });
        this.onReceive?.({ name, payload, success });
      },
    });
    console.log(`connected to ${this.connection.getServer()}`);
  }

  public setOnReceive(
    onReceive: ((message: GameApiIncomingMessage) => void) | null,
  ): void {
    this.onReceive = onReceive;
  }

  public async close(): Promise<void> {
    await this.connection?.drain();
    await this.connection?.closed();
  }

  public send(message: GameApiOutgoingMessage & { replyTo: string }): void {
    if (!this.connection) {
      throw new Error("not connected to nats");
    }
    this.connection.publish(message.name, JSON.stringify(message.payload), {
      reply: message.replyTo,
    });
  }
}
