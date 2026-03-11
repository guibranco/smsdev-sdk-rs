$ProjectName = Read-Host -Prompt 'Project name (readable version)'
$ProjectDescription = Read-Host -Prompt 'Brief resume of this project'
$GitHubUsername = Read-Host -Prompt 'GitHub username'
$GitHubRepo = Read-Host -Prompt 'GitHub repository'
$AppVeyorId = Read-Host -Prompt 'AppVeyor project ID'

Remove-Item README.md
Rename-Item -Path ".\README.template.md" -NewName ".\README.md"

Remove-Item Cargo.toml
Rename-Item -Path ".\Cargo.toml.template" -NewName ".\Cargo.toml"

(Get-Content README.md) | ForEach-Object { $_ -replace "{username}", $GitHubUsername } | Set-Content README.md
(Get-Content README.md) | ForEach-Object { $_ -replace "{repo}", $GitHubRepo } | Set-Content README.md
(Get-Content README.md) | ForEach-Object { $_ -replace "{appVeyorId}", $AppVeyorId } | Set-Content README.md
(Get-Content README.md) | ForEach-Object { $_ -replace "API Client Boilerplate", $ProjectName } | Set-Content README.md
(Get-Content README.md) | ForEach-Object { $_ -replace "{Project Description}", $ProjectDescription } | Set-Content README.md
(Get-Content README.md) | ForEach-Object { $_ -replace "https://project.name.com/", $DocumentationWebsite } | Set-Content README.md

(Get-Content .wakatime-project) | ForEach-Object { $_ -replace "API Client Boilerplate .NET", "$ProjectName SDK .NET" } | Set-Content .wakatime-project

(Get-Content _config.yml) | ForEach-Object { $_ -replace "API Client Boilerplate", $ProjectName } | Set-Content _config.yml
(Get-Content _config.yml) | ForEach-Object { $_ -replace "A template repository for RUST API clients projects.", $ProjectDescription } | Set-Content _config.yml
(Get-Content _config.yml) | ForEach-Object { $_ -replace "GuilhermeStracini/apiclient-boilerplate-rs", "$GitHubUsername/$GitHubRepo" } | Set-Content _config.yml

Remove-Item initial-setup.bat
Remove-Item initial-setup.ps1
Remove-Item initial-setup.sh
