# Code Vein Modding Tool
Code Vein modding helper tool for managing UE4 projects.

## Installation
Download at [Releases](https://github.com/AmionSky/cvmodtool/releases).

Only need to download the `.exe` then it will download the resources on first use.
<br>*(Or manually download the resources and unzip it next to the executable)*

    cvmodtool.exe update --resources

![console output preview](https://github.com/AmionSky/cvmodtool/blob/master/readmeres/cvmodtool.jpg?raw=true)

## Overview
- Create new UE4 projects with correct configuration, helper assets and build scripts.
- Build/Cook unreal project content.
- Package only the necessary files into a .pak file.
- Install/Copy the .pak into your mods folder.

## Configuration

### config.toml
The tool needs a `config.toml` next to the `cvmodtool.exe` with contents:
```toml
engine = 'Path\To\UE_4.18' # This folder should contain the "Engine" directory
moddir = 'Path\To\CodeVein\Content\Paks\~mods' # This folder needs to exist

# Optional
[profiles] # User defined profiles
my-profile = ['base']
```

### cvmod.toml
Mod configuration file. Automatically created by the `create` command. Make sure to update the `includes` if necessary.
```toml
project = 'TestProject' # Name of the Unreal project
pakname = 'TestProject_P' # Name of the .pak file to create
includes = ['Maps', 'ModResources'] # Folders/files to include in the final package

# Optional
packagedir = 'Package' # Directory to create the package in (default: Package)
credits = ['Amion'] # Credits of the included modules
```

## Profiles
Profiles used for project creation. The profiles can be found in `resources\profiles.toml`.
|Name|Description|
|-|-|
|default|Includes the most used modules for custom maps.|
|full|Includes all the modules.|
|min|Minimal profile. Only includes the `base` module.|
|empty|Doesn't include any module. *(Not even `base`)*

## Modules
Modules are packages of project content that you can include on `create`. The modules can be found in `resources\modules`. Modules depend on each other. Make sure to include all the required dependency of all modules.
|Name|Description|
|-|-|
|`base`|The base of the Unreal project. Should always include.|
|`extensions`|C++ code. For tighter integration with Code Vein.|
|`gitsupport`|`gitignore` and `gitattributes` file.|
|`spawner`|Contains the ActorProxySpawner.|
|`interactive`|Contains the Interactive Object Base.|
|`enemies`|Dummy actors of all the enemies in the game.|
|`startmap`|Starting `work` map. *(Removed reflection capture)*|
|`workmapdoor`|Spawns an actor in the base that you can access to enter the `work` map.|
|`stateful`|Contains mistle and enemy spawner that you can use to emulate the main gamemode. Works even with unloading sub levels. Requires the `LevelState` actor in the persistent level.|

## Commands
*For more information on all the commands just use the `--help` option argument.*

### **Create**
    cvmodtool.exe create [OPTIONS] <ProjectName>
Creates a new Unreal project in the current directory inside a folder with an identical name of the project name. Automatically creates mod config (`cvmod.toml`) and a `build-and-install.bat` for quick iteration.
|Option|Usage|Description|
|-|-|-|
|profile|`-p <ProfileName>`<br>`--profile <ProfileName>`|Specify the profile to use for selecting the modules for install. Profiles are defined at `resources/profiles.toml`.<br>[default: default]|
|modules|`-m <ModuleNames...>`<br>`--modules <ModuleNames...>`|Additional modules to install.|

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

If you included C++ code in your project, to make the `build` command work properly, you need to build the Visual Studio project first. Unreal automatically does that if you open the project.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]|
---

### **Package**
    cvmodtool.exe package [OPTIONS]
Packages the project into a .pak file based on configuration found inside the mod config (`cvmod.toml`). Requires the mod config in the project directory.

Make sure to update the mod config's `includes` field with the content folders to include in the pak.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]|
|no-copy|`--no-copy`|Don't copy the latest cooked content. Only run UnrealPak.|
|no-compress|`--no-compress`|Don't compress the .pak file.|
---

### **Install**
    cvmodtool.exe install [OPTIONS] <optional-pakfile>
Copies the .pak file to the specified mods folder (Usually it's the Code Vein ~mods folder). Requires the mod config (`cvmod.toml`) in the project directory.

If the `pakfile` is defined the mod config won't be used, instead it will just copy that pak file into the mods folder.
|Option|Usage|Description|
|-|-|-|
|config|`-c <ModConfig>`<br>`--config <ModConfig>`|Specify the mod configuration file to use.<br>[default: cvmod.toml]|
---

### **Update**
    cvmodtool.exe update [OPTIONS]
Updates the executable and the resources. Resource update will delete the resources directory!

Not specifying any options will update both the executable and the resources to the latest release on GitHub. If both options are specified it is the same as calling it without any.
|Option|Usage|Description|
|-|-|-|
|executable|`-e`<br>`--executable`|Only update the executable|
|resources|`-r`<br>`--resources`|Only update the resources|
---
