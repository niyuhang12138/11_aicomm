<template>
  <div class="flex flex-col bg-gray-100 border-t border-gray-200 relative bottom-0">
    <div class="flex items-center">
      <button
        @click="triggerFileUpload"
        class="p-2 mr-2 text-gray-600 hover:text-blue-600 focus:outline-none"
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
      <input
        type="file"
        ref="fileInput"
        @change="uploadFile"
        multiple
        accept="image/*"
        class="hidden"
      />
      <!-- Add more image buttons here if needed -->
    </div>

    <div>
      <textarea
        v-model="message"
        @keyup.enter="sendMessage"
        placeholder="Type a message..."
        class="w-full px-4 text-sm bg-gray-100 border-none rounded-lg focus:outline-none resize-none"
        rows="3"
      ></textarea>
    </div>
    <div v-if="files.length > 0" class="flex flex-wrap p-2">
      <img
        v-for="file in files"
        :key="file.path"
        :src="file.fullpath"
        class="h-64 object-cover rounded mr-2 mb-2"
        alt="Uploaded image"
      />
    </div>

    <button
      @click="sendMessage"
      class="absolute bottom-4 right-4 p-1 text-white bg-blue-600 rounded-full hover:bg-blue-700 focus:outline-none"
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
const files = ref<Array<IF.IFileShow>>([])
const fileInput = ref<HTMLInputElement | null>(null)

async function sendMessage() {
  if (message.value.trim()) {
    await main_store.sendMessage({
      content: message.value,
      files: files.value.map((file) => file.path),
    })

    message.value = ''
    files.value = []
  }
}

function triggerFileUpload() {
  if (fileInput.value) {
    fileInput.value.click()
  }
}

async function uploadFile() {
  if (!fileInput.value) return
  const data = Array.from(fileInput.value.files as FileList)
  if (data.length === 0) return
  files.value = await main_store.uploadFile(data)
}
</script>

<style scoped></style>
