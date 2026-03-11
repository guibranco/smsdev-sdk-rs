# 🚀 API Client Boilerplate Template for Rust 🦀  

🏗️ **A streamlined boilerplate template to kickstart your Rust API client wrappers** (also known as **SDKs**).  

---

## 🌟 About  

This repository is a **template** to jumpstart your Rust development for **API client wrappers**.  
It provides a solid foundation with preconfigured project structure, CI integration, and helper scripts.  

🔧 **What's included?**  
- 📦 A ready-to-use **Cargo.toml** with project placeholders.  
- 🛠️ Scripts for quick **automated setup** (Windows & Linux/Mac).  
- ✅ [AppVeyor](https://www.appveyor.com/) CI pipeline configuration.  
   > 💡 **AppVeyor is free for open-source projects.** For private repositories, you can use alternatives like **GitHub Actions**, **CircleCI**, or **Azure DevOps**.  
- 📄 Clean project layout with room for source code and documentation.  

---

## 🚀 Getting Started  

### ⚡ Automated Setup  

> 🐧 **Now supported for Windows, Linux, and Mac environments!**  

1. Click the green **"Use this template"** button at the top.  
2. Name your new repository and clone it to your machine.  
3. Run the setup script based on your OS:  
   - 🪟 On Windows: Execute `initial-setup.bat` or `initial-setup.ps1` 
   - 🐧 On Linux/Mac: Execute `initial-setup.sh`
4. Follow the on-screen instructions and get started in seconds! 🚀  

---

### 🛠️ Manual Setup  

Would you prefer to do it step-by-step? No problem!  

1. Click the green **"Use this template"** button.  
2. Clone your newly created repository to your machine.  
3. Clean up:  
   - Delete this file (`README.md`) and rename `README.template.md` to `README.md`.  
4. Update the placeholders:  
   - **AppVeyor CI**: Edit `appveyor.yml` with your secure tokens or configure a different CI tool (GitHub Actions, CircleCI, Azure Pipelines).  
   - **Cargo.toml**: Update the package name, version, and other metadata.  
   - **Badges**: Fix all *{username}/{repo}* paths in `README.md`.  
5. Add usage instructions for your API client in `README.md`.  
6. Update `_config.yml` for GitHub Pages documentation (if needed).  

---

## 📂 Project Structure  

```
📦 your-project-name  
├── Cargo.toml               # Project metadata and dependencies  
├── README.md                # Your main documentation  
├── appveyor.yml             # AppVeyor CI configuration  
├── Src/                     # Place your API client code here  
├── Tests/                   # Add unit and integration tests here  
└── _config.yml              # Optional GitHub Pages configuration  
```  

---

## 🛡️ License  

📄 Licensed under the **MIT License**.  
This means you are free to use, modify, and distribute the code with attribution.  

🔗 [View LICENSE](https://github.com/guibranco/apiclient-boilerplate-rs/blob/main/LICENSE) | [Learn about MIT](http://opensource.org/licenses/MIT)  

> 🔒 Feel free to replace this with any license that best suits your project!  

---

## 🎉 Contribute & Customize  

💬 Got ideas? Found an issue? Open an issue or contribute with a pull request!  
Don’t forget to ⭐ **star** this repository if it helped save you time. 🚀  

---

🦀 **Happy Coding in Rust!** 🎯 
