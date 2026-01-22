import { For, createSignal, Show } from 'solid-js';
import { format, parse, addMinutes } from 'date-fns';
import type { ColumnWithBlocks, TimeBlock } from '../types';
import { DragDropProvider, DragDropSensors, SortableProvider, createSortable, useDragDropContext } from '@thisbeyond/solid-dnd';
import './TimeGrid.css';

interface TimeGridProps {
  column: ColumnWithBlocks;
  startTime: string; // HH:mm format
  endTime: string; // HH:mm format
  onBlockCreate: (startTime: string, endTime: string) => void;
  onBlockUpdate: (block: TimeBlock, updates: Partial<TimeBlock>) => void;
  onBlockDelete: (blockId: number) => void;
}

function TimeSlots(props: { startTime: string; endTime: string }) {
  const slots = () => {
    const start = parse(props.startTime, 'HH:mm:ss', new Date());
    const end = parse(props.endTime, 'HH:mm:ss', new Date());
    const timeSlots: string[] = [];

    let current = start;
    while (current <= end) {
      timeSlots.push(format(current, 'h:mm a'));
      current = addMinutes(current, 15);
    }

    return timeSlots;
  };

  return (
    <div class="time-labels">
      <For each={slots()}>
        {(time, index) => (
          <div
            class="time-slot"
            classList={{ quarter: index() % 4 !== 0 }}
          >
            {index() % 4 === 0 ? time : ''}
          </div>
        )}
      </For>
    </div>
  );
}

interface TimeBlockComponentProps {
  block: TimeBlock;
  startTime: string;
  onUpdate: (updates: Partial<TimeBlock>) => void;
  onDelete: () => void;
}

function TimeBlockComponent(props: TimeBlockComponentProps) {
  const [isEditing, setIsEditing] = createSignal(false);
  const [title, setTitle] = createSignal(props.block.title);

  const sortable = createSortable(props.block.id);

  const position = () => {
    const startOfDay = parse(props.startTime, 'HH:mm:ss', new Date());
    const blockStart = new Date(props.block.start_time);
    const blockEnd = new Date(props.block.end_time);

    const minutesFromStart = (blockStart.getTime() - startOfDay.getTime()) / (1000 * 60);
    const duration = (blockEnd.getTime() - blockStart.getTime()) / (1000 * 60);

    return {
      top: minutesFromStart,
      height: duration,
    };
  };

  const handleSave = () => {
    if (title() !== props.block.title) {
      props.onUpdate({ title: title() });
    }
    setIsEditing(false);
  };

  const colorClass = () => {
    const color = props.block.color || 'blue';
    return `time-block ${color}`;
  };

  return (
    <div
      ref={sortable.ref}
      class={colorClass()}
      classList={{ completed: props.block.is_completed, dragging: sortable.isActiveDraggable }}
      style={{
        top: `${position().top}px`,
        height: `${position().height}px`,
      }}
      onClick={() => setIsEditing(true)}
    >
      <Show
        when={isEditing()}
        fallback={
          <>
            <div class="block-title">{props.block.title}</div>
            <div class="block-time">
              {format(new Date(props.block.start_time), 'h:mm a')} -{' '}
              {format(new Date(props.block.end_time), 'h:mm a')}
            </div>
          </>
        }
      >
        <input
          type="text"
          value={title()}
          onInput={(e) => setTitle(e.currentTarget.value)}
          onBlur={handleSave}
          onKeyPress={(e) => e.key === 'Enter' && handleSave()}
          autofocus
        />
        <button class="delete-btn" onClick={(e) => { e.stopPropagation(); props.onDelete(); }}>
          ×
        </button>
      </Show>
    </div>
  );
}

export function TimeGrid(props: TimeGridProps) {
  const [dragStartY, setDragStartY] = createSignal<number | null>(null);
  const [dragEndY, setDragEndY] = createSignal<number | null>(null);

  const handleMouseDown = (e: MouseEvent) => {
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const y = e.clientY - rect.top;
    setDragStartY(y);
    setDragEndY(y);
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (dragStartY() === null) return;
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const y = e.clientY - rect.top;
    setDragEndY(y);
  };

  const handleMouseUp = () => {
    const start = dragStartY();
    const end = dragEndY();

    if (start !== null && end !== null && Math.abs(end - start) > 15) {
      // Snap to 15-minute increments
      const minY = Math.min(start, end);
      const maxY = Math.max(start, end);

      const startMinutes = Math.round(minY / 15) * 15;
      const endMinutes = Math.round(maxY / 15) * 15;

      const startOfDay = parse(props.startTime, 'HH:mm:ss', new Date());
      const blockStart = addMinutes(startOfDay, startMinutes);
      const blockEnd = addMinutes(startOfDay, endMinutes);

      props.onBlockCreate(
        blockStart.toISOString(),
        blockEnd.toISOString()
      );
    }

    setDragStartY(null);
    setDragEndY(null);
  };

  const selectionBox = () => {
    const start = dragStartY();
    const end = dragEndY();
    if (start === null || end === null) return null;

    const top = Math.min(start, end);
    const height = Math.abs(end - start);

    return { top, height };
  };

  return (
    <div class="timeline-container">
      <TimeSlots startTime={props.startTime} endTime={props.endTime} />
      <div
        class="blocks-area"
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={() => {
          setDragStartY(null);
          setDragEndY(null);
        }}
      >
        <DragDropProvider>
          <DragDropSensors />
          <SortableProvider ids={props.column.blocks.map((b) => b.id)}>
            <For each={props.column.blocks}>
              {(block) => (
                <TimeBlockComponent
                  block={block}
                  startTime={props.startTime}
                  onUpdate={(updates) => props.onBlockUpdate(block, updates)}
                  onDelete={() => props.onBlockDelete(block.id)}
                />
              )}
            </For>
          </SortableProvider>
        </DragDropProvider>

        <Show when={selectionBox()}>
          {(box) => (
            <div
              class="selection-box"
              style={{
                top: `${box().top}px`,
                height: `${box().height}px`,
              }}
            />
          )}
        </Show>
      </div>
    </div>
  );
}
