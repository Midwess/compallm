import type { GroupUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsMembersGroupUpdate",
    "kind": "HoudiniMutation",
    "hash": "5d9289ab59c306b6ce72f027ca9a01942084ef183f8a970dfcdfefcdaef22ec4",
    "raw": `mutation SettingsMembersGroupUpdate($groupUpdateId: ID!, $groupUpdateInput: GroupUpdateInput!) {
    groupUpdate(id: $groupUpdateId, input: $groupUpdateInput) {
        node {
            id
            memberIds
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
            "groupUpdate": {
                "type": "GroupPayload",
                "keyRaw": "groupUpdate(id: $groupUpdateId, input: $groupUpdateInput)",

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

                                    "memberIds": {
                                        "type": "ID",
                                        "keyRaw": "memberIds",
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
            "groupUpdateId": "ID",
            "groupUpdateInput": "GroupUpdateInput",
        },

        "types": {
            "GroupUpdateInput": {
                "addMemberIds": "String",
                "addRoleIds": "String",
                "description": "String",
                "name": "String",
                "removeMemberIds": "String",
                "removeRoleIds": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsMembersGroupUpdate = {
	readonly "input": SettingsMembersGroupUpdate$input;
	readonly "result": SettingsMembersGroupUpdate$result;
};

export type SettingsMembersGroupUpdate$result = {
	readonly groupUpdate: {
		readonly node: {
			readonly id: string;
			readonly memberIds: (string)[];
		};
	};
};

export type SettingsMembersGroupUpdate$input = {
	groupUpdateId: string;
	groupUpdateInput: GroupUpdateInput;
};

export type SettingsMembersGroupUpdate$optimistic = {
	readonly groupUpdate?: {
		readonly node?: {
			readonly id?: string;
			readonly memberIds?: (string)[];
		};
	};
};

export type SettingsMembersGroupUpdate$unmasked = {
	readonly groupUpdate: {
		readonly node: {
			readonly id: string;
			readonly memberIds: (string)[];
			readonly __typename: "Group";
		};
		readonly __typename: "GroupPayload";
	};
};

export type SettingsMembersGroupUpdate$artifact = typeof artifact

"HoudiniHash=5d9289ab59c306b6ce72f027ca9a01942084ef183f8a970dfcdfefcdaef22ec4"
