use paperclip::actix::Apiv2Schema;
use serde::Serialize;


#[derive(Serialize,Apiv2Schema)]
pub struct Page<T> {
    pub number_of_pages: i64,
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total_elements: i64,
}

impl<T> Page<T> {
    pub fn new(
        number_of_pages: i64,
        data: Vec<T>,
        page_num: i64,
        page_size: i64,
        total_elements: i64,
    ) -> Page<T> {
        Page {
            number_of_pages,
            data,
            page_num,
            page_size,
            total_elements,
        }
    }
}
