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
      <label class="form-label">目標日</label>
      <input v-model="form.target_date" type="date" class="form-control" required />
    </div>
    <div class="mb-3">
      <label class="form-label">ステータス</label>
      <select v-model="form.status" class="form-select">
        <option value="studying">学習中</option>
        <option value="scheduled">受験予定</option>
        <option value="achieved">達成</option>
        <option value="suspended">中断</option>
      </select>
    </div>
    <div class="mb-3">
      <label class="form-label">メモ</label>
      <textarea v-model="form.memo" class="form-control" rows="3" maxlength="1000"></textarea>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import CertAutocomplete from '../master/CertAutocomplete.vue'
import type { GoalForm } from '../../types'

const props = defineProps<{
  initial?: Partial<GoalForm>
}>()

const form = reactive<GoalForm>({
  certification_name: props.initial?.certification_name || '',
  master_id: props.initial?.master_id || null,
  target_date: props.initial?.target_date || '',
  status: props.initial?.status || 'studying',
  memo: props.initial?.memo || '',
})

watch(() => props.initial, (val) => {
  if (val) {
    form.certification_name = val.certification_name || ''
    form.master_id = val.master_id || null
    form.target_date = val.target_date || ''
    form.status = val.status || 'studying'
    form.memo = val.memo || ''
  }
})

function onSelect(payload: { name: string; master_id: string | null }) {
  form.certification_name = payload.name
  form.master_id = payload.master_id
}

defineExpose({ form })
</script>
