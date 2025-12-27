/**
 * Tests for useAnyForm hook - Core functionality
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useAnyForm } from '../hooks/useAnyForm';
import { createMockSchema, createMultiStepSchema } from './mocks/anyform-js';

// Helper to create a successful fetch response
function mockFetchSuccess(data: unknown) {
  return vi.fn().mockResolvedValue({
    ok: true,
    json: () => Promise.resolve({ success: true, data }),
  });
}

// Helper to create a failed fetch response
function mockFetchError(message: string) {
  return vi.fn().mockResolvedValue({
    ok: false,
    status: 500,
    json: () => Promise.resolve({ error: { message } }),
  });
}

describe('useAnyForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('initialization', () => {
    it('should start in loading state when no initialSchema', async () => {
      const schema = createMockSchema();
      global.fetch = mockFetchSuccess(schema);

      const { result } = renderHook(() => useAnyForm('test-form'));

      expect(result.current.isLoading).toBe(true);
      expect(result.current.schema).toBe(null);

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
    });

    it('should fetch form schema on mount', async () => {
      const schema = createMockSchema();
      global.fetch = mockFetchSuccess(schema);

      const { result } = renderHook(() =>
        useAnyForm('test-form', { baseUrl: 'http://localhost:3000' })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(global.fetch).toHaveBeenCalledWith(
        'http://localhost:3000/api/forms/test-form/json'
      );
      expect(result.current.schema).toEqual(schema);
    });

    it('should not fetch when initialSchema is provided', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      expect(result.current.isLoading).toBe(false);
      expect(result.current.schema).toEqual(schema);
      expect(global.fetch).not.toHaveBeenCalled();
    });

    it('should apply initial values', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'John', email: 'john@example.com' },
        })
      );

      await waitFor(() => {
        expect(result.current.values.name).toBe('John');
        expect(result.current.values.email).toBe('john@example.com');
      });
    });

    it('should handle fetch errors', async () => {
      global.fetch = mockFetchError('Form not found');

      const { result } = renderHook(() => useAnyForm('missing-form'));

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.error).toBe('Form not found');
      expect(result.current.schema).toBe(null);
    });
  });

  describe('value management', () => {
    it('should update values with setValue', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      act(() => {
        result.current.setValue('name', 'Alice');
      });

      expect(result.current.values.name).toBe('Alice');
    });

    it('should update multiple values with setValues', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      act(() => {
        result.current.setValues({
          name: 'Bob',
          email: 'bob@example.com',
        });
      });

      expect(result.current.values.name).toBe('Bob');
      expect(result.current.values.email).toBe('bob@example.com');
    });

    it('should track touched fields', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.touched.name).toBeUndefined();

      act(() => {
        result.current.setTouched('name');
      });

      expect(result.current.touched.name).toBe(true);
    });
  });

  describe('validation', () => {
    it('should validate a single field', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // Empty required field should have errors
      act(() => {
        const errors = result.current.validateField('name');
        expect(errors).toContain('Name is required');
      });
    });

    it('should validate all fields', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      act(() => {
        const allErrors = result.current.validateAll();
        expect(allErrors.name).toBeDefined();
        expect(allErrors.email).toBeDefined();
      });
    });

    it('should report isValid correctly', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Test', email: 'test@example.com' },
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // With valid values, should be valid
      expect(result.current.isValid).toBe(true);
    });
  });

  describe('multi-step navigation', () => {
    it('should provide step state for multi-step forms', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.step).not.toBeNull();
      expect(result.current.step?.index).toBe(0);
      expect(result.current.step?.total).toBe(3);
      expect(result.current.step?.isFirst).toBe(true);
      expect(result.current.step?.isLast).toBe(false);
    });

    it('should navigate to next step', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      act(() => {
        result.current.nextStep();
      });

      expect(result.current.step?.index).toBe(1);
      expect(result.current.step?.isFirst).toBe(false);
    });

    it('should navigate to previous step', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // Go to step 2
      act(() => {
        result.current.nextStep();
      });

      // Go back to step 1
      act(() => {
        result.current.prevStep();
      });

      expect(result.current.step?.index).toBe(0);
    });

    it('should go to specific step by ID', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      act(() => {
        result.current.goToStep('step-3');
      });

      expect(result.current.step?.index).toBe(2);
      expect(result.current.step?.isLast).toBe(true);
    });

    it('should show progress correctly', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.step?.progress).toEqual([1, 3]);

      act(() => {
        result.current.nextStep();
      });

      expect(result.current.step?.progress).toEqual([2, 3]);
    });
  });

  describe('form submission', () => {
    it('should submit form and call onSuccess', async () => {
      const schema = createMockSchema();
      const onSuccess = vi.fn();

      // Only mock the submission call since we're using initialSchema
      global.fetch = vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve({ id: 'submission-123' }),
      });

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Test', email: 'test@example.com' },
          onSuccess,
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      await act(async () => {
        await result.current.submit();
      });

      expect(result.current.isSubmitted).toBe(true);
      expect(onSuccess).toHaveBeenCalledWith({ id: 'submission-123' });
    });

    it('should call onError on submission failure', async () => {
      const schema = createMockSchema();
      const onError = vi.fn();

      // Only mock the submission call since we're using initialSchema
      global.fetch = vi.fn().mockResolvedValue({
        ok: false,
        json: () =>
          Promise.resolve({ error: { code: 'FAIL', message: 'Server error' } }),
      });

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Test', email: 'test@example.com' },
          onError,
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      await act(async () => {
        await result.current.submit();
      });

      expect(onError).toHaveBeenCalledWith({
        code: 'FAIL',
        message: 'Server error',
      });
    });

    it('should use custom onSubmit handler', async () => {
      const schema = createMockSchema();
      const customSubmit = vi.fn().mockResolvedValue(undefined);

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Test', email: 'test@example.com' },
          onSubmit: customSubmit,
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      await act(async () => {
        await result.current.submit();
      });

      expect(customSubmit).toHaveBeenCalledWith({
        name: 'Test',
        email: 'test@example.com',
      });
    });
  });

  describe('reset', () => {
    it('should reset form to initial state', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', {
          initialSchema: schema,
          initialValues: { name: 'Initial' },
        })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      // Modify state
      act(() => {
        result.current.setValue('name', 'Modified');
        result.current.setTouched('name');
      });

      expect(result.current.values.name).toBe('Modified');
      expect(result.current.touched.name).toBe(true);

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.values.name).toBe('Initial');
      expect(result.current.touched.name).toBeUndefined();
      expect(result.current.isSubmitted).toBe(false);
    });
  });

  describe('visibility helpers', () => {
    it('should return visible fields', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.visibleFields).toHaveLength(2);
      expect(result.current.visibleFields[0].name).toBe('name');
      expect(result.current.visibleFields[1].name).toBe('email');
    });

    it('should return visible steps', async () => {
      const schema = createMultiStepSchema();

      const { result } = renderHook(() =>
        useAnyForm('multi-step', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.visibleSteps).toHaveLength(3);
    });

    it('should check field visibility', async () => {
      const schema = createMockSchema();

      const { result } = renderHook(() =>
        useAnyForm('test-form', { initialSchema: schema })
      );

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.isFieldVisible('name')).toBe(true);
      expect(result.current.isFieldVisible('nonexistent')).toBe(false);
    });
  });
});
