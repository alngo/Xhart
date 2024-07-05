#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait IUserCounter {
    fn count_user_by_email(&self, email: &String) -> usize;
}

#[cfg_attr(test, automock)]
pub trait IBusinessRule {
    fn is_broken(&self) -> bool;
    fn message(&self) -> String;
}

pub struct UserEmailMustBeUnique<'c, C> {
    user_counter: &'c C,
    email: String
}

impl <'c, C> UserEmailMustBeUnique<'c, C>
where
    C: IUserCounter
{
    pub fn new(user_counter: &'c C, email: String) -> Self {
        UserEmailMustBeUnique {
            user_counter,
            email
        }
    }
}

impl<'c, C> IBusinessRule for UserEmailMustBeUnique<'c, C>
where
    C: IUserCounter
{
    fn is_broken(&self) -> bool {
        self.user_counter.count_user_by_email(&self.email) > 0
    }

    fn message(&self) -> String {
        "User email must be unique.".to_string()
    }
}
