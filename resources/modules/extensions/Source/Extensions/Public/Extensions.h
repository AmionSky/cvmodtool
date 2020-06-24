#pragma once

#include "Engine.h"
#include "CoreMinimal.h"
#include "UMG.h"
#include "AIModule.h"
#include "Modules/ModuleInterface.h"
#include "Modules/ModuleManager.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "Extensions.generated.h"

DECLARE_LOG_CATEGORY_EXTERN(Extensions, All, All)

class FExtensionsModule: public IModuleInterface
{
public:
	virtual void StartupModule() override;
	virtual void ShutdownModule() override;

};

UENUM(BlueprintType)
enum class ECharacterDownType : uint8
{
	None 	UMETA(DisplayName = "None"),
	Prone 	UMETA(DisplayName = "Prone"),
	Supine	UMETA(DisplayName = "Supine"),
	Stun	UMETA(DisplayName = "Stun"),
	Custom	UMETA(DisplayName = "Custom"),
	ECharacterDownType_MAX
};

UENUM(BlueprintType)
enum class ECharacterActionNPCType : uint8
{
	Louis 			UMETA(DisplayName = "Louis"),
	Yakumo 			UMETA(DisplayName = "Yakumo"),
	Io				UMETA(DisplayName = "Io"),
	Mia				UMETA(DisplayName = "Mia"),
	Jack			UMETA(DisplayName = "Jack"),
	Eva				UMETA(DisplayName = "Eva"),
	Oliver			UMETA(DisplayName = "Oliver"),
	MiaAngel		UMETA(DisplayName = "MiaAngel"),
	JackPast		UMETA(DisplayName = "JackPast"),
	LouisAnother	UMETA(DisplayName = "LouisAnother"),
	LouisLost		UMETA(DisplayName = "LouisLost"),
	YakumoAnother	UMETA(DisplayName = "YakumoAnother"),
	YakumoLost		UMETA(DisplayName = "YakumoLost"),
	IoAnother		UMETA(DisplayName = "IoAnother"),
	IoLost			UMETA(DisplayName = "IoLost"),
	MiaAnother		UMETA(DisplayName = "MiaAnother"),
	MiaLost			UMETA(DisplayName = "MiaLost"),
	JackAnother		UMETA(DisplayName = "JackAnother"),
	JackLost		UMETA(DisplayName = "JackLost"),
	EvaAnother		UMETA(DisplayName = "EvaAnother"),
	EvaLost			UMETA(DisplayName = "EvaLost"),
	ECharacterActionNPCType_MAX
};

UENUM(BlueprintType)
enum class EN_EnemyType : uint8
{
	Normal 	UMETA(DisplayName = "Normal"),
	InvaderWave 	UMETA(DisplayName = "InvaderWave"),
	EN_MAX
};

UENUM(BlueprintType)
enum class ECharacterTeamType : uint8
{
	Friend 	UMETA(DisplayName = "Friend"),
	Enemy 	UMETA(DisplayName = "Enemy"),
	ECharacterTeamType_MAX
};

UENUM(BlueprintType)
enum class ECharacterType : uint8
{
	Player 	UMETA(DisplayName = "Player"),
	NPC 	UMETA(DisplayName = "NPC"),
	Enemy	UMETA(DisplayName = "Enemy"),
	Deku	UMETA(DisplayName = "Deku"),
	AvatarCustomize	UMETA(DisplayName = "AvatarCustomize"),
	MAX
};

UENUM(BlueprintType)
enum class EArticleCategory : uint8
{
	Item_Expendables UMETA(DisplayName = "Item_Expendables"),
	Item_Important 	UMETA(DisplayName = "Item_Important"),
	Item_BloodCrystal 	UMETA(DisplayName = "Item_BloodCrystal"),
	Item_Emotion 	UMETA(DisplayName = "Item_Emotion"),
	Item_BuildUP	UMETA(DisplayName = "Item_BuildUP"),
	Item_GameSystem	UMETA(DisplayName = "Item_GameSystem"),
	Item_Other	UMETA(DisplayName = "Item_Other"),
	Weapon	UMETA(DisplayName = "Weapon"),
	Weapon_Sword	UMETA(DisplayName = "Weapon_Sword"),
	Weapon_LargeSword	UMETA(DisplayName = "Weapon_LargeSword"),
	Weapon_Bayonet	UMETA(DisplayName = "Weapon_Bayonet"),
	Weapon_Halberd	UMETA(DisplayName = "Weapon_Halberd"),
	Weapon_Hammer	UMETA(DisplayName = "Weapon_Hammer"),
	Jinkaku	UMETA(DisplayName = "Jinkaku"),
	Jinkaku_Gauntlet	UMETA(DisplayName = "Jinkaku_Gauntlet"),
	Jinkaku_LongCoat	UMETA(DisplayName = "Jinkaku_LongCoat"),
	Jinkaku_Drape	UMETA(DisplayName = "Jinkaku_Drape"),
	Jinkaku_Muffler	UMETA(DisplayName = "Jinkaku_Muffler"),
	Ketsugi	UMETA(DisplayName = "Ketsugi"),
	BloodCode	UMETA(DisplayName = "BloodCode"),
	MAX
};


