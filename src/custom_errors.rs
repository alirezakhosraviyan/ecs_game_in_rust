use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Function with_component has called before calling create_entity")]
    CreateComponentNeverCalled,
    #[error("Attempted to add a component to unregistered component")]
    UnregisteredComponentAdded,
    #[error("Can not delete the specified resource")]
    CanNotDeleteResource,
    #[error("Component Not Found")]
    ComponentNotFound,
}