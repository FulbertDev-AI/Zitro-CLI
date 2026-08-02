# ZITRO CLI

ZITRO CLI is a command-line tool for auditing the carbon footprint of web applications. It analyzes web pages and calculates their environmental impact based on the EcoIndex methodology, enriched with country-specific energy mix data.

## Features

- **Carbon Footprint Analysis**: Measures the environmental impact of web applications
- **EcoIndex Score**: Calculates a score from 0 to 100 based on page weight, number of requests, and DOM size
- **Country-Specific Energy Mix**: Adjusts calculations based on the energy mix of different countries
- **Production Readiness Assessment**: Provides Go/No-Go recommendations for deployment
- **Detailed Resource Analysis**: Identifies the heaviest resources and provides optimization recommendations
- **Multi-Platform**: Available for Windows, macOS, and Linux

## Installation

### Global Installation (Recommended)

```bash
npm install -g zitro-cli

Requirements

Node.js (v12 or higher)
Internet connection to download binaries
Usage
Basic Scan : zitro scan https://example.com

Scan with Country-Specific Energy Mix : zitro scan https://example.com -c TG

Or with the long flag: zitro scan https://example.com --country TG

Available Commands
# Show help
zitro --help

# Show version
zitro --version

# Scan a web application
zitro scan <URL> [options]

Command Options
scan
Analyzes the carbon footprint of a web application.
Arguments:
URL - The URL of the application to audit (e.g., http://localhost:3000)
Options:
-c, --country <CODE> - ISO country code for energy mix (e.g., TG, CI, SN, FR, US)

Supported Countries
ZITRO CLI supports the following country codes for energy mix calculations:
TG (Togo): 400 gCO2eq/kWh
CI (Côte d'Ivoire): 350 gCO2eq/kWh
SN (Senegal): 450 gCO2eq/kWh
FR (France): 56 gCO2eq/kWh
US (United States): 380 gCO2eq/kWh
And many more...
If no country is specified, the world average (475 gCO2eq/kWh) is used.
Examples
Scan a local development server : zitro scan http://localhost:3000

Scan with Togo's energy mix : zitro scan http://localhost:3000 -c TG

Scan a production website  : zitro scan https://myapp.com -c FR

OUTPUT

The tool provides:
Real-time metrics display in the terminal
EcoIndex score and grade
Estimated carbon emissions in grams of CO2eq
Production deployment recommendation

Integration
ZITRO CLI is designed to be used in CI/CD pipelines for continuous environmental monitoring of web applications.
Example CI/CD Usage
# In your CI/CD pipeline
zitro scan https://staging.myapp.com -c FR

# Fail the build if grade is below B
if zitro scan https://staging.myapp.com | grep -q "DECONSEILLE"; then
  exit 1
fi

Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
License
MIT License - see LICENSE file for details

Repository : https://github.com/FulbertDev-AI/Zitro-CLI.git