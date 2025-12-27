/**
 * AnyFormProvider - Context for app-wide anyform configuration
 *
 * Provides default baseUrl, tailwind setting, and classNames to all useAnyForm hooks.
 */

import { createContext, useMemo } from 'react';
import type {
  AnyFormProviderProps,
  AnyFormContextValue,
  ClassNames,
} from '../types';

export const AnyFormContext = createContext<AnyFormContextValue | null>(null);

/**
 * Provider component for anyform configuration.
 *
 * @example
 * ```tsx
 * import { AnyFormProvider } from '@anyform/react';
 *
 * function App() {
 *   return (
 *     <AnyFormProvider baseUrl="https://api.example.com" tailwind>
 *       <ContactForm />
 *       <FeedbackForm />
 *     </AnyFormProvider>
 *   );
 * }
 * ```
 */
export function AnyFormProvider({
  baseUrl = '',
  tailwind = false,
  classNames = {},
  children,
}: AnyFormProviderProps) {
  const value = useMemo<AnyFormContextValue>(
    () => ({
      baseUrl,
      tailwind,
      classNames: classNames as ClassNames,
    }),
    [baseUrl, tailwind, classNames]
  );

  return (
    <AnyFormContext.Provider value={value}>{children}</AnyFormContext.Provider>
  );
}