USTRUCT(BlueprintType)
struct EXTENSIONS_API FFieldMetaInfo
{
		GENERATED_USTRUCT_BODY()

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		FName FieldAsciiName;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		TWeakObjectPtr<UObject> DataAsset;
};

USTRUCT(BlueprintType)
struct EXTENSIONS_API FItemContext
{
	GENERATED_USTRUCT_BODY()

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		UClass *ItemFunction;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		UObject *UseAnimation;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		int32 SortOrder;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		int32 MaxInventoryNumber;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bIgnoreSaving;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bIgnoreStatusAilmentStun;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bLocalOnlyEffect;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bAllowUseInHideout;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bAllowUseByGuestOnly;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bAllowUseInMultiplay;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bAllowMultipleUseAtOnce;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bSendableToStorage;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bRegeneration;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bExcludeEmpty;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		bool bIgnoreCosume;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		int32 SellingPrice;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		int32 BuyingPrice;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		FText BriefDescription;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		FName Subcategory;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		EArticleCategory Category;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		UObject *Icon;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		FText HelpText;

		UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Struct")
		FText Name;
};

USTRUCT(BlueprintType)
struct EXTENSIONS_API FFieldMetaInfoList
{
	GENERATED_USTRUCT_BODY()

};


UCLASS(Blueprintable)
class EXTENSIONS_API UCharacterHelper : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Character Name",  ToolTip = "Retrieves character name for provided character."), Category = "Extensions|CharacterHelper")
	static FText GetCharacterName(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Character Codename", ToolTip = "Retrieves character codename for provided character."), Category = "Extensions|CharacterHelper")
		static FText GetCodeName(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Character MP Codename", ToolTip = "Retrieves character multiplayer codename for provided character."), Category = "Extensions|CharacterHelper")
		static FString GetMultiplayCodeName(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Add EXP", ToolTip = "Add experience to provided character."), Category = "Extensions|CharacterHelper")
		static void AddExp(UObject *target, int32 Value);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Spawn Buddy", ToolTip = "Spawn buddy for provided character."), Category = "Extensions|CharacterHelper")
		static void BuddySpawn(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Reset Buddy", ToolTip = "Reset buddy for provided character."), Category = "Extensions|CharacterHelper")
		static void ResetBuddy(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Reset Regeneration", ToolTip = "Reset Regeneration Usage Count for provided character."), Category = "Extensions|CharacterHelper")
		static void ResetRegenerationUsageCount(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Gain Item", ToolTip = "Give item to provided character."), Category = "Extensions|CharacterHelper")
		static bool GainItem(int32 ItemCount, UObject *ItemArticle, UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Try Execute Death", ToolTip = "Kill provided character."), Category = "Extensions|CharacterHelper")
		static bool TryExecuteDeath(UObject *target, bool IsForce, bool IsSkipDeathAnimation, ECharacterDownType DownType);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Set Health", ToolTip = "Set health of provided character."), Category = "Extensions|CharacterHelper")
		static void SetHealth(UObject *target, float Value);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Set Buddy", ToolTip = "Set partner of provided character."), Category = "Extensions|CharacterHelper")
		static void SetBuddy(UObject *target, ECharacterActionNPCType Type);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Get Buddy", ToolTip = "Get partner of provided character."), Category = "Extensions|CharacterHelper")
		static ECharacterActionNPCType GetBuddy(UObject *target);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Get Buddy Instance", ToolTip = "Get partner object of provided character."), Category = "Extensions|CharacterHelper")
		static UObject* GetBuddyInstance(UObject *target, ECharacterActionNPCType BuddyType);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Set Team Type", ToolTip = "Set team of provided character."), Category = "Extensions|CharacterHelper")
		static void SetTeamType(UObject *target, ECharacterTeamType Value);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Set Enemy Type", ToolTip = "Set enemy type of provided character."), Category = "Extensions|CharacterHelper")
		static void SetEnemyType(UObject *target, EN_EnemyType EnemyType);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Get Host Actor", ToolTip = "Get host player.", WorldContext = "WorldContextObject"), Category = "Extensions|CharacterHelper")
		static UObject* GetHostActor(UObject *WorldContextObject);
};

UCLASS(Blueprintable)
class EXTENSIONS_API UDebugMenuComponent : public UUserWidget
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Open Debug Menu", ToolTip = "Open debug menu."), Category = "Extensions|Debug")
		static void OpenDebugMenu();
};

UCLASS(Blueprintable, BlueprintType)
class EXTENSIONS_API UFieldMetaInfoAsset : public UObject
{
		GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Meta Info")
	FString DefinedField;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Meta Info")
		TMap<FName, FFieldMetaInfo> FieldMetaInfoList;

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Using", ToolTip = "?", WorldContext = "WorldContextObject"), Category = "Extensions|Field Meta Info Asset")
		static void Using(UObject *WorldContextObject);
};

UCLASS(Blueprintable)
class EXTENSIONS_API UBuddyComponent : public UActorComponent
{
	GENERATED_BODY()

public:

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Buddy Component")
		UObject *BuddyInstance;

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "OnBuddyDestroyed", ToolTip = "?"), Category = "Extensions|Buddy Component")
		static void OnBuddyDestroyed(UObject *DestroyedActor);
};

UCLASS(Blueprintable)
class EXTENSIONS_API UArticleBase : public UObject
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Item Article Base")
		FString ArticleID = "DA_Item_Expendables_Recovery_2_NAME";

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Item Article Base")
		bool bIsUseArticleFlag = true;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Item Article Base")
		bool bPlayerOwning = true;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Item Article Base")
		FString ArticleFlagSymbol = "IAF_Expendables_Recovery_2";
};

UCLASS(Blueprintable)
class EXTENSIONS_API UItemArticleBase : public UArticleBase
{
	GENERATED_BODY()

public:

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|Item Article Base")
	FItemContext ArticleContext;
};

UCLASS(Blueprintable)
class EXTENSIONS_API AFieldTriggerItemBase : public AActor
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Item Base")
		int32 ItemCount;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Item Base")
		int32 Grind;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Item Base")
		UObject *ItemArticle;

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "SetItemArticle", ToolTip = "Set item"), Category = "Extensions|Field Trigger Item")
		static void SetItemArticle(UObject *InItemArticle);
};

