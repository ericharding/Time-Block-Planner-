import { For, Show, createSignal } from 'solid-js';
import { format } from 'date-fns';
import { TimeGrid } from './TimeGrid';
import type { TimeBlock } from '../types';
import {
  planData,
  currentDate,
  setCurrentDate,
  isLoading,
  error,
  createBlock,
  updateBlock,
  deleteBlock,
  createInterruption,
} from '../stores/planStore';
import './Planner.css';

export function Planner() {
  const [startTime, setStartTime] = createSignal('08:00');
  const [endTime, setEndTime] = createSignal('17:00');

  const handleCreateBlock = async (columnId: number, startTime: string, endTime: string) => {
    const title = prompt('Enter block title:');
    if (!title) return;

    try {
      await createBlock(columnId, title, startTime, endTime);
    } catch (err) {
      alert('Failed to create block');
    }
  };

  const handleUpdateBlock = async (
    block: TimeBlock,
    columnId: number,
    updates: Partial<TimeBlock>
  ) => {
    try {
      await updateBlock(block.id, columnId, updates);
    } catch (err) {
      alert('Failed to update block');
    }
  };

  const handleDeleteBlock = async (blockId: number, columnId: number) => {
    if (!confirm('Delete this block?')) return;

    try {
      await deleteBlock(blockId, columnId);
    } catch (err) {
      alert('Failed to delete block');
    }
  };

  const handleInterruption = async () => {
    const now = new Date().toISOString();
    try {
      await createInterruption(now);
    } catch (err) {
      alert('Failed to create interruption');
    }
  };

  return (
    <div class="planner">
      <div class="planner-header">
        <h1>📅 Daily Time Planner</h1>

        <div class="controls">
          <input
            type="date"
            value={currentDate()}
            onInput={(e) => setCurrentDate(e.currentTarget.value)}
          />

          <label>
            Start:
            <input
              type="time"
              value={startTime()}
              onInput={(e) => setStartTime(e.currentTarget.value)}
            />
          </label>

          <label>
            End:
            <input
              type="time"
              value={endTime()}
              onInput={(e) => setEndTime(e.currentTarget.value)}
            />
          </label>

          <button class="interruption-btn" onClick={handleInterruption}>
            ⚡ Mark Interruption
          </button>
        </div>
      </div>

      <Show when={error()}>
        <div class="error">{error()}</div>
      </Show>

      <Show when={isLoading()}>
        <div class="loading">Loading...</div>
      </Show>

      <Show when={planData()}>
        {(data) => (
          <div class="planner-container" classList={{ 'single-column': data().columns.length === 1 }}>
            <For each={data().columns}>
              {(column) => (
                <div class="planner-column">
                  <div
                    class="column-header"
                    classList={{ interrupted: column.column_type === 'revision' }}
                  >
                    {column.column_type === 'original'
                      ? "Today's Plan"
                      : `Revision ${column.column_order} (${
                          column.interruption_time
                            ? format(new Date(column.interruption_time), 'h:mm a')
                            : ''
                        })`}
                  </div>
                  <TimeGrid
                    column={column}
                    startTime={`${startTime()}:00`}
                    endTime={`${endTime()}:00`}
                    onBlockCreate={(start, end) => handleCreateBlock(column.id, start, end)}
                    onBlockUpdate={(block, updates) =>
                      handleUpdateBlock(block, column.id, updates)
                    }
                    onBlockDelete={(blockId) => handleDeleteBlock(blockId, column.id)}
                  />
                </div>
              )}
            </For>
          </div>
        )}
      </Show>
    </div>
  );
}
