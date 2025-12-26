export interface GameApiIncomingMessage {
  name: string;
  payload: unknown;
  success: boolean;
}

export interface GameApiOutgoingMessage {
  name: string;
  payload: unknown;
  replyTo: string;
}

export type GameApiTransmitter = (
  message: GameApiOutgoingMessage,
) => Promise<void>;

export class GameApi {
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
      const handlers = this.waitingForReply.get(message.name);
      this.waitingForReply.delete(message.name);
      if (message.success) {
        handlers.resolve(message.payload);
      } else {
        handlers.reject(message.payload);
      }
    }
    // other stuff like events
  }

  public send(message: GameApiOutgoingMessage): Promise<unknown> {
    return new Promise<unknown>((resolve, reject) => {
      this.waitingForReply.set(message.replyTo, { resolve, reject });
      void this.transmitter(message).catch(console.error);
    });
  }
}
