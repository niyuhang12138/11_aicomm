<template>
  <div class="w-64 bg-gray-800 text-white flex flex-col h-screen p-4 text-sm">
    <div class="flex items-center justify-between mb-6">
      <div class="font-bold text-base truncate cursor-pointer" @click="toggleDropdown">
        <span>{{ workspace.name }}</span>
        <button class="dropdown-toggle">&nbsp;▼</button>
      </div>
      <div
        v-if="dropdownVisible"
        class="absolute top-12 left-0 w-48 bg-gray-800 border border-gray-700 rounded-md shadow-lg z-10"
      >
        <ul class="mb-6">
          <li @click="logout" class="px-4 py-2 hover:bg-gray-700 cursor-pointer">Logout</li>
          <!-- Add more dropdown items here as needed -->
        </ul>
      </div>
      <button class="text-gray-400 text-xl hover:text-white">+</button>
    </div>

    <div class="mb-6">
      <h2 class="text-xs uppercase text-gray-400 mb-2">Channels</h2>
      <ul>
        <li
          v-for="channel in get_channels"
          :key="channel.id"
          @click="selectChannel(channel.id)"
          :class="[
            'px-2 py-1 rounded cursor-pointer',
            { 'bg-blue-600': channel.id === active_channel?.id },
          ]"
        >
          # {{ channel.name }}
        </li>
      </ul>
    </div>

    <div>
      <h2 class="text-xs uppercase text-gray-400 mb-2">Direct Messages</h2>
      <ul>
        <li
          v-for="user in filter_users"
          :key="user.id"
          @click="selectChannel(user.channel_id)"
          :class="[
            'flex items-center px-2 py-1 rounded cursor-pointer',
            { 'bg-blue-600': user.channel_id === active_channel?.id },
          ]"
        >
          <img
            :src="`https://ui-avatars.com/api/?name=${user.fullname.replace(' ', '+')}`"
            class="w-6 h-6 rounded-full mr-2"
            alt="Avatar"
          />
          {{ user.fullname }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts" setup>
import router from '@/router'
import useMainStore from '@/stores'
import { storeToRefs } from 'pinia'
import { computed, ref } from 'vue'

// init ...
const main_store = useMainStore()

const { user, workspace, get_channels, get_single_channels, users, active_channel } =
  storeToRefs(main_store)

const filter_users = computed<Array<IF.IUserSingleChannel>>(() => {
  const filter_users: Array<IF.IUserSingleChannel> = []
  get_single_channels.value.forEach((channel) => {
    const id = channel.members.filter((member) => member !== user.value.id)[0]
    const new_user = users.value[id]
    filter_users.push({
      channel_id: channel.id,
      ...new_user,
    })
  })
  return filter_users
})

const dropdownVisible = ref(false)

function toggleDropdown() {
  dropdownVisible.value = !dropdownVisible.value
}

function logout() {
  main_store.reset()
  router.replace('/login')
}

async function selectChannel(channel_id: number) {
  main_store.setActiveChannel(channel_id)
}
</script>

<style scoped></style>
