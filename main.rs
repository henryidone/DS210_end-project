mod extremefeatures;
mod inputsandload;
mod graphs; //take in modules which seperate by graphs, extremefeatures, and the inputs and loads

use extremefeatures::{findextremerisk, generateriskpercent};
use inputsandload::{load, getinput, numberinput, childrencategories};
use graphs::{createcreditgraph, calculateweights};
use std::error::Error;

//main function executes all attributes, has inputs and outputs and does what the user says
fn main() -> Result<(), Box<dyn Error>> {
    let clients = load("application_record.csv")?;
    let creditgraph = createcreditgraph("credit_record.csv")?;
    let clientsatrisk = creditgraph.bfs();
    let weights = calculateweights(&clients, &clientsatrisk);
    let mut sortedweights: Vec<_> = weights.iter().collect();
    sortedweights.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let showinfluential = getinput("Would you like to know the most influential attributes affecting credit risk? (Yes or No): ");
    if showinfluential == "Yes" {
        println!("\nThe most influential attributes are:");
        for ((feature, value), weight) in sortedweights.iter().take(5) {
            println!("{} = {}: {:.2}%", feature, value, weight);
        }
    }
    let showhighest = getinput("Would you like to know the attributes with the highest possible credit risk? (Yes or No): ");
    if showhighest == "Yes" {
        let greatestrisk = findextremerisk(&weights, true);
        let greatestriskpercent = generateriskpercent(&greatestrisk, &weights);
        println!("\nAttributes with the highest possible credit risk:");
        for (feature, value) in &greatestrisk {
            println!("{}: {}", feature, value);
        }
        println!("Credit Risk Percentage: {:.2}%", greatestriskpercent);
    }
    let generaterisk = getinput("Would you like to calculate your own credit risk? (Yes or No): ");
    if generaterisk == "Yes" {
        let gender = getinput("Biological gender (Male or Female): ");
        let property = getinput("Property Ownership (Yes or No): ");
        let children = childrencategories(numberinput("Number of Children: "));
        let income = numberinput("Annual Income (no commas): ");
        let married = getinput("Marital Status (Single/Married/Other): ");
        let features = vec![
            ("Gender".to_string(), gender),
            ("Property Ownership".to_string(), property),
            ("Number of Children".to_string(), children),
            ("Annual Income".to_string(), income.to_string()),
            ("Marital Status".to_string(), married),
        ];
        let riskpercentage = generateriskpercent(&features, &weights);
        println!("\nYour predicted credit risk percentage is {:.2}%.", riskpercentage);
    }
    Ok(()) // a result so if gets to this okay function, it means it works
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardcoded_credit_risk_calculation() {
        // Ensure dataset files are accessible
        assert!(
            std::path::Path::new("application_record.csv").exists(),
            "application_record.csv is missing!"
        );
        assert!(
            std::path::Path::new("credit_record.csv").exists(),
            "credit_record.csv is missing!"
        );

        // Load the application and credit record datasets
        let clients = load("application_record.csv").expect("Failed to load application record");
        let credit_graph = createcreditgraph("credit_record.csv").expect("Failed to create credit graph");

        // Calculate risky clients
        let clients_at_risk = credit_graph.bfs();
        assert!(
            !clients_at_risk.is_empty(),
            "No clients at risk were found. Check your dataset or logic."
        );

        // Calculate weights
        let weights = calculateweights(&clients, &clients_at_risk);
        assert!(
            !weights.is_empty(),
            "No weights were calculated. Check your logic and dataset."
        );

        // Define hardcoded test inputs
        let test_features = vec![
            ("Gender".to_string(), "Male".to_string()),
            ("Property Ownership".to_string(), "Yes".to_string()),
            ("Number of Children".to_string(), "No children".to_string()),
            ("Annual Income".to_string(), "<20,000".to_string()),
            ("Marital Status".to_string(), "Married".to_string()),
        ];

        // Calculate risk percentage
        let risk_percentage = generateriskpercent(&test_features, &weights);
        println!("Calculated risk percentage: {:.2}%", risk_percentage);

        // Assert correctness
        assert!(
            (risk_percentage - 27.57_f64).abs() < 0.01,
            "Expected risk percentage to be ~27.57%, but got {:.2}%",
            risk_percentage
        );
    }
}
