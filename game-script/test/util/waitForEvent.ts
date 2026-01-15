import {
  AbstractBaseEvent,
  BaseGameApi,
  Constructor,
} from "../../framework/BaseGameApi";

export function waitForEvent<T extends AbstractBaseEvent>(
  api: BaseGameApi,
  event: Constructor<T>,
  additionalMatcher?: (event: T) => boolean,
): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    api.subscribe(event, (received) => {
      if (additionalMatcher) {
        if (additionalMatcher(received)) {
          resolve(received);
        }
      } else {
        resolve(received);
      }
    });
  });
}
