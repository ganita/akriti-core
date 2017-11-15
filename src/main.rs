/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

extern crate cassowary;

use cassowary::{Solver, Variable};
use cassowary::WeightedRelation::*;
use cassowary::strength::*;

struct Column {
    width: Variable,
}

fn main() {
    let mut solver = Solver::new();

    let mut constraints = Vec::new();

    let table_width = Variable::new();
    constraints.push(table_width |EQ(REQUIRED)| 300.0);

    let col1 = Column { width: Variable::new() };
    constraints.push(col1.width |GE(STRONG)| 10.0);
    constraints.push(col1.width |GE(STRONG)| 20.0);

    let col2 = Column { width: Variable::new() };
    constraints.push(col2.width |GE(STRONG)| 10.0);
    constraints.push(col1.width + col2.width | GE(STRONG) | 30.0);

    constraints.push(col1.width + col2.width |EQ(REQUIRED)| table_width);
    constraints.push(col1.width |EQ(REQUIRED)| col2.width);

    solver.add_constraints(&constraints).unwrap();

    println!("{:?}", solver.fetch_changes());

}
