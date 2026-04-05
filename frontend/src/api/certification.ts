import apiClient from './client'
import type { Certification, CertificationForm } from '../types'

export const certificationApi = {
  list() {
    return apiClient.get<{ certifications: Certification[] }>('/api/certifications')
  },

  create(form: CertificationForm) {
    return apiClient.post<Certification>('/api/certifications', form)
  },

  update(id: string, form: CertificationForm) {
    return apiClient.put<Certification>(`/api/certifications/${id}`, form)
  },

  remove(id: string) {
    return apiClient.delete(`/api/certifications/${id}`)
  },
}
