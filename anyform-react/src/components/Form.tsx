/**
 * AnyForm - Headless form component using render props pattern
 *
 * Provides a convenient wrapper around useAnyForm for component-based usage.
 */

import { useAnyForm } from '../hooks/useAnyForm';
import type { AnyFormProps } from '../types';

/**
 * Headless form component that provides form state via render props.
 *
 * @example
 * ```tsx
 * import { AnyForm } from '@anyform/react';
 *
 * function ContactPage() {
 *   return (
 *     <AnyForm slug="contact" options={{ tailwind: true }}>
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
 *     </AnyForm>
 *   );
 * }
 * ```
 */
export function AnyForm({ slug, options, children }: AnyFormProps) {
  const form = useAnyForm(slug, options);
  return <>{children(form)}</>;
}
