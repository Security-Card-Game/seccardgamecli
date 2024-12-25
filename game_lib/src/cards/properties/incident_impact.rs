use crate::cards::properties::incident_impact::IncidentImpact::Fixed;
use crate::cards::serialization::helper::Number;
use crate::world::part_of_hundred::PartOfHundred;
use crate::world::resources::Resources;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IncidentImpact {
    PartOfRevenue(PartOfHundred),
    Fixed(Resources)
}

impl IncidentImpact {
    pub fn none() -> Self {
        Fixed(Resources::new(0))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::cards::properties::incident_impact::IncidentImpact::{Fixed, PartOfRevenue};
    use crate::cards::properties::incident_impact::{IncidentImpact, PartOfHundred};
    use crate::world::resources::Resources;
    use fake::Dummy;
    use rand::Rng;


    pub struct FakePartOfRevenueIncidentImpact;
    pub struct FakeFixedIncidentImpact;

    impl Dummy<FakePartOfRevenueIncidentImpact> for IncidentImpact {
        fn dummy_with_rng<R: Rng + ?Sized>(_config: &FakePartOfRevenueIncidentImpact, rng: &mut R) -> Self {
            PartOfRevenue(
                PartOfHundred {
                    value: rng.gen()
                }
            )
        }
    }

    impl Dummy<FakeFixedIncidentImpact> for IncidentImpact {
        fn dummy_with_rng<R: Rng + ?Sized>(_config: &FakeFixedIncidentImpact, rng: &mut R) -> Self {
            Fixed(
                Resources::new(rng.gen())
            )
        }
    }

    #[test]
    fn create_none_incident_impact_does_not_cost_resources_and_is_fixed() {
        let sut = IncidentImpact::none();
        match sut {
            PartOfRevenue(_) => panic!("Should be a fixed impact"),
            Fixed(r) =>  assert_eq!(r.value(), &0)
        }
    }

    #[test]
    fn create_fix_incident_with_10_resources() {
        let fixed = Fixed(Resources::new(10));
        assert_eq!(fixed, Fixed(Resources::new(10)));
    }

    #[test]
    fn create_relative_incident_impact_of_0_works() {
        let sut = PartOfRevenue(PartOfHundred::new(0));
        match sut {
            PartOfRevenue(ph) => {
                assert_eq!(ph.value, 0);
            }
            Fixed(_) => panic!("Should be a relative impact")
        }
    }

    #[test]
    fn create_relative_incident_impact_of_100_works() {
        let sut = PartOfRevenue(PartOfHundred::new(100));
        match sut {
            PartOfRevenue(ph) => {
                assert_eq!(ph.value, 100);
            }
            Fixed(_) => panic!("Should be a relative impact")
        }
    }

    #[test]
    #[should_panic]
    fn create_relative_incident_impact_of_101_panics() {
        PartOfRevenue(PartOfHundred::new(101));
    }

}