// TODO: Is it worthit to chec for name? 

_thread = [] spawn {
	addMissionEventHandler ["ExtensionCallback", {
		params ["_name", "_function", "_data"];
		systemChat str [_name,_function,_data];
	}];
	addMissionEventHandler ["ExtensionCallback", {
		params ["_name", "_function", "_data"];
		if (_name == "ASMPrcv" && _function != "dbg_error") then {
			_asmp_player = missionNamespace getVariable _function;
			if (isNil "_asmp_player") then {
				_asmp_player = "B_Soldier_F" createVehicle parseSimpleArray _data;
				missionNamespace setVariable [_function, _asmp_player];
			};
			_asmp_player setPos parseSimpleArray _data;
		};
	}];
		
	sleep 1;

	res = "ASMP" callExtension ["connect", [1,2,3]];
	while {true} do {
		res = "ASMP" callExtension ["send", [str position player]];
		sleep 1;
	};
};