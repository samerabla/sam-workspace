use dioxus::prelude::*;
use dioxus_ssr::render_element;
use headless_chrome::{Browser, LaunchOptions};
use std::process::Command;
use std::str;

pub fn html_to_pdf(html: &str, output_file: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Launch a headless Chrome instance
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;

    // Load the HTML content
    tab.navigate_to(format!("data:text/html;charset=utf-8,{}",html).as_str())?;

    // Wait for the page to load
    tab.wait_until_navigated()?;

    // Save the page as a PDF
    let pdf_data = tab.print_to_pdf(None)?;
    std::fs::write(output_file, pdf_data)?;

    Ok(())
}

pub fn rsx_to_html(element: Element) -> String {
    // Render the component to a styled HTML string
    render_element(element)
}

pub fn get_default_printer_name() -> Option<String> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance Win32_Printer | Where-Object { $_.Default -eq $true }).Name"
        ])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Convert the output to a String
                let printer_name = str::from_utf8(&output.stdout).unwrap_or("").trim().to_string();
                if !printer_name.is_empty() {
                    return Some(printer_name);
                }
            } else {
                eprintln!(
                    "PowerShell returned an error: {}",
                    str::from_utf8(&output.stderr).unwrap_or("Unknown error")
                );
            }
        }
        Err(e) => eprintln!("Failed to execute PowerShell command: {}", e),
    }
    None
}

pub fn print_pdf_with_pdftoprinter(pdf_path: &str, printer_name: &str) {
    let command = Command::new("./PDFtoPrinter.exe")
        .args([pdf_path, printer_name])
        .spawn();

    match command {
        Ok(_) => println!("Print job sent successfully."),
        Err(e) => eprintln!("Failed to print PDF with PDFtoPrinter: \n   {}", e),
    }
}