/**
 * Multi-Step Form Example
 *
 * This example demonstrates how to use @wordpuppi/anyform-react to build
 * a multi-step wizard form with progress indicator.
 */

import { useAnyForm } from '@wordpuppi/anyform-react';

export function MultiStepForm() {
  const form = useAnyForm('onboarding', {
    baseUrl: 'http://localhost:3000',
    tailwind: true,
  });

  if (form.isLoading) {
    return <div className="p-8 text-center">Loading...</div>;
  }

  if (form.error) {
    return <div className="p-4 text-red-600">Error: {form.error}</div>;
  }

  if (form.isSubmitted) {
    return (
      <div className="p-8 text-center">
        <h2 className="text-2xl font-bold text-green-600">All done!</h2>
        <p className="mt-2 text-gray-600">Your onboarding is complete.</p>
      </div>
    );
  }

  // Single step form fallback
  if (!form.step) {
    return <div>This is a single-step form</div>;
  }

  return (
    <form {...form.getFormProps()} className="max-w-lg mx-auto space-y-8">
      {/* Progress indicator */}
      <div className="space-y-2">
        <div className="flex justify-between text-sm text-gray-600">
          <span>Step {form.step.progress[0]} of {form.step.progress[1]}</span>
          <span>{Math.round((form.step.progress[0] / form.step.progress[1]) * 100)}%</span>
        </div>
        <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
          <div
            className="h-full bg-indigo-600 transition-all duration-300"
            style={{
              width: `${(form.step.progress[0] / form.step.progress[1]) * 100}%`,
            }}
          />
        </div>
      </div>

      {/* Step title */}
      <div>
        <h2 className="text-xl font-semibold text-gray-900">
          {form.step.current?.name}
        </h2>
        {form.step.current?.description && (
          <p className="mt-1 text-gray-600">{form.step.current.description}</p>
        )}
      </div>

      {/* Fields for current step */}
      <div className="space-y-4">
        {form.visibleFields.map((field) => (
          <div key={field.name} className="space-y-1">
            <label {...form.getLabelProps(field.name)}>
              {field.label}
              {field.required && <span className="text-red-500 ml-1">*</span>}
            </label>

            {field.field_type === 'textarea' ? (
              <textarea {...form.getTextareaProps(field.name)} />
            ) : (
              <input {...form.getFieldProps(field.name)} />
            )}

            {form.errors[field.name]?.map((error, i) => (
              <p key={i} className="text-sm text-red-600">{error}</p>
            ))}
          </div>
        ))}
      </div>

      {/* Navigation buttons */}
      <div className="flex justify-between pt-4 border-t border-gray-200">
        <button
          type="button"
          onClick={form.prevStep}
          disabled={!form.step.canGoPrev}
          className="px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Back
        </button>

        {form.step.isLast ? (
          <button
            type="submit"
            disabled={form.isSubmitting}
            className="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50"
          >
            {form.isSubmitting ? 'Submitting...' : 'Complete'}
          </button>
        ) : (
          <button
            type="button"
            onClick={form.nextStep}
            disabled={!form.step.canGoNext}
            className="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50"
          >
            Next
          </button>
        )}
      </div>
    </form>
  );
}
