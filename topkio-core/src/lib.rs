use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use std::fmt;

/// 泛型上下文类型约束
pub trait AgentContext: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AgentContext for T {}

/// 指令生成器特质
#[async_trait(?Send)]
pub trait InstructionGenerator<TContext: AgentContext> {
    async fn generate_instructions(
        &self,
        context: &RunContextWrapper<TContext>,
        agent: &Agent<TContext>
    ) -> String;
}

/// 指令类型枚举
pub enum AgentInstructions<TContext: AgentContext> {
    Static(String),
    Dynamic(Arc<dyn InstructionGenerator<TContext>>),
    None,
}

impl<TContext: AgentContext> fmt::Debug for AgentInstructions<TContext> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(arg0) => f.debug_tuple("Static").field(arg0).finish(),
            Self::Dynamic(_) => f.debug_tuple("Dynamic").field(&"<instruction_generator>").finish(),
            Self::None => write!(f, "None"),
        }
    }
}

/// Agent核心结构
#[derive(Debug)]
pub struct Agent<TContext: AgentContext> {
    pub name: String,
    pub instructions: AgentInstructions<TContext>,
    pub handoff_description: Option<String>,
    // 隐藏内部实现细节
    _marker: std::marker::PhantomData<TContext>,
}

impl<TContext: AgentContext> Agent<TContext> {
    /// 创建新Agent
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instructions: AgentInstructions::None,
            handoff_description: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// 设置静态指令
    pub fn with_static_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = AgentInstructions::Static(instructions.into());
        self
    }

    /// 设置动态指令生成器
    pub fn with_dynamic_instructions<G>(mut self, generator: G) -> Self
    where
        G: InstructionGenerator<TContext> + 'static,
    {
        self.instructions = AgentInstructions::Dynamic(Arc::new(generator));
        self
    }

    /// 执行指令生成
    pub async fn generate_instructions(
        &self,
        context: &RunContextWrapper<TContext>,
    ) -> Option<String> {
        match &self.instructions {
            AgentInstructions::Static(s) => Some(s.clone()),
            AgentInstructions::Dynamic(generator) => {
                Some(generator.generate_instructions(context, self).await)
            }
            AgentInstructions::None => None,
        }
    }
}

/// 运行上下文包装器
pub struct RunContextWrapper<TContext: AgentContext> {
    inner: Arc<tokio::sync::RwLock<TContext>>,
}

impl<TContext: AgentContext> RunContextWrapper<TContext> {
    pub fn new(context: TContext) -> Self {
        Self {
            inner: Arc::new(tokio::sync::RwLock::new(context)),
        }
    }

    pub async fn get(&self) -> tokio::sync::RwLockReadGuard<'_, TContext> {
        self.inner.read().await
    }

    pub async fn get_mut(&mut self) -> tokio::sync::RwLockWriteGuard<'_, TContext> {
        self.inner.write().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_instructions() {
        let agent = Agent::new("TestAgent")
            .with_static_instructions("Do something");
        
        let context = RunContextWrapper::new(());
        assert_eq!(agent.generate_instructions(&context).await.unwrap(), "Do something");
    }

    struct TestGenerator;
    
    #[async_trait]
    impl<T: AgentContext> InstructionGenerator<T> for TestGenerator {
        async fn generate_instructions(
            &self,
            _: &RunContextWrapper<T>,
            agent: &Agent<T>
        ) -> String {
            format!("Dynamic instructions for {}", agent.name)
        }
    }

    #[tokio::test]
    async fn test_dynamic_instructions() {
        let agent = Agent::new("TestAgent")
            .with_dynamic_instructions(TestGenerator);
        
        let context = RunContextWrapper::new(());
        assert_eq!(
            agent.generate_instructions(&context).await.unwrap(),
            "Dynamic instructions for TestAgent"
        );
    }
}
