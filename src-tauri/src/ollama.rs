use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = "llama3:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();

    
    // start the timer
    let mut start = std::time::Instant::now();
    let ollama = Ollama::default();
    // log the time taken to initialize the model
    println!("Time taken to initialize the model: {:?}", start.elapsed());
    
    start = std::time::Instant::now();
    let list = ollama.list_local_models().await?;
    // log the list and erapsed time
    println!("Time taken to list local models: {:?}", start.elapsed());
    println!("model list: {:?}", list);

    start = std::time::Instant::now();
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
        println!("Time taken to generate: {:?}", start.elapsed());
    }
    
    Ok(())
}
