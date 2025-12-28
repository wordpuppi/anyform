/**
 * AutoFormField - Automatically renders the correct input type based on field.field_type.
 *
 * Eliminates the need for a switch statement in user code.
 */

import type { FieldJson, UseAnyFormReturn } from '../types';

export interface AutoFormFieldProps {
  /** Field definition from the form schema */
  field: FieldJson;
  /** Form state from useAnyForm hook */
  form: UseAnyFormReturn;
  /** Custom class for the wrapper div */
  className?: string;
  /** Custom class for error messages */
  errorClassName?: string;
  /** Override the default render for a field */
  renderField?: (field: FieldJson, form: UseAnyFormReturn) => React.ReactNode;
}

/**
 * Auto-renders the correct input type based on field.field_type.
 *
 * @example
 * ```tsx
 * import { useAnyForm, AutoFormField } from '@wordpuppi/anyform-react';
 *
 * function ContactForm() {
 *   const form = useAnyForm('contact', { baseUrl: '...' });
 *
 *   return (
 *     <form {...form.getFormProps()}>
 *       {form.visibleFields.map((field) => (
 *         <AutoFormField key={field.name} field={field} form={form} />
 *       ))}
 *       <button type="submit">Submit</button>
 *     </form>
 *   );
 * }
 * ```
 */
export function AutoFormField({
  field,
  form,
  className = '',
  errorClassName = 'text-red-500 text-sm',
  renderField,
}: AutoFormFieldProps) {
  // Allow custom override
  if (renderField) {
    return <>{renderField(field, form)}</>;
  }

  const errors = form.errors[field.name];
  const hasError = errors && errors.length > 0;

  const renderErrors = () =>
    hasError &&
    errors.map((e, i) => (
      <span key={i} className={errorClassName}>
        {e}
      </span>
    ));

  switch (field.field_type) {
    case 'textarea':
      return (
        <div className={className}>
          <label {...form.getLabelProps(field.name)} />
          <textarea {...form.getTextareaProps(field.name)} />
          {renderErrors()}
        </div>
      );

    case 'select':
    case 'multi_select': {
      const props = form.getSelectProps(field.name);
      return (
        <div className={className}>
          <label {...form.getLabelProps(field.name)} />
          <select {...props}>
            {!props.multiple && <option value="">Select...</option>}
            {props.options.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
          {renderErrors()}
        </div>
      );
    }

    case 'radio': {
      const props = form.getRadioGroupProps(field.name);
      return (
        <fieldset className={className}>
          <legend>{field.label}</legend>
          {props.options.map((opt) => (
            <label key={opt.value} className="flex items-center gap-2">
              <input {...props.getOptionProps(opt)} />
              {opt.label}
            </label>
          ))}
          {renderErrors()}
        </fieldset>
      );
    }

    case 'checkbox':
      return (
        <div className={className}>
          <label className="flex items-center gap-2">
            <input {...form.getCheckboxProps(field.name)} />
            {field.label}
          </label>
          {renderErrors()}
        </div>
      );

    default: {
      const fieldProps = form.getFieldProps(field.name);
      return (
        <div className={className}>
          <label {...form.getLabelProps(field.name)} />
          <input
            {...fieldProps}
            value={fieldProps.value as string | number | readonly string[] | undefined}
          />
          {renderErrors()}
        </div>
      );
    }
  }
}
