/**
 * useAnyForm - Core React hook for anyform integration
 *
 * Provides form state management, validation, and multi-step navigation
 * using a hybrid REST + WASM approach.
 */

import { useState, useEffect, useCallback, useRef, useContext } from 'react';
import { createEngine, type IFormEngine } from '../engine';
import {
  mergeClasses,
  getInputClass,
  getInputType,
  defaultClasses,
} from '../utils/tailwind';
import { AnyFormContext } from '../context/AnyFormProvider';
import type {
  UseAnyFormOptions,
  UseAnyFormReturn,
  FormJson,
  FieldJson,
  FieldOptionJson,
  StepJson,
  StepState,
  FormProps,
  FieldProps,
  SelectProps,
  CheckboxProps,
  RadioGroupProps,
  RadioOptionProps,
  TextareaProps,
  LabelProps,
  FieldMeta,
  StepProps,
  SubmissionResponse,
  ApiError,
} from '../types';

/**
 * React hook for anyform integration.
 *
 * @example
 * ```tsx
 * const form = useAnyForm('contact', {
 *   baseUrl: 'http://localhost:3000',
 *   tailwind: true,
 *   onSuccess: (result) => console.log('Submitted!', result),
 * });
 *
 * return (
 *   <form {...form.getFormProps()}>
 *     {form.visibleFields.map((field) => (
 *       <input key={field.name} {...form.getFieldProps(field.name)} />
 *     ))}
 *     <button type="submit">Submit</button>
 *   </form>
 * );
 * ```
 */
