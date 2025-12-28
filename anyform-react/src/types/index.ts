/**
 * Type definitions for @wordpuppi/anyform-react
 */

/** Field option for select/radio/checkbox fields */
export interface FieldOptionJson {
  id: string;
  label: string;
  value: string;
  order: number;
}

/** Validation rules for a field */
export interface ValidationRules {
  min_length?: number;
  max_length?: number;
  min?: number;
  max?: number;
  step?: number;
  pattern?: string;
  pattern_message?: string;
  min_selections?: number;
  max_selections?: number;
  allowed_extensions?: string[];
  max_file_size?: number;
  allowed_mime_types?: string[];
  min_date?: string;
  max_date?: string;
  custom?: unknown;
}

/** Condition operator for field/step visibility */
export type ConditionOp =
  | 'eq'
  | 'neq'
  | 'gt'
  | 'gte'
  | 'lt'
  | 'lte'
  | 'contains'
  | 'in'
  | 'empty'
  | 'not_empty';

/** Simple condition rule */
export interface SimpleCondition {
  field: string;
  op: ConditionOp;
  value?: unknown;
}

/** Condition rule (simple, AND, or OR) */
export type ConditionRule =
  | SimpleCondition
  | { and: ConditionRule[] }
  | { or: ConditionRule[] };

/** UI display options for a field */
export interface UiOptions {
  css_class?: string;
  input_class?: string;
  label_class?: string;
  width?: string;
  rows?: number;
  cols?: number;
  autocomplete?: string;
  autofocus?: boolean;
  disabled?: boolean;
  readonly?: boolean;
  inputmode?: string;
  heading_level?: number;
  max_rating?: number;
  scale_min?: number;
  scale_max?: number;
  scale_step?: number;
  scale_labels?: {
    min_label?: string;
    max_label?: string;
    mid_label?: string;
  };
  show_char_count?: boolean;
  custom_attributes?: Record<string, string>;
  condition?: ConditionRule;
}

/** Field type enumeration */
export type FieldType =
  | 'text'
  | 'email'
  | 'url'
  | 'tel'
  | 'number'
  | 'textarea'
  | 'select'
  | 'multi_select'
  | 'radio'
  | 'checkbox'
  | 'date'
  | 'datetime'
  | 'time'
  | 'file'
  | 'image'
  | 'hidden'
  | 'heading'
  | 'paragraph'
  | 'rating'
  | 'scale'
  | 'nps'
  | 'matrix';

/** Field definition from form schema */
export interface FieldJson {
  id: string;
  name: string;
  label: string;
  field_type: FieldType;
  order: number;
  required: boolean;
  placeholder?: string;
  help_text?: string;
  default_value?: unknown;
  validation: ValidationRules;
  ui_options: UiOptions;
  options: FieldOptionJson[];
}

/** Step definition for multi-step forms */
export interface StepJson {
  id: string;
  name: string;
  description?: string;
  order: number;
  condition?: ConditionRule;
  fields: FieldJson[];
}

/** Form settings */
export interface FormSettings {
  submit_label?: string;
  success_message?: string;
  redirect_url?: string;
  notify_emails?: string[];
  show_progress?: boolean;
  allow_partial_save?: boolean;
  css_class?: string;
  action_url?: string;
  method?: string;
  is_quiz?: boolean;
  show_answers?: boolean;
  custom?: unknown;
}

/** Complete form schema */
export interface FormJson {
  id: string;
  name: string;
  slug: string;
  description?: string;
  action_url?: string;
  action_method?: string;
  settings: FormSettings;
  steps: StepJson[];
}

/** Submission response from server */
export interface SubmissionResponse {
  id: string;
  score?: number;
  result?: {
    key: string;
    title: string;
    description?: string;
  };
}

/** API error response */
export interface ApiError {
  code: string;
  message: string;
  details?: unknown;
}

/** Custom class names for Tailwind integration */
export interface ClassNames {
  form?: string;
  field?: string;
  fieldError?: string;
  label?: string;
  input?: string;
  inputError?: string;
  textarea?: string;
  select?: string;
  checkbox?: string;
  radio?: string;
  button?: string;
  buttonPrimary?: string;
  buttonSecondary?: string;
  helpText?: string;
  errorMessage?: string;
  stepContainer?: string;
  stepProgress?: string;
}

/** Engine type for form state management */
export type EngineType = 'js' | 'wasm';

/** Options for useAnyForm hook */
export interface UseAnyFormOptions {
  /** Engine type: 'js' (default, bundled) or 'wasm' (lazy-loaded from CDN) */
  engine?: EngineType;

  /** Base URL for the anyform API (default: '') */
  baseUrl?: string;

  /** Initial form values (for controlled forms or SSR hydration) */
  initialValues?: Record<string, unknown>;

  /** Pre-fetched form schema (for SSR/RSC hydration) */
  initialSchema?: FormJson;

  /** Enable Tailwind utility classes on returned props */
  tailwind?: boolean;

  /** Custom class names to merge with or replace Tailwind classes */
  classNames?: ClassNames;

  /** Validate on change (default: true for touched fields) */
  validateOnChange?: boolean;

  /** Validate on blur (default: true) */
  validateOnBlur?: boolean;

  /** Custom submission handler (overrides default REST submission) */
  onSubmit?: (values: Record<string, unknown>) => Promise<void>;

  /** Called when submission succeeds */
  onSuccess?: (result: SubmissionResponse) => void;

