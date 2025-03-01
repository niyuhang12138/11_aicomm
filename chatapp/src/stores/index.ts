import { ref, computed, readonly, reactive } from 'vue'
import { defineStore } from 'pinia'
import request from '@/service'
import type { AxiosResponse } from 'axios'
import { jwtDecode, type JwtPayload } from 'jwt-decode'

export const useMainStore = defineStore('main_store', () => {
  /**
   * User information
   */

  const user = reactive<Interface.IUser>(
    localStorage.getItem('user')
      ? JSON.parse(localStorage.getItem('user')!)
      : {
          id: 0,
          fullname: '',
          email: '',
          create: '',
        },
  )

  function setUser(state: Interface.IUser) {
    Object.assign(user, state)
  }

  /**
   * Authentication token
   */
  const token = ref(localStorage.getItem('token') || '')

  function setToken(new_token: string) {
    token.value = new_token
  }

  /**
   * Current workspace
   */
  const workspace = reactive<Interface.IWorkspace>(
    localStorage.getItem('workspace')
      ? JSON.parse(localStorage.getItem('workspace')!)
      : { id: 0, name: '' },
  )

  function setWorkspace(new_workspace: Interface.IWorkspace) {
    Object.assign(workspace, new_workspace)
  }

  /**
   * List of channels
   */

  const channels = ref<Array<Interface.IChannel>>(
    localStorage.getItem('channels') ? JSON.parse(localStorage.getItem('channels')!) : [],
  )

  function setChannels(new_channels: Array<Interface.IChannel>) {
    channels.value = new_channels
  }

  const get_channels = computed(() => channels.value.filter((channel) => channel.type !== 'Single'))

  const get_single_channels = computed(() => {
    return channels.value.filter((channel) => channel.type === 'Single')
  })

  /**
   * Messages hashmap, keyed by channel ID
   */

  const messages = ref<{
    [key: number]: Array<Interface.IMessage>
  }>({})

  function setMessages(channel_id: number, new_messages: Array<Interface.IMessage>) {
    messages.value[channel_id] = new_messages
  }

  /**
   * users hashmap, keyed by user ID
   */
  const users = ref<{
    [key: number]: Interface.IUserInner
  }>(localStorage.getItem('users') ? JSON.parse(localStorage.getItem('users')!) : {})

  function setUsers(new_users: { [key: number]: Interface.IUserInner }) {
    users.value = new_users
  }

  /**
   * active channel
   */
  const active_channel = ref<null | Interface.IChannel>(null)

  function setActiveChannel(channel_id: number) {
    // active_channel.value = channel_id
    const channel = channels.value.find((channel) => channel.id === channel_id)
    if (channel) {
      active_channel.value = channel
    }
  }

  function addChannel(channel: Interface.IChannel) {
    channels.value.push(channel)
    messages.value[channel.id] = []
  }

  function addMessage(channel_id: number, message: Interface.IMessage) {
    console.log('addMessage: ---> ', message)
    if (messages.value[channel_id]) {
      messages.value[channel_id].push(message)
    } else {
      messages.value[channel_id] = [message]
    }
  }

  function getChannelMessages(channel_id: number): Array<Interface.IMessage> {
    return messages.value[channel_id] || []
  }

  const getMessageForActiveChannel = computed(() => {
    const find_messages = active_channel.value ? messages.value[active_channel.value.id] || [] : []

    return find_messages
  })

  // function getMessageForActiveChannel(): Array<Interface.IMessage> {
  //   return
  // }

  async function signup(data: Interface.ISignup) {
    try {
      const response = await request.post('/signup', data)
      await loadState(response)
    } catch (err) {
      console.error('signup error: ---> ', err)
      throw err
    }
  }

  async function signin(data: Interface.ISignin) {
    try {
      const response = await request.post('/signin', data)
      await loadState(response)
    } catch (err) {
      console.error('signin error: ---> ', err)
      throw err
    }
  }

  async function fetchMessagesForChannel(channel_id: number) {
    if (!messages.value[channel_id] || messages.value[channel_id].length === 0) {
      try {
        const response = await request.get(`/chats/${channel_id}/message`)
        let messages = response.data as Array<Interface.IMessage>
        messages.forEach((message) => {
          console.log(message)
          message.sender = users.value[message.sender_id] || {
            id: 0,
            fullname: 'None',
            email: 'None@chatapp.com',
          }
        })
        setMessages(channel_id, messages)
      } catch (err) {
        console.error('fetchMessagesForChannel error: ---> ', err)
        throw err
      }
    }
  }

  async function sendMessage(message: Interface.ISendMessage) {
    if (!active_channel.value) return
    const channel_id = active_channel.value!.id
    try {
      await request.post(`/chats/${channel_id}/message`, message)
      // const response = await request.post(`/chats/${channel_id}/message`, message)
      // const new_message = response.data as Interface.IMessage
      // new_message.sender = user
      // addMessage(channel_id, new_message)
    } catch (err) {
      console.error('sendMessage error: ---> ', err)
      throw err
    }
  }

  function reset() {
    localStorage.clear()
    Object.assign(user, {
      id: 0,
      fullname: '',
      email: '',
      create: '',
    })
    token.value = ''
    Object.assign(workspace, { id: 0, name: '' })
    channels.value = []
    messages.value = {}
    users.value = {}
    active_channel.value = null
  }

  return {
    // user ...
    user,
    setUser,
    users,
    setUsers,
    // token ...
    token,
    setToken,
    // workspace ...
    workspace,
    setWorkspace,
    // channels ...
    channels,
    setChannels,
    get_channels,
    get_single_channels,
    // messages ...
    messages,
    setMessages,
    getChannelMessages,
    // active channel
    active_channel,
    setActiveChannel,
    getMessageForActiveChannel,
    // other ...
    addChannel,
    addMessage,
    signin,
    signup,
    reset,
    fetchMessagesForChannel,
    sendMessage,
  }
})

export default useMainStore

async function loadState(response: AxiosResponse) {
  try {
    const mainStore = useMainStore()
    const token = response.data.token
    const user = jwtDecode(token) as {
      ws_id: number
      ws_name: string
      id: number
      fullname: string
      created: string
      email: string
    } // Decode the JWT to get user info

    const users_response = await request.get('/users', {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })

    const users_data = users_response.data as Array<Interface.IUserInner>
    const users: {
      [key: number]: Interface.IUserInner
    } = {}
    users_data.forEach((user) => {
      users[user.id] = user
    })

    const channels_response = await request.get('/chats', {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })

    const channels = channels_response.data as Array<Interface.IChannel>

    const workspace = { id: user.ws_id, name: user.ws_name }

    const new_user = {
      id: user.id,
      fullname: user.fullname,
      email: user.email,
      create: user.created,
    }

    mainStore.setWorkspace(workspace)
    mainStore.setUser(new_user)
    mainStore.setToken(token)
    mainStore.setUsers(users)
    mainStore.setChannels(channels)

    localStorage.setItem('user', JSON.stringify(new_user))
    localStorage.setItem('workspace', JSON.stringify(workspace))
    localStorage.setItem('token', token)
    localStorage.setItem('users', JSON.stringify(users))
    localStorage.setItem('channels', JSON.stringify(channels))
  } catch (err) {
    console.error('loadState error: ---> ', err)
    throw err
  }
}
