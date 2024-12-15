mod extremefeatures;
mod inputsandload;
mod graphs; //take in modules which seperate by graphs, extremefeatures, and the inputs and loads

use extremefeatures::{findextremerisk, generateriskpercent};
use inputsandload::{load, getinput, numberinput, childrencategories};
use graphs::{createcreditgraph, calculateweights};
use std::error::Error;

//main function executes all attributes, has inputs and outputs and does what the user says
fn main() -> Result<(), Box<dyn Error>> { //because all outputs print statement, output is a result
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
    let showhighest = getinput("Would you like to know an example of one of the attributes with the highest possible credit risk (there is a tie between several)? (Yes or No): ");
    if showhighest == "Yes" {
        let greatestrisk = findextremerisk(&weights);
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
    use std::collections::HashMap; 
    use std::fs; 


    #[test]
    fn generateriskpercenttest() { //Checks that risk calculation works
        let mut weights = HashMap::new();
        weights.insert(("Gender".to_string(), "Male".to_string()), 10.0);
        weights.insert(("Property Ownership".to_string(), "Yes".to_string()), 15.0);
        let features = vec![
            ("Gender".to_string(), "Male".to_string()),
            ("Property Ownership".to_string(), "Yes".to_string()),
        ];
        assert_eq!(extremefeatures::generateriskpercent(&features, &weights), 25.0);
    }

    
    #[test]
    fn testbfs() { //checks bfs to prove that it identifies nodes in the right way
        let mut graph = graphs::Graph::new();
        graph.addedge("1", "1".to_string());
        graph.addedge("2", "C".to_string());
        let risky = graph.bfs();
        assert!(risky.contains("1"));
        assert!(!risky.contains("2"));
    }

    #[test]
    fn testcalculateweights() { // makes sure weights are calculated properly
        let clients = vec![
            inputsandload::Clientrecord {
                id: "1".to_string(),
                gender: "Male".to_string(),
                property: "Yes".to_string(),
                children: "No children".to_string(),
                income: 50000,
                married: "Single".to_string(),
            },
            inputsandload::Clientrecord {
                id: "2".to_string(),
                gender: "Female".to_string(),
                property: "No".to_string(),
                children: "1-2 children".to_string(),
                income: 40000,
                married: "Married".to_string(),
            },
        ];
        let clientsatrisk = ["1".to_string()].iter().cloned().collect();
        let weights = graphs::calculateweights(&clients, &clientsatrisk);
        assert!(weights.contains_key(&("Gender".to_string(), "Male".to_string())));
    }

    #[test]
    fn testcreatecreditgraph() { //checks if a graph is created correctly from a test file
        let testfile = "test_credit.csv";
        let data = "ID,Date,Status\n\
                    1,2020-01,C\n\
                    2,2020-01,1";
        fs::write(testfile, data).unwrap();
        let graph = graphs::createcreditgraph(testfile).unwrap();
        assert!(graph.adjlist.contains_key("1"));
        assert!(graph.adjlist.contains_key("2"));
        fs::remove_file(testfile).unwrap();
    }
}