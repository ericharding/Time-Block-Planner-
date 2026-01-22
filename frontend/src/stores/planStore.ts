import { createSignal, createEffect } from 'solid-js';
import { format } from 'date-fns';
import type { PlanResponse, TimeBlock } from '../types';
import { plans, blocks, columns } from '../api/client';

export const [currentDate, setCurrentDate] = createSignal(format(new Date(), 'yyyy-MM-dd'));
export const [planData, setPlanData] = createSignal<PlanResponse | null>(null);
export const [isLoading, setIsLoading] = createSignal(false);
export const [error, setError] = createSignal<string | null>(null);

export async function loadPlan(date: string) {
  setIsLoading(true);
  setError(null);

  try {
    const data = await plans.getByDate(date);
    setPlanData(data);
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to load plan');
  } finally {
    setIsLoading(false);
  }
}

export async function createBlock(
  columnId: number,
  title: string,
  startTime: string,
  endTime: string,
  color?: string
) {
  try {
    const newBlock = await blocks.create({
      column_id: columnId,
      title,
      start_time: startTime,
      end_time: endTime,
      color,
    });

    // Update local state
    const current = planData();
    if (current) {
      const updatedColumns = current.columns.map((col) =>
        col.id === columnId ? { ...col, blocks: [...col.blocks, newBlock] } : col
      );
      setPlanData({ ...current, columns: updatedColumns });
    }

    return newBlock;
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to create block');
    throw err;
  }
}

export async function updateBlock(
  id: number,
  columnId: number,
  updates: {
    title?: string;
    start_time?: string;
    end_time?: string;
    color?: string;
    is_completed?: boolean;
  }
) {
  try {
    const updatedBlock = await blocks.update(id, updates);

    // Update local state
    const current = planData();
    if (current) {
      const updatedColumns = current.columns.map((col) =>
        col.id === columnId
          ? {
              ...col,
              blocks: col.blocks.map((block) => (block.id === id ? updatedBlock : block)),
            }
          : col
      );
      setPlanData({ ...current, columns: updatedColumns });
    }

    return updatedBlock;
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to update block');
    throw err;
  }
}

export async function deleteBlock(id: number, columnId: number) {
  try {
    await blocks.delete(id);

    // Update local state
    const current = planData();
    if (current) {
      const updatedColumns = current.columns.map((col) =>
        col.id === columnId
          ? { ...col, blocks: col.blocks.filter((block) => block.id !== id) }
          : col
      );
      setPlanData({ ...current, columns: updatedColumns });
    }
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to delete block');
    throw err;
  }
}

export async function createInterruption(interruptionTime: string) {
  const current = planData();
  if (!current) return;

  try {
    const newColumn = await columns.create(current.plan.id, interruptionTime);

    // Update local state
    setPlanData({
      ...current,
      columns: [...current.columns, { ...newColumn, blocks: [] }],
    });

    return newColumn;
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to create interruption');
    throw err;
  }
}

// Auto-load plan when date changes
createEffect(() => {
  loadPlan(currentDate());
});
