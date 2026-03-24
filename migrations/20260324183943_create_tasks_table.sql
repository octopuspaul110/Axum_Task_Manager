-- Add migration script here

CREATE TYPE task_status   AS ENUM ('todo','in_progress','done');
CREATE TYPE task_priority AS ENUM ('low','medium','high');

CREATE TABLE tasks (

    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id          UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title               TEXT NOT NULL,
    description         TEXT,
    status              task_status     NOT NULL DEFAULT 'todo',
    priority            task_priority   NOT NULL DEFAULT 'medium',

    due_date            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

-- index for listing tasks in a project
CREATE INDEX idx_tasks_project_id ON tasks(project_id);

-- index for filtering tasks by status (e.g show only 'todo' tasks)
CREATE INDEX idx_tasks_status ON tasks(status) 



