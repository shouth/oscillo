mod generated;

pub use generated::*;

pub trait TypedNode: Sized {
    type Value;
    type Node;

    fn can_cast(value: Self::Value) -> bool;
    fn cast(node: Self::Node) -> Option<Self>;
    fn syntax(&self) -> &Self::Node;
}

pub mod support {
    use syntree::{pointer::Width, Node};

    use super::TypedNode;

    pub fn child<'a, T, I, W, N>(node: &Node<'a, T, I, W>, index: usize) -> Option<N>
    where
        T: Copy,
        W: Width,
        N: TypedNode<Value = T, Node = Node<'a, T, I, W>>
    {
        node.children()
            .filter(|c| N::can_cast(*c.value()))
            .nth(index)
            .and_then(|c| N::cast(c.clone()))
    }

    pub fn children<'a, T, I, W, N>(node: &Node<'a, T, I, W>) -> impl Iterator<Item = N> + 'a
    where
        T: Copy,
        W: Width + 'a,
        N: TypedNode<Value = T, Node = Node<'a, T, I, W>>
    {
        node.children()
            .filter(|c| N::can_cast(*c.value()))
            .filter_map(|c| N::cast(c.clone()))
    }
}
