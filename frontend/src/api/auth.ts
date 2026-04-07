import client from './client'
import type { LoginRequest, RegisterRequest, AuthToken } from '../types/api'
import type { User } from '../types/domain'

export const authApi = {
  register: (data: RegisterRequest) => client.post<{ data: User }>('/auth/register', data),
  login: (data: LoginRequest) => client.post<{ data: AuthToken }>('/auth/login', data),
  logout: () => client.post('/auth/logout'),
  getMe: () => client.get<{ data: User }>('/users/me'),
  updateMe: (data: Partial<User>) => client.put<{ data: User }>('/users/me', data),
  requestReset: (email: string) => client.post('/auth/password-reset/request', { email }),
  confirmReset: (token: string, new_password: string) =>
    client.post('/auth/password-reset/confirm', { token, new_password }),
}
