use bumpalo::Bump;

pub struct AllocManager;

impl AllocManager {

    pub fn new(size: usize) -> Bump {
        let bump: Bump = Bump::new();
        bump.set_allocation_limit(Some(size));
        return bump;
    }

}