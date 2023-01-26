use std::collections::HashMap;

#[derive(Debug)]
struct Account {
    id: u32,
}

fn main() {
    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];

    let mut resolvers = HashMap::new();
    for a in accounts {
        resolvers.entry(a.id).or_insert(Vec::new()).push(a);
    }
    println!("{:?}", resolvers);

    let accounts = [Account { id: 1 }, Account { id: 2 }, Account { id: 3 }];

    let mut resolvers = HashMap::new();
    accounts.into_iter().for_each(|a| {
        resolvers.entry(a.id).or_insert(Vec::new()).push(a);
    });
    println!("{:?}", resolvers);
}
