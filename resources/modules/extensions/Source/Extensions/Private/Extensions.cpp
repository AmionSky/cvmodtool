#include "Extensions.h"
#include "Modules/ModuleManager.h"
#include "Modules/ModuleInterface.h"

#define LOCTEXT_NAMESPACE "Extensions"

IMPLEMENT_GAME_MODULE(FExtensionsModule, Extensions);

DEFINE_LOG_CATEGORY(Extensions)

void FExtensionsModule::StartupModule()
{
	UE_LOG(Extensions, Display, TEXT("Code Vein Extensions module dummy loaded"));
}

void FExtensionsModule::ShutdownModule()
{
	UE_LOG(Extensions, Display, TEXT("Code Vein Extensions module dummy unloaded"));
}

FText UCharacterHelper::GetCharacterName(UObject *target)
{
	FText ReturnValue;
	return ReturnValue;
}

FString UCharacterHelper::GetMultiplayCodeName(UObject *target)
{
	FString ReturnValue;
	return ReturnValue;
}

FText UCharacterHelper::GetCodeName(UObject *target)
{
	FText ReturnValue;
	return ReturnValue;
}

void UCharacterHelper::AddExp(UObject *target, int32 Value){}
void UCharacterHelper::BuddySpawn(UObject *target){}
void UCharacterHelper::ResetBuddy(UObject *target) {}
void UCharacterHelper::ResetRegenerationUsageCount(UObject *target) {}

bool UCharacterHelper::GainItem(int32 ItemCount, UObject *ItemArticle, UObject *target)
{
	return true;
}

bool UCharacterHelper::TryExecuteDeath(UObject *target, bool IsForce, bool IsSkipDeathAnimation, ECharacterDownType DownType)
{
	return true;
}

void UCharacterHelper::SetHealth(UObject *target, float Value) {}
void UCharacterHelper::SetBuddy(UObject *target, ECharacterActionNPCType Type) {}
ECharacterActionNPCType UCharacterHelper::GetBuddy(UObject *target) { return ECharacterActionNPCType::Yakumo; };
UObject* UCharacterHelper::GetBuddyInstance(UObject *target, ECharacterActionNPCType BuddyType) { return target; }
void UCharacterHelper::SetTeamType(UObject *target, ECharacterTeamType Value) {}
void UCharacterHelper::SetEnemyType(UObject *target, EN_EnemyType EnemyType) {}
UObject* UCharacterHelper::GetHostActor(UObject *WorldContextObject) { return WorldContextObject; }


void UDebugMenuComponent::OpenDebugMenu() {}

void UFieldMetaInfoAsset::Using(UObject *WorldContextObject){}

void AFieldTriggerItemBase::SetItemArticle(UObject *InItemArticle) {}

void UBuddyComponent::OnBuddyDestroyed(UObject *DestroyedActor) {}

bool AAppLevelStreamingVolume::IsAllForceDisabled(UObject *WorldContextObject) { return true; }
bool AAppLevelStreamingVolume::IsAutoEnableFromFirstSpawnToQuitGame() { return true; }
bool AAppLevelStreamingVolume::IsEitherLevelShouldBeLoaded() { return true; }
void AAppLevelStreamingVolume::SetForceDisable(bool bIsDisable) {}
void AAppLevelStreamingVolume::SetForceDisableAll(bool bIsDisable, UObject *WorldContextObject) {}

void AAppPlayerState::GetOnlineDisplayName(bool& ReturnValue, FString& OutOnlineDisplayName) {
	OutOnlineDisplayName = "";
	ReturnValue = true;

	return;
}

void AItemFunctionBase::Kill() { return; }
void AItemFunctionBase::UseItem(int32 UseCount) { return; }


#undef LOCTEXT_NAMESPACE
