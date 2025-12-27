/**
 * Basic Contact Form Example
 *
 * This example demonstrates how to use @anyform/react to build
 * a contact form with validation and error handling.
 */

import { useAnyForm } from '@anyform/react';

export function ContactForm() {
  const form = useAnyForm('contact', {
    baseUrl: 'http://localhost:3000',
    tailwind: true,
    onSuccess: (result) => {
      alert(`Form submitted! ID: ${result.id}`);
    },
    onError: (error) => {
      console.error('Submission failed:', error.message);
    },
  });

  // Loading state
  if (form.isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600" />
      </div>
    );
  }

  // Error state
  if (form.error) {
    return (
      <div className="p-4 bg-red-50 border border-red-200 rounded-md">
        <p className="text-red-600">Error loading form: {form.error}</p>
        <button
          onClick={form.reset}
          className="mt-2 text-sm text-red-600 underline"
        >
          Try again
        </button>
      </div>
    );
  }

  // Success state
  if (form.isSubmitted) {
    return (
      <div className="p-6 bg-green-50 border border-green-200 rounded-md text-center">
        <h2 className="text-xl font-semibold text-green-800">Thank you!</h2>
        <p className="text-green-600 mt-2">Your message has been sent.</p>
        <button
          onClick={form.reset}
          className="mt-4 px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
        >
          Send another message
        </button>
      </div>
    );
  }

  return (
    <form {...form.getFormProps()} className="max-w-md mx-auto space-y-6">
      <h1 className="text-2xl font-bold text-gray-900">
        {form.schema?.name ?? 'Contact Us'}
      </h1>

      {form.schema?.description && (
        <p className="text-gray-600">{form.schema.description}</p>
      )}

      {form.visibleFields.map((field) => (
        <div key={field.name} className="space-y-1">
          {/* Label */}
          <label {...form.getLabelProps(field.name)}>
            {field.label}
            {field.required && <span className="text-red-500 ml-1">*</span>}
          </label>

          {/* Field input based on type */}
          {renderField(form, field)}

          {/* Help text */}
          {field.help_text && (
            <p className="text-sm text-gray-500">{field.help_text}</p>
          )}

          {/* Error messages */}
          {form.errors[field.name]?.map((error, i) => (
            <p key={i} className="text-sm text-red-600" id={`${field.name}-error`}>
              {error}
            </p>
          ))}
        </div>
      ))}

      <button
        type="submit"
        disabled={form.isSubmitting}
        className="w-full py-2 px-4 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {form.isSubmitting ? 'Sending...' : 'Send Message'}
      </button>
    </form>
  );
}

/**
 * Renders the appropriate input element based on field type.
 */
function renderField(form: ReturnType<typeof useAnyForm>, field: typeof form.visibleFields[0]) {
  switch (field.field_type) {
    case 'textarea':
      return <textarea {...form.getTextareaProps(field.name)} />;

    case 'select':
    case 'multi_select': {
      const props = form.getSelectProps(field.name);
      return (
        <select {...props}>
          <option value="">Select...</option>
          {props.options.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      );
    }

    case 'radio': {
      const props = form.getRadioGroupProps(field.name);
      return (
        <div className="space-y-2">
          {props.options.map((opt) => (
            <label key={opt.value} className="flex items-center space-x-2">
              <input {...props.getOptionProps(opt)} />
              <span>{opt.label}</span>
            </label>
          ))}
        </div>
      );
    }

    case 'checkbox':
      return (
        <div className="flex items-center space-x-2">
          <input {...form.getCheckboxProps(field.name)} />
          <span className="text-sm text-gray-600">{field.label}</span>
        </div>
      );

    default:
      return <input {...form.getFieldProps(field.name)} />;
  }
}
