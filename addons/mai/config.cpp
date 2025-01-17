class CfgPatches
{
	class MAI_main
	{
		units[] = {"MAI_ModuleBuildSpawn"};
		requiredVersion = 1.0;
		requiredAddons[] = {
			"A3_Modules_F",
			"CBA_main"
		};
	};
};

#include "CfgEden.hpp"
#include "CfgFunctions.hpp"
#include "CfgEventHandlers.hpp"
#include "XEH.hpp"
#include "CfgModuleCategories.hpp"
#include "CfgBuildspawn.hpp"