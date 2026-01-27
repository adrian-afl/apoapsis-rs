import * as crypto from "crypto";
import {
  eventsConstructors,
  AbstractBaseEvent,
} from "../generated/RemoteGameEvents";
import fastify from "fastify";
import * as fs from "node:fs";

export interface GameApiIncomingMessage {
  name: string;
  payload: unknown;
  success: boolean;
}

export interface GameApiOutgoingMessage {
  name: string;
  payload: unknown;
}

export type GameApiTransmitter = (
  message: GameApiOutgoingMessage & { replyTo: string },
) => Promise<void>;

export type Constructor<T> = new (...args: any[]) => T;

export type EventHandler<EventType extends AbstractBaseEvent> = (
  event: EventType,
) => Promise<void> | void;

type AnyEventHandler = EventHandler<AbstractBaseEvent>;

class Subscription {
  public constructor(
    public readonly id: number,
    public readonly eventName: string,
    public readonly handler: AnyEventHandler,
  ) {}
}

export class BaseGameApi {
  private readonly waitingForReply: Map<
    string,
    {
      resolve: (payload: unknown) => void | Promise<void>;
      reject: (payload: unknown) => void | Promise<void>;
    }
  >;
  private readonly transmitter: GameApiTransmitter;

  private subscriptions: Subscription[] = [];
  private lastSubscriptionId = 1;

  public constructor(transmitter: GameApiTransmitter) {
    this.transmitter = transmitter;
    this.waitingForReply = new Map<
      string,
      {
        resolve: (payload: unknown) => void | Promise<void>;
        reject: (payload: unknown) => void | Promise<void>;
      }
    >();

    const server = fastify();

    server.post("/remote/:command", async (request, reply) => {
      const { command } = request.params as { command: string };
      return this.send({
        name: "command." + command,
        payload: request.body,
      });
    });

    server.get("/", async (request, reply) => {
      reply.header("content-type", "text/html");
      reply.send(fs.readFileSync("./threejs.html"));
    });

    server.listen({ port: 9999 }, (err, address) => {
      if (err) {
        console.error(err);
        process.exit(1);
      }
      console.log(`Fastify Debug Server listening at ${address}`);
    });
  }

  public receive(message: GameApiIncomingMessage): void {
    if (this.waitingForReply.has(message.name)) {
      const handlers = this.waitingForReply.get(message.name)!;
      this.waitingForReply.delete(message.name);
      // console.log({ message });
      if (message.success) {
        handlers.resolve(message.payload);
      } else {
        handlers.reject(message.payload);
      }
      return;
    }
    // other stuff like events
    if (message.name.startsWith("event.")) {
      console.log(message.name.substring("event.".length));
      void this.publish(
        eventsConstructors[message.name.substring("event.".length)](
          message.payload,
        ),
      ).catch((e) => console.error(e));
    }
  }

  public async send(message: GameApiOutgoingMessage): Promise<unknown> {
    const replyTo = `replyTo/${crypto.randomUUID()}`;
    const promise = new Promise<unknown>((resolve, reject) => {
      this.waitingForReply.set(replyTo, { resolve, reject });
      setTimeout(() => {
        reject(
          new Error(`Timed out waiting for command reply: ${message.name}`),
        );
      }, 500000);
    });
    await this.transmitter({ ...message, replyTo });

    return promise;
  }

  public subscribe<T extends AbstractBaseEvent>(
    eventClass: Constructor<T>,
    handler: EventHandler<T>,
  ): number {
    const eventName = eventClass.name;
    const id = this.lastSubscriptionId++;
    this.subscriptions.push(
      new Subscription(id, eventName, handler as AnyEventHandler),
    );
    // console.log(`Subscribed on ${eventName}, subid ${id}`);
    return id;
  }

  public unsubscribeBySubscriptionId(id: number): void {
    // console.log(`Unsubscribed subid ${id}`);
    this.subscriptions = this.subscriptions.filter((s) => s.id !== id);
  }

  public unsubscribeByEventClass<T extends AbstractBaseEvent>(
    eventClass: Constructor<T>,
  ): void {
    const eventName = eventClass.name;
    this.subscriptions = this.subscriptions.filter(
      (s) => s.eventName !== eventName,
    );
  }

  public unsubscribeAllSubscriptions(): void {
    this.subscriptions = [];
  }

  private async publish<T extends AbstractBaseEvent>(event: T): Promise<void> {
    await Promise.all(
      this.subscriptions
        .filter((s) => s.eventName === event.constructor.name)
        .map(async (s) => {
          await s.handler(event);
        }),
    );
  }
}