  /** Called when submission fails */
  onError?: (error: ApiError) => void;
}

/** Multi-step navigation state */
export interface StepState {
  current: StepJson | null;
  index: number;
  total: number;
  isFirst: boolean;
  isLast: boolean;
  canGoNext: boolean;
  canGoPrev: boolean;
  progress: [number, number];
}

/** Props returned by getFormProps() */
export interface FormProps {
  onSubmit: (e: React.FormEvent) => void;
  className?: string;
}

/** Props returned by getFieldProps() */
export interface FieldProps {
  name: string;
  id: string;
  value: unknown;
  onChange: (
    e: React.ChangeEvent<
      HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
    >
  ) => void;
  onBlur: (e: React.FocusEvent) => void;
  'aria-invalid'?: boolean;
  'aria-describedby'?: string;
  className?: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  type?: string;
  min?: number | string;
  max?: number | string;
  step?: number | string;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  autoComplete?: string;
  autoFocus?: boolean;
  rows?: number;
  cols?: number;
}

/** Props returned by getStepProps() */
export interface StepProps {
  onNext: () => void;
  onPrev: () => void;
  canGoNext: boolean;
  canGoPrev: boolean;
  isLastStep: boolean;
}

/** Props returned by getSelectProps() */
export interface SelectProps {
  name: string;
  id: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
  onBlur: (e: React.FocusEvent) => void;
  'aria-invalid'?: boolean;
  'aria-describedby'?: string;
  className?: string;
  required?: boolean;
  disabled?: boolean;
  multiple?: boolean;
  options: FieldOptionJson[];
}

/** Props returned by getCheckboxProps() */
export interface CheckboxProps {
  name: string;
  id: string;
  checked: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onBlur: (e: React.FocusEvent) => void;
  'aria-invalid'?: boolean;
  'aria-describedby'?: string;
  className?: string;
  required?: boolean;
  disabled?: boolean;
  type: 'checkbox';
}

/** Props for a single radio option */
export interface RadioOptionProps {
  name: string;
  id: string;
  value: string;
  checked: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onBlur: (e: React.FocusEvent) => void;
  className?: string;
  disabled?: boolean;
  type: 'radio';
}

/** Props returned by getRadioGroupProps() */
export interface RadioGroupProps {
  name: string;
  value: string;
  options: FieldOptionJson[];
  'aria-invalid'?: boolean;
  'aria-describedby'?: string;
  required?: boolean;
  disabled?: boolean;
  getOptionProps: (option: FieldOptionJson) => RadioOptionProps;
}

/** Props returned by getTextareaProps() */
export interface TextareaProps {
  name: string;
  id: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onBlur: (e: React.FocusEvent) => void;
  'aria-invalid'?: boolean;
  'aria-describedby'?: string;
  className?: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  rows?: number;
  cols?: number;
  minLength?: number;
  maxLength?: number;
}

/** Props for rendering a field label */
export interface LabelProps {
  htmlFor: string;
  className?: string;
  children: string;
  required?: boolean;
}

/** Helper to get field metadata */
export interface FieldMeta {
  field: FieldJson | null;
  value: unknown;
  errors: string[];
  touched: boolean;
  hasError: boolean;
  isVisible: boolean;
}

/** Return type for useAnyForm hook */
export interface UseAnyFormReturn {
  // State
  schema: FormJson | null;
  values: Record<string, unknown>;
  errors: Record<string, string[]>;
  touched: Record<string, boolean>;
  isValid: boolean;
  isLoading: boolean;
  isSubmitting: boolean;
  isSubmitted: boolean;
  error: string | null;

  // Multi-step navigation (null if single-step)
  step: StepState | null;

  // Actions
  setValue: (field: string, value: unknown) => void;
  setValues: (values: Record<string, unknown>) => void;
  setTouched: (field: string) => void;
  validateField: (field: string) => string[];
  validateAll: () => Record<string, string[]>;
  nextStep: () => boolean;
  prevStep: () => boolean;
  goToStep: (stepId: string) => boolean;
  submit: () => Promise<void>;
  reset: () => void;

  // Props helpers
  getFormProps: () => FormProps;
  getFieldProps: (fieldName: string) => FieldProps;
  getSelectProps: (fieldName: string) => SelectProps;
  getCheckboxProps: (fieldName: string) => CheckboxProps;
  getRadioGroupProps: (fieldName: string) => RadioGroupProps;
  getTextareaProps: (fieldName: string) => TextareaProps;
  getLabelProps: (fieldName: string) => LabelProps;
  getFieldMeta: (fieldName: string) => FieldMeta;
  getStepProps: () => StepProps;

  // Visibility helpers
  isFieldVisible: (fieldName: string) => boolean;
  isStepVisible: (stepId: string) => boolean;
  visibleFields: FieldJson[];
  visibleSteps: StepJson[];

  // Internal state (WASM or JS FormState)
  formState: unknown;
}

/** Props for AnyForm component */
export interface AnyFormProps {
  slug: string;
  options?: UseAnyFormOptions;
  children: (form: UseAnyFormReturn) => React.ReactNode;
}

/** Props for AnyFormProvider context */
export interface AnyFormProviderProps {
  baseUrl?: string;
  tailwind?: boolean;
  classNames?: ClassNames;
  children: React.ReactNode;
}

/** Context value for AnyFormProvider */
export interface AnyFormContextValue {
  baseUrl: string;
  tailwind: boolean;
  classNames: ClassNames;
}
