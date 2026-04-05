export interface User {
  id: string
  username: string
  created_at: string
}

export interface Certification {
  id: string
  certification_name: string
  master_id: string
  acquired_date: string | null
  created_at: string
}

export type GoalStatus = 'exam_date' | 'passed' | 'failed' | 'abandoned'

export interface Goal {
  id: string
  certification_name: string
  master_id: string
  target_date: string
  status: GoalStatus
  memo: string | null
  study_hours: number
  created_at: string
}

export interface MasterCertification {
  id: string
  name: string
  category: string
}

export interface CommunityGoal {
  certification_name: string
  status: GoalStatus
  study_hours: number
  target_date: string
}

export interface CommunityUser {
  id: string
  username: string
  certification_count: number
  goal_count: number
  achieved_count: number
  total_study_hours: number
  has_good_mark: boolean
  is_favorite: boolean
  goals: CommunityGoal[]
}

export interface CommunityUserDetail {
  id: string
  username: string
  has_good_mark: boolean
  certifications: Certification[]
  goals: Goal[]
}

export interface PaginatedResponse<T> {
  users: T[]
  total: number
  page: number
  per_page: number
}

export interface SignUpForm {
  username: string
  email: string
  password: string
}

export interface SignInForm {
  email: string
  password: string
}

export interface CertificationForm {
  certification_name: string
  master_id: string | null
  acquired_date: string
}

export interface GoalForm {
  certification_name: string
  master_id: string | null
  target_date: string
  status: GoalStatus
  memo: string
  study_hours: number
}
