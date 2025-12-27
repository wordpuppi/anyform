/**
 * Type declarations for Next.js-specific features
 */

// Extend React with cache() from React 19/canary
declare module 'react' {
  export function cache<T extends (...args: any[]) => Promise<any>>(fn: T): T;
}

// Extend fetch RequestInit with Next.js options
declare global {
  interface RequestInit {
    next?: {
      revalidate?: number | false;
      tags?: string[];
    };
  }
}

export {};
