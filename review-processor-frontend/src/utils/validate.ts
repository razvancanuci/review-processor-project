/**
 * Token validation utilities
 * This module provides functionality to validate JWT tokens against the backend API.
 */

import { api } from 'boot/axios'
import { Notify } from 'quasar'

/**
 * Validates the stored authentication token
 * 
 * This function:
 * 1. Checks if a token exists in localStorage
 * 2. Validates the token against the backend API
 * 3. Handles error cases and shows notifications
 * 4. Removes invalid tokens from storage
 * 
 * @returns {Promise<boolean>} True if the token is valid, false otherwise
 * 
 * @example
 * ```typescript
 * const isValid = await validateToken();
 * if (!isValid) {
 *   // Redirect to login page
 *   router.push('/login');
 * }
 * ```
 */
export async function validateToken(): Promise<boolean> {
  // Check if token exists in localStorage
  const tkn = localStorage.getItem('auth_tkn');
  if (!tkn) {
    return false;
  }

  // Validate token with backend API
  const result = await api.get('/api/idm/sso', {
    headers: {
      'Authorization': `Bearer ${tkn}`
    }
  }).catch((err) => {
    // Show error notification
    Notify.create({
      message: err.message,
      color: 'negative',
      position: 'top'
    });
    return { status: 401 };
  });

  // Handle invalid token
  if (result.status === 401) {
    localStorage.removeItem('auth_tkn');
    return false;
  }

  return true;
}
