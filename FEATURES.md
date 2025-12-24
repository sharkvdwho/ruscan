# Ruscan Features

##  Implemented Features

### Target Selection
- **Single IP Address Scanning** (`-i, --ip`)
  - Supports both IPv4 and IPv6 addresses
  - Example: `ruscan ps -i 127.0.0.1`

- **Domain Name Resolution** (`-d, --domain`)
  - Resolves domain names to IP addresses
  - Supports multiple IP addresses per domain
  - Example: `ruscan ps -d example.com`

- **CIDR Notation Scanning** (`-c, --cidr`)
  - Network scanning using CIDR notation
  - Supports both IPv4 and IPv6 CIDR ranges
  - Example: `ruscan ps -c 192.168.1.0/24`

- **IP Range Scanning** (`-R, --ip-range`)
  - Scan IP address ranges (start-end format)
  - Supports both IPv4 and IPv6 ranges
  - Example: `ruscan ps -R 192.168.1.1-192.168.1.254`

- **IP Address Mask Handling**
  - Implemented via CIDR notation support
  - Allows subnet mask-based scanning

### Port Selection
- **Single Port Scanning** (`-p, --port`)
  - Scan a single port on target(s)
  - Example: `ruscan ps -i 127.0.0.1 -p 443`

- **Port Range Scanning** (`-r, --range`)
  - Scan a range of ports
  - Example: `ruscan ps -i 127.0.0.1 -r 1-65535`

- **Port List Scanning** (`-l, --list`)
  - Scan multiple specific ports (comma-separated)
  - Example: `ruscan ps -i 127.0.0.1 -l 21,80,443`

- **Default Common Ports**
  - Automatically scans common ports if none specified
  - Default ports: 21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995, 3306, 3389, 5432, 8080

### IPv6 Support
- Full IPv6 address support
- IPv6 CIDR notation
- IPv6 IP range scanning
- IPv6 domain resolution

### Output & Reporting
- **Multiple Output Formats** (`-o, --output`)
  - JSON format (`-o json`)
  - CSV format (`-o csv`)
  - HTML format (`-o html`)
  - Text format (`-o text`, default)

- **File Output** (`-f, --file`)
  - Save scan results to files
  - Supports all output formats
  - Example: `ruscan ps -i 127.0.0.1 -f results.json`

- **Smart Output Filtering**
  - Shows only open ports when scanning multiple IPs
  - Shows only open ports when scanning large port ranges (>100 ports)
  - Shows all ports (open/closed) for small single-IP scans

### Service Detection
- **Service & Version Detection** (`-s, --service-detection`)
  - Banner grabbing for common services
  - Service identification (HTTP, SSH, FTP, SMTP, Telnet)
  - Version extraction for HTTP and SSH
  - Example: `ruscan ps -i 127.0.0.1 -s`

### Performance & Rate Control
- **Adaptive Rate Control** (`-t, --threads`)
  - Configurable maximum concurrent connections
  - Default: 1000 threads
  - Prevents overwhelming target networks
  - Example: `ruscan ps -i 127.0.0.1 -t 100`

- **Multi-threaded Scanning**
  - Parallel port scanning for improved performance
  - Thread pool management with semaphore

### User Experience
- **Elapsed Time Tracking**
  - Displays scan completion time with millisecond precision (3 decimal places)
  - Format: `[+] Scan completed in X.XXX seconds`

- **Progress Indicators**
  - Real-time scan progress messages
  - Color-coded output (green for open, red for closed)
  - Scan statistics (hosts, ports, total connections)

- **Error Handling**
  - Clear error messages for invalid inputs
  - Domain resolution error handling
  - Invalid IP/CIDR/range validation

## 🚧 Planned Features (Not Yet Implemented)

### Network Protocols
- **Smoltcp for TCP Streams**
  - Low-level TCP stream handling
  - Custom packet crafting capabilities

### Advanced Scanning
- **Advanced Scan Techniques**
  - SYN scan (stealth scan)
  - UDP scanning
  - FIN scan, Xmas scan, NULL scan
  - ACK scan for firewall detection
  - Custom scan timing options

### System Detection
- **OS Fingerprinting**
  - Operating system detection
  - TCP/IP stack fingerprinting
  - Service version detection improvements

### Architecture
- **Plugin-based Architecture**
  - Extensible plugin system
  - Custom scan modules
  - Third-party plugin support

