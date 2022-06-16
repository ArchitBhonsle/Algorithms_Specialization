#[derive(Clone, Debug)]
struct Job {
    weight: usize,
    length: usize,
}

/// Parse the jobs.txt file into a list of jobs
fn parse_jobs(string: &str) -> Vec<Job> {
    let mut lines = string.lines();
    lines.next().unwrap(); // skip the length

    lines
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            let weight = split.next().unwrap().parse::<usize>().unwrap();
            let length = split.next().unwrap().parse::<usize>().unwrap();

            Job { weight, length }
        })
        .collect()
}

/// Sequence the jobs using the ratio criterion
fn sequence_jobs(jobs: &Vec<Job>) -> Vec<Job> {
    let mut jobs = jobs.clone();

    jobs.sort_by(|a, b| {
        let a_factor = a.weight as f64 / a.length as f64;
        let b_factor = b.weight as f64 / b.length as f64;

        b_factor.partial_cmp(&a_factor).unwrap()
    });

    jobs
}

/// Sequence the jobs using the difference criterion
fn sequence_jobs_wrong(jobs: &Vec<Job>) -> Vec<Job> {
    let mut jobs = jobs.clone();

    jobs.sort_by(|a, b| {
        let a_factor = a.weight as f64 - a.length as f64;
        let b_factor = b.weight as f64 - b.length as f64;

        b_factor.partial_cmp(&a_factor).unwrap()
    });

    jobs
}

/// Objective function is the sum of weighted completion times
fn evaluate_objective_function(jobs: &Vec<Job>) -> usize {
    let mut time_so_far = 0;
    let mut res = 0;

    for job in jobs.iter() {
        time_so_far += job.length;
        res += time_so_far * job.weight;
    }

    res
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_sequencing() {
        let jobs = vec![Job { weight: 3, length: 5 }, Job { weight: 1, length: 2 }];

        let correct = sequence_jobs(&jobs);
        let wrong = sequence_jobs_wrong(&jobs);

        let correct_objective = evaluate_objective_function(&correct);
        let wrong_objective = evaluate_objective_function(&wrong);

        assert_eq!(correct_objective, 22);
        assert_eq!(wrong_objective, 23);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/jobs.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let jobs = parse_jobs(&buffer);

        let correct = sequence_jobs(&jobs);
        let wrong = sequence_jobs_wrong(&jobs);

        let correct_objective = evaluate_objective_function(&correct);
        let wrong_objective = evaluate_objective_function(&wrong);

        dbg!(wrong_objective, correct_objective);
    }
}
