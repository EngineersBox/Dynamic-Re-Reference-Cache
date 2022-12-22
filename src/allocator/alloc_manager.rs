use bumpalo::Bump;

pub struct AllocManager {
    pub (crate) bump: Bump,
}

impl AllocManager {

    pub fn new(size: usize) -> AllocManager {
        let mananger: AllocManager = AllocManager {
            bump: Bump::new()
        };
        mananger.bump.set_allocation_limit(Some(size));
        return manager;
    }

}