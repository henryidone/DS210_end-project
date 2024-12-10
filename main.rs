use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
struct Clientrecord {
    id: String,
    gender: String,
    property: String,
    children: String,
    income: String,
    married: String,
}

struct Graph {
    adjlist: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjlist: HashMap::new(),
        }
    }

    fn addedge(&mut self, id: &str, status: String) {
        self.adjlist
            .entry(id.to_string())
            .or_insert_with(Vec::new)
            .push(status);
    }

    fn bfs(&self) -> HashSet<String> {
        let mut risky = HashSet::new();
        for (id, statuses) in &self.adjlist {
            if statuses.iter().any(|status| status != "C" && status != "X") {
                risky.insert(id.clone());
            }
        }
        risky
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let clients = load("application_record.csv")?;
    let creditgraph = createcreditgraph("credit_record.csv")?;
    let clientsatrisk = creditgraph.bfs();
    let weights = calculateweights(&clients, &clientsatrisk);

    let mut sortedweights: Vec<_> = weights.iter().collect();
    sortedweights.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    let showinfluential = getinput("Would you like to know the most influential attributes affecting credit risk? (Yes/No): ");
    if showinfluential.eq_ignore_ascii_case("yes") {
        println!("\nThe most influential attributes are:");
        for ((feature, value), weight) in sortedweights.iter().take(5) {
            println!("{} - {}: {:.2}%", feature, value, weight);
        }
    }

    let showhighest = getinput("Would you like to know the attributes with the highest possible credit risk? (Yes/No): ");
    if showhighest.eq_ignore_ascii_case("yes") {
        let greatestrisk = findextremerisk(&weights, true);
        let greatestriskpercent = generateriskpercent(&greatestrisk, &weights);
        println!("\nAttributes with the highest possible credit risk:");
        for (feature, value) in &greatestrisk {
            println!("{}: {}", feature, value);
        }
        println!("Credit Risk Percentage: {:.2}%", greatestriskpercent);
    }

    let generaterisk = getinput("Would you like to calculate your own credit risk? (Yes/No): ");
    if generaterisk.eq_ignore_ascii_case("yes") {
        let gender = getinput("Biological gender (Male/Female): ");
        let property = getinput("Property Ownership (Yes/No): ");
        let children = childrencategories(numberinput("Number of Children: "));
        let income = incomecategories(numberinput("Annual Income (no commas): "));
        let married = getinput("Marital Status (Single/Married/Other): ");
        let features = vec![
            ("Gender".to_string(), gender),
            ("Property Ownership".to_string(), property),
            ("Number of Children".to_string(), children),
            ("Annual Income".to_string(), income),
            ("Marital Status".to_string(), married),
        ];
        let riskpercentage = generateriskpercent(&features, &weights);
        println!("\nYour predicted credit risk percentage is {:.2}%.", riskpercentage);
    }

    Ok(())
}

fn load(path: &str) -> Result<Vec<Clientrecord>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let mut records = Vec::new();
    let lines = data.lines().skip(1);
    for line in lines {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 9 {
            records.push(Clientrecord {
                id: fields[0].to_string(),
                gender: fields[1].to_string(),
                property: fields[3].to_string(),
                children: childrencategories(fields[4].parse().unwrap_or(0)),
                income: incomecategories(fields[5].parse().unwrap_or(0)),
                married: fields[8].to_string(),
            });
        }
    }
    Ok(records)
}

fn createcreditgraph(path: &str) -> Result<Graph, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let mut graph = Graph::new();
    let lines = data.lines().skip(1);
    for line in lines {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3 {
            let id = fields[0];
            let status = fields[2].to_string();
            graph.addedge(id, status);
        }
    }
    Ok(graph)
}

fn calculateweights(
    clients: &[Clientrecord],
    clientsatrisk: &HashSet<String>,
) -> HashMap<(String, String), f64> {
    let mut raw = HashMap::new();
    let mut numatrisk = 0;

    for client in clients {
        if clientsatrisk.contains(&client.id) {
            let features = vec![
                ("Gender", client.gender.clone()),
                ("Property Ownership", client.property.clone()),
                ("Number of Children", client.children.clone()),
                ("Annual Income", client.income.clone()),
                ("Marital Status", client.married.clone()),
            ];
            for (key, value) in features {
                *raw.entry((key.to_string(), value)).or_insert(0) += 1;
                numatrisk += 1;
            }
        }
    }

    let mut weights = HashMap::new();
    for ((feature, value), count) in raw {
        let relativeweight = (count as f64 / numatrisk as f64) * 100.0;
        weights.insert((feature, value), relativeweight);
    }

    weights
}

fn childrencategories(children: u32) -> String {
    match children {
        0 => "No children".to_string(),
        1..=2 => "1-2 children".to_string(),
        3..=4 => "3-4 children".to_string(),
        _ => "5+ children".to_string(),
    }
}

fn incomecategories(income: u32) -> String {
    match income {
        0..=20_000 => "<20,000".to_string(),
        20_001..=40_000 => "20,001-40,000".to_string(),
        40_001..=70_000 => "40,001-70,000".to_string(),
        70_001..=100_000 => "70,001-100,000".to_string(),
        100_001..=150_000 => "100,001-150,000".to_string(),
        150_001..=300_000 => "150,001-300,000".to_string(),
        300_001..=1_000_000 => "300,001-1,000_000".to_string(),
        _ => ">1,000_000".to_string(),
    }
}

fn generateriskpercent(
    features: &[(String, String)],
    weights: &HashMap<(String, String), f64>,
) -> f64 {
    let mut riskscore = 0.0;
    for (feature, value) in features {
        if let Some(weight) = weights.get(&(feature.clone(), value.clone())) {
            riskscore += weight;
        }
    }
    riskscore.min(100.0)
}

fn getinput(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn numberinput(prompt: &str) -> u32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number!")
}

fn findextremerisk(
    weights: &HashMap<(String, String), f64>,
    highest: bool,
) -> Vec<(String, String)> {
    let mut extreme = Vec::new();
    let mut totalrisk = if highest { 0.0 } else { 100.0 };
    let genderoptions = vec!["Male", "Female"];
    let propertyoptions = vec!["Yes", "No"];
    let childrenoptions = vec!["No children", "1-2 children", "3-4 children", "5+ children"];
    let incomeoptions = vec![
        "<20,000",
        "20,001-40,000",
        "40,001-70,000",
        "70,001-100,000",
        "100,001-150,000",
        "150,001-300,000",
        "300,001-1,000,000",
        ">1,000,000",
    ];
    let maritaloptions = vec!["Single", "Married", "Other"];
    for gender in &genderoptions {
        for property in &propertyoptions {
            for children in &childrenoptions {
                for income in &incomeoptions {
                    for married in &maritaloptions {
                        let features = vec![
                            ("Gender".to_string(), gender.to_string()),
                            ("Property Ownership".to_string(), property.to_string()),
                            ("Number of Children".to_string(), children.to_string()),
                            ("Annual Income".to_string(), income.to_string()),
                            ("Marital Status".to_string(), married.to_string()),
                        ];
                        let risk = generateriskpercent(&features, weights);
                        if (highest && risk > totalrisk) || (!highest && risk < totalrisk) {
                            totalrisk = risk;
                            extreme = features.clone();
                        }
                    }
                }
            }
        }
    }
    extreme
}
