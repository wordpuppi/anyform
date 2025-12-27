/**
 * Test setup file
 *
 * Configures Jest-DOM matchers and global mocks.
 */

import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock anyform-js WASM module
vi.mock('anyform-js', async () => {
  const { MockFormState } = await import('./mocks/anyform-js');

  return {
    default: vi.fn().mockResolvedValue(undefined),
    FormState: MockFormState,
    FormClient: vi.fn(),
    hydrate: vi.fn(),
    hydrate_all: vi.fn(),
    version: vi.fn().mockReturnValue('0.4.0-mock'),
  };
});

// Mock fetch globally
global.fetch = vi.fn();

// Reset mocks between tests
beforeEach(() => {
  vi.clearAllMocks();
  (global.fetch as ReturnType<typeof vi.fn>).mockReset();
});
