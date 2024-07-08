#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait BusinessRule {
    fn is_broken(&self) -> bool;
    fn message(&self) -> String;
}
