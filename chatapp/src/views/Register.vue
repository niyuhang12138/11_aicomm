<template>
  <div class="register max-w-lg mx-auto my-32 p-12 bg-white rounded-lg shadow-lg text-center">
    <h1 class="mb-5 text-4xl font-bold text-gray-800">Create Your Account</h1>
    <p class="subtitle text-lg text-gray-600 mb-10">Join us and start collaborating</p>
    <form @submit.prevent="register">
      <div class="form-group mb-8 text-left">
        <label for="fullName" class="block mb-3 font-semibold text-lg text-gray-700"
          >Full Name</label
        >
        <input
          type="text"
          id="fullName"
          v-model="fullName"
          placeholder="Enter your full name"
          required
          class="w-full p-4 border border-gray-300 rounded-lg text-lg transition duration-300 ease-in-out focus:border-blue-500 focus:shadow-lg focus:outline-none"
        />
      </div>

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
        <label for="workspaceName" class="block mb-3 font-semibold text-lg text-gray-700"
          >Workspace Name</label
        >
        <input
          type="text"
          id="workspaceName"
          v-model="workspaceName"
          placeholder="Enter your workspace name"
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
        class="register-button w-full p-4 bg-blue-500 text-white border-none rounded-lg text-lg font-semibold transition duration-300 ease-in-out hover:bg-blue-600 hover:translate-y-[-3px] hover:shadow-lg active:translate-y-0 active:shadow-none mt-8"
      >
        Register
      </button>
    </form>

    <p class="login-link mt-8 text-lg text-gray-600">
      Already have an account?
      <router-link
        to="/login"
        class="text-blue-500 font-semibold transition duration-300 ease-in-out hover:text-blue-600"
        >Login here</router-link
      >.
    </p>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import useMainStore from '@/stores'
import { useRouter } from 'vue-router'

// init ...
const main_store = useMainStore()
const router = useRouter()

const fullName = ref('nyh')
const email = ref('nyh@chatapp.com')
const workspaceName = ref('chatapp')
const password = ref('123456')

async function register() {
  try {
    const userData: Interface.ISignup = {
      fullname: fullName.value,
      email: email.value,
      workspace: workspaceName.value,
      password: password.value,
    }

    await main_store.signup(userData)

    router.push('/chat')
  } catch (err) {
    console.log('page: Register: ---> ', err)
    throw err
  }
}
</script>

<style scoped></style>
