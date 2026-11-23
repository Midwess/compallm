import type { GroupUpdateInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsGroupGroupUpdate",
    "kind": "HoudiniMutation",
    "hash": "cf78845ed36874c52920989f768dec34449962c95298b5759c822a6c04afed81",
    "raw": `mutation SettingsGroupGroupUpdate($groupUpdateId: ID!, $groupUpdateInput: GroupUpdateInput!, $includeGroupUpdateNode: Boolean! = false, $includeGroupUpdateNodeMemberIds: Boolean! = false) {
    groupUpdate(id: $groupUpdateId, input: $groupUpdateInput) {
        node {
            id
            name
            description
            memberIds @include(if: $includeGroupUpdateNodeMemberIds)
            roles @include(if: $includeGroupUpdateNode) {
                edges {
                    node {
                        id
                        name
                        description
                        color
                        __typename
                    }
                    __typename
                }
                __typename
            }
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

                                    "name": {
                                        "type": "String",
                                        "keyRaw": "name",
                                        "visible": true,
                                    },

                                    "description": {
                                        "type": "String",
                                        "keyRaw": "description",
                                        "visible": true,
                                    },

                                    "memberIds": {
                                        "type": "ID",
                                        "keyRaw": "memberIds",

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeGroupUpdateNodeMemberIds",
                                                    }
                                                }
                                            }
                                        }],

                                        "visible": true,
                                    },

                                    "roles": {
                                        "type": "RoleConnection",
                                        "keyRaw": "roles",

                                        "directives": [{
                                            "name": "include",
                                            "arguments": {
                                                "if": {
                                                    "kind": "Variable",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "includeGroupUpdateNode",
                                                    }
                                                }
                                            }
                                        }],


                                        "selection": {
                                            "fields": {
                                                "edges": {
                                                    "type": "RoleEdge",
                                                    "keyRaw": "edges",

                                                    "selection": {
                                                        "fields": {
                                                            "node": {
                                                                "type": "Role",
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

                                                                        "description": {
                                                                            "type": "String",
                                                                            "keyRaw": "description",
                                                                            "visible": true,
                                                                        },

                                                                        "color": {
                                                                            "type": "String",
                                                                            "keyRaw": "color",
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
            "includeGroupUpdateNodeMemberIds": "Boolean",
            "includeGroupUpdateNode": "Boolean",
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

        "defaults": {
            "includeGroupUpdateNodeMemberIds": false,
            "includeGroupUpdateNode": false,
        },

        "runtimeScalars": {},
    },

} as const

export default artifact

export type SettingsGroupGroupUpdate = {
	readonly "input": SettingsGroupGroupUpdate$input;
	readonly "result": SettingsGroupGroupUpdate$result;
};

export type SettingsGroupGroupUpdate$result = {
	readonly groupUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly memberIds: (string)[];
			readonly roles: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly description: string;
						readonly color: string;
					};
				})[];
			};
		};
	};
};

export type SettingsGroupGroupUpdate$input = {
	groupUpdateId: string;
	groupUpdateInput: GroupUpdateInput;
	includeGroupUpdateNodeMemberIds: boolean;
	includeGroupUpdateNode: boolean;
};

export type SettingsGroupGroupUpdate$optimistic = {
	readonly groupUpdate?: {
		readonly node?: {
			readonly id?: string;
			readonly name?: string;
			readonly description?: string;
			readonly memberIds?: (string)[];
			readonly roles?: {
				readonly edges?: ({
					readonly node?: {
						readonly id?: string;
						readonly name?: string;
						readonly description?: string;
						readonly color?: string;
					};
				})[];
			};
		};
	};
};

export type SettingsGroupGroupUpdate$unmasked = {
	readonly groupUpdate: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly memberIds: (string)[];
			readonly roles: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly description: string;
						readonly color: string;
						readonly __typename: "Role";
					};
					readonly __typename: "RoleEdge";
				})[];
				readonly __typename: "RoleConnection";
			};
			readonly __typename: "Group";
		};
		readonly __typename: "GroupPayload";
	};
};

export type SettingsGroupGroupUpdate$artifact = typeof artifact

"HoudiniHash=cf78845ed36874c52920989f768dec34449962c95298b5759c822a6c04afed81"
