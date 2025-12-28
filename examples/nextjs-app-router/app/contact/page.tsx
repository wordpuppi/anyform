/**
 * Next.js App Router Example - Contact Form
 *
 * This example demonstrates how to use @wordpuppi/anyform-next with
 * React Server Components for optimal performance.
 */

import { AnyFormRSC } from '@wordpuppi/anyform-next';

export default function ContactPage() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4">
      <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
        <AnyFormRSC slug="contact" options={{ tailwind: true }}>
          {(form) => <ContactFormUI form={form} />}
        </AnyFormRSC>
      </div>
    </div>
  );
}

function ContactFormUI({ form }: { form: ReturnType<typeof import('@wordpuppi/anyform-next').useAnyForm> }) {
  // Success state
  if (form.isSubmitted) {
    return (
      <div className="text-center py-8">
        <div className="text-6xl mb-4">✓</div>
        <h2 className="text-2xl font-bold text-gray-900">Thank you!</h2>
        <p className="text-gray-600 mt-2">We'll be in touch soon.</p>
        <button
          onClick={form.reset}
          className="mt-6 px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
        >
          Send another message
        </button>
      </div>
    );
  }

  return (
    <form {...form.getFormProps()}>
      <h1 className="text-2xl font-bold text-gray-900 mb-6">
        {form.schema?.name ?? 'Contact Us'}
      </h1>

      <div className="space-y-4">
        {form.visibleFields.map((field) => (
          <div key={field.name}>
            <label {...form.getLabelProps(field.name)}>
              {field.label}
              {field.required && <span className="text-red-500 ml-1">*</span>}
            </label>

            {field.field_type === 'textarea' ? (
              <textarea {...form.getTextareaProps(field.name)} className="mt-1" />
            ) : field.field_type === 'select' ? (
              <SelectField form={form} name={field.name} />
            ) : (
              <input {...form.getFieldProps(field.name)} className="mt-1" />
            )}

            {form.errors[field.name]?.map((err, i) => (
              <p key={i} className="text-sm text-red-600 mt-1">{err}</p>
            ))}
          </div>
        ))}
      </div>

      <button
        type="submit"
        disabled={form.isSubmitting}
        className="mt-6 w-full py-2 px-4 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50"
      >
        {form.isSubmitting ? 'Sending...' : 'Send Message'}
      </button>
    </form>
  );
}

function SelectField({
  form,
  name,
}: {
  form: ReturnType<typeof import('@wordpuppi/anyform-next').useAnyForm>;
  name: string;
}) {
  const props = form.getSelectProps(name);

  return (
    <select {...props} className="mt-1">
      <option value="">Select...</option>
      {props.options.map((opt) => (
        <option key={opt.value} value={opt.value}>
          {opt.label}
        </option>
      ))}
    </select>
  );
}
