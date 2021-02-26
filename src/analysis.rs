use crate::models::Course;

pub fn print_analysis(courses: Vec<Course>) {
    let batches = {
        let mut batches = vec!();
        for course in courses.iter() {
            for batch in course.batches.iter() {
                batches.push((course.course_code, batch))
            }
        }
        batches.sort_unstable_by_key(|x| (x.1.evaluation_time, x.0));
        batches
    };
    if batches.len() == 0 {
        println!("Nothing left!");
        return;
    }
    let urgent = {
        let mut chunks = batches.array_chunks::<3>();
        chunks.next().map(|x| x.as_slice()).unwrap_or(chunks.remainder())
    };

    println!("Task view:");
    for batch in urgent.iter() {
        println!("{} @ {}: {}", batch.0, batch.1.evaluation_time, batch.1.evaluation_desc);
        for desc in batch.1.learning_descs.iter().take(3) {
            println!("- {}", desc);
        }
        if let Some(more) = batch.1.learning_descs.len().checked_sub(3 + 1) {
            println!("- ... and {} more", more + 1);
        }
    }

    println!();
    println!("Urgency view:");
    let mut items = 0;
    for batch in urgent {
        items += batch.1.learning_descs.len();
        println!("{} task(s) left before {} @ {}: {}",
                 items,
                 batch.0,
                 batch.1.evaluation_time,
                 batch.1.evaluation_desc,
        );
    }
    if let Some(more) = batches.len().checked_sub(3 + 1) {
        println!("... and {} more evaluation(s)", more + 1);
    }
}
