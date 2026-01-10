import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    testTimeout: 3600 * 1000,
  },
});
