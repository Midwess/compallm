const artifact = {
    "name": "SettingsMeAndOrganizationAndRolesAndGroups",
    "kind": "HoudiniQuery",
    "hash": "2be9a8e750e63b7cefa9410be6db78d09a196037d6dbf78421f07625f45b61f6",
    "raw": `query SettingsMeAndOrganizationAndRolesAndGroups($includeFields: Boolean! = false, $includeFields2: Boolean! = false, $includeFields3: Boolean! = false) {
    me {
        node {
            id
            organizationOwner
            __typename
        }
        __typename
    }
    organization @include(if: $includeFields) {
        node {
            ownerMemberIds
            __typename
            id
        }
        __typename
    }
    roles @include(if: $includeFields2) {
        edges {
            node {
                id
                name
                color
                owner
                __typename
            }
            __typename
        }
        __typename
    }
    groups @include(if: $includeFields3) {
        edges {
            node {
                id
                name
                roles @include(if: $includeFields2) {
                    edges {
                        node {
                            id
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

                "directives": [{
                    "name": "include",
                    "arguments": {
                        "if": {
                            "kind": "Variable",
                            "name": {
                                "kind": "Name",
                                "value": "includeFields",
                            }
                        }
                    }
                }],


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
                                "value": "includeFields2",
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

            "groups": {
                "type": "GroupConnection",
                "keyRaw": "groups",

                "directives": [{
                    "name": "include",
                    "arguments": {
                        "if": {
                            "kind": "Variable",
                            "name": {
                                "kind": "Name",
                                "value": "includeFields3",
                            }
                        }
                    }
                }],


                "selection": {
                    "fields": {
                        "edges": {
                            "type": "GroupEdge",
                            "keyRaw": "edges",

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
                                                                    "value": "includeFields2",
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
            "includeFields": "Boolean",
            "includeFields2": "Boolean",
            "includeFields3": "Boolean",
        },

        "types": {},

        "defaults": {
            "includeFields": false,
            "includeFields2": false,
            "includeFields3": false,
        },

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsMeAndOrganizationAndRolesAndGroups = {
	readonly "input": SettingsMeAndOrganizationAndRolesAndGroups$input;
	readonly "result": SettingsMeAndOrganizationAndRolesAndGroups$result | undefined;
};

export type SettingsMeAndOrganizationAndRolesAndGroups$result = {
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
	/**
	 * Return the complete visible role metadata collection, ordered by ID.
	 */
	readonly roles: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly color: string;
				readonly owner: boolean;
			};
		})[];
	};
	/**
	 * Return the complete visible group metadata collection, ordered by ID.
	 */
	readonly groups: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly roles: {
					readonly edges: ({
						readonly node: {
							readonly id: string;
						};
					})[];
				};
			};
		})[];
	};
};

export type SettingsMeAndOrganizationAndRolesAndGroups$input = {
	includeFields: boolean;
	includeFields2: boolean;
	includeFields3: boolean;
};

export type SettingsMeAndOrganizationAndRolesAndGroups$unmasked = {
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
	/**
	 * Return the complete visible role metadata collection, ordered by ID.
	 */
	readonly roles: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly color: string;
				readonly owner: boolean;
				readonly __typename: "Role";
			};
			readonly __typename: "RoleEdge";
		})[];
		readonly __typename: "RoleConnection";
	};
	/**
	 * Return the complete visible group metadata collection, ordered by ID.
	 */
	readonly groups: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly name: string;
				readonly roles: {
					readonly edges: ({
						readonly node: {
							readonly id: string;
							readonly __typename: "Role";
						};
						readonly __typename: "RoleEdge";
					})[];
					readonly __typename: "RoleConnection";
				};
				readonly __typename: "Group";
			};
			readonly __typename: "GroupEdge";
		})[];
		readonly __typename: "GroupConnection";
	};
};

export type SettingsMeAndOrganizationAndRolesAndGroups$artifact = typeof artifact

"HoudiniHash=2be9a8e750e63b7cefa9410be6db78d09a196037d6dbf78421f07625f45b61f6"
