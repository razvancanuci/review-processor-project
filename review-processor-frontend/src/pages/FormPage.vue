<!--
  Review Form Page Component
  This component provides a form for users to submit reviews with:
  - A rating from 1 to 10
  - An optional suggestion using a rich text editor
  - Form validation and error handling
  - Success feedback
-->

<script setup lang="ts">
import Editor from '@tinymce/tinymce-vue'
import { apiKey } from 'boot/tinyMCE'
import { computed, ref } from 'vue'
import { api } from 'boot/axios'
import { Notify, useQuasar } from 'quasar'
import { useRouter } from 'vue-router'

// Form data model
const model = ref({
  rating: 0,        // Rating value (1-10)
  suggestion: ''    // Rich text suggestion content
});

// Quasar and router instances
const $q = useQuasar();
const router = useRouter();

// Form submission state
const formSubmitted = ref(false);

// Computed property for responsive rating size
const ratingSizeComputed = computed(() => {
  return $q.screen.width < 900 ? '4em' : '6em';
});

/**
 * Submits the review form to the backend API
 * 
 * This function:
 * 1. Shows a loading indicator
 * 2. Sends the review data to the API
 * 3. Handles success and error cases
 * 4. Shows appropriate notifications
 * 5. Redirects to login on authentication errors
 */
const sendReview = async () => {
  $q.loading.show();
  const result = await api.post('/api/reviews', model.value, {
    headers: {
      'Authorization': `Bearer ${localStorage.getItem('auth_tkn')}`,
    }
  })
  .catch(async (error) => {
    if (error.response) {
      // Show API error message
      Notify.create({
        message: error.response.data.message,
        color: 'negative',
        position: 'top'
      });
    } else {
      // Show network/authentication error
      Notify.create({
        message: error.message,
        color: 'negative',
        position: 'top'
      });
      $q.loading.hide();
      localStorage.removeItem('auth_tkn');
      await router.push('/login')
    }
    $q.loading.hide();
    throw error;
  });

  if (result.status === 201) {
    // Show success message and update form state
    Notify.create({
      message: 'Review submitted successfully',
      color: 'positive',
      position: 'top'
    });
    $q.loading.hide();
    formSubmitted.value = true;
  }
};
</script>

<template>
  <h2 class="text-center">Review form</h2>
  <!-- Review form with validation -->
  <q-form @submit="sendReview" v-if="!formSubmitted">
    <!-- Rating section -->
    <div class="flex flex-center column">
      <h5> Add rating from 1 to 10:</h5>
      <q-rating
        v-model="model.rating"
        :size="ratingSizeComputed"
        :max="10"
        color="yellow-10"
        :rules="[
          (val: number) => val > 0 || 'Please rate the app',
          (val: number) => val <= 10 || 'Rating must be between 1 and 10'
        ]"
      />
    </div>

    <!-- Suggestion section -->
    <div class="flex flex-center column">
      <h5>Feel free to add your suggestion below: (optional)</h5>
    </div>

    <!-- Rich text editor -->
    <Editor
      :api-key="apiKey"
      v-model="model.suggestion"
      :init="{
        plugins: 'wordcount link lists',
        menubar: false,
        toolbar: 'undo redo | bullist numlist | link',
      }"
    />

    <!-- Submit button -->
    <div class="flex flex-center">
      <q-btn
        color="green-10"
        class="q-mt-lg"
        style="width: 20%;"
        type="submit"
      >
        Submit
      </q-btn>
    </div>
  </q-form>

  <!-- Success feedback -->
  <div v-else class="flex flex-center column">
    <q-icon 
      name="check_circle" 
      color="positive" 
      size="600px" 
      class="animate-bounce self-center"
    ></q-icon>
  </div>
</template>

<style scoped>
</style>
