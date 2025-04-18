pub mod user_handler;
pub mod chat_handler;
#[macro_export]
macro_rules! merge_update {
    ($active_model:expr, $update:expr, $( $field:ident ),+ $(,)?) => {
        $(
            if let Some(val) = $update.$field {
                $active_model.$field = Set(val);
            }
        )+
    };
}

// In src/macros.rs
#[macro_export]
macro_rules! merge_update_optional {
    ($active_model:expr, $update:expr, $( $field:ident ),+ $(,)?) => {
        $(
            if let Some(val) = $update.$field {
                $active_model.$field = sea_orm::Set(Some(val));
            }
        )+
    };
}
