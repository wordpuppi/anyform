'use server';

/**
 * Server Actions for form submission
 *
 * These run on the server and can be called directly from client components.
 */

import type { SubmissionResponse, ApiError } from '@wordpuppi/anyform-react';

/** Result type for submitForm action */
export type SubmitFormResult =
  | { success: true; data: SubmissionResponse }
  | { success: false; error: ApiError };

/**
 * Server Action to submit form data.
 *
 * @example
 * ```tsx
 * 'use client';
 * import { submitForm } from '@wordpuppi/anyform-next/server';
 *
 * function ContactForm() {
 *   const handleSubmit = async (values) => {
 *     const result = await submitForm('contact', values);
 *     if (result.success) {
 *       router.push('/thanks');
 *     } else {
 *       setError(result.error.message);
 *     }
 *   };
 *   // ...
 * }
 * ```
 */
export async function submitForm(
  slug: string,
  data: Record<string, unknown>,
  options?: {
    baseUrl?: string;
    actionUrl?: string;
    method?: string;
  }
): Promise<SubmitFormResult> {
  const baseUrl = options?.baseUrl ?? process.env.ANYFORM_API_URL ?? '';
  const url = options?.actionUrl ?? `${baseUrl}/api/forms/${slug}`;
  const method = options?.method ?? 'POST';

  try {
    const response = await fetch(url, {
      method,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });

    const json = await response.json();

    if (!response.ok) {
      return {
        success: false,
        error: json.error ?? {
          code: 'SUBMISSION_FAILED',
          message: json.message || `Submission failed: ${response.status}`,
        },
      };
    }

    return {
      success: true,
      data: json.data ?? json,
    };
  } catch (e) {
    return {
      success: false,
      error: {
        code: 'NETWORK_ERROR',
        message: e instanceof Error ? e.message : 'Network error',
      },
    };
  }
}

/**
 * Server Action to validate form data without submitting.
 *
 * Useful for server-side validation before final submission.
 */
export async function validateForm(
  slug: string,
  data: Record<string, unknown>,
  baseUrl?: string
): Promise<{ valid: boolean; errors: Record<string, string[]> }> {
  const apiBaseUrl = baseUrl ?? process.env.ANYFORM_API_URL ?? '';
  const url = `${apiBaseUrl}/api/forms/${slug}/validate`;

  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });

    const json = await response.json();

    if (json.valid !== undefined) {
      return {
        valid: json.valid,
        errors: json.errors ?? {},
      };
    }

    // If endpoint doesn't exist, return valid
    return { valid: true, errors: {} };
  } catch {
    // Validation endpoint might not exist, return valid
    return { valid: true, errors: {} };
  }
}
