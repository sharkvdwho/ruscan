use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub host: String,
    pub port: u16,
    pub status: String,
    pub service: Option<String>,
    pub version: Option<String>,
}

pub struct ReportGenerator {
    results: Vec<ScanResult>,
}

impl ReportGenerator {
    pub fn new() -> Self {
        ReportGenerator {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: ScanResult) {
        self.results.push(result);
    }

    pub fn generate(&self, format: &str, file_path: Option<&str>) -> Result<(), String> {
        match format.to_lowercase().as_str() {
            "json" => self.generate_json(file_path),
            "csv" => self.generate_csv(file_path),
            "html" => self.generate_html(file_path),
            "text" => self.generate_text(file_path),
            _ => Err(format!("Unsupported output format: {}", format)),
        }
    }

    fn generate_json(&self, file_path: Option<&str>) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.results)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        
        if let Some(path) = file_path {
            let mut file = File::create(path)
                .map_err(|e| format!("Failed to create file {}: {}", path, e))?;
            file.write_all(json.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            println!("Results saved to {}", path);
        } else {
            println!("{}", json);
        }
        Ok(())
    }

    fn generate_csv(&self, file_path: Option<&str>) -> Result<(), String> {
        if let Some(path) = file_path {
            let mut wtr = csv::Writer::from_path(path)
                .map_err(|e| format!("Failed to create CSV file {}: {}", path, e))?;

            wtr.write_record(&["Host", "Port", "Status", "Service", "Version"])
                .map_err(|e| format!("Failed to write CSV header: {}", e))?;

            for result in &self.results {
                wtr.write_record(&[
                    &result.host,
                    &result.port.to_string(),
                    &result.status,
                    result.service.as_deref().unwrap_or(""),
                    result.version.as_deref().unwrap_or(""),
                ]).map_err(|e| format!("Failed to write CSV record: {}", e))?;
            }

            wtr.flush().map_err(|e| format!("Failed to flush CSV: {}", e))?;
            println!("Results saved to {}", path);
        } else {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());

            wtr.write_record(&["Host", "Port", "Status", "Service", "Version"])
                .map_err(|e| format!("Failed to write CSV header: {}", e))?;

            for result in &self.results {
                wtr.write_record(&[
                    &result.host,
                    &result.port.to_string(),
                    &result.status,
                    result.service.as_deref().unwrap_or(""),
                    result.version.as_deref().unwrap_or(""),
                ]).map_err(|e| format!("Failed to write CSV record: {}", e))?;
            }

            wtr.flush().map_err(|e| format!("Failed to flush CSV: {}", e))?;
        }
        Ok(())
    }

    fn generate_html(&self, file_path: Option<&str>) -> Result<(), String> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <title>Ruscan Scan Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #4CAF50; color: white; }
        tr:nth-child(even) { background-color: #f2f2f2; }
        .open { color: green; font-weight: bold; }
        .closed { color: red; }
    </style>
</head>
<body>
    <h1>Ruscan Scan Results</h1>
    <table>
        <tr>
            <th>Host</th>
            <th>Port</th>
            <th>Status</th>
            <th>Service</th>
            <th>Version</th>
        </tr>
"#);

        for result in &self.results {
            let status_class = if result.status == "open" { "open" } else { "closed" };
            html.push_str(&format!(
                r#"        <tr>
            <td>{}</td>
            <td>{}</td>
            <td class="{}">{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>
"#,
                result.host,
                result.port,
                status_class,
                result.status,
                result.service.as_deref().unwrap_or("N/A"),
                result.version.as_deref().unwrap_or("N/A"),
            ));
        }

        html.push_str(r#"    </table>
</body>
</html>"#);

        if let Some(path) = file_path {
            let mut file = File::create(path)
                .map_err(|e| format!("Failed to create HTML file {}: {}", path, e))?;
            file.write_all(html.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            println!("Results saved to {}", path);
        } else {
            println!("{}", html);
        }
        Ok(())
    }

    fn generate_text(&self, file_path: Option<&str>) -> Result<(), String> {
        let mut output = String::new();
        
        for result in &self.results {
            let service_info = if let Some(service) = &result.service {
                if let Some(version) = &result.version {
                    format!(" ({}/{})", service, version)
                } else {
                    format!(" ({})", service)
                }
            } else {
                String::new()
            };
            
            output.push_str(&format!(
                "{}:{} - {} {}\n",
                result.host,
                result.port,
                result.status,
                service_info
            ));
        }

        if let Some(path) = file_path {
            let mut file = File::create(path)
                .map_err(|e| format!("Failed to create file {}: {}", path, e))?;
            file.write_all(output.as_bytes())
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            println!("Results saved to {}", path);
        } else {
            print!("{}", output);
        }
        Ok(())
    }
}

