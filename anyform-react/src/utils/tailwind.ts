/**
 * Tailwind CSS utility class generators for anyform
 *
 * These are sensible defaults that can be overridden via classNames option.
 */

import type { ClassNames, FieldType } from '../types';

/** Default Tailwind classes */
export const defaultClasses: Required<ClassNames> = {
  form: 'space-y-6',
  field: 'space-y-1',
  fieldError: 'space-y-1',
  label: 'block text-sm font-medium text-gray-700',
  input:
    'block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm',
  inputError:
    'block w-full rounded-md border-red-300 shadow-sm focus:border-red-500 focus:ring-red-500 sm:text-sm text-red-900 placeholder-red-300',
  textarea:
    'block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm',
  select:
    'block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm',
  checkbox: 'h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500',
  radio: 'h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500',
  button:
    'inline-flex justify-center rounded-md border border-transparent px-4 py-2 text-sm font-medium shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2',
  buttonPrimary:
    'bg-indigo-600 text-white hover:bg-indigo-700 focus:ring-indigo-500',
  buttonSecondary:
    'bg-white text-gray-700 border-gray-300 hover:bg-gray-50 focus:ring-indigo-500',
  helpText: 'text-sm text-gray-500',
  errorMessage: 'text-sm text-red-600',
  stepContainer: 'space-y-8',
  stepProgress: 'flex items-center justify-between',
};

/**
 * Merges default Tailwind classes with custom class overrides.
 */
export function mergeClasses(
  custom: ClassNames = {},
  defaults: Required<ClassNames> = defaultClasses
): Required<ClassNames> {
  return {
    form: custom.form ?? defaults.form,
    field: custom.field ?? defaults.field,
    fieldError: custom.fieldError ?? defaults.fieldError,
    label: custom.label ?? defaults.label,
    input: custom.input ?? defaults.input,
    inputError: custom.inputError ?? defaults.inputError,
    textarea: custom.textarea ?? defaults.textarea,
    select: custom.select ?? defaults.select,
    checkbox: custom.checkbox ?? defaults.checkbox,
    radio: custom.radio ?? defaults.radio,
    button: custom.button ?? defaults.button,
    buttonPrimary: custom.buttonPrimary ?? defaults.buttonPrimary,
    buttonSecondary: custom.buttonSecondary ?? defaults.buttonSecondary,
    helpText: custom.helpText ?? defaults.helpText,
    errorMessage: custom.errorMessage ?? defaults.errorMessage,
    stepContainer: custom.stepContainer ?? defaults.stepContainer,
    stepProgress: custom.stepProgress ?? defaults.stepProgress,
  };
}

/**
 * Gets the appropriate input class based on field type and error state.
 */
export function getInputClass(
  fieldType: FieldType,
  hasError: boolean,
  classes: Required<ClassNames>
): string {
  if (hasError) {
    return classes.inputError;
  }

  switch (fieldType) {
    case 'textarea':
      return classes.textarea;
    case 'select':
    case 'multi_select':
      return classes.select;
    case 'checkbox':
      return classes.checkbox;
    case 'radio':
      return classes.radio;
    default:
      return classes.input;
  }
}

/**
 * Gets the HTML input type for a field type.
 */
export function getInputType(fieldType: FieldType): string {
  switch (fieldType) {
    case 'email':
      return 'email';
    case 'url':
      return 'url';
    case 'tel':
      return 'tel';
    case 'number':
    case 'rating':
    case 'scale':
    case 'nps':
      return 'number';
    case 'date':
      return 'date';
    case 'datetime':
      return 'datetime-local';
    case 'time':
      return 'time';
    case 'file':
    case 'image':
      return 'file';
    case 'hidden':
      return 'hidden';
    case 'checkbox':
      return 'checkbox';
    case 'radio':
      return 'radio';
    default:
      return 'text';
  }
}
