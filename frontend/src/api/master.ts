import apiClient from './client'
import type { MasterCertification } from '../types'

export const masterApi = {
  search(q: string) {
    return apiClient.get<{ certifications: MasterCertification[] }>('/api/master/certifications', {
      params: { q },
    })
  },
}
