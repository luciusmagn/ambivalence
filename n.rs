let mut vars = vec![];
                    for var in all_args.args {
                        vars.push(common::Arg {
                            typ: var.typ,
                            name: var.name
                        })
                    }

let mut vars: Vec<_> = all_args.iter().map(|x| common::Arg {typ: x.typ, name: x.name}).collect();
