use crate::tokenizer::Token;

enum NodeTypes{
    Object,
    Array,
    Boolean,
    String,
    Number,
    Nil
}

enum PrimitiveDataType{
    String,
    Boolean,
    Number

}

struct Literal{
    literal_type: PrimitiveDataType,
    token: Token
}


struct SimpleProperty{
    key: Literal,
    value: Literal
}

struct ComplexProperty{
    
}

enum DerivedDataType{
    SimpleArray(PrimitiveDataType),
    ComplexArray(DerivedDataType),
    SimpleObject(SimpleProperty),
    ComplexObject(ComplexProperty)
}

trait Value {
    fn v(&self)->String;
}

pub struct Node {
    node_type: NodeTypes

}