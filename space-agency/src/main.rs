use personnel::{AstronautJob,Candidate};

fn main() {
    let a = 1;
    let mut jobScore :i32 = 0;
    let candidates = Candidate::load_candidate_file();
    for c in &candidates {
        jobScore = match AstronautJob::from_str(c) {
            AstronautJob::Biogeochemist => 251,
            AstronautJob::Biologist => 257,
            AstronautJob::Engineer => 263,
            AstronautJob::Geologist => 269,
            AstronautJob::Mechanic => 271,
            AstronautJob::Medic => 277,
            AstronautJob::RoverOp => 281,
            AstronautJob::Scientist => 283,
        };
        jobScore *= c.primary_job * c.secondary_job;
        if jobScore > 576 {
            jobScore = jobScore.rem_euclid(576) - 1 
        }
    }
}
