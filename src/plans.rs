use substrait::proto;

use crate::types::NamedStruct;

#[derive(Debug)]
pub struct Plan {
    pub root: Box<Rel>,
}

impl From<&proto::Plan> for Plan {
    fn from(p: &proto::Plan) -> Self {
        let root = p.relations.get(0).expect("assume 1 rel for now");
        let rel_type = root.rel_type.as_ref().expect("expecting presence of plan");
        return Plan {
            root: Box::new(match rel_type {
                proto::plan_rel::RelType::Rel(rel) => Rel::from(rel),
                proto::plan_rel::RelType::Root(rel_root) => {
                    Rel::from(rel_root.input.as_ref().expect("argh"))
                }
            }),
        };
    }
}

// Should this be an enum or should it be done via traits ?!?!?
#[derive(Debug)]
pub enum Rel {
    Read(Read),
    Project(Project),
}

impl From<&proto::Rel> for Rel {
    fn from(rel: &proto::Rel) -> Self {
        Rel::from(rel.rel_type.as_ref().expect("rel_type must be set"))
    }
}

impl From<&proto::rel::RelType> for Rel {
    fn from(rel_type: &proto::rel::RelType) -> Self {
        match rel_type {
            proto::rel::RelType::Read(rr) => Rel::Read(Read::from(rr.as_ref())),
            // RelType::Filter(_) => {}
            // RelType::Fetch(_) => {}
            // RelType::Aggregate(_) => {}
            // RelType::Sort(_) => {}
            // RelType::Join(_) => {}
            proto::rel::RelType::Project(pr) => Rel::Project(Project::from(pr.as_ref())),
            // RelType::Set(_) => {}
            // RelType::ExtensionSingle(_) => {}
            // RelType::ExtensionMulti(_) => {}
            // RelType::ExtensionLeaf(_) => {}
            // RelType::Cross(_) => {}
            // RelType::Reference(_) => {}
            // RelType::Write(_) => {}
            // RelType::Ddl(_) => {}
            // RelType::HashJoin(_) => {}
            // RelType::MergeJoin(_) => {}
            // RelType::NestedLoopJoin(_) => {}
            // RelType::Window(_) => {}
            // RelType::Exchange(_) => {}
            // RelType::Expand(_) => {}
            _ => panic!("cannot handle rel_type: {:?}", rel_type),
        }
    }
}

#[derive(Debug)]
pub struct Read {
    pub base_schema: NamedStruct,
}

impl From<&proto::ReadRel> for Read {
    fn from(rr: &proto::ReadRel) -> Self {
        Read {
            base_schema: NamedStruct::from(
                rr.base_schema.as_ref().expect("base schema must be set"),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub input: Box<Rel>,
    pub expressions: Vec<String>,
}

impl From<&proto::ProjectRel> for Project {
    fn from(pr: &proto::ProjectRel) -> Self {
        let input = pr.input.as_ref().expect("project input must be set");
        Project {
            input: Box::new(Rel::from(input.as_ref())),
            expressions: vec![],
        }
    }
}
