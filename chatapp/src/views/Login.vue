<template>
  <div class="login max-w-lg mx-auto my-32 p-12 bg-white rounded-lg shadow-lg text-center">
    <h1 class="mb-5 text-4xl font-bold text-gray-800">Welcome Back</h1>
    <p class="subtitle text-lg text-gray-600 mb-10">Please login to your account</p>
    <form @submit.prevent="login">
      <div class="form-group mb-8 text-left">
        <label for="email" class="block mb-3 font-semibold text-lg text-gray-700">Email</label>
        <input
          type="email"
          id="email"
          v-model="email"
          placeholder="Enter your email"
          required
          class="w-full p-4 border border-gray-300 rounded-lg text-lg transition duration-300 ease-in-out focus:border-blue-500 focus:shadow-lg focus:outline-none"
        />
      </div>

      <div class="form-group mb-8 text-left">
        <label for="password" class="block mb-3 font-semibold text-lg text-gray-700"
          >Password</label
        >
        <input
          type="password"
          id="password"
          v-model="password"
          placeholder="Enter your password"
          required
          class="w-full p-4 border border-gray-300 rounded-lg text-lg transition duration-300 ease-in-out focus:border-blue-500 focus:shadow-lg focus:outline-none"
        />
      </div>

      <button
        type="submit"
        class="login-button w-full p-4 bg-blue-500 text-white border-none rounded-lg text-lg font-semibold transition duration-300 ease-in-out hover:bg-blue-600 hover:translate-y-[-3px] hover:shadow-lg active:translate-y-0 active:shadow-none mt-8"
      >
        Login
      </button>
    </form>

    <p class="register-link mt-8 text-lg text-gray-600">
      Don't have an account?
      <router-link
        to="/register"
        class="text-blue-500 font-semibold transition duration-300 ease-in-out hover:text-blue-600"
        >Register here</router-link
      >.
    </p>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import useMainStore from '@/stores'

// init ...
const main_store = useMainStore()
const router = useRouter()

const email = ref('')
const password = ref('')

async function login() {
  const loginData: Interface.ISignin = {
    email: email.value,
    password: password.value,
  }

  await main_store.signin(loginData)

  router.push('/chat')
}
</script>

<style scoped></style>
