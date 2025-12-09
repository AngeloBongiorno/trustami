use crate::utils::TfIdf;

pub fn present_results_cli(mut results: Vec<TfIdf>) {
    results.sort_by(|a, b| a.score.total_cmp(&b.score));

    results.reverse();

    for result in results {
        println!("{}\n", result);
    }
}
