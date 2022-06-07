use crate::{get_candidate_score, AstronautJob, Candidate};

#[test]
fn test_candidate_score() {
    let candidate = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: None,
        age: 18,
        health: 100,
    };
    let nonexisting = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: None,
        age: 0,
        health: 0,
    };
    let bill = Candidate {
        primary_job: AstronautJob::Scientist,
        secondary_job: Some(AstronautJob::Medic),
        age: 33,
        health: 151,
    };
    let cara = Candidate {
        primary_job: AstronautJob::Engineer,
        secondary_job: Some(AstronautJob::Mechanic),
        age: 20,
        health: 175,
    };
    let unknown: Candidate = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biologist),
        age: 30,
        health: 99,
    };
    let test: Candidate = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Scientist),
        age: 60,
        health: 200,
    };
    assert_eq!(2682, get_candidate_score(&candidate));
    assert_eq!(0, get_candidate_score(&nonexisting));
    assert_eq!(2870, get_candidate_score(&bill));
    assert_eq!(216, get_candidate_score(&cara));
    assert_eq!(2316, get_candidate_score(&test));
}
