/// Generates derive attributes with (hardcoded) correct trait bounds
macro_rules! perfect_derive {
    ($(#[$attr:meta])* $vis:vis $struct_enum:ident $name:ident <T: LiteralTypes> $($rest:tt)*) => {
        #[derive(derivative::Derivative, serde::Serialize, serde::Deserialize)]
        #[derivative(
            Debug(bound = "T::CustomCommand: std::fmt::Debug, T::UberIdentifier: std::fmt::Debug, T::String: std::fmt::Debug"),
            Clone(bound = "T::CustomCommand: Clone, T::UberIdentifier: Clone, T::String: Clone"),
            PartialEq(bound = "T::CustomCommand: PartialEq, T::UberIdentifier: PartialEq, T::String: PartialEq"),
            Eq(bound = "T::CustomCommand: Eq, T::UberIdentifier: Eq, T::String: Eq"),
            Hash(bound = "T::CustomCommand: std::hash::Hash, T::UberIdentifier: std::hash::Hash, T::String: std::hash::Hash")
        )]
        #[serde(bound(
            serialize = "T::CustomCommand: serde::Serialize, T::UberIdentifier: serde::Serialize, T::String: serde::Serialize",
            deserialize = "T::CustomCommand: serde::Deserialize<'de>, T::UberIdentifier: serde::Deserialize<'de>, T::String: serde::Deserialize<'de>"
        ))]
        $(#[$attr])* $vis $struct_enum $name <T: LiteralTypes> $($rest)*
    };
}
pub(crate) use perfect_derive;
