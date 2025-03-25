<script setup lang="ts">
import { GoogleLogin } from 'vue3-google-login'
import { useRouter } from 'vue-router'

const router = useRouter();
/* eslint-disable @typescript-eslint/no-explicit-any */
const login = async (response: any) => {

    localStorage.setItem('auth_tkn', response.credential);
    await router.push('/');
}

</script>

<template>
  <h2 class="text-center">Login with your corporate account</h2>
  <div class="flex flex-row justify-center">
  <GoogleLogin :callback = "login"/>
  </div>
</template>

<style scoped>

h2 {
  margin-top: 20%;
}

img {
  cursor: pointer;
}

</style>
