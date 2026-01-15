import { BaseGameApi, Constructor } from "../../framework/BaseGameApi";
import { AbstractBaseEvent } from "../../generated/RemoteGameEvents";

export function waitForEvent<T extends AbstractBaseEvent>(
  api: BaseGameApi,
  event: Constructor<T>,
  additionalMatcher?: (event: T) => boolean,
): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const sub = api.subscribe(event, (received) => {
      if (additionalMatcher) {
        if (additionalMatcher(received)) {
          resolve(received);
          api.unsubscribeBySubscriptionId(sub);
        }
      } else {
        resolve(received);
      }
    });
  });
}
