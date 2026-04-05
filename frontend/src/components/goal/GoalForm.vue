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
        <option value="exam_date">受験日</option>
        <option value="passed">合格</option>
        <option value="failed">不合格</option>
        <option value="abandoned">断念</option>
      </select>
    </div>
    <div class="mb-3">
      <label class="form-label">勉強時間</label>
      <div class="input-group">
        <button class="btn btn-outline-secondary" type="button" @click="decrementHours">-</button>
        <input
          v-model.number="form.study_hours"
          type="number"
          class="form-control text-center"
          min="0"
          step="0.5"
          style="max-width: 100px"
        />
        <button class="btn btn-outline-secondary" type="button" @click="incrementHours">+</button>
        <span class="input-group-text">時間</span>
      </div>
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
  status: props.initial?.status || 'exam_date',
  memo: props.initial?.memo || '',
  study_hours: props.initial?.study_hours || 0,
})

watch(() => props.initial, (val) => {
  if (val) {
    form.certification_name = val.certification_name || ''
    form.master_id = val.master_id || null
    form.target_date = val.target_date || ''
    form.status = val.status || 'exam_date'
    form.memo = val.memo || ''
    form.study_hours = val.study_hours || 0
  }
})

function incrementHours() {
  form.study_hours = Math.round((form.study_hours + 0.5) * 10) / 10
}

function decrementHours() {
  if (form.study_hours >= 0.5) {
    form.study_hours = Math.round((form.study_hours - 0.5) * 10) / 10
  }
}

function onSelect(payload: { name: string; master_id: string | null }) {
  form.certification_name = payload.name
  form.master_id = payload.master_id
}

defineExpose({ form })
</script>
