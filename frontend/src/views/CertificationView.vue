<template>
  <div class="container py-4">
    <div class="d-flex justify-content-between align-items-center mb-4">
      <h3>所持資格</h3>
      <button class="btn btn-primary" @click="openAdd">+ 新規登録</button>
    </div>

    <div v-if="certStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <template v-else>
      <EmptyState v-if="certStore.certifications.length === 0" message="まだ資格が登録されていません。" />
      <CertCard
        v-for="cert in certStore.certifications"
        :key="cert.id"
        :cert="cert"
        @edit="openEdit"
        @delete="confirmDelete"
      />
    </template>

    <FormModal
      :show="showForm"
      :title="editing ? '資格を編集' : '資格を登録'"
      :submit-text="editing ? '更新' : '登録'"
      @close="showForm = false"
      @submit="handleSubmit"
    >
      <CertForm ref="certFormRef" :initial="formInitial" />
    </FormModal>

    <ConfirmModal
      :show="showConfirm"
      message="この資格を削除しますか？"
      @confirm="handleDelete"
      @cancel="showConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useCertificationStore } from '../stores/certification'
import { useToast } from '../composables/useToast'
import CertCard from '../components/certification/CertCard.vue'
import CertForm from '../components/certification/CertForm.vue'
import FormModal from '../components/common/FormModal.vue'
import ConfirmModal from '../components/common/ConfirmModal.vue'
import EmptyState from '../components/common/EmptyState.vue'
import type { Certification, CertificationForm } from '../types'

const certStore = useCertificationStore()
const toast = useToast()

const showForm = ref(false)
const showConfirm = ref(false)
const editing = ref(false)
const editingId = ref('')
const deletingId = ref('')
const certFormRef = ref<InstanceType<typeof CertForm> | null>(null)
const formInitial = ref<CertificationForm | undefined>(undefined)

onMounted(() => certStore.fetchAll())

function openAdd() {
  editing.value = false
  formInitial.value = undefined
  showForm.value = true
}

function openEdit(cert: Certification) {
  editing.value = true
  editingId.value = cert.id
  formInitial.value = {
    certification_name: cert.certification_name,
    master_id: cert.master_id,
    acquired_date: cert.acquired_date || '',
  }
  showForm.value = true
}

function confirmDelete(cert: Certification) {
  deletingId.value = cert.id
  showConfirm.value = true
}

async function handleSubmit() {
  const form = certFormRef.value?.form
  if (!form || !form.certification_name) return

  try {
    if (editing.value) {
      await certStore.update(editingId.value, form)
      toast.show('資格を更新しました', 'success')
    } else {
      await certStore.add(form)
      toast.show('資格を登録しました', 'success')
    }
    showForm.value = false
  } catch (e: any) {
    toast.show(e.response?.data?.error || 'エラーが発生しました', 'error')
  }
}

async function handleDelete() {
  try {
    await certStore.remove(deletingId.value)
    toast.show('資格を削除しました', 'success')
  } catch (e: any) {
    toast.show('削除に失敗しました', 'error')
  }
  showConfirm.value = false
}
</script>
