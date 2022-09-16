use paperclip_common::serialize_context::Context as SerializeContext;
use paperclip_parser::graph::Dependency;

pub struct Context<'dependency> {
    pub content: SerializeContext,
    pub dependency: &'dependency Dependency,
}

impl<'dependency> Context<'dependency> {
    pub fn new(dependency: &'dependency Dependency) -> Self {
        Self {
            dependency,
            content: SerializeContext::new(0),
        }
    }
    pub fn add_buffer(&mut self, buffer: &str) {
        self.content.add_buffer(buffer);
    }
    pub fn start_block(&mut self) {
        self.content.start_block();
    }
    pub fn end_block(&mut self) {
        self.content.end_block();
    }
    pub fn get_buffer(&self) -> String {
      self.content.buffer.to_string()
    }
}