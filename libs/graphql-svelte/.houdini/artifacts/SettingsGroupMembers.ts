import type { MemberFilterInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsGroupMembers",
    "kind": "HoudiniQuery",
    "hash": "19a98483ca9779d9dffaf8e3b0c79c42458b599d806f307f93552f5545b46a9e",
    "raw": `query SettingsGroupMembers($membersAfter: String, $membersFilter: MemberFilterInput) {
    members(first: 30, after: $membersAfter, filter: $membersFilter) {
        edges {
            node {
                id
                displayName
                email
                available
                effectiveRoleIds
                groupIds
                __typename
            }
            __typename
        }
        pageInfo {
            endCursor
            hasNextPage
            __typename
        }
        __typename
    }
}
`,

    "rootType": "Query",
    "stripVariables": [] as Array<string>,

    "selection": {
        "fields": {
            "members": {
                "type": "MemberConnection",
                "keyRaw": "members(first: 30, after: $membersAfter, filter: $membersFilter)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "MemberEdge",
                            "keyRaw": "edges",

                            "selection": {
                                "fields": {
                                    "node": {
                                        "type": "Member",
                                        "keyRaw": "node",

                                        "selection": {
                                            "fields": {
                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
                                                    "visible": true,
                                                },

                                                "displayName": {
                                                    "type": "String",
                                                    "keyRaw": "displayName",
                                                    "visible": true,
                                                },

                                                "email": {
                                                    "type": "String",
                                                    "keyRaw": "email",
                                                    "visible": true,
                                                },

                                                "available": {
                                                    "type": "Boolean",
                                                    "keyRaw": "available",
                                                    "visible": true,
                                                },

                                                "effectiveRoleIds": {
                                                    "type": "ID",
                                                    "keyRaw": "effectiveRoleIds",
                                                    "visible": true,
                                                },

                                                "groupIds": {
                                                    "type": "ID",
                                                    "keyRaw": "groupIds",
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

                        "pageInfo": {
                            "type": "PageInfo",
                            "keyRaw": "pageInfo",

                            "selection": {
                                "fields": {
                                    "endCursor": {
                                        "type": "String",
                                        "keyRaw": "endCursor",
                                        "nullable": true,
                                        "visible": true,
                                    },

                                    "hasNextPage": {
                                        "type": "Boolean",
                                        "keyRaw": "hasNextPage",
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
            "membersAfter": "String",
            "membersFilter": "MemberFilterInput",
        },

        "types": {
            "IDFilter": {
                "eq": "ID",
            },
            "MemberFilterInput": {
                "displayName": "StringFilter",
                "email": "StringFilter",
                "or": "MemberMatchInput",
                "roleId": "IDFilter",
            },
            "MemberMatchInput": {
                "displayName": "StringFilter",
                "email": "StringFilter",
            },
            "StringFilter": {
                "eq": "String",
                "gt": "String",
                "gte": "String",
                "ilike": "String",
                "in": "String",
                "iregex": "String",
                "is": "FilterIs",
                "like": "String",
                "lt": "String",
                "lte": "String",
                "neq": "String",
                "regex": "String",
                "startsWith": "String",
            },
        },

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsGroupMembers = {
	readonly "input": SettingsGroupMembers$input;
	readonly "result": SettingsGroupMembers$result | undefined;
};

export type SettingsGroupMembers$result = {
	readonly members: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly displayName: string;
				readonly email: string;
				readonly available: boolean;
				/**
				 * Direct and group-inherited role identities in the verified organization.
				 */
				readonly effectiveRoleIds: (string)[];
				/**
				 * Group identities in the verified organization; no policy source is exposed.
				 */
				readonly groupIds: (string)[];
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type SettingsGroupMembers$input = {
	membersAfter?: string | null;
	membersFilter?: MemberFilterInput | null;
};

export type SettingsGroupMembers$unmasked = {
	readonly members: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly displayName: string;
				readonly email: string;
				readonly available: boolean;
				/**
				 * Direct and group-inherited role identities in the verified organization.
				 */
				readonly effectiveRoleIds: (string)[];
				/**
				 * Group identities in the verified organization; no policy source is exposed.
				 */
				readonly groupIds: (string)[];
				readonly __typename: "Member";
			};
			readonly __typename: "MemberEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "MemberConnection";
	};
};

export type SettingsGroupMembers$artifact = typeof artifact

"HoudiniHash=19a98483ca9779d9dffaf8e3b0c79c42458b599d806f307f93552f5545b46a9e"