export function useAnyForm(
  slug: string,
  options: UseAnyFormOptions = {}
): UseAnyFormReturn {
  // Get context defaults
  const context = useContext(AnyFormContext);

  // Merge options with context defaults
  const engine = options.engine ?? 'js'; // Default to JS engine
  const baseUrl = options.baseUrl ?? context?.baseUrl ?? '';
  const tailwind = options.tailwind ?? context?.tailwind ?? false;
  const classNames = mergeClasses(
    { ...context?.classNames, ...options.classNames },
    defaultClasses
  );
  const validateOnChange = options.validateOnChange ?? true;
  const validateOnBlur = options.validateOnBlur ?? true;

  // State
  const [formState, setFormState] = useState<IFormEngine | null>(null);
  const [schema, setSchema] = useState<FormJson | null>(
    options.initialSchema ?? null
  );
  const [isLoading, setIsLoading] = useState(!options.initialSchema);
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSubmitted, setIsSubmitted] = useState(false);

  // Track values/errors/touched in React state for re-renders
  const [values, setValues] = useState<Record<string, unknown>>(
    options.initialValues ?? {}
  );
  const [errors, setErrors] = useState<Record<string, string[]>>({});
  const [touched, setTouched] = useState<Record<string, boolean>>({});

  // Force re-render counter
  const [, setRenderKey] = useState(0);
  const triggerRender = useCallback(() => setRenderKey((k) => k + 1), []);

  // Ref to track if component is mounted
  const mountedRef = useRef(true);

  // Initialize engine and fetch form
  useEffect(() => {
    mountedRef.current = true;

    async function initialize() {
      try {
        let formSchema = options.initialSchema;

        // Fetch schema if not provided
        if (!formSchema) {
          const response = await fetch(`${baseUrl}/api/forms/${slug}/json`);
          const json = await response.json();

          if (!response.ok) {
            throw new Error(
              json.error?.message || `Failed to load form: ${response.status}`
            );
          }

          // Handle wrapped response { success: true, data: FormJson }
          formSchema = json.data ?? json;
        }

        if (!mountedRef.current) return;
        if (!formSchema) {
          throw new Error('Failed to load form schema');
        }

        // Create form engine (JS or WASM based on option)
        const state = await createEngine(formSchema, engine);

        // Apply initial values
        if (options.initialValues) {
          for (const [key, value] of Object.entries(options.initialValues)) {
            state.set_value(key, value);
          }
        }

        setFormState(state);
        setSchema(formSchema as FormJson);
        setValues(state.get_values() as Record<string, unknown>);
        setIsLoading(false);
      } catch (e) {
        if (mountedRef.current) {
          setError(e instanceof Error ? e.message : 'Unknown error');
          setIsLoading(false);
        }
      }
    }

    initialize();

    return () => {
      mountedRef.current = false;
    };
  }, [slug, baseUrl, engine, options.initialSchema]);

  // Sync values from WASM state
  const syncValues = useCallback(() => {
    if (formState) {
      setValues(formState.get_values() as Record<string, unknown>);
    }
  }, [formState]);

  // Sync errors from WASM state
  const syncErrors = useCallback(() => {
    if (formState) {
      setErrors(formState.get_all_errors() as Record<string, string[]>);
    }
  }, [formState]);

  // Set a field value
  const setValue = useCallback(
    (field: string, value: unknown) => {
      if (!formState) return;

      formState.set_value(field, value);
      syncValues();

      // Validate if field is touched
      if (validateOnChange && touched[field]) {
        formState.validate_field(field);
        syncErrors();
      }

      triggerRender();
    },
    [formState, touched, validateOnChange, syncValues, syncErrors, triggerRender]
  );

  // Set multiple values
  const setValuesAction = useCallback(
    (newValues: Record<string, unknown>) => {
      if (!formState) return;

      for (const [key, value] of Object.entries(newValues)) {
        formState.set_value(key, value);
      }
      syncValues();
      triggerRender();
    },
    [formState, syncValues, triggerRender]
  );

  // Mark field as touched
  const setTouchedAction = useCallback(
    (field: string) => {
      if (!formState) return;

      formState.mark_touched(field);
      setTouched((prev) => ({ ...prev, [field]: true }));

      if (validateOnBlur) {
        formState.validate_field(field);
        syncErrors();
      }

      triggerRender();
    },
    [formState, validateOnBlur, syncErrors, triggerRender]
  );

  // Validate a single field
  const validateField = useCallback(
    (field: string): string[] => {
      if (!formState) return [];

      const fieldErrors = formState.validate_field(field);
      syncErrors();
      return fieldErrors;
    },
    [formState, syncErrors]
  );

  // Validate all visible fields
  const validateAll = useCallback((): Record<string, string[]> => {
    if (!formState) return {};

    const allErrors = formState.validate_all() as Record<string, string[]>;
    setErrors(allErrors);
    return allErrors;
  }, [formState]);

  // Multi-step navigation
  const nextStep = useCallback((): boolean => {
    if (!formState) return false;

    const success = formState.next_step();
    if (success) {
      triggerRender();
    }
    return success;
  }, [formState, triggerRender]);

  const prevStep = useCallback((): boolean => {
    if (!formState) return false;

    const success = formState.prev_step();
    if (success) {
      triggerRender();
    }
    return success;
  }, [formState, triggerRender]);

  const goToStep = useCallback(
    (stepId: string): boolean => {
      if (!formState) return false;

      const success = formState.go_to_step(stepId);
      if (success) {
        triggerRender();
      }
      return success;
    },
    [formState, triggerRender]
  );

  // Submit form
  const submit = useCallback(async (): Promise<void> => {
    if (!formState || !schema) return;

    // Validate all fields first
    const allErrors = validateAll();
    const hasErrors = Object.values(allErrors).some((e) => e.length > 0);

    if (hasErrors) {
      // Mark all fields as touched to show errors
      const visibleFieldNames = (formState.visible_fields() as FieldJson[]).map(
        (f) => f.name
      );
      for (const name of visibleFieldNames) {
        formState.mark_touched(name);
      }
      setTouched((prev) => {
        const next = { ...prev };
        for (const name of visibleFieldNames) {
          next[name] = true;
        }
        return next;
      });
      return;
    }

    setIsSubmitting(true);

    try {
      const formValues = formState.get_values() as Record<string, unknown>;

      // Use custom onSubmit if provided
      if (options.onSubmit) {
        await options.onSubmit(formValues);
        setIsSubmitted(true);
        return;
      }

      // Get action URL from schema (PRD 0.4.2 feature)
      const actionUrl =
        schema.action_url ||
        schema.settings?.action_url ||
        `${baseUrl}/api/forms/${slug}`;
      const actionMethod =
        schema.action_method || schema.settings?.method || 'POST';

      const response = await fetch(actionUrl, {
        method: actionMethod,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formValues),
      });

      const json = await response.json();

      if (!response.ok) {
        const apiError: ApiError = json.error ?? {
          code: 'SUBMISSION_FAILED',
          message: json.message || 'Submission failed',
        };
        options.onError?.(apiError);
        setError(apiError.message);
        return;
      }

      const result: SubmissionResponse = json.data ?? json;
      setIsSubmitted(true);
      options.onSuccess?.(result);
    } catch (e) {
      const apiError: ApiError = {
        code: 'NETWORK_ERROR',
        message: e instanceof Error ? e.message : 'Network error',
      };
      options.onError?.(apiError);
      setError(apiError.message);
    } finally {
      setIsSubmitting(false);
    }
  }, [formState, schema, slug, baseUrl, options, validateAll]);

  // Reset form
  const reset = useCallback(async () => {
    if (!schema) return;

    // Re-create engine
    const state = await createEngine(schema, engine);

    if (options.initialValues) {
      for (const [key, value] of Object.entries(options.initialValues)) {
        state.set_value(key, value);
      }
    }

    setFormState(state);
    setValues(state.get_values() as Record<string, unknown>);
    setErrors({});
    setTouched({});
    setIsSubmitted(false);
    setError(null);
  }, [schema, engine, options.initialValues]);

  // Get form props
  const getFormProps = useCallback((): FormProps => {
    return {
      onSubmit: (e: React.FormEvent) => {
        e.preventDefault();
        submit();
      },
      className: tailwind ? classNames.form : undefined,
    };
  }, [submit, tailwind, classNames]);

  // Get field props
  const getFieldProps = useCallback(
    (fieldName: string): FieldProps => {
      if (!formState || !schema) {
        return {
          name: fieldName,
          id: fieldName,
          value: '',
          onChange: () => {},
          onBlur: () => {},
        };
      }

      // Find field definition
      const field = schema.steps
        .flatMap((s) => s.fields)
        .find((f) => f.name === fieldName);

      const fieldValue = formState.get_value(fieldName);
      const fieldErrors = formState.get_errors(fieldName);
      const hasError = fieldErrors.length > 0 && touched[fieldName];

      const props: FieldProps = {
        name: fieldName,
        id: `field-${fieldName}`,
        value: fieldValue ?? '',
        onChange: (e) => {
          const target = e.target;
          let newValue: unknown;

          if (target.type === 'checkbox') {
            newValue = (target as HTMLInputElement).checked;
          } else if (target.type === 'number') {
            newValue = target.value === '' ? '' : Number(target.value);
          } else {
            newValue = target.value;
          }

          setValue(fieldName, newValue);
        },
        onBlur: () => setTouchedAction(fieldName),
        'aria-invalid': hasError || undefined,
        'aria-describedby': hasError ? `${fieldName}-error` : undefined,
      };

      // Add field-specific props
      if (field) {
        props.placeholder = field.placeholder;
        props.required = field.required;
        props.disabled = field.ui_options?.disabled;
        props.readOnly = field.ui_options?.readonly;
        props.type = getInputType(field.field_type);
        props.autoFocus = field.ui_options?.autofocus;
        props.autoComplete = field.ui_options?.autocomplete;

        // Validation attributes
        if (field.validation.min_length !== undefined) {
          props.minLength = field.validation.min_length;
        }
        if (field.validation.max_length !== undefined) {
          props.maxLength = field.validation.max_length;
        }
        if (field.validation.min !== undefined) {
          props.min = field.validation.min;
        }
        if (field.validation.max !== undefined) {
          props.max = field.validation.max;
        }
        if (field.validation.step !== undefined) {
          props.step = field.validation.step;
        }
        if (field.validation.pattern !== undefined) {
          props.pattern = field.validation.pattern;
        }

        // Textarea specific
        if (field.field_type === 'textarea') {
          props.rows = field.ui_options?.rows;
          props.cols = field.ui_options?.cols;
        }

        // Tailwind classes
        if (tailwind) {
          props.className = getInputClass(field.field_type, hasError, classNames);
        }
      }

      return props;
    },
    [formState, schema, touched, setValue, setTouchedAction, tailwind, classNames]
  );

  // Get step navigation props
  const getStepProps = useCallback((): StepProps => {
    return {
      onNext: nextStep,
      onPrev: prevStep,
      canGoNext: formState?.can_go_next() ?? false,
      canGoPrev: formState?.can_go_prev() ?? false,
      isLastStep: formState?.is_last_step() ?? true,
    };
  }, [formState, nextStep, prevStep]);

  // Helper to find field definition
  const findField = useCallback(
    (fieldName: string): FieldJson | null => {
      if (!schema) return null;
      return (
        schema.steps.flatMap((s) => s.fields).find((f) => f.name === fieldName) ??
        null
      );
    },
    [schema]
  );

  // Get field metadata
  const getFieldMeta = useCallback(
    (fieldName: string): FieldMeta => {
      const field = findField(fieldName);
      const value = formState?.get_value(fieldName) ?? null;
      const fieldErrors = formState?.get_errors(fieldName) ?? [];
      const isTouched = touched[fieldName] ?? false;

      return {
        field,
        value,
        errors: fieldErrors,
        touched: isTouched,
        hasError: fieldErrors.length > 0 && isTouched,
        isVisible: formState?.is_field_visible(fieldName) ?? false,
      };
    },
    [formState, schema, touched, findField]
  );

  // Get select props
  const getSelectProps = useCallback(
    (fieldName: string): SelectProps => {
      const meta = getFieldMeta(fieldName);
      const field = meta.field;

      return {
        name: fieldName,
        id: `field-${fieldName}`,
        value: (meta.value as string) ?? '',
        onChange: (e: React.ChangeEvent<HTMLSelectElement>) => {
          if (e.target.multiple) {
            const selected = Array.from(e.target.selectedOptions).map(
              (opt) => opt.value
            );
            setValue(fieldName, selected);
          } else {
            setValue(fieldName, e.target.value);
          }
        },
        onBlur: () => setTouchedAction(fieldName),
        'aria-invalid': meta.hasError || undefined,
        'aria-describedby': meta.hasError ? `${fieldName}-error` : undefined,
        className: tailwind ? classNames.select : undefined,
        required: field?.required,
        disabled: field?.ui_options?.disabled,
        multiple: field?.field_type === 'multi_select',
        options: field?.options ?? [],
      };
    },
    [getFieldMeta, setValue, setTouchedAction, tailwind, classNames]
  );

  // Get checkbox props
  const getCheckboxProps = useCallback(
    (fieldName: string): CheckboxProps => {
      const meta = getFieldMeta(fieldName);
      const field = meta.field;

      return {
        name: fieldName,
        id: `field-${fieldName}`,
        checked: Boolean(meta.value),
        onChange: (e: React.ChangeEvent<HTMLInputElement>) => {
          setValue(fieldName, e.target.checked);
        },
        onBlur: () => setTouchedAction(fieldName),
        'aria-invalid': meta.hasError || undefined,
        'aria-describedby': meta.hasError ? `${fieldName}-error` : undefined,
        className: tailwind ? classNames.checkbox : undefined,
        required: field?.required,
        disabled: field?.ui_options?.disabled,
        type: 'checkbox',
      };
    },
    [getFieldMeta, setValue, setTouchedAction, tailwind, classNames]
  );

  // Get radio group props
  const getRadioGroupProps = useCallback(
    (fieldName: string): RadioGroupProps => {
      const meta = getFieldMeta(fieldName);
      const field = meta.field;
      const currentValue = (meta.value as string) ?? '';

      const getOptionProps = (option: FieldOptionJson): RadioOptionProps => ({
        name: fieldName,
        id: `field-${fieldName}-${option.value}`,
        value: option.value,
        checked: currentValue === option.value,
        onChange: (e: React.ChangeEvent<HTMLInputElement>) => {
          setValue(fieldName, e.target.value);
        },
        onBlur: () => setTouchedAction(fieldName),
        className: tailwind ? classNames.radio : undefined,
        disabled: field?.ui_options?.disabled,
        type: 'radio',
      });

      return {
        name: fieldName,
        value: currentValue,
        options: field?.options ?? [],
        'aria-invalid': meta.hasError || undefined,
        'aria-describedby': meta.hasError ? `${fieldName}-error` : undefined,
        required: field?.required,
        disabled: field?.ui_options?.disabled,
        getOptionProps,
      };
    },
    [getFieldMeta, setValue, setTouchedAction, tailwind, classNames]
  );

  // Get textarea props
  const getTextareaProps = useCallback(
    (fieldName: string): TextareaProps => {
      const meta = getFieldMeta(fieldName);
      const field = meta.field;

      return {
        name: fieldName,
        id: `field-${fieldName}`,
        value: (meta.value as string) ?? '',
        onChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => {
          setValue(fieldName, e.target.value);
        },
        onBlur: () => setTouchedAction(fieldName),
        'aria-invalid': meta.hasError || undefined,
        'aria-describedby': meta.hasError ? `${fieldName}-error` : undefined,
        className: tailwind
          ? meta.hasError
            ? classNames.inputError
            : classNames.textarea
          : undefined,
        placeholder: field?.placeholder,
        required: field?.required,
        disabled: field?.ui_options?.disabled,
        readOnly: field?.ui_options?.readonly,
        rows: field?.ui_options?.rows,
        cols: field?.ui_options?.cols,
        minLength: field?.validation?.min_length,
        maxLength: field?.validation?.max_length,
      };
    },
    [getFieldMeta, setValue, setTouchedAction, tailwind, classNames]
  );

  // Get label props
  const getLabelProps = useCallback(
    (fieldName: string): LabelProps => {
      const field = findField(fieldName);

      return {
        htmlFor: `field-${fieldName}`,
        className: tailwind ? classNames.label : undefined,
        children: field?.label ?? fieldName,
        required: field?.required,
      };
    },
    [findField, tailwind, classNames]
  );

  // Visibility helpers
  const isFieldVisible = useCallback(
    (fieldName: string): boolean => {
      return formState?.is_field_visible(fieldName) ?? false;
    },
    [formState]
  );

  const isStepVisible = useCallback(
    (stepId: string): boolean => {
      return formState?.is_step_visible(stepId) ?? false;
    },
    [formState]
  );

  // Computed properties
  const visibleFields: FieldJson[] = formState
    ? (formState.visible_fields() as FieldJson[])
    : [];

  const visibleSteps: StepJson[] = formState
    ? (formState.visible_steps() as StepJson[])
    : [];

  const isValid = formState?.is_valid() ?? false;

  // Build step state (null if single-step form)
  let step: StepState | null = null;
  if (formState && visibleSteps.length > 1) {
    const currentStep = formState.current_step() as StepJson | null;
    const progress = formState.progress();
    const currentIndex = formState.current_step_index();

    step = {
      current: currentStep,
      index: currentIndex,
      total: visibleSteps.length,
      isFirst: currentIndex === 0,
      isLast: formState.is_last_step(),
      canGoNext: formState.can_go_next(),
      canGoPrev: formState.can_go_prev(),
      progress: [progress[0], progress[1]] as [number, number],
    };
  }

  return {
    // State
    schema,
    values,
    errors,
    touched,
    isValid,
    isLoading,
    isSubmitting,
    isSubmitted,
    error,
    step,

    // Actions
    setValue,
    setValues: setValuesAction,
    setTouched: setTouchedAction,
    validateField,
    validateAll,
    nextStep,
    prevStep,
    goToStep,
    submit,
    reset,

    // Props helpers
    getFormProps,
    getFieldProps,
    getSelectProps,
    getCheckboxProps,
    getRadioGroupProps,
    getTextareaProps,
    getLabelProps,
    getFieldMeta,
    getStepProps,

    // Visibility
    isFieldVisible,
    isStepVisible,
    visibleFields,
    visibleSteps,

    // Internal
    formState,
  };
}
