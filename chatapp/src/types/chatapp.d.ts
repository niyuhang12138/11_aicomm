declare namespace Interface {
  interface ISignup {
    email: string
    fullname: string
    password: string
    workspace: string
  }

  interface ISignin {
    email: string
    password: string
  }

  interface IUser {
    id: number
    fullname: string
    email: string
    create: string
  }

  interface IWorkspace {
    id: number
    name: string
  }

  interface IUserInner {
    id: number
    fullname: string
    email: string
  }

  interface IChannel {
    id: number
    ws_id: number
    name: string
    type: 'Single' | 'Group' | 'PrivateChannel' | 'PublicChannel'
    members: Array<number>
    created_at: string
  }

  enum ChannelType {
    Single = 'Single',
    Group = 'Group',
    PrivateChannel = 'PrivateChannel',
    PublicChannel = 'PublicChannel',
  }

  interface IUserSingleChannel extends IUserInner {
    channel_id: number
  }

  interface IMessage {
    id: number
    chat_id: number
    sender_id: number
    content: string
    files: Array<string>
    created_at: string
    sender: IUserInner
  }

  interface ISendMessage {
    content: string
    files?: Array<string>
  }

  interface IConfig {
    server: IServerConfig
  }

  interface IServerConfig {
    chat: string
    notification: string
  }
}
