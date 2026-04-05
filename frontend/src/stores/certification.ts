import { defineStore } from 'pinia'
import { ref } from 'vue'
import { certificationApi } from '../api/certification'
import type { Certification, CertificationForm } from '../types'

export const useCertificationStore = defineStore('certification', () => {
  const certifications = ref<Certification[]>([])
  const loading = ref(false)

  async function fetchAll() {
    loading.value = true
    try {
      const { data } = await certificationApi.list()
      certifications.value = data.certifications
    } finally {
      loading.value = false
    }
  }

  async function add(form: CertificationForm) {
    const { data } = await certificationApi.create(form)
    certifications.value.unshift(data)
  }

  async function update(id: string, form: CertificationForm) {
    const { data } = await certificationApi.update(id, form)
    const idx = certifications.value.findIndex((c) => c.id === id)
    if (idx !== -1) certifications.value[idx] = data
  }

  async function remove(id: string) {
    await certificationApi.remove(id)
    certifications.value = certifications.value.filter((c) => c.id !== id)
  }

  return { certifications, loading, fetchAll, add, update, remove }
})
