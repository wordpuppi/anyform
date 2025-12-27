/**
 * Tests for useAnyForm hook - Props helpers
 *
 * Tests getFieldProps, getSelectProps, getCheckboxProps, getRadioGroupProps,
 * getTextareaProps, getLabelProps, and getFieldMeta.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useAnyForm } from '../hooks/useAnyForm';
import { createFieldTypesSchema, createMockSchema } from './mocks/anyform-js';

describe('useAnyForm props helpers', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getFormProps', () => {
    it('should return form props with onSubmit handler', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const formProps = result.current.getFormProps();

      expect(formProps.onSubmit).toBeInstanceOf(Function);
    });

    it('should include className when tailwind is enabled', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema, tailwind: true })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const formProps = result.current.getFormProps();

      expect(formProps.className).toBeDefined();
      expect(formProps.className).toContain('space-y');
    });

    it('should not include className when tailwind is disabled', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema, tailwind: false })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const formProps = result.current.getFormProps();

      expect(formProps.className).toBeUndefined();
    });
  });

  describe('getFieldProps', () => {
    it('should return basic input props', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getFieldProps('text_field');

      expect(props.name).toBe('text_field');
      expect(props.id).toBe('field-text_field');
      expect(props.type).toBe('text');
      expect(props.onChange).toBeInstanceOf(Function);
      expect(props.onBlur).toBeInstanceOf(Function);
    });

    it('should return correct type for email field', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getFieldProps('email_field');

      expect(props.type).toBe('email');
      expect(props.required).toBe(true);
    });

    it('should include validation attributes', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getFieldProps('text_field');

      expect(props.minLength).toBe(2);
      expect(props.maxLength).toBe(100);
      expect(props.placeholder).toBe('Enter text');
    });

    it('should include aria-invalid when field has errors and is touched', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // Touch and validate field (required but empty)
      act(() => {
        result.current.setTouched('name');
        result.current.validateField('name');
      });

      const props = result.current.getFieldProps('name');

      expect(props['aria-invalid']).toBe(true);
      expect(props['aria-describedby']).toBe('name-error');
    });

    it('should update value via onChange', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getFieldProps('name');

      act(() => {
        props.onChange({
          target: { value: 'New Value', type: 'text' },
        } as React.ChangeEvent<HTMLInputElement>);
      });

      expect(result.current.values.name).toBe('New Value');
    });

    it('should mark as touched via onBlur', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getFieldProps('name');

      expect(result.current.touched.name).toBeUndefined();

      act(() => {
        props.onBlur({} as React.FocusEvent);
      });

      expect(result.current.touched.name).toBe(true);
    });
  });

  describe('getSelectProps', () => {
    it('should return select props with options', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getSelectProps('select_field');

      expect(props.name).toBe('select_field');
      expect(props.options).toHaveLength(3);
      expect(props.options[0].value).toBe('opt1');
      expect(props.options[0].label).toBe('Option 1');
      expect(props.multiple).toBe(false);
    });

    it('should set multiple=true for multi_select fields', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getSelectProps('multi_select_field');

      expect(props.multiple).toBe(true);
    });

    it('should update value on change', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getSelectProps('select_field');

      act(() => {
        props.onChange({
          target: { value: 'opt2', multiple: false },
        } as React.ChangeEvent<HTMLSelectElement>);
      });

      expect(result.current.values.select_field).toBe('opt2');
    });
  });

  describe('getCheckboxProps', () => {
    it('should return checkbox props with checked state', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getCheckboxProps('checkbox_field');

      expect(props.name).toBe('checkbox_field');
      expect(props.type).toBe('checkbox');
      expect(props.checked).toBe(false);
    });

    it('should update checked state on change', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getCheckboxProps('checkbox_field');

      act(() => {
        props.onChange({
          target: { checked: true },
        } as React.ChangeEvent<HTMLInputElement>);
      });

      expect(result.current.values.checkbox_field).toBe(true);
    });
  });

  describe('getRadioGroupProps', () => {
    it('should return radio group props with options', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getRadioGroupProps('radio_field');

      expect(props.name).toBe('radio_field');
      expect(props.options).toHaveLength(2);
      expect(props.getOptionProps).toBeInstanceOf(Function);
    });

    it('should return option props via getOptionProps', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const groupProps = result.current.getRadioGroupProps('radio_field');
      const optionProps = groupProps.getOptionProps(groupProps.options[0]);

      expect(optionProps.name).toBe('radio_field');
      expect(optionProps.type).toBe('radio');
      expect(optionProps.value).toBe('yes');
      expect(optionProps.checked).toBe(false);
    });

    it('should update value when option is selected', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const groupProps = result.current.getRadioGroupProps('radio_field');
      const optionProps = groupProps.getOptionProps(groupProps.options[0]);

      act(() => {
        optionProps.onChange({
          target: { value: 'yes' },
        } as React.ChangeEvent<HTMLInputElement>);
      });

      expect(result.current.values.radio_field).toBe('yes');
    });

    it('should show correct option as checked', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { radio_field: 'no' },
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const groupProps = result.current.getRadioGroupProps('radio_field');
      const yesProps = groupProps.getOptionProps(groupProps.options[0]);
      const noProps = groupProps.getOptionProps(groupProps.options[1]);

      expect(yesProps.checked).toBe(false);
      expect(noProps.checked).toBe(true);
    });
  });

  describe('getTextareaProps', () => {
    it('should return textarea props', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getTextareaProps('textarea_field');

      expect(props.name).toBe('textarea_field');
      expect(props.rows).toBe(4);
    });

    it('should update value on change', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getTextareaProps('textarea_field');

      act(() => {
        props.onChange({
          target: { value: 'Long text content' },
        } as React.ChangeEvent<HTMLTextAreaElement>);
      });

      expect(result.current.values.textarea_field).toBe('Long text content');
    });
  });

  describe('getLabelProps', () => {
    it('should return label props', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getLabelProps('email_field');

      expect(props.htmlFor).toBe('field-email_field');
      expect(props.children).toBe('Email Field');
      expect(props.required).toBe(true);
    });

    it('should include className when tailwind is enabled', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema, tailwind: true })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getLabelProps('email_field');

      expect(props.className).toBeDefined();
    });
  });

  describe('getFieldMeta', () => {
    it('should return field metadata', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Test' },
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const meta = result.current.getFieldMeta('name');

      expect(meta.field).not.toBeNull();
      expect(meta.field?.name).toBe('name');
      expect(meta.value).toBe('Test');
      expect(meta.touched).toBe(false);
      expect(meta.errors).toEqual([]);
      expect(meta.hasError).toBe(false);
      expect(meta.isVisible).toBe(true);
    });

    it('should show hasError when touched with errors', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // Touch and validate
      act(() => {
        result.current.setTouched('name');
        result.current.validateField('name');
      });

      const meta = result.current.getFieldMeta('name');

      expect(meta.touched).toBe(true);
      expect(meta.hasError).toBe(true);
      expect(meta.errors.length).toBeGreaterThan(0);
    });
  });

  describe('getStepProps', () => {
    it('should return step navigation props', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const props = result.current.getStepProps();

      expect(props.onNext).toBeInstanceOf(Function);
      expect(props.onPrev).toBeInstanceOf(Function);
      expect(typeof props.canGoNext).toBe('boolean');
      expect(typeof props.canGoPrev).toBe('boolean');
      expect(typeof props.isLastStep).toBe('boolean');
    });
  });

  describe('tailwind classes', () => {
    it('should include tailwind classes when enabled', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema, tailwind: true })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const inputProps = result.current.getFieldProps('text_field');
      const selectProps = result.current.getSelectProps('select_field');
      const checkboxProps = result.current.getCheckboxProps('checkbox_field');
      const textareaProps = result.current.getTextareaProps('textarea_field');

      expect(inputProps.className).toBeDefined();
      expect(selectProps.className).toBeDefined();
      expect(checkboxProps.className).toBeDefined();
      expect(textareaProps.className).toBeDefined();
    });

    it('should not include tailwind classes when disabled', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema, tailwind: false })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const inputProps = result.current.getFieldProps('text_field');
      const selectProps = result.current.getSelectProps('select_field');

      expect(inputProps.className).toBeUndefined();
      expect(selectProps.className).toBeUndefined();
    });

    it('should use custom classNames when provided', async () => {
      const schema = createFieldTypesSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          tailwind: true,
          classNames: {
            input: 'custom-input',
            select: 'custom-select',
          },
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      const inputProps = result.current.getFieldProps('text_field');
      const selectProps = result.current.getSelectProps('select_field');

      expect(inputProps.className).toBe('custom-input');
      expect(selectProps.className).toBe('custom-select');
    });
  });
});