UCLASS(Blueprintable)
class EXTENSIONS_API AAppCharacter : public AActor
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "App Character")
		ECharacterType CharacterType;
};

UCLASS(Blueprintable)
class EXTENSIONS_API AAppGameMode : public AGameMode
{
	GENERATED_BODY()

public:
};

UCLASS(Blueprintable)
class EXTENSIONS_API AApp_GameState : public AGameStateBase
{
	GENERATED_BODY()

public:
};

UCLASS(Blueprintable)
class EXTENSIONS_API AAppHUD : public AHUD
{
	GENERATED_BODY()

public:
};

UCLASS(Blueprintable)
class EXTENSIONS_API UAppGameInstance : public UGameInstance
{
	GENERATED_BODY()

public:
};

UCLASS(Blueprintable)
class EXTENSIONS_API AAppLevelStreamingVolume : public ALevelStreamingVolume
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		UObject *PausedToken;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		UObject *DeferredTeleportDriver;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		UObject *RawPtrGameFieldNameTexture;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		FFieldMetaInfo FieldMetaInfo;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		TArray<USoftObjectProperty*> ExcludableLevels;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Extensions|AppLevelStreamingVolume")
		bool bAutoEnableFromFirstSpawnToQuitGame;

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "IsAllForceDisabled", ToolTip = "?", WorldContext = "WorldContextObject"), Category = "Extensions|AppLevelStreamingVolume")
		static bool IsAllForceDisabled(UObject *WorldContextObject);
	
	UFUNCTION(BlueprintCallable, meta = (DisplayName = "IsAutoEnableFromFirstSpawnToQuitGame", ToolTip = "?"), Category = "Extensions|AppLevelStreamingVolume")
		static	bool IsAutoEnableFromFirstSpawnToQuitGame();

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "IsEitherLevelShouldBeLoaded", ToolTip = "?"), Category = "Extensions|AppLevelStreamingVolume")
		static bool	IsEitherLevelShouldBeLoaded();

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "SetForceDisable", ToolTip = "?"), Category = "Extensions|AppLevelStreamingVolume")
		static void SetForceDisable(bool bIsDisable);

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "SetForceDisableAll", ToolTip = "?", WorldContext = "WorldContextObject"), Category = "Extensions|AppLevelStreamingVolume")
		static void SetForceDisableAll(bool bIsDisable, UObject *WorldContextObject);
};

UCLASS(Blueprintable)
class EXTENSIONS_API AAppPlayerState : public APlayerState
{
	GENERATED_BODY()
public:
	UFUNCTION(BlueprintCallable, meta = (DisplayName = "GetOnlineDisplayName", ToolTip = "?"), Category = "Extensions|AppPlayerState")
		static void GetOnlineDisplayName(bool& ReturnValue, FString& OutOnlineDisplayName);
};

UCLASS(Blueprintable)
class EXTENSIONS_API AItemFunctionBase : public AActor
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintImplementableEvent)
	void OnStarted();

	UFUNCTION(BlueprintImplementableEvent)
	void OnEnded();

	UFUNCTION(BlueprintImplementableEvent)
	void OnDefeatBoss();

	UFUNCTION(BlueprintImplementableEvent)
	void PostUseItem();

	UFUNCTION(BlueprintImplementableEvent)
	void OnAbort();

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "Kill", ToolTip = "?"), Category = "Extensions|ItemFunctionBase")
	static void Kill();

	UFUNCTION(BlueprintCallable, meta = (DisplayName = "UseItem", ToolTip = "?"), Category = "Extensions|ItemFunctionBase")
	static void UseItem(int32 UseCount);
};