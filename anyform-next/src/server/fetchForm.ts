/**
 * Server-side form schema fetching for React Server Components
 *
 * Uses React's cache() for deduplication within a single RSC render.
 */

import { cache } from 'react';
import type { FormJson } from '@anyform/react';

/**
 * Fetches a form schema from the anyform API.
 *
 * This function is cached using React's cache() for deduplication
 * within a single RSC render tree.
 *
 * @example
 * ```tsx
 * // app/forms/[slug]/page.tsx
 * import { fetchFormSchema } from '@anyform/next/server';
 *
 * export default async function FormPage({ params }) {
 *   const schema = await fetchFormSchema(params.slug);
 *   return <ClientForm initialSchema={schema} />;
 * }
 * ```
 */
export const fetchFormSchema = cache(
  async (slug: string, baseUrl?: string): Promise<FormJson> => {
    const apiBaseUrl = baseUrl ?? process.env.ANYFORM_API_URL ?? '';
    const url = `${apiBaseUrl}/api/forms/${slug}/json`;

    const response = await fetch(url, {
      next: { revalidate: 60 }, // ISR: revalidate every 60 seconds
    });

    if (!response.ok) {
      throw new Error(`Failed to fetch form: ${response.statusText}`);
    }

    const json = await response.json();

    // Handle wrapped response { success: true, data: FormJson }
    if (json.data) {
      return json.data as FormJson;
    }

    return json as FormJson;
  }
);

/**
 * Fetches a form schema without caching.
 *
 * Use this when you need fresh data on every request.
 */
export async function fetchFormSchemaNoCache(
  slug: string,
  baseUrl?: string
): Promise<FormJson> {
  const apiBaseUrl = baseUrl ?? process.env.ANYFORM_API_URL ?? '';
  const url = `${apiBaseUrl}/api/forms/${slug}/json`;

  const response = await fetch(url, {
    cache: 'no-store',
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch form: ${response.statusText}`);
  }

  const json = await response.json();

  // Handle wrapped response
  if (json.data) {
    return json.data as FormJson;
  }

  return json as FormJson;
}
