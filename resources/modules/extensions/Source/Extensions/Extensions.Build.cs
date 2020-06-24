// Copyright 1998-2017 Epic Games, Inc. All Rights Reserved.
using UnrealBuildTool;

public class Extensions : ModuleRules
{
    public Extensions(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        PublicDependencyModuleNames.AddRange(
            new string[] {
                    "Core",
                    "CoreUObject",
                    "Engine",
            }
        );

        PrivateDependencyModuleNames.AddRange(
            new string[] {
                    "Slate",
                    "SlateCore",
                    "UMG",
                    "AIModule"
            }
        );

        PublicIncludePaths.AddRange(
            new string[]
            {
                    "Runtime/Extensions/Public",
            }
        );

        PrivateIncludePaths.AddRange(
            new string[] {
                    "Runtime/Extensions/Private",
            }
        );
    }
}