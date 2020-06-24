// Fill out your copyright notice in the Description page of Project Settings.

using UnrealBuildTool;
using System.Collections.Generic;

public class PROJECTNAMETarget : TargetRules
{
    public PROJECTNAMETarget(TargetInfo Target) : base(Target)
    {
        Type = TargetType.Game;

        if (bBuildEditor)
        {
            ExtraModuleNames.AddRange(new string[] { "Extensions" });
        }
    }
}
