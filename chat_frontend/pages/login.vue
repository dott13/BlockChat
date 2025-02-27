<template>
  <div class="flex items-center justify-center">
    <div class="w-full max-w-md p-8 space-y-6 bg-white rounded shadow">
      <h1 class="text-2xl font-bold text-center text-gray-700">Login</h1>
      <form @submit.prevent="handleLogin" class="space-y-4">
        <div>
          <UiInput label="Username" v-model="credentials.username" placeholder="Enter your username" />
        </div>
        <div>
          <UiInput 
            label="Password"
            type="password"
            v-model="credentials.password"
            placeholder="Enter your password"
          />
        </div>
        <button
          type="submit"
          class="w-full px-4 py-2 font-semibold text-white bg-blue-500 rounded hover:bg-blue-600 focus:outline-none"
        >
          Login
        </button>
      </form>
      <div v-if="pending" class="text-center text-gray-600">Logging in...</div>
      <div v-if="error" class="text-center text-red-500">
        Error: {{ error }}
      </div>
      <div v-if="data" class="text-center text-green-500">
        Login successful! Token: {{ data.token }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { UiInput } from "#components";
import { ref } from "vue";
import { useAuth } from "~/composables/useAuth"; // Adjust the path if needed

const { login } = useAuth();
const credentials = ref({ username: "", password: "" });
const data = ref(null);
const error = ref(null);
const pending = ref(false);

const handleLogin = async () => {
  pending.value = true;
  error.value = null;
  data.value = null;

  const res = await login(credentials.value);
  pending.value = false;

  if (res.error) {
    error.value = res.error.message;
  } else {
    data.value = res.data;
    // Store the token in localStorage
    if (res.data && res.data.token) {
      localStorage.setItem("token", res.data.token);
    }
  }
};
</script>