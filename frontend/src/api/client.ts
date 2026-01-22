import type {
  AuthResponse,
  PlanResponse,
  TimeBlock,
  Column,
  CreateBlockRequest,
  UpdateBlockRequest,
} from '../types';

const API_BASE = '/api';

async function fetchAPI<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${url}`, {
    ...options,
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Request failed' }));
    throw new Error(error.error || 'Request failed');
  }

  return response.json();
}

// Auth API
export const auth = {
  async register(username: string, password: string): Promise<AuthResponse> {
    return fetchAPI<AuthResponse>('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  },

  async login(username: string, password: string): Promise<AuthResponse> {
    return fetchAPI<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  },

  async logout(): Promise<void> {
    await fetchAPI('/auth/logout', { method: 'POST' });
  },

  async me(): Promise<{ id: number; username: string }> {
    return fetchAPI('/auth/me');
  },
};

// Plans API
export const plans = {
  async getByDate(date: string): Promise<PlanResponse> {
    return fetchAPI<PlanResponse>(`/plans/${date}`);
  },

  async create(date: string, startTime: string, endTime: string): Promise<PlanResponse> {
    return fetchAPI<PlanResponse>('/plans', {
      method: 'POST',
      body: JSON.stringify({
        plan_date: date,
        start_time: startTime,
        end_time: endTime,
      }),
    });
  },

  async update(
    id: number,
    data: { start_time?: string; end_time?: string }
  ): Promise<PlanResponse> {
    return fetchAPI<PlanResponse>(`/plans/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },
};

// Columns API
export const columns = {
  async create(planId: number, interruptionTime: string): Promise<Column> {
    return fetchAPI<Column>('/columns', {
      method: 'POST',
      body: JSON.stringify({
        plan_id: planId,
        interruption_time: interruptionTime,
      }),
    });
  },

  async delete(id: number): Promise<void> {
    await fetchAPI(`/columns/${id}`, { method: 'DELETE' });
  },
};

// Blocks API
export const blocks = {
  async create(data: CreateBlockRequest): Promise<TimeBlock> {
    return fetchAPI<TimeBlock>('/blocks', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  async update(id: number, data: UpdateBlockRequest): Promise<TimeBlock> {
    return fetchAPI<TimeBlock>(`/blocks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },

  async delete(id: number): Promise<void> {
    await fetchAPI(`/blocks/${id}`, { method: 'DELETE' });
  },
};
