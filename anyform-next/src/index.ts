/**
 * @wordpuppi/anyform-next - Next.js integration for anyform
 *
 * Provides React Server Components and Server Actions for Next.js applications.
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
 *           {form.visibleFields.map((f) => (
 *             <input key={f.name} {...form.getFieldProps(f.name)} />
 *           ))}
 *           <button type="submit">Submit</button>
 *         </form>
 *       )}
 *     </AnyFormRSC>
 *   );
 * }
 * ```
 */

// Re-export everything from @wordpuppi/anyform-react
export * from '@wordpuppi/anyform-react';

// Next.js specific components
export { AnyFormRSC, type AnyFormRSCProps } from './components/AnyFormRSC';
export { AnyFormClient, type AnyFormClientProps } from './components/AnyFormClient';

// Note: Server exports are in '@wordpuppi/anyform-next/server'
// - fetchFormSchema, fetchFormSchemaNoCache
// - submitForm, validateForm
