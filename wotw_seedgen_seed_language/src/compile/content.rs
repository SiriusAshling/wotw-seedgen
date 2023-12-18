use super::{Compile, SnippetCompiler};
use crate::{
    ast::{self, TriggerBinding},
    output::{intermediate::Literal, Action, ActionCondition, Event, Trigger},
};
use wotw_seedgen_parse::{Error, Span};

impl<'source> Compile<'source> for ast::Content<'source> {
    type Output = ();

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        match self {
            ast::Content::Event(_, event) => {
                event.compile(compiler);
            }
            ast::Content::Function(_, function) => {
                function.compile(compiler);
            }
            ast::Content::Command(_, command) => {
                command.compile(compiler);
            }
            ast::Content::Annotation(..) => {}
        }
    }
}

impl<'source> Compile<'source> for ast::Event<'source> {
    type Output = ();

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        let trigger = self.trigger.compile(compiler);
        let action = self.action.compile(compiler);

        if let (Some(trigger), Some(Some(action))) = (trigger, action) {
            compiler
                .global
                .output
                .events
                .push(Event { trigger, action });
        }
    }
}
impl<'source> Compile<'source> for ast::Trigger<'source> {
    type Output = Option<Trigger>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        match self {
            ast::Trigger::Pseudo(pseudo) => Some(Trigger::Pseudo(pseudo.data)),
            ast::Trigger::Binding(_, binding) => {
                let span = binding.span();

                let uber_state = match binding {
                    TriggerBinding::UberIdentifier(uber_identifier) => {
                        uber_identifier.compile(compiler)?
                    }
                    TriggerBinding::Identifier(identifier) => {
                        match compiler.resolve(&identifier)? {
                            Literal::UberIdentifier(uber_state) => uber_state.clone(),
                            other => {
                                let found = other.literal_type();
                                compiler.errors.push(Error::custom(
                                    format!("Expected UberIdentifier, but found {found}"),
                                    identifier.span,
                                ));
                                return None;
                            }
                        }
                    }
                };

                match uber_state.value {
                    None => Some(Trigger::Binding(uber_state.uber_identifier)),
                    Some(_) => {
                        compiler.errors.push(Error::custom(
                            "cannot bind to an alias which resolves to an integer state comparison"
                                .to_string(),
                            span,
                        ));
                        None
                    }
                }
            }
            ast::Trigger::Condition(expression) => {
                expression.compile_into(compiler).map(Trigger::Condition)
            }
        }
    }
}

impl<'source> Compile<'source> for ast::FunctionDefinition<'source> {
    type Output = ();

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        let actions = self
            .actions
            .content
            .into_iter()
            .flatten()
            .filter_map(|action| action.compile(compiler))
            .collect::<Vec<_>>();

        let index = compiler
            .function_indices
            .get(self.identifier.data.0)
            .unwrap();
        compiler.global.output.action_lookup[*index] = Action::Multi(actions);
    }
}

impl<'source> Compile<'source> for ast::Action<'source> {
    type Output = Option<Action>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        match self {
            ast::Action::Function(function_call) => function_call.compile(compiler),
            ast::Action::Condition(_, condition) => condition
                .compile(compiler)
                .map(Box::new)
                .map(Action::Condition),
            ast::Action::Multi(actions) => {
                let actions = actions
                    .content
                    .into_iter()
                    .flatten()
                    .filter_map(|action| action.compile(compiler))
                    .collect();
                Some(Action::Multi(actions))
            }
        }
    }
}
impl<'source> Compile<'source> for ast::ActionCondition<'source> {
    type Output = Option<ActionCondition>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        let condition = self.condition.compile_into(compiler);
        let action = self.action.compile(compiler);

        Some(ActionCondition {
            condition: condition?,
            action: action??,
        })
    }
}
