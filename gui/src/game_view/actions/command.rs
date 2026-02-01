use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) enum Command {
    SetResourceGain(usize),
    PayResources(usize),
    SetMultiplier(isize),
    CloseCard(Uuid),
    DeactivateCard(Uuid),
    ActivateCard(Uuid),
    IncreaseReputation(u8),
    DecreaseReputation(u8),
}
