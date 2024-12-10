use std::collections::HashMap;

pub fn findextremerisk(
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

pub fn generateriskpercent(
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
