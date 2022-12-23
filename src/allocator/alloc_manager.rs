use bumpalo::Bump;

pub struct AllocManager(pub Bump);

impl AllocManager {

    pub fn new(size: usize) -> AllocManager {
        let bump: Bump = Bump::new();
        bump.set_allocation_limit(Some(size));
        return AllocManager(bump);
    }

}