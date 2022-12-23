use bumpalo::AllocOrInitError;
use bumpalo::Bump;

pub struct AllocManager;

impl AllocManager {

    pub fn new(size: usize) -> Bump {
        let bump: Bump = Bump::new();
        bump.set_allocation_limit(Some(size));
        return bump;
    }

    pub fn handled_alloc<T>(bump: &Bump, val: T) -> &T {
        match bump.try_alloc_with(|| val) {
            Ok(result) => result,
            Err(error) => {
                error!(&crate::LOGGER, format!(
                    "Unable to reserve memory for structure {}: {}",
                    std::any::type_name_of_val(&val),
                    error.to_string(),
                ));
                return val;
            }
        }
    }

}