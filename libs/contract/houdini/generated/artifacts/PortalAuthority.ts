const artifact = {
    "name": "PortalAuthority",
    "kind": "HoudiniQuery",
    "hash": "9b079f98fca4eff331a19a041753f1189ee36767f76703021c209ff42e61e66d",
    "raw": `query PortalAuthority {
    me {
        node {
            id
            organizationId
            organizationPermissions {
                action
                allowed
                __typename
            }
            cedarSource
            cedarEntities
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

                                    "organizationId": {
                                        "type": "ID",
                                        "keyRaw": "organizationId",
                                        "visible": true,
                                    },

                                    "organizationPermissions": {
                                        "type": "OrganizationPermission",
                                        "keyRaw": "organizationPermissions",

                                        "selection": {
                                            "fields": {
                                                "action": {
                                                    "type": "String",
                                                    "keyRaw": "action",
                                                    "visible": true,
                                                },

                                                "allowed": {
                                                    "type": "Boolean",
                                                    "keyRaw": "allowed",
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

                                    "cedarSource": {
                                        "type": "String",
                                        "keyRaw": "cedarSource",
                                        "visible": true,
                                    },

                                    "cedarEntities": {
                                        "type": "String",
                                        "keyRaw": "cedarEntities",
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
    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type PortalAuthority = {
	readonly "input"?: PortalAuthority$input;
	readonly "result": PortalAuthority$result | undefined;
};

export type PortalAuthority$result = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationId: string;
			readonly organizationPermissions: ({
				readonly action: string;
				readonly allowed: boolean;
			})[];
			/**
			 * Effective policies for this verified user only. UI decisions do not grant server authority.
			 */
			readonly cedarSource: string;
			/**
			 * Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials.
			 */
			readonly cedarEntities: string;
		};
	};
};

export type PortalAuthority$input = null | undefined;

export type PortalAuthority$unmasked = {
	readonly me: {
		readonly node: {
			readonly id: string;
			readonly organizationId: string;
			readonly organizationPermissions: ({
				readonly action: string;
				readonly allowed: boolean;
				readonly __typename: "OrganizationPermission";
			})[];
			/**
			 * Effective policies for this verified user only. UI decisions do not grant server authority.
			 */
			readonly cedarSource: string;
			/**
			 * Cedar JSON entity facts for the verified principal and its role/group ancestry; contains no credentials.
			 */
			readonly cedarEntities: string;
			readonly __typename: "Me";
		};
		readonly __typename: "MePayload";
	};
};

export type PortalAuthority$artifact = typeof artifact

"HoudiniHash=9b079f98fca4eff331a19a041753f1189ee36767f76703021c209ff42e61e66d"
