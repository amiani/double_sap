#![warn(clippy::all, clippy::pedantic)]

mod succinct_tree;
mod sweep_and_prune;
mod radix_sort;

pub use sweep_and_prune::*;

#[cfg(test)]
mod tests {
    use super::*;
}
