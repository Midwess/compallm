import type { GroupCreateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsGroupCreate",
    "kind": "HoudiniMutation",
    "hash": "94509c832f1b906f0c1a7685cf955e8735a8797055cde5f0a8528eba8be7ebc2",
    "raw": `mutation SettingsGroupCreate($groupCreateInput: GroupCreateInput!) {
    groupCreate(input: $groupCreateInput) {
        node {
            id
            name
            __typename
        }
        __typename
    }
}
`,

    "rootType": "Mutation",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "groupCreate": {
                "type": "GroupPayload",
                "keyRaw": "groupCreate(input: $groupCreateInput)",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Group",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },
                                },
                            },

                            "visible": true,
                        },

                        "__typename": {
                            "type": "String",
                            "keyRaw": "__typename",
                        },
                    },
                },

                "visible": true,
            },
        },
    },

    "pluginData": {},

    "input": {
        "fields": {
            "groupCreateInput": "GroupCreateInput",
        },

        "types": {
            "GroupCreateInput": {
                "description": "String",
                "name": "String",
                "roleIds": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsGroupCreate = {
	readonly "input": SettingsGroupCreate$input;
	readonly "result": SettingsGroupCreate$result;
};

export type SettingsGroupCreate$result = {
	/**
	 * Create a group and its initial role assignments in the verified active organization.
	 */
	readonly groupCreate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
		};
	};
};

export type SettingsGroupCreate$input = {
	groupCreateInput: GroupCreateInput;
};

export type SettingsGroupCreate$optimistic = {
	/**
	 * Create a group and its initial role assignments in the verified active organization.
	 */
	readonly groupCreate?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
		};
	};
};

export type SettingsGroupCreate$unmasked = {
	/**
	 * Create a group and its initial role assignments in the verified active organization.
	 */
	readonly groupCreate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly __typename: "Group";
		};
		readonly __typename: "GroupPayload";
	};
};

export type SettingsGroupCreate$artifact = typeof artifact

"HoudiniHash=94509c832f1b906f0c1a7685cf955e8735a8797055cde5f0a8528eba8be7ebc2"
