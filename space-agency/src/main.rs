use personnel::{AstronautJob, Candidate};
#[cfg(test)]
mod test;

fn get_job_code(job: &AstronautJob) -> i32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn get_candidate_score(c: &Candidate) -> i32 {
    let mut job_score = get_job_code(&c.primary_job);
    job_score *= match &c.secondary_job {
        Some(job) => get_job_code(job),
        None => job_score,
    };
    if job_score > 576 {
        job_score %= 576;
    }
    let mut candidate_score: i32 = (job_score + c.health as i32) * c.age as i32;
    if candidate_score > 3928 {
        candidate_score %= 3928;
    }
    candidate_score
}

fn main() {
    let mut candidate_score: i32;
    let mut candidates = Candidate::load_candidate_file();
    candidates.sort_by(|a, b| {
        // We want to sort in descending order
        get_candidate_score(b).cmp(&get_candidate_score(a))

        // If we wanted to instead sort with the lowest number first, we could use this instead:
        // a.cmp(b)
    });
}
