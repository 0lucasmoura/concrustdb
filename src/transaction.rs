use std::collections::HashMap;

enum Operation{
    Read,
    Write
}

struct ExecutionLine{
    action_taken: vec!<Transaction.id>
}

struct Transaction{
    //id: i8,
    //name: &str,
    actions: vec!<(Operation, &str)>,
}

impl Transaction{
    pub fn init() -> Transaction{
        Transaction{
            exec_line: ExecutionLine,
            actions: []
        }
    }

    pub fn make_op(&mut self, op: Operation, data: &str) -> (){
        self.exec_line.actions.push((op, data))
    }

    pub fn commit_op(&mut self) -> vec! {
        self.exec_line.actions
    }
}
