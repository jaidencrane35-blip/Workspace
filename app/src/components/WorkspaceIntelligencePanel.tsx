import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  Project,
  Task,
  Workspace,
  WorkspaceIntelligenceState,
} from "../types/domain";

interface WorkspaceIntelligencePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function WorkspaceIntelligencePanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
}: WorkspaceIntelligencePanelProps) {
  const [state, setState] = useState<WorkspaceIntelligenceState | null>(null);
  const [projectName, setProjectName] = useState("Workspace AI");
  const [taskTitle, setTaskTitle] = useState(
    "Build Workspace Intelligence Layer",
  );
  const [projects, setProjects] = useState<Project[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);

  async function run(ok: string, action: () => Promise<void>) {
    onBusy(true);
    onError(null);
    try {
      await action();
      onMessage(ok);
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  }

  useEffect(() => {
    if (!workspace) {
      setProjects([]);
      setTasks([]);
      setState(null);
      return;
    }
    let cancelled = false;
    void (async () => {
      try {
        const listed = await invokeIpc<Project[]>("list_projects", {
          workspaceId: workspace.id,
          limit: 50,
        });
        if (!cancelled) setProjects(listed);
      } catch (err: unknown) {
        if (!cancelled) {
          onError(err instanceof Error ? err.message : String(err));
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [workspace, onError]);

  return (
    <div className="intelligence-panel">
      <header className="intelligence-hero">
        <p className="assistant-kicker">Workspace intelligence</p>
        <h2>Where you are. What you are working on. What is waiting.</h2>
        <p className="lede">
          Read-only understanding of your work context. Recommendations explain
          why — they never execute or grant permission.
        </p>
      </header>

      <section>
        <h3>Current work</h3>
        {projects.length > 0 && (
          <p className="muted">
            Durable projects:{" "}
            {projects.map((project) => project.name).join(", ")}
          </p>
        )}
        <div className="row">
          <input
            value={projectName}
            disabled={busy || !workspace}
            onChange={(e) => setProjectName(e.target.value)}
            aria-label="Project name"
            placeholder="Project name"
          />
          <button
            type="button"
            disabled={busy || !workspace || !projectName.trim()}
            onClick={() =>
              void run("Project created", async () => {
                if (!workspace) return;
                const project = await invokeIpc<Project>("create_project", {
                  workspaceId: workspace.id,
                  name: projectName.trim(),
                  description: null,
                  metadata: null,
                });
                setProjects((prev) => [project, ...prev]);
                await invokeIpc("set_active_work", {
                  workspaceId: workspace.id,
                  projectId: project.id,
                  taskId: null,
                });
              })
            }
          >
            Create project
          </button>
          <input
            value={taskTitle}
            disabled={busy || !workspace}
            onChange={(e) => setTaskTitle(e.target.value)}
            aria-label="Task title"
            placeholder="Task title"
          />
          <button
            type="button"
            disabled={
              busy || !workspace || !taskTitle.trim() || projects.length === 0
            }
            onClick={() =>
              void run("Task created", async () => {
                if (!workspace || projects.length === 0) return;
                const project = projects[0];
                const task = await invokeIpc<Task>("create_task", {
                  projectId: project.id,
                  workspaceId: workspace.id,
                  title: taskTitle.trim(),
                  priority: "high",
                });
                setTasks((prev) => [task, ...prev]);
                await invokeIpc("set_active_work", {
                  workspaceId: workspace.id,
                  projectId: project.id,
                  taskId: task.id,
                });
              })
            }
          >
            Create task
          </button>
        </div>
      </section>

      <section>
        <div className="row">
          <button
            type="button"
            disabled={busy || !workspace}
            onClick={() =>
              void run("Workspace intelligence generated", async () => {
                if (!workspace) return;
                const next = await invokeIpc<WorkspaceIntelligenceState>(
                  "generate_workspace_intelligence",
                  { workspaceId: workspace.id },
                );
                setState(next);
              })
            }
          >
            Refresh workspace understanding
          </button>
        </div>
      </section>

      {state && (
        <>
          <section>
            <h3>Workspace summary</h3>
            <p>{state.summary}</p>
            <p className="muted">
              Health: {state.workspace_health} · Authority effect:{" "}
              {state.authority_effect}
            </p>
          </section>

          <section>
            <h3>Context overview</h3>
            <dl>
              <dt>Project</dt>
              <dd>{state.current_project?.name ?? "None"}</dd>
              <dt>Task</dt>
              <dd>{state.current_task?.title ?? "None"}</dd>
              <dt>Applications</dt>
              <dd>
                {state.current_applications.map((app) => app.name).join(", ") ||
                  "None registered"}
              </dd>
            </dl>
          </section>

          <section>
            <h3>Suggested next actions</h3>
            <ul className="intelligence-list">
              {state.recommended_actions.map((rec) => (
                <li key={rec.id}>
                  <strong>{rec.title}</strong>
                  <div className="muted">{rec.explanation}</div>
                </li>
              ))}
            </ul>
          </section>

          <section>
            <h3>Pending decisions</h3>
            {state.pending_approvals.length === 0 ? (
              <p className="muted">No pending approvals.</p>
            ) : (
              <ul className="intelligence-list">
                {state.pending_approvals.map((item) => (
                  <li key={item.id}>
                    <strong>{item.summary}</strong>
                    <div className="muted">{item.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h3>Blocked actions</h3>
            {state.blocked_actions.length === 0 ? (
              <p className="muted">No blocked actions.</p>
            ) : (
              <ul className="intelligence-list">
                {state.blocked_actions.map((item) => (
                  <li key={item.id}>
                    <strong>{item.summary}</strong>
                    <div className="muted">{item.explanation}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h3>Memory highlights</h3>
            {state.memory_highlights.length === 0 ? (
              <p className="muted">No memory influence.</p>
            ) : (
              <ul className="intelligence-list">
                {state.memory_highlights.map((item) => (
                  <li key={item.id}>
                    <strong>{item.label}</strong>
                    <div className="muted">{item.summary}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h3>Preference influence</h3>
            {state.preference_highlights.length === 0 ? (
              <p className="muted">No preference influence.</p>
            ) : (
              <ul className="intelligence-list">
                {state.preference_highlights.map((item) => (
                  <li key={item.id}>
                    <strong>{item.label}</strong>
                    <div className="muted">{item.summary}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section>
            <h3>Recent activity</h3>
            <ul className="intelligence-list muted">
              {state.recent_activity.slice(0, 8).map((item, index) => (
                <li key={`${item.timestamp}-${index}`}>
                  {item.event_type}: {item.summary}
                </li>
              ))}
            </ul>
          </section>

          {state.pending_plans.length > 0 && (
            <section>
              <h3>Pending plans</h3>
              <ul className="intelligence-list muted">
                {state.pending_plans.map((plan) => (
                  <li key={plan}>{plan}</li>
                ))}
              </ul>
            </section>
          )}
        </>
      )}

      {!workspace && (
        <p className="muted">
          Create or activate a workspace on Canvas before generating
          intelligence.
        </p>
      )}

      {tasks.length > 0 && (
        <p className="muted">Tasks created this session: {tasks.length}</p>
      )}
    </div>
  );
}
