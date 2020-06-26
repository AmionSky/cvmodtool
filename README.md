# Code Vein Modding Tool
Code Vein modding helper tool for managing UE4 projects.

Download at [Releases](https://github.com/AmionSky/cvmodtool/releases). Requires [Visual C++ 2019](https://support.microsoft.com/en-us/help/2977003/the-latest-supported-visual-c-downloads) installed.

![showace](https://github.com/AmionSky/cvmodtool/blob/master/readmeres/cvmodtool.jpg?raw=true)

## Functions
- Create new UE4 projects with correct configuration, helper assets and build scripts.
- Build/Cook unreal project content.
- Package the necessary files only into a .pak file.
- Install/Copy the .pak into your mods folder.

## Mod Config file
By default named `cvmod.toml`. Make sure to update the `includes` if necessary.
```toml
pakname = 'TestProject_P' # Name of the .pak file to create
project = 'TestProject' # Name of the Unreal project
packagedir = 'Package' # Directory to create the package in
includes = ['Maps', 'ModResources'] # Folders/files to include in the final package
credits = ['Amion'] # Credits of the included modules
```

## Notes
The tool needs a `config.toml` inside the `resources` directory with contents:
```toml
engine = "Path\\To\\UE_4.18" # This folder should contain the "Engine" directory
moddir = "Path\\To\\CodeVein\\Content\\Paks\\~mods" # This folder needs to exist
```
To make the `build` command work properly, you need to build the Visual Studio project first. Unreal automatically does that if you open the project first. *(Only if you included C++ code in your project)*

## Commands
*For more information on all the commands just use the `--help` option argument.*

### **Create**
    cvmodtool.exe create [OPTIONS] <ProjectName>
Creates a new Unreal project in the current directory inside a folder with an identical name of the project name. Automatically creates mod config (`cvmod.toml`) and a `build-and-install.bat` for quick iteration.
|Option|Usage|Description|
|-|-|-|
|profile|`-p <ProfileName>`<br>`--profile <ProfileName>`|Specify the profile to use for selecting the modules for install. Profiles are defined at `resources/profiles.toml`.<br>[default: default]
|modules|`-m <ModuleNames...>`<br>`--modules <ModuleNames...>`|Additional modules to install.

Examples:
```
cvmodtool.exe create TestProject
```
```
cvmodtool.exe create TestProject --profile empty --modules base gitsupport
```
---

### **Build**
    cvmodtool.exe build [OPTIONS]
Cooks the project's content. Requires the mod config (`cvmod.toml`) in the project directory.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]
---

### **Package**
    cvmodtool.exe package [OPTIONS]
Packages the project into a .pak file based on configuration found inside the mod config (`cvmod.toml`). Requires the mod config in the project directory.

Make sure to update the mod config's `includes` field with the content folders to include in the pak.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]
|no-copy|`--no-copy`|Don't copy the latest cooked content. Only run UnrealPak.
---

### **Install**
    cvmodtool.exe install [OPTIONS] <optional-pakfile>
Copies the .pak file to the specified mods folder (Usually it's the Code Vein ~mods folder). Requires the mod config (`cvmod.toml`) in the project directory.

If the `pakfile` is defined the mod config won't be used, instead it will just copy that pak file into the mods folder.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]
---
