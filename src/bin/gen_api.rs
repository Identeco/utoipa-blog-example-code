use identeco_utoipa_example::ApiDocs;

fn main() {
    let content = ApiDocs::generate();
    std::fs::write("openapi.yml", content).unwrap();
}
