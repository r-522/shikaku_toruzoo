import apiClient from './client'
import type { SignUpForm, SignInForm, User } from '../types'

export const authApi = {
  signup(form: SignUpForm) {
    return apiClient.post('/api/auth/signup', form)
  },

  signin(form: SignInForm) {
    return apiClient.post<{ user: { id: string; username: string } }>('/api/auth/signin', form)
  },

  signout() {
    return apiClient.post('/api/auth/signout')
  },

  fetchMe() {
    return apiClient.get<User>('/api/auth/me')
  },
}
