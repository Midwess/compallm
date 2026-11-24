const artifact = {
    "name": "SettingsGroupGroupAndRolesAndMembers",
    "kind": "HoudiniQuery",
    "hash": "d80cbc98ab939c6ea726184351e980e5d0721b20c7fa44d9200f030aac095008",
    "raw": `query SettingsGroupGroupAndRolesAndMembers($groupId: ID!, $includeGroupNode: Boolean! = false, $includeGroupNodeMemberIds: Boolean! = false) {
    group(id: $groupId) {
        node {
            id
            name
            description
            roles @include(if: $includeGroupNode) {
                edges {
                    node {
                        id
                        name
                        description
                        color
                        owner
                        __typename
                    }
                    __typename
                }
                __typename
            }
            memberIds @include(if: $includeGroupNodeMemberIds)
            __typename
        }
        __typename
    }
    roles @include(if: $includeGroupNode) {
        edges {
            node {
                id
                name
                description
                color
                owner
                __typename
            }
            __typename
        }
        __typename
    }
    members(first: 30) @include(if: $includeGroupNodeMemberIds) {
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
            "group": {
                "type": "GroupPayload",
                "keyRaw": "group(id: $groupId)",
                "nullable": true,

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
                                                        "value": "includeGroupNode",
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
                                                        "value": "includeGroupNodeMemberIds",
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
                                "value": "includeGroupNode",
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

            "members": {
                "type": "MemberConnection",
                "keyRaw": "members(first: 30)",

                "directives": [{
                    "name": "include",
                    "arguments": {
                        "if": {
                            "kind": "Variable",
                            "name": {
                                "kind": "Name",
                                "value": "includeGroupNodeMemberIds",
                            }
                        }
                    }
                }],


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
            "groupId": "ID",
            "includeGroupNode": "Boolean",
            "includeGroupNodeMemberIds": "Boolean",
        },

        "types": {},

        "defaults": {
            "includeGroupNode": false,
            "includeGroupNodeMemberIds": false,
        },

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsGroupGroupAndRolesAndMembers = {
	readonly "input": SettingsGroupGroupAndRolesAndMembers$input;
	readonly "result": SettingsGroupGroupAndRolesAndMembers$result | undefined;
};

export type SettingsGroupGroupAndRolesAndMembers$result = {
	/**
	 * Find visible group metadata in the active organization.
	 */
	readonly group: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly roles: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly description: string;
						readonly color: string;
						readonly owner: boolean;
					};
				})[];
			};
			readonly memberIds: (string)[];
		};
	} | null;
	/**
	 * Return the complete visible role metadata collection, ordered by ID.
	 */
	readonly roles: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly description: string;
				readonly color: string;
				readonly owner: boolean;
			};
		})[];
	};
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

export type SettingsGroupGroupAndRolesAndMembers$input = {
	groupId: string;
	includeGroupNode: boolean;
	includeGroupNodeMemberIds: boolean;
};

export type SettingsGroupGroupAndRolesAndMembers$unmasked = {
	/**
	 * Find visible group metadata in the active organization.
	 */
	readonly group: {
		readonly node: {
			readonly id: string;
			readonly name: string;
			readonly description: string;
			readonly roles: {
				readonly edges: ({
					readonly node: {
						readonly id: string;
						readonly name: string;
						readonly description: string;
						readonly color: string;
						readonly owner: boolean;
						readonly __typename: "Role";
					};
					readonly __typename: "RoleEdge";
				})[];
				readonly __typename: "RoleConnection";
			};
			readonly memberIds: (string)[];
			readonly __typename: "Group";
		};
		readonly __typename: "GroupPayload";
	} | null;
	/**
	 * Return the complete visible role metadata collection, ordered by ID.
	 */
	readonly roles: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly description: string;
				readonly color: string;
				readonly owner: boolean;
				readonly __typename: "Role";
			};
			readonly __typename: "RoleEdge";
		})[];
		readonly __typename: "RoleConnection";
	};
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

export type SettingsGroupGroupAndRolesAndMembers$artifact = typeof artifact

"HoudiniHash=d80cbc98ab939c6ea726184351e980e5d0721b20c7fa44d9200f030aac095008"
