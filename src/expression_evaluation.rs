/// An operation to perform on two subexpressions.
#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
pub enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

pub fn hello_from_file(){
    println!("Hello from expression evaluation file!!")
}

pub fn eval(e: Expression) -> i64 {
    match e {
        Expression::Op {op, right, left} => {
            let right_value = eval(*right);
            let left_value = eval(*left);
            match op {
                Operation::Add => left_value + right_value,
                Operation::Sub => left_value - right_value,
                Operation::Mul => left_value * right_value,
                Operation::Div => {
                    if right_value == 0 {
                        println!("You can't div by 0!!!");
                        return 0;
                    } else if left_value == 0 {
                        println!("You can't div by 0!!!");
                        return 0;
                    } else {
                        return left_value / right_value;
                    }
                },
            }
        },
        Expression::Value(value) => value,  
    }
}

#[test]
pub fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
pub fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
pub fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
pub fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

#[test]
pub fn test_div() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(2)),
        }),
        5
    )
}