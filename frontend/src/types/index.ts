export interface User {
  id: number;
  username: string;
}

export interface Plan {
  id: number;
  user_id: number;
  plan_date: string;
  start_time: string;
  end_time: string;
  created_at: string;
  updated_at: string;
}

export interface Column {
  id: number;
  plan_id: number;
  column_order: number;
  column_type: 'original' | 'revision';
  interruption_time: string | null;
  created_at: string;
}

export interface TimeBlock {
  id: number;
  column_id: number;
  title: string;
  start_time: string;
  end_time: string;
  color: string | null;
  is_completed: boolean;
  created_at: string;
  updated_at: string;
}

export interface ColumnWithBlocks extends Column {
  blocks: TimeBlock[];
}

export interface PlanResponse {
  plan: Plan;
  columns: ColumnWithBlocks[];
}

export interface AuthResponse {
  user: User;
  session_id: string;
}

export interface CreateBlockRequest {
  column_id: number;
  title: string;
  start_time: string;
  end_time: string;
  color?: string;
}

export interface UpdateBlockRequest {
  title?: string;
  start_time?: string;
  end_time?: string;
  color?: string;
  is_completed?: boolean;
}
