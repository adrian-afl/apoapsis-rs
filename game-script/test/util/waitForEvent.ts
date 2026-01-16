import { BaseGameApi, Constructor } from "../../framework/BaseGameApi";
import { AbstractBaseEvent } from "../../generated/RemoteGameEvents";
import { clearTimeout } from "node:timers";

export function waitForEvent<T extends AbstractBaseEvent>(
  api: BaseGameApi,
  event: Constructor<T>,
  additionalOptions?: {
    additionalMatcher?: (event: T) => boolean;
    timeout?: number;
  },
): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    if (additionalOptions?.timeout !== undefined) {
      timeoutId = setTimeout(reject, additionalOptions?.timeout);
    }
    const sub = api.subscribe(event, (received) => {
      if (additionalOptions?.additionalMatcher !== undefined) {
        if (additionalOptions?.additionalMatcher(received)) {
          if (timeoutId !== null) {
            clearTimeout(timeoutId);
          }
          resolve(received);
          api.unsubscribeBySubscriptionId(sub);
        }
      } else {
        resolve(received);
      }
    });
  });
}
