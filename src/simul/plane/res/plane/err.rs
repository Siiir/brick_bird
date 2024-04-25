#[derive(Debug, thiserror::Error)]
#[error("simulation plane hasn't been spawned")]
pub struct NotSpawned;

#[derive(Debug, thiserror::Error)]
#[error("{NotSpawned} fully, because sector {0} is missing")]
pub struct SectorXMissing<X>(X);

#[derive(Debug, thiserror::Error)]
#[error("Cannot spawn this simulation plane because it already has been spawned.")]
pub struct DoubleSpawning;

#[cfg(test)]
mod test {
    use crate::simul::plane::err::SectorXMissing;

    #[test]
    fn sector_n_missing() {
        assert_eq!(
            format!("{}", SectorXMissing { 0: 4 }),
            "simulation plane hasn't been spawned fully, because sector 4 is missing"
        )
    }
}
