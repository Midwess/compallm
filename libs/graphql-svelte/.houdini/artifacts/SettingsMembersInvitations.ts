const artifact = {
    "name": "SettingsMembersInvitations",
    "kind": "HoudiniQuery",
    "hash": "268ddec9f5d9ad473564eb2bb995d6244128b1a4b3460d2d97bf69f3fc093acd",
    "raw": `query SettingsMembersInvitations($invitationsAfter: String) {
    invitations(first: 20, after: $invitationsAfter) {
        edges {
            node {
                id
                email
                status
                expiresAt
                issuedAt
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
            "invitations": {
                "type": "InvitationConnection",
                "keyRaw": "invitations(first: 20, after: $invitationsAfter)",

                "selection": {
                    "fields": {
                        "edges": {
                            "type": "InvitationEdge",
                            "keyRaw": "edges",

                            "selection": {
                                "fields": {
                                    "node": {
                                        "type": "Invitation",
                                        "keyRaw": "node",

                                        "selection": {
                                            "fields": {
                                                "id": {
                                                    "type": "ID",
                                                    "keyRaw": "id",
                                                    "visible": true,
                                                },

                                                "email": {
                                                    "type": "String",
                                                    "keyRaw": "email",
                                                    "visible": true,
                                                },

                                                "status": {
                                                    "type": "String",
                                                    "keyRaw": "status",
                                                    "visible": true,
                                                },

                                                "expiresAt": {
                                                    "type": "Float",
                                                    "keyRaw": "expiresAt",
                                                    "visible": true,
                                                },

                                                "issuedAt": {
                                                    "type": "Float",
                                                    "keyRaw": "issuedAt",
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
            "invitationsAfter": "String",
        },

        "types": {},

        "defaults": {},

        "runtimeScalars": {},
    },

    "policy": "CacheOrNetwork",
    "partial": false
} as const

export default artifact

export type SettingsMembersInvitations = {
	readonly "input": SettingsMembersInvitations$input;
	readonly "result": SettingsMembersInvitations$result | undefined;
};

export type SettingsMembersInvitations$result = {
	readonly invitations: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly email: string;
				readonly status: string;
				readonly expiresAt: number;
				readonly issuedAt: number;
			};
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
		};
	};
};

export type SettingsMembersInvitations$input = {
	invitationsAfter?: string | null;
};

export type SettingsMembersInvitations$unmasked = {
	readonly invitations: {
		readonly edges: ({
			readonly node: {
				readonly id: string;
				readonly email: string;
				readonly status: string;
				readonly expiresAt: number;
				readonly issuedAt: number;
				readonly __typename: "Invitation";
			};
			readonly __typename: "InvitationEdge";
		})[];
		readonly pageInfo: {
			readonly endCursor: string | null;
			readonly hasNextPage: boolean;
			readonly __typename: "PageInfo";
		};
		readonly __typename: "InvitationConnection";
	};
};

export type SettingsMembersInvitations$artifact = typeof artifact

"HoudiniHash=268ddec9f5d9ad473564eb2bb995d6244128b1a4b3460d2d97bf69f3fc093acd"
