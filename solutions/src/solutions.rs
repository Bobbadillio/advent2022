pub mod aocday01;
pub mod aocday02;
pub mod aocday03;
pub mod aocday04;
pub mod aocday05;
pub mod aocday06;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
