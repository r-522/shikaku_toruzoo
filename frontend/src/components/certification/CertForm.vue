<template>
  <div>
    <div class="mb-3">
      <label class="form-label">資格名</label>
      <CertAutocomplete
        v-model="form.certification_name"
        @select="onSelect"
      />
    </div>
    <div class="mb-3">
      <label class="form-label">取得日</label>
      <input v-model="form.acquired_date" type="date" class="form-control" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import CertAutocomplete from '../master/CertAutocomplete.vue'
import type { CertificationForm } from '../../types'

const props = defineProps<{
  initial?: CertificationForm
}>()

const form = reactive<CertificationForm>({
  certification_name: props.initial?.certification_name || '',
  master_id: props.initial?.master_id || null,
  acquired_date: props.initial?.acquired_date || '',
})

watch(() => props.initial, (val) => {
  if (val) {
    form.certification_name = val.certification_name
    form.master_id = val.master_id
    form.acquired_date = val.acquired_date
  }
})

function onSelect(payload: { name: string; master_id: string | null }) {
  form.certification_name = payload.name
  form.master_id = payload.master_id
}

defineExpose({ form })
</script>
