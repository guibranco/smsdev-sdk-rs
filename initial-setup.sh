#!/bin/bash

# Prompt the user for input
read -p -r 'Project name (readable version): ' ProjectName
read -p -r 'Project keyword (Cargo.toml): ' ProjectKeyword
read -p -r 'Brief resume of this project: ' ProjectDescription
read -p -r 'GitHub username: ' GitHubUsername
read -p -r 'GitHub repository: ' GitHubRepo
read -p -r 'AppVeyor project ID (badge): ' AppVeyorId
read -p -r 'API documentation URL: ' DocumentationWebsite
read -p -r 'Author name: ' AuthorName
read -p -r 'Author e-mail: ' AuthorEmail

# Remove and rename files
rm README.md
mv "README.template.md" "README.md"

rm Cargo.toml
mv "Cargo.toml.template" "Cargo.toml"

rm sonar-project.properties
mv "sonar-project.properties.template" "sonar-project.properties"

# Replace placeholders in README.md
sed -i "s/{username}/$GitHubUsername/g" README.md
sed -i "s/{repo}/$GitHubRepo/g" README.md
sed -i "s/{appVeyorId}/$AppVeyorId/g" README.md
sed -i "s/{Project Name}/$ProjectName/g" README.md
sed -i "s/{Project Description}/$ProjectDescription/g" README.md
sed -i "s/https:\/\/project.name.com\//$DocumentationWebsite/" README.md

# Replace placeholders in .wakatime-project
sed -i "s/API Client Boilerplate .NET/$ProjectName SDK .NET/" .wakatime-project

# Replace placeholders in _config.yml
sed -i "s/API Client Boilerplate/$ProjectName/" _config.yml
sed -i "s/A template repository for .NET API clients projects./$ProjectDescription/" _config.yml
sed -i "s/GuilhermeStracini\/apiclient-boilerplate-dotnet/$GitHubUsername\/$GitHubRepo/" _config.yml

# Replace placeholders in Cargo.toml
sed -i "s/{username}/$GitHubUsername/g" Cargo.toml
sed -i "s/{repository}/$GitHubRepo/g" Cargo.toml
sed -i "s/{authorName}/$AuthorName/g" Cargo.toml
sed -i "s/{authoremail}/$AuthorEmail/g" Cargo.toml
sed -i "s/{keyword}/$ProjectKeyword/g" Cargo.toml

# Replace placeholders in sonar-project.properties
sed -i "s/{username}/$GitHubUsername/g" sonar-project.properties
sed -i "s/{repository}/$GitHubRepo/g" sonar-project.properties

# Remove initial-setup files
rm initial-setup.bat
rm initial-setup.ps1
rm initial-setup.sh
