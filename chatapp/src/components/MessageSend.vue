<template>
  <div class="flex items-center p-4 bg-white border-t border-gray-200">
    <input
      v-model="message"
      @keyup.enter="sendMessage"
      placeholder="Type a message..."
      class="flex-1 px-4 py-3 mr-2 text-sm bg-gray-100 border-none rounded-lg focus:outline-none"
      type="text"
    />
    <input
      type="file"
      @change="uploadFile"
      ref="fileInput"
      multiple
      class="hidden"
      accept="image/*"
    />
    <button
      @click="triggerFileUpload"
      class="p-2 text-gray-600 hover:text-gray-800 focus:outline-none mr-2"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-5 h-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
        />
      </svg>
    </button>
    <button
      @click="sendMessage"
      class="p-2 text-white bg-blue-600 rounded-full hover:bg-blue-700 focus:outline-none"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-5 h-5"
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
const fileInput = ref<HTMLInputElement | null>(null)

async function sendMessage() {
  if (message.value.trim()) {
    await main_store.sendMessage({
      content: message.value,
    })

    message.value = ''
    if (fileInput.value) {
      fileInput.value.value = ''
    }
  }
}

function triggerFileUpload() {
  if (fileInput.value) {
    fileInput.value.click()
  }
}

async function uploadFile() {
  if (!fileInput.value) return
  const files = Array.from(fileInput.value.files as FileList)
  if (files.length === 0) return
  // 清除file put中的files

}
</script>

<style scoped></style>
