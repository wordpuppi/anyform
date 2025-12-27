'use client';

/**
 * AnyFormClient - Client component for RSC hydration
 *
 * Used internally by AnyFormRSC to handle client-side interactivity.
 */

import { AnyForm } from '@anyform/react';
import type { UseAnyFormOptions, UseAnyFormReturn, FormJson } from '@anyform/react';

export interface AnyFormClientProps {
  slug: string;
  initialSchema: FormJson;
  baseUrl?: string;
  options?: Omit<UseAnyFormOptions, 'initialSchema' | 'baseUrl'>;
  children: (form: UseAnyFormReturn) => React.ReactNode;
}

/**
 * Client component that wraps AnyForm with pre-fetched schema.
 *
 * This component is used by AnyFormRSC to hydrate server-rendered forms.
 */
export function AnyFormClient({
  slug,
  initialSchema,
  baseUrl,
  options,
  children,
}: AnyFormClientProps) {
  return (
    <AnyForm
      slug={slug}
      options={{
        ...options,
        initialSchema,
        baseUrl,
      }}
    >
      {children}
    </AnyForm>
  );
}
