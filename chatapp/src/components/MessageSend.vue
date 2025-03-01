<template>
  <div class="flex items-center p-4 bg-white border-t border-gray-200">
    <input
      v-model="message"
      @keyup.enter="sendMessage"
      placeholder="Type a message..."
      class="flex-1 px-4 py-3 mr-2 text-sm bg-gray-100 border-none rounded-lg focus:outline-none"
      type="text"
    />
    <button
      @click="sendMessage"
      class="p-2 text-white bg-blue-600 rounded-full hover:bg-blue-700 focus:outline-none"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="icon"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M5 12h14M12 5l7 7-7 7"
        />
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import useMainStore from '@/stores'

// init ...
const main_store = useMainStore()

const message = ref('')
async function sendMessage() {
  if (message.value.trim()) {
    await main_store.sendMessage({
      content: message.value,
    })

    message.value = ''
  }
}
</script>

<style scoped></style>
