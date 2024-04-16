use super::{AstCollection, Collection};
use crate::{Ast, Error, Parser, Result, Tokenize};
use std::{iter, ops::ControlFlow, option, slice, vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<Item, Punctuation> {
    pub items: Vec<(Item, Punctuation)>,
    pub last: Option<Item>,
}
impl<Item, Punctuation> Default for Punctuated<Item, Punctuation> {
    #[inline]
    fn default() -> Self {
        Self {
            items: Default::default(),
            last: Default::default(),
        }
    }
}
impl<'source, T, Item, Punctuation> AstCollection<'source, T> for Punctuated<Item, Punctuation>
where
    T: Tokenize,
    Item: Ast<'source, T>,
    Punctuation: Ast<'source, T>,
{
    fn ast_item(&mut self, parser: &mut Parser<'source, T>) -> ControlFlow<Option<Error>> {
        match Item::ast(parser) {
            Ok(item) => match Punctuation::ast(parser) {
                Ok(punctuation) => {
                    self.items.push((item, punctuation));
                    ControlFlow::Continue(())
                }
                Err(_) => {
                    self.last = Some(item);
                    ControlFlow::Break(None)
                }
            },
            Err(err) => ControlFlow::Break(Some(err)),
        }
    }
}
impl<'source, T, Item, Punctuation> Ast<'source, T> for Punctuated<Item, Punctuation>
where
    T: Tokenize,
    Item: Ast<'source, T>,
    Punctuation: Ast<'source, T>,
{
    #[inline]
    fn ast(parser: &mut Parser<'source, T>) -> Result<Self> {
        <Collection<Self>>::ast(parser).map(|c| c.0)
    }
}
impl<Item, Punctuation> Punctuated<Item, Punctuation> {
    #[inline]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len() + self.last.is_some() as usize
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.last.is_none()
    }
    pub fn first(&self) -> Option<&Item> {
        self.items
            .first()
            .map(|(item, _)| item)
            .or(self.last.as_ref())
    }
}
impl<Item, Punctuation> IntoIterator for Punctuated<Item, Punctuation> {
    type Item = Item;
    type IntoIter = iter::Chain<
        iter::Map<vec::IntoIter<(Item, Punctuation)>, fn((Item, Punctuation)) -> Item>,
        option::IntoIter<Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .into_iter()
            .map((|(item, _)| item) as fn((Item, Punctuation)) -> Item)
            .chain(self.last)
    }
}
impl<'a, Item, Punctuation> IntoIterator for &'a Punctuated<Item, Punctuation> {
    type Item = &'a Item;
    type IntoIter = iter::Chain<
        iter::Map<slice::Iter<'a, (Item, Punctuation)>, fn(&(Item, Punctuation)) -> &Item>,
        option::Iter<'a, Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter()
            .map((|(item, _)| item) as fn(&(Item, Punctuation)) -> &Item)
            .chain(&self.last)
    }
}
impl<'a, Item, Punctuation> IntoIterator for &'a mut Punctuated<Item, Punctuation> {
    type Item = &'a mut Item;
    type IntoIter = iter::Chain<
        iter::Map<
            slice::IterMut<'a, (Item, Punctuation)>,
            fn(&mut (Item, Punctuation)) -> &mut Item,
        >,
        option::IterMut<'a, Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter_mut()
            .map((|(item, _)| item) as fn(&mut (Item, Punctuation)) -> &mut Item)
            .chain(&mut self.last)
    }
}