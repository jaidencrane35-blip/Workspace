use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceIntentService;
use workspace_domain::{
    Capability, Project, ProjectStatus, Task, TaskPriority, TaskStatus, WorkGoal, WorkflowContext,
};

pub struct CreateProject {
    pub workspace_id: String,
    pub name: String,
    pub description: Option<String>,
    pub metadata: Option<String>,
}

impl CreateProject {
    pub fn new(
        workspace_id: String,
        name: String,
        description: Option<String>,
        metadata: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            name,
            description,
            metadata,
        }
    }
}

impl crate::commands::Command for CreateProject {
    fn name(&self) -> &'static str {
        "CreateProject"
    }
}

impl MutationCommand for CreateProject {
    type Output = Project;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Project> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::create_project(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.metadata.clone(),
        )
    }
}

pub struct UpdateProject {
    pub project_id: String,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<ProjectStatus>,
}

impl UpdateProject {
    pub fn new(
        project_id: String,
        name: Option<String>,
        description: Option<Option<String>>,
        status: Option<ProjectStatus>,
    ) -> Self {
        Self {
            project_id,
            name,
            description,
            status,
        }
    }
}

impl crate::commands::Command for UpdateProject {
    fn name(&self) -> &'static str {
        "UpdateProject"
    }
}

impl MutationCommand for UpdateProject {
    type Output = Project;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Project> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::update_project(
            &ctx.database,
            &ctx.actor_context,
            self.project_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.status,
        )
    }
}

pub struct GetProject {
    pub project_id: String,
}

impl GetProject {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
}

impl crate::commands::Command for GetProject {
    fn name(&self) -> &'static str {
        "GetProject"
    }
}

impl QueryCommand for GetProject {
    type Output = Project;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Project> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::get_project(&ctx.database, self.project_id)
    }
}

pub struct ListProjects {
    pub workspace_id: String,
    pub limit: Option<usize>,
}

impl ListProjects {
    pub fn new(workspace_id: String, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit,
        }
    }
}

impl crate::commands::Command for ListProjects {
    fn name(&self) -> &'static str {
        "ListProjects"
    }
}

impl QueryCommand for ListProjects {
    type Output = Vec<Project>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<Project>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::list_projects(
            &ctx.database,
            &self.workspace_id,
            self.limit.unwrap_or(50),
        )
    }
}

pub struct CreateTask {
    pub project_id: String,
    pub workspace_id: String,
    pub title: String,
    pub priority: TaskPriority,
}

impl CreateTask {
    pub fn new(
        project_id: String,
        workspace_id: String,
        title: String,
        priority: TaskPriority,
    ) -> Self {
        Self {
            project_id,
            workspace_id,
            title,
            priority,
        }
    }
}

impl crate::commands::Command for CreateTask {
    fn name(&self) -> &'static str {
        "CreateTask"
    }
}

impl MutationCommand for CreateTask {
    type Output = Task;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Task> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::create_task(
            &ctx.database,
            &ctx.actor_context,
            self.project_id.clone(),
            self.workspace_id.clone(),
            self.title.clone(),
            self.priority,
        )
    }
}

pub struct UpdateTask {
    pub task_id: String,
    pub title: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
}

impl UpdateTask {
    pub fn new(
        task_id: String,
        title: Option<String>,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
    ) -> Self {
        Self {
            task_id,
            title,
            status,
            priority,
        }
    }
}

impl crate::commands::Command for UpdateTask {
    fn name(&self) -> &'static str {
        "UpdateTask"
    }
}

impl MutationCommand for UpdateTask {
    type Output = Task;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Task> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::update_task(
            &ctx.database,
            &ctx.actor_context,
            self.task_id.clone(),
            self.title.clone(),
            self.status,
            self.priority,
        )
    }
}

pub struct GetTask {
    pub task_id: String,
}

impl GetTask {
    pub fn new(task_id: String) -> Self {
        Self { task_id }
    }
}

impl crate::commands::Command for GetTask {
    fn name(&self) -> &'static str {
        "GetTask"
    }
}

impl QueryCommand for GetTask {
    type Output = Task;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Task> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::get_task(&ctx.database, self.task_id)
    }
}

pub struct ListTasks {
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub limit: Option<usize>,
}

impl ListTasks {
    pub fn new(workspace_id: String, project_id: Option<String>, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            project_id,
            limit,
        }
    }
}

impl crate::commands::Command for ListTasks {
    fn name(&self) -> &'static str {
        "ListTasks"
    }
}

impl QueryCommand for ListTasks {
    type Output = Vec<Task>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<Task>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::list_tasks(
            &ctx.database,
            &self.workspace_id,
            self.project_id.as_deref(),
            self.limit.unwrap_or(50),
        )
    }
}

pub struct CreateWorkGoal {
    pub workspace_id: String,
    pub description: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

impl CreateWorkGoal {
    pub fn new(
        workspace_id: String,
        description: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            description,
            project_id,
            task_id,
        }
    }
}

impl crate::commands::Command for CreateWorkGoal {
    fn name(&self) -> &'static str {
        "CreateWorkGoal"
    }
}

impl MutationCommand for CreateWorkGoal {
    type Output = WorkGoal;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<WorkGoal> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::create_goal(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.description.clone(),
            self.project_id.clone(),
            self.task_id.clone(),
        )
    }
}

pub struct GetWorkflowContext {
    pub workspace_id: String,
}

impl GetWorkflowContext {
    pub fn new(workspace_id: String) -> Self {
        Self { workspace_id }
    }
}

impl crate::commands::Command for GetWorkflowContext {
    fn name(&self) -> &'static str {
        "GetWorkflowContext"
    }
}

impl QueryCommand for GetWorkflowContext {
    type Output = WorkflowContext;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkflowContext> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::get_or_create_workflow_context(&ctx.database, &self.workspace_id)
    }
}

pub struct SetActiveWork {
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

impl SetActiveWork {
    pub fn new(
        workspace_id: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            project_id,
            task_id,
        }
    }
}

impl crate::commands::Command for SetActiveWork {
    fn name(&self) -> &'static str {
        "SetActiveWork"
    }
}

impl MutationCommand for SetActiveWork {
    type Output = WorkflowContext;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<WorkflowContext> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceIntentService::set_active_work(
            &ctx.database,
            &ctx.actor_context,
            &self.workspace_id,
            self.project_id.clone(),
            self.task_id.clone(),
        )
    }
}
