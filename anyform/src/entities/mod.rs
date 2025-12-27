//! SeaORM entity definitions for anyform.

pub mod field;
pub mod field_option;
pub mod form;
pub mod result;
pub mod step;
pub mod submission;

pub mod prelude {
    pub use super::field::{
        ActiveModel as FieldActiveModel, Column as FieldColumn, Entity as FieldEntity,
        Model as Field, Relation as FieldRelation,
    };
    pub use super::field_option::{
        ActiveModel as FieldOptionActiveModel, Column as FieldOptionColumn,
        Entity as FieldOptionEntity, Model as FieldOption, Relation as FieldOptionRelation,
    };
    pub use super::form::{
        ActiveModel as FormActiveModel, Column as FormColumn, Entity as FormEntity, Model as Form,
        Relation as FormRelation,
    };
    pub use super::result::{
        ActiveModel as ResultActiveModel, Column as ResultColumn, Entity as ResultEntity,
        Model as FormResult, Relation as ResultRelation,
    };
    pub use super::step::{
        ActiveModel as StepActiveModel, Column as StepColumn, Entity as StepEntity, Model as Step,
        Relation as StepRelation,
    };
    pub use super::submission::{
        ActiveModel as SubmissionActiveModel, Column as SubmissionColumn,
        Entity as SubmissionEntity, Model as Submission, Relation as SubmissionRelation,
    };
}
