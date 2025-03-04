<template>
  <div class="flex-1 overflow-y-auto p-5" id="message-list">
    <div v-if="getMessageForActiveChannel.length === 0" class="text-center text-gray-400 mt-5">
      No messages in this channel yet.
    </div>
    <template v-else>
      <div
        v-for="message in getMessageForActiveChannel"
        :key="message.id"
        class="flex items-start mb-5"
      >
        <img
          :src="`https://ui-avatars.com/api/?name=${message.sender.fullname.replace(' ', '+')}`"
          class="w-10 h-10 rounded-full mr-3"
          alt="Avatar"
        />
        <div class="max-w-4/5">
          <div class="flex items-center mb-1">
            <span class="font-bold mr-2">{{ message.sender.fullname }}</span>
            <span class="text-xs text-gray-500">{{ formatTime(message.created_at) }}</span>
          </div>
          <div class="text-sm leading-relaxed break-words whitespace-pre-wrap">
            {{ message.content }}
          </div>
          <div v-if="message.files && message.files.length > 0" class="grid grid-cols-3 gap-2 mt-2">
            <div v-for="(file, index) in message.files" :key="index" class="relative">
              <img
                :src="getFileUrl(file)"
                :class="{
                  'h-32 object-cover cursor-pointer': true,
                  'w-auto h-auto': enlargedImage[message.id],
                }"
                @click="toggleImage(message.id)"
                alt="Uploaded file"
              />
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import useMainStore from '@/stores'
import { storeToRefs } from 'pinia'
import { BASE_URL } from '@/global'

// init ...
const main_store = useMainStore()

const { getMessageForActiveChannel, active_channel, token } = storeToRefs(main_store)

watch(
  () => active_channel.value,
  async () => {
    if (active_channel.value) {
      await main_store.fetchMessagesForChannel(active_channel.value?.id)
    }
  },
)

const formatTime = (time: string) => {
  const date = new Date(time)
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

let observer: MutationObserver | null = null

onMounted(() => {
  const message_list = document.getElementById('message-list')
  if (!message_list) return
  const config: MutationObserverInit = {
    childList: true,
  }

  const callback = function (mutationsList: MutationRecord[], observer: MutationObserver) {
    message_list.scrollTop = message_list.scrollHeight
  }

  observer = new MutationObserver(callback)
  observer.observe(message_list, config)
})

onUnmounted(() => {
  observer?.disconnect()
})

const enlargedImage = ref<{
  [key: number]: boolean
}>({})

function getFileUrl(path: string) {
  return `${BASE_URL}${path}?access_token=${token.value}`
}

function toggleImage(message_id: number) {
  enlargedImage.value[message_id] = !enlargedImage.value[message_id]
}
</script>

<style scoped></style>
