<template>
    <div class="">
        <h1>Login</h1>
        <form @submit.prevent="handleLogin">
            <input v-model="credentials.username" placeholder="Username"/>
            <input v-model="credentials.password" type="password" placeholder="Password"/>
            <button type="submit">Login</button>
        </form>
        <div v-if="pending">Logging in...</div>
        <div v-if="error">Error: {{ error }}</div>
        <div v-if="data">Login successful! Token: {{ data.token }}</div>
    </div>
</template>

<script setup>
    import { ref } from 'vue';

    const { login } = useAuth();
    const credentials = ref({ username: '', password: ''});
    const data = ref(null);
    const error = ref(null);
    const pending = ref(null);

    const handleLogin = async () => {
        pending.value = true;
        const res = await login(credentials.value);
        pending.value = false;
            if (res.error) {
                error.value = res.error.message;
            } else {
                data.value = res.data;
                // Here you could store the token in localStorage or a global store (e.g., Pinia)
            }
};
</script>