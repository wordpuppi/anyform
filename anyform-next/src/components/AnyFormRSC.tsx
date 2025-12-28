/**
 * AnyFormRSC - React Server Component for form rendering
 *
 * Fetches form schema on the server and hydrates on the client.
 */

import { fetchFormSchema } from '../server/fetchForm';
import { AnyFormClient } from './AnyFormClient';
import type { UseAnyFormOptions, UseAnyFormReturn } from '@wordpuppi/anyform-react';

export interface AnyFormRSCProps {
  /** Form slug to fetch */
  slug: string;
  /** Base URL for the anyform API (uses ANYFORM_API_URL env var if not specified) */
  baseUrl?: string;
  /** Options passed to useAnyForm (excludes initialSchema and baseUrl) */
  options?: Omit<UseAnyFormOptions, 'initialSchema' | 'baseUrl'>;
  /** Render function that receives form state */
  children: (form: UseAnyFormReturn) => React.ReactNode;
}

/**
 * React Server Component that fetches form schema on the server
 * and hydrates on the client with full interactivity.
 *
 * @example
 * ```tsx
 * // app/contact/page.tsx
 * import { AnyFormRSC } from '@wordpuppi/anyform-next';
 *
 * export default function ContactPage() {
 *   return (
 *     <AnyFormRSC slug="contact" options={{ tailwind: true }}>
 *       {(form) => (
 *         <form {...form.getFormProps()}>
 *           {form.visibleFields.map((field) => (
 *             <div key={field.name}>
 *               <label>{field.label}</label>
 *               <input {...form.getFieldProps(field.name)} />
 *               {form.errors[field.name]?.map((err, i) => (
 *                 <span key={i} className="text-red-500">{err}</span>
 *               ))}
 *             </div>
 *           ))}
 *           <button type="submit" disabled={form.isSubmitting}>
 *             {form.isSubmitting ? 'Submitting...' : 'Submit'}
 *           </button>
 *         </form>
 *       )}
 *     </AnyFormRSC>
 *   );
 * }
 * ```
 */
export async function AnyFormRSC({
  slug,
  baseUrl,
  options,
  children,
}: AnyFormRSCProps) {
  // Fetch schema on the server
  const schema = await fetchFormSchema(slug, baseUrl);

  // Pass to client component for hydration
  return (
    <AnyFormClient
      slug={slug}
      initialSchema={schema}
      baseUrl={baseUrl}
      options={options}
    >
      {children}
    </AnyFormClient>
  );
}
