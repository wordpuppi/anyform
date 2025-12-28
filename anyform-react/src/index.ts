/**
 * @wordpuppi/anyform-react - React hooks for anyform
 *
 * Provides headless form state, validation, and multi-step navigation
 * for React applications.
 *
 * @example
 * ```tsx
 * import { useAnyForm } from '@wordpuppi/anyform-react';
 *
 * function ContactForm() {
 *   const form = useAnyForm('contact', { tailwind: true });
 *
 *   return (
 *     <form {...form.getFormProps()}>
 *       {form.visibleFields.map((field) => (
 *         <input key={field.name} {...form.getFieldProps(field.name)} />
 *       ))}
 *       <button type="submit">Submit</button>
 *     </form>
 *   );
 * }
 * ```
 */

// Hook
export { useAnyForm } from './hooks/useAnyForm';

// Components
export { AnyForm } from './components/Form';
export { AutoFormField } from './components/AutoFormField';
export type { AutoFormFieldProps } from './components/AutoFormField';

// Context
export { AnyFormProvider, AnyFormContext } from './context/AnyFormProvider';

// Engine
export { createEngine, createJsEngine, isWasmLoaded } from './engine';
export type { IFormEngine } from './engine';

// Utils (deprecated, use createEngine instead)
export { ensureWasmInit, isWasmInitialized } from './utils/wasm';
export {
  defaultClasses,
  mergeClasses,
  getInputClass,
  getInputType,
} from './utils/tailwind';

// Types
export type {
  // Form schema types
  FormJson,
  StepJson,
  FieldJson,
  FieldOptionJson,
  FieldType,
  FormSettings,
  ValidationRules,
  UiOptions,
  ConditionRule,
  ConditionOp,
  // Hook types
  UseAnyFormOptions,
  UseAnyFormReturn,
  StepState,
  FieldMeta,
  // Props types
  FormProps,
  FieldProps,
  SelectProps,
  CheckboxProps,
  RadioGroupProps,
  RadioOptionProps,
  TextareaProps,
  LabelProps,
  StepProps,
  // Response types
  SubmissionResponse,
  ApiError,
  // Styling types
  ClassNames,
  // Engine types
  EngineType,
  // Component props
  AnyFormProps,
  AnyFormProviderProps,
  AnyFormContextValue,
} from './types';
