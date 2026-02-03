use features::shared::domain::value::muid::Muid;

use crate::feature::library::domain::cdc::SeriesChange;

#[derive(Clone, Debug)]
pub struct Series {
    id: Muid,
    title: String,
    image_id: Option<Muid>,
    _changes: Vec<SeriesChange>,
}

impl Series {
    pub fn new(title: String) -> Self {
        let mut this = Self {
            id: Muid::new_now(),
            title,
            image_id: None,
            _changes: vec![],
        };

        this._changes.push(SeriesChange::Created(this.clone()));

        this
    }

    pub fn from_parts(id: Muid, title: String, image_id: Option<Muid>) -> Self {
        Self {
            id,
            title,
            image_id,
            _changes: vec![],
        }
    }

    pub fn id(&self) -> &Muid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn image_id(&self) -> Option<&Muid> {
        self.image_id.as_ref()
    }

    pub fn change_events(&mut self) -> Vec<SeriesChange> {
        std::mem::take(&mut self._changes)
    }
}
