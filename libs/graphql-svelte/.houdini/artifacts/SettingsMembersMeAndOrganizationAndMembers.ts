import type { MemberFilterInput, MemberOrderInput } from "$houdini/graphql/inputs";
const artifact = {
    "name": "SettingsMembersMeAndOrganizationAndMembers",
    "kind": "HoudiniQuery",
    "hash": "d8227f4bf85d079bef874219fa5881c50f40d9c15c9c62a7e092db6ed3ea112a",
    "raw": `query SettingsMembersMeAndOrganizationAndMembers($includeMembersEdgesNode: Boolean! = false, $includeMembersEdgesNode2: Boolean! = false, $includeMembersEdgesNode3: Boolean! = false, $membersAfter: String, $membersFilter: MemberFilterInput, $membersOrderBy: [MemberOrderInput!]) {
    me {
        node {
            id
            organizationOwner
            __typename
        }
        __typename
    }
    organization {
        node {
            ownerMemberIds
            __typename
            id
        }
        __typename
    }
    members(first: 20, after: $membersAfter, filter: $membersFilter, currentUserFirst: true, orderBy: $membersOrderBy) {
        edges {
            node {
                id
                displayName
                email
                available
                joinedAt
                role @include(if: $includeMembersEdgesNode) {
                    id
                    name
                    description
                    color
                    owner
                    __typename
                }
                groups @include(if: $includeMembersEdgesNode2) {
                    id
                    name
                    description
                    __typename
                }
                effectiveRoleIds @include(if: $includeMembersEdgesNode3)
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
            "me": {
                "type": "MePayload",
                "keyRaw": "me",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Me",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
                                        "visible": true,
                                    },

                                    "organizationOwner": {
                                        "type": "Boolean",
                                        "keyRaw": "organizationOwner",
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

            "organization": {
                "type": "OrganizationPayload",
                "keyRaw": "organization",

                "selection": {
                    "fields": {
                        "node": {
                            "type": "Organization",
                            "keyRaw": "node",

                            "selection": {
                                "fields": {
                                    "ownerMemberIds": {
                                        "type": "ID",
                                        "keyRaw": "ownerMemberIds",
                                        "visible": true,
                                    },

                                    "__typename": {
                                        "type": "String",
                                        "keyRaw": "__typename",
                                    },

                                    "id": {
                                        "type": "ID",
                                        "keyRaw": "id",
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

            "members": {
                "type": "MemberConnection",
                "keyRaw": "members(first: 20, after: $membersAfter, filter: $membersFilter, currentUserFirst: true, orderBy: $membersOrderBy)",

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

                                                "joinedAt": {
                                                    "type": "Float",
                                                    "keyRaw": "joinedAt",
                                                    "visible": true,
                                                },

                                                "role": {
                                                    "type": "Role",
                                                    "keyRaw": "role",

                                                    "directives": [{
                                                        "name": "include",
                                                        "arguments": {
                                                            "if": {
                                                                "kind": "Variable",
                                                                "name": {
                                                                    "kind": "Name",
                                                                    "value": "includeMembersEdgesNode",
                                                                }
                                                            }
                                                        }
                                                    }],


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

                                                            "owner": {
                                                                "type": "Boolean",
                                                                "keyRaw": "owner",
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

                                                "groups": {
                                                    "type": "Group",
                                                    "keyRaw": "groups",

                                                    "directives": [{
                                                        "name": "include",
                                                        "arguments": {
                                                            "if": {
                                                                "kind": "Variable",
                                                                "name": {
                                                                    "kind": "Name",
                                                                    "value": "includeMembersEdgesNode2",
                                                                }
                                                            }
                                                        }
                                                    }],


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

                                                            "__typename": {
                                                                "type": "String",
                                                                "keyRaw": "__typename",
                                                            },
                                                        },
                                                    },

                                                    "visible": true,
                                                },

                                                "effectiveRoleIds": {
                                                    "type": "ID",
                                                    "keyRaw": "effectiveRoleIds",

                                                    "directives": [{
                                                        "name": "include",
                                                        "arguments": {
                                                            "if": {
                                                                "kind": "Variable",
                                                                "name": {
                                                                    "kind": "Name",
                                                                    "value": "includeMembersEdgesNode3",
                                                                }
                                                            }
                                                        }
                                                    }],

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
            "membersOrderBy": "MemberOrderInput",
            "includeMembersEdgesNode": "Boolean",
            "includeMembersEdgesNode2": "Boolean",
            "includeMembersEdgesNode3": "Boolean",
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
            "MemberOrderInput": {
                "displayName": "OrderByDirection",
                "email": "OrderByDirection",
                "id": "OrderByDirection",
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

        "defaults": {
            "includeMembersEdgesNode": false,
            "includeMembersEdgesNode2": false,
            "includeMembersEdgesNode3": false,
        },

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsMembersMeAndOrganizationAndMembers = {
	readonly "input": SettingsMembersMeAndOrganizationAndMembers$input;
	readonly "result": SettingsMembersMeAndOrganizationAndMembers$result | undefined;
};

export type SettingsMembersMeAndOrganizationAndMembers$result = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationOwner: boolean;
		};
	};
	/**
	 * Return the active organization from verified user authority.
	 */
	readonly organization: {
		readonly node: {
			readonly ownerMemberIds: (string)[];
		};
	};
	readonly members: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly displayName: string;
				readonly email: string;
				readonly available: boolean;
				readonly joinedAt: number;
				readonly role: {
					readonly id: string;
					readonly name: string;
					readonly description: string;
					readonly color: string;
					readonly owner: boolean;
				};
				readonly groups: ({
					readonly id: string;
					readonly name: string;
					readonly description: string;
				})[];
				/**
				 * Direct and group-inherited role identities in the verified organization.
				 */
				readonly effectiveRoleIds: (string)[];
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type SettingsMembersMeAndOrganizationAndMembers$input = {
	membersAfter?: string | null;
	membersFilter?: MemberFilterInput | null;
	membersOrderBy?: (MemberOrderInput)[] | null;
	includeMembersEdgesNode: boolean;
	includeMembersEdgesNode2: boolean;
	includeMembersEdgesNode3: boolean;
};

export type SettingsMembersMeAndOrganizationAndMembers$unmasked = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationOwner: boolean;
			readonly __typename: "Me";
		};
		readonly __typename: "MePayload";
	};
	/**
	 * Return the active organization from verified user authority.
	 */
	readonly organization: {
		readonly node: {
			readonly ownerMemberIds: (string)[];
			readonly __typename: "Organization";
			readonly id: string;
		};
		readonly __typename: "OrganizationPayload";
	};
	readonly members: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly displayName: string;
				readonly email: string;
				readonly available: boolean;
				readonly joinedAt: number;
				readonly role: {
					readonly id: string;
					readonly name: string;
					readonly description: string;
					readonly color: string;
					readonly owner: boolean;
					readonly __typename: "Role";
				};
				readonly groups: ({
					readonly id: string;
					readonly name: string;
					readonly description: string;
					readonly __typename: "Group";
				})[];
				/**
				 * Direct and group-inherited role identities in the verified organization.
				 */
				readonly effectiveRoleIds: (string)[];
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

export type SettingsMembersMeAndOrganizationAndMembers$artifact = typeof artifact

"HoudiniHash=d8227f4bf85d079bef874219fa5881c50f40d9c15c9c62a7e092db6ed3ea112a"
