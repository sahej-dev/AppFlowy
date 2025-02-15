use crate::entities::FieldType;
use crate::impl_type_option;
use crate::services::cell::{CellBytes, CellData, CellDataChangeset, CellDataOperation, CellDisplayable};
use crate::services::field::{
    make_selected_select_options, SelectOptionCellChangeset, SelectOptionCellDataPB, SelectOptionIds,
    SelectOptionOperation, SelectOptionPB,
};
use crate::services::field::{BoxTypeOptionBuilder, TypeOptionBuilder};
use bytes::Bytes;
use flowy_derive::ProtoBuf;
use flowy_error::{FlowyError, FlowyResult};
use flowy_grid_data_model::revision::{CellRevision, FieldRevision, TypeOptionDataDeserializer, TypeOptionDataFormat};
use serde::{Deserialize, Serialize};

// Single select
#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct SingleSelectTypeOptionPB {
    #[pb(index = 1)]
    pub options: Vec<SelectOptionPB>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_type_option!(SingleSelectTypeOptionPB, FieldType::SingleSelect);

impl SelectOptionOperation for SingleSelectTypeOptionPB {
    fn selected_select_option(&self, cell_data: CellData<SelectOptionIds>) -> SelectOptionCellDataPB {
        let mut select_options = make_selected_select_options(cell_data, &self.options);
        // only keep option in single select
        select_options.truncate(1);
        SelectOptionCellDataPB {
            options: self.options.clone(),
            select_options,
        }
    }

    fn options(&self) -> &Vec<SelectOptionPB> {
        &self.options
    }

    fn mut_options(&mut self) -> &mut Vec<SelectOptionPB> {
        &mut self.options
    }
}

impl CellDataOperation<SelectOptionIds, SelectOptionCellChangeset> for SingleSelectTypeOptionPB {
    fn decode_cell_data(
        &self,
        cell_data: CellData<SelectOptionIds>,
        decoded_field_type: &FieldType,
        field_rev: &FieldRevision,
    ) -> FlowyResult<CellBytes> {
        if !decoded_field_type.is_select_option() {
            return Ok(CellBytes::default());
        }

        self.display_data(cell_data, decoded_field_type, field_rev)
    }

    fn apply_changeset(
        &self,
        changeset: CellDataChangeset<SelectOptionCellChangeset>,
        _cell_rev: Option<CellRevision>,
    ) -> Result<String, FlowyError> {
        let content_changeset = changeset.try_into_inner()?;
        let new_cell_data: String;

        let mut insert_option_ids = content_changeset
            .insert_option_ids
            .into_iter()
            .filter(|insert_option_id| self.options.iter().any(|option| &option.id == insert_option_id))
            .collect::<Vec<String>>();

        // In single select, the insert_option_ids should only contain one select option id.
        // Sometimes, the insert_option_ids may contain list of option ids. For example,
        // copy/paste a ids string.
        if insert_option_ids.is_empty() {
            new_cell_data = "".to_string()
        } else {
            // Just take the first select option
            let _ = insert_option_ids.drain(1..);
            new_cell_data = insert_option_ids.pop().unwrap();
        }

        Ok(new_cell_data)
    }
}

#[derive(Default)]
pub struct SingleSelectTypeOptionBuilder(SingleSelectTypeOptionPB);
impl_into_box_type_option_builder!(SingleSelectTypeOptionBuilder);
impl_builder_from_json_str_and_from_bytes!(SingleSelectTypeOptionBuilder, SingleSelectTypeOptionPB);

impl SingleSelectTypeOptionBuilder {
    pub fn add_option(mut self, opt: SelectOptionPB) -> Self {
        self.0.options.push(opt);
        self
    }
}

impl TypeOptionBuilder for SingleSelectTypeOptionBuilder {
    fn field_type(&self) -> FieldType {
        FieldType::SingleSelect
    }

    fn data_format(&self) -> &dyn TypeOptionDataFormat {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::services::cell::CellDataOperation;

    use crate::services::field::type_options::*;
    use crate::services::field::FieldBuilder;

    #[test]
    fn single_select_insert_multi_option_test() {
        let google = SelectOptionPB::new("Google");
        let facebook = SelectOptionPB::new("Facebook");
        let single_select = SingleSelectTypeOptionBuilder::default()
            .add_option(google.clone())
            .add_option(facebook.clone());

        let field_rev = FieldBuilder::new(single_select).name("Platform").build();
        let type_option = SingleSelectTypeOptionPB::from(&field_rev);
        let option_ids = vec![google.id.clone(), facebook.id];
        let data = SelectOptionCellChangeset::from_insert_options(option_ids).to_str();
        let select_option_ids: SelectOptionIds = type_option.apply_changeset(data.into(), None).unwrap().into();

        assert_eq!(&*select_option_ids, &vec![google.id]);
    }

    #[test]
    fn single_select_unselect_multi_option_test() {
        let google = SelectOptionPB::new("Google");
        let facebook = SelectOptionPB::new("Facebook");
        let single_select = SingleSelectTypeOptionBuilder::default()
            .add_option(google.clone())
            .add_option(facebook.clone());

        let field_rev = FieldBuilder::new(single_select).name("Platform").build();
        let type_option = SingleSelectTypeOptionPB::from(&field_rev);
        let option_ids = vec![google.id.clone(), facebook.id];

        // insert
        let data = SelectOptionCellChangeset::from_insert_options(option_ids.clone()).to_str();
        let select_option_ids: SelectOptionIds = type_option.apply_changeset(data.into(), None).unwrap().into();
        assert_eq!(&*select_option_ids, &vec![google.id]);

        // delete
        let data = SelectOptionCellChangeset::from_delete_options(option_ids).to_str();
        let select_option_ids: SelectOptionIds = type_option.apply_changeset(data.into(), None).unwrap().into();
        assert!(select_option_ids.is_empty());
    }

    #[test]
    fn single_select_insert_non_exist_option_test() {
        let google = SelectOptionPB::new("Google");
        let single_select = SingleSelectTypeOptionBuilder::default();
        let field_rev = FieldBuilder::new(single_select).name("Platform").build();
        let type_option = SingleSelectTypeOptionPB::from(&field_rev);

        let option_ids = vec![google.id];
        let data = SelectOptionCellChangeset::from_insert_options(option_ids).to_str();
        let cell_option_ids = type_option.apply_changeset(data.into(), None).unwrap();

        assert!(cell_option_ids.is_empty());
    }

    #[test]
    fn single_select_insert_invalid_option_id_test() {
        let single_select = SingleSelectTypeOptionBuilder::default();
        let field_rev = FieldBuilder::new(single_select).name("Platform").build();
        let type_option = SingleSelectTypeOptionPB::from(&field_rev);

        let data = SelectOptionCellChangeset::from_insert_option_id("").to_str();
        let cell_option_ids = type_option.apply_changeset(data.into(), None).unwrap();
        assert_eq!(cell_option_ids, "");
    }

    #[test]
    fn single_select_invalid_changeset_data_test() {
        let single_select = SingleSelectTypeOptionBuilder::default();
        let field_rev = FieldBuilder::new(single_select).name("Platform").build();
        let type_option = SingleSelectTypeOptionPB::from(&field_rev);

        // The type of the changeset should be SelectOptionCellChangeset
        assert!(type_option.apply_changeset("123".to_owned().into(), None).is_err());
    }
}
