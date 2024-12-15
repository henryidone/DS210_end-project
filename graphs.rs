use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub struct Graph {
    pub adjlist: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self { //initializes adjacency list
        Graph {
            adjlist: HashMap::new(),
        }
    }

    pub fn addedge(&mut self, id: &str, status: String) { //adds edge to adjacency list
        self.adjlist.entry(id.to_string()).or_insert_with(Vec::new).push(status);
    }

    pub fn bfs(&self) -> HashSet<String> { //bfs function that adds to risky if anything but C or X to risky. C represents paid on time and X represents no credit taken
        let mut risky = HashSet::new();
        for (id, statuses) in &self.adjlist {
            if statuses.iter().any(|status| status != "C" && status != "X") {
                risky.insert(id.clone());
            }
        }
        risky
    }
}

pub fn createcreditgraph(path: &str) -> Result<Graph, Box<dyn Error>> { //create creditgraph using adjacency list
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

pub fn calculateweights( //compute the weights
    clients: &[crate::inputsandload::Clientrecord],
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
                ("Annual Income", client.income.to_string()),
                ("Marital Status", client.married.clone()),
            ];
            for (key, value) in features {
                *raw.entry((key.to_string(), value)).or_insert(0) += 1; //use .or_insert to put in value if there is not already 
                numatrisk += 1;
            }
        }
    }
    
    let mut weights = HashMap::new();
    for ((feature, value), count) in raw {
        let relativeweight = (count as f64 / numatrisk as f64) * 100.0; //use the count/total equation and make it a percentage to get the weight
        weights.insert((feature, value), relativeweight); //stores the weight
    }
    weights
}
