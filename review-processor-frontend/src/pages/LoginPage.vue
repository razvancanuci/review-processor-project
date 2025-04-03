<!--
  Login Page Component
  This component provides:
  - Google OAuth login functionality
  - Token storage
  - Automatic redirection after successful login
-->

<script setup lang="ts">
import { GoogleLogin } from 'vue3-google-login'
import { useRouter } from 'vue-router'

// Router instance for navigation
const router = useRouter();

/**
 * Handles the Google OAuth login response
 * 
 * This function:
 * 1. Stores the JWT token in localStorage
 * 2. Redirects the user to the home page
 * 
 * @param response - The Google OAuth response containing the JWT token
 */
/* eslint-disable @typescript-eslint/no-explicit-any */
const login = async (response: any) => {
    // Store the JWT token for future API requests
    localStorage.setItem('auth_tkn', response.credential);
    // Redirect to the home page
    await router.push('/');
}

</script>

<template>
  <!-- Login page header -->
  <h2 class="text-center">Login with your corporate account</h2>
  
  <!-- Google OAuth login button container -->
  <div class="flex flex-row justify-center">
    <GoogleLogin :callback="login"/>
  </div>
</template>

<style scoped>
/* Center the login header */
h2 {
  margin-top: 20%;
}

/* Add pointer cursor to clickable elements */
img {
  cursor: pointer;
}

</style>
