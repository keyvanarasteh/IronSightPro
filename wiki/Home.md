Welcome to the official **IronSight EDR Wiki**. 

IronSight is a modular Endpoint Detection & Response system built with Rust 2024. This wiki serves as the primary technical documentation hub for developers, security engineers, and researchers wanting to understand the inner workings of the EDR pipeline.

## Getting Started

If you simply want to test IronSight, we recommend starting with the `README.md` in the main repository to see the installation and Docker setup instructions. If you want to dive deeper into the code, check out the following pages:

### Core Documentation

- **[Architecture Deep Dive](Architecture)**: Learn how the 10 separate IronSight crates work together to create the monitoring pipeline from data collection to active response.
- **[Detection Engine](Detection-Engine)**: Understand the heuristic scoring algorithm, signal processing, and threat evaluation mechanism.
- **[Automated Response](Automated-Response)**: Read about the incident reaction protocols (`Suspend -> Dump -> Kill`) and how memory forensics is integrated into the workflow.
- **[Kernel Monitoring (WIP)](Kernel-Monitoring)**: (Coming soon) Explore the eBPF tracepoint abstractions on Linux and ETW consumer logic on Windows.

## Contributing

The IronSight project is always looking for new contributors. If you are interested in memory safety, low-level OS APIs, or security auditing logic, check out our [Contributing Guide](https://github.com/keyvanarasteh/IronSight/blob/main/CONTRIBUTING.md) to see how you can help.
