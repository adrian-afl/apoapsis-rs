import * as crypto from "crypto";

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
) => void;

export class BaseGameApi {
  private readonly waitingForReply: Map<
    string,
    {
      resolve: (payload: unknown) => void | Promise<void>;
      reject: (payload: unknown) => void | Promise<void>;
    }
  >;
  private readonly transmitter: GameApiTransmitter;

  public constructor(transmitter: GameApiTransmitter) {
    this.transmitter = transmitter;
    this.waitingForReply = new Map<
      string,
      {
        resolve: (payload: unknown) => void | Promise<void>;
        reject: (payload: unknown) => void | Promise<void>;
      }
    >();
  }

  public receive(message: GameApiIncomingMessage): void {
    if (this.waitingForReply.has(message.name)) {
      const handlers = this.waitingForReply.get(message.name)!;
      this.waitingForReply.delete(message.name);
      console.log({ message });
      if (message.success) {
        handlers.resolve(message.payload);
      } else {
        handlers.reject(message.payload);
      }
    }
    // other stuff like events
  }

  public send(message: GameApiOutgoingMessage): Promise<unknown> {
    const replyTo = `replyTo/${crypto.randomUUID()}`;
    return new Promise<unknown>((resolve, reject) => {
      this.waitingForReply.set(replyTo, { resolve, reject });
      this.transmitter({ ...message, replyTo });
    });
  }
}
